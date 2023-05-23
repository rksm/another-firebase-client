use super::{listener_updates::*, RdbClient};
use anyhow::Result;
use eventsource_client as es;
use futures::{Stream, TryStreamExt};
use serde_json::Value;
use std::{ops::DerefMut, sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};
use url::Url;

#[derive(serde::Deserialize)]
struct RdbEvent {
    path: String,
    data: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug)]
enum RdbControlMessage {
    Stop,
}

pub type OnChangeCallback = Box<dyn FnMut(&[String], &Value, &[u8])>;

pub struct Listener {
    pub rdb_client: RdbClient,
}

impl Listener {
    pub fn new(rdb_client: RdbClient) -> Self {
        Self { rdb_client }
    }

    pub fn run<S: AsRef<str>>(
        self,
        path: S,
    ) -> mpsc::Receiver<(Vec<String>, Arc<Mutex<ObservedValue>>)> {
        let url = format!(
            "https://{}.firebaseio.com/{}.json",
            self.rdb_client.project_id(),
            path.as_ref()
        );
        let rdb_client = self.rdb_client;

        pub fn apply_action(
            action: Action,
            value: ObservedValue,
        ) -> Result<(Vec<String>, ObservedValue)> {
            tracing::trace!("applying action {:?}", action);
            let (path, val) = match action {
                Action::Put(action) => value.apply_put(action)?,
                Action::Patch(action) => value.apply_patch(action)?,
            };
            Ok((path, val))
        }

        let (change_tx, change_rx) = mpsc::channel(5);

        tokio::spawn(async move {
            let value = Arc::new(Mutex::new(ObservedValue::new()));

            'outer: loop {
                let (tx, mut rx) = mpsc::channel(32);
                let (_tx2, rx2) = mpsc::channel(1);

                let url = Url::parse(&url)?;
                let auth = rdb_client
                    .auth
                    .get_token()
                    .await?
                    .map(|t| format!("Bearer {t}"));

                let task = start_http_connection(url, auth, tx, rx2);

                while let Some(evt) = rx.recv().await {
                    if evt.event_type == "auth_revoked" {
                        continue 'outer;
                    }

                    let payload = evt.field("data").unwrap_or_default();

                    tracing::trace!(
                        "received event {} payload: {}",
                        evt.event_type,
                        String::from_utf8_lossy(payload)
                    );

                    let action = match &evt.event_type as &str {
                        "put" => {
                            let data: RdbEvent = serde_json::from_slice(payload).unwrap();
                            Action::Put((data.path, data.data).into())
                        }
                        "patch" => {
                            let data: RdbEvent = serde_json::from_slice(payload).unwrap();
                            Action::Patch((data.path, data.data).into())
                        }
                        "keep-alive" => continue,
                        _ => {
                            tracing::debug!("ignoring event {}", evt.event_type);
                            continue;
                        }
                    };

                    let changed_path = {
                        let mut v = value.lock().await;
                        let value_ref = v.deref_mut();

                        let inner = std::mem::replace(value_ref, ObservedValue::new());
                        let (changed_path, patched_value) = apply_action(action, inner)?;
                        let _ = std::mem::replace(value_ref, patched_value);
                        changed_path
                    };

                    if let Err(err) = change_tx
                        // .send((changed_path, patched_value.as_ref().clone()))
                        .send((changed_path, value.clone()))
                        .await
                    {
                        eprintln!("Error sending rdb update {}", err);
                    }
                }

                match task.await? {
                    Ok(_) => {
                        break;
                    }
                    Err(err) => {
                        eprintln!("Error receiving updates {:?}", err);
                    }
                };
            }

            tracing::debug!("stopped streaming {}", url);

            Result::<()>::Ok(())
        });

        change_rx
    }

    pub fn stream<S: AsRef<str>>(
        self,
        path: S,
    ) -> impl Stream<Item = (Vec<String>, Arc<Mutex<ObservedValue>>)> {
        let rx = self.run(path);
        futures::stream::unfold(rx, |mut rx| async move {
            rx.recv().await.map(|update| (update, rx))
        })
    }
}

fn start_http_connection(
    url: Url,
    auth: Option<String>,
    tx: mpsc::Sender<es::Event>,
    mut rx: mpsc::Receiver<RdbControlMessage>,
) -> JoinHandle<Result<(), es::Error>> {
    tokio::spawn(async move {
        let client = es::Client::for_url(url.as_ref())?;
        let client = if let Some(auth) = auth {
            client.header("Authorization", &auth)?
        } else {
            client
        };
        let client = client
            .header("Accept", "text/event-stream")?
            .reconnect(
                es::ReconnectOptions::reconnect(true)
                    .retry_initial(false)
                    .delay(Duration::from_secs(1))
                    .backoff_factor(2)
                    .delay_max(Duration::from_secs(60))
                    .build(),
            )
            .build();

        let mut stream = Box::pin(client.stream());

        loop {
            tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Some(RdbControlMessage::Stop) => {
                            eprintln!("received stop message");
                            break;
                        },
                        None => {
                            eprintln!("rdb stream control channel closed, exiting");
                            break;
                        }
                    }
                }

                event = stream.try_next() => {
                    match event {
                        Ok(Some(event)) => {
                            if let Err(err) = tx.send(event.clone()).await {
                                eprintln!("error sending event from sse stream: {}", err);
                            };

                        },
                        Ok(None) => break,
                        Err(err) => return Err(err),
                    }
                }
            };
        }

        Ok(())
    })
}
