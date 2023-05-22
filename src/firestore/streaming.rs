use anyhow::Result;
pub use firestore_grpc::google::firestore::v1::ListDocumentsResponse;
use firestore_grpc::tonic::{self, IntoStreamingRequest};
use firestore_grpc::tonic::{metadata::MetadataValue, Request};
use firestore_grpc::v1::listen_response::ResponseType;
use firestore_grpc::v1::target::query_target::QueryType;
use firestore_grpc::v1::target::{QueryTarget, TargetType};
use firestore_grpc::v1::target_change::TargetChangeType;
use firestore_grpc::v1::{self as firestore, ListenResponse, TargetChange};

use chrono::prelude::*;
use futures::{Stream, StreamExt};
use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use tokio::sync::mpsc;

use super::collection::*;
use super::conversion::IntoFirestoreDocumentValue;
use super::structured_query::{OrderBuilder, StructuredQueryBuilder};
use crate::firestore::client::get_client;

pub type UnaryFilterOperator = firestore::structured_query::unary_filter::Operator;
pub type FieldFilterOperator = firestore::structured_query::field_filter::Operator;
pub type CompositeFilterOperator = firestore::structured_query::composite_filter::Operator;

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

// "Rust" see https://github.com/googleapis/python-firestore/issues/51
// "Rust".as_bytes().iter().rev().enumerate().map(|(i, b)| ((*b) as i32) << (8 * i)).sum();
static GRPC_TARGET_ID: i32 = 0x52757374;

pub struct ListenRequestBuilder {
    client: super::FirebaseClient,
    collection: String,
    database: String,
    parent: String,
    resume_token: Option<Vec<u8>>,
    once: bool,
    structured_query: StructuredQueryBuilder,
}

impl ListenRequestBuilder {
    pub(crate) fn new<S1: AsRef<str>, S2: ToString>(
        client: super::FirebaseClient,
        project_id: S1,
        collection_id: S2,
    ) -> Self {
        let project_id = project_id.as_ref();
        let database = format!("projects/{}/databases/(default)", project_id);
        let parent = format!("{}/documents", database);
        let mut structured_query = StructuredQueryBuilder::new();
        structured_query.from(collection_id.to_string());
        Self {
            client,
            database,
            parent,
            collection: collection_id.to_string(),
            resume_token: None,
            once: false,
            structured_query,
        }
    }

    #[must_use]
    pub fn database<S: AsRef<str>>(mut self, database: S) -> Self {
        self.database = format!("projects/{}/databases/{}", self.database, database.as_ref());
        self
    }

    #[must_use]
    pub fn parent<S: AsRef<str>>(mut self, parent: S) -> Self {
        self.parent = format!("{}/{}", self.database, parent.as_ref());
        self
    }

    #[must_use]
    pub fn resume_token(self, resume_token: Vec<u8>) -> Self {
        self.resume_token_maybe(Some(resume_token))
    }

    #[must_use]
    pub fn resume_token_maybe(mut self, resume_token: Option<Vec<u8>>) -> Self {
        self.resume_token = resume_token;
        self
    }

    #[must_use]
    pub fn once(mut self) -> Self {
        self.once = true;
        self
    }

    #[must_use]
    pub fn limit(mut self, limit: i32) -> Self {
        self.structured_query.limit(limit);
        self
    }

    #[must_use]
    pub fn offset(mut self, offset: i32) -> Self {
        self.structured_query.offset(offset);
        self
    }

    pub fn order_by<S: ToString>(self, field: S) -> OrderBuilder<Self> {
        OrderBuilder::new(
            self,
            |me, order, start_at, end_at| {
                me.structured_query.order_by.push(order);
                me.structured_query.start_at = start_at;
                me.structured_query.end_at = end_at;
            },
            field,
        )
    }

    #[must_use]
    pub fn unary_filter<S: ToString>(mut self, field: S, op: UnaryFilterOperator) -> Self {
        self.structured_query.unary_filter(field, op);
        self
    }

    #[must_use]
    pub fn field_filter<T, S>(mut self, field: S, op: FieldFilterOperator, value: T) -> Self
    where
        T: IntoFirestoreDocumentValue,
        S: ToString,
    {
        self.structured_query.field_filter(field, op, value);
        self
    }

    pub async fn build(
        &mut self,
    ) -> Result<(CollectionStream<tonic::Status>, CollectionStreamController)> {
        let (control_rx, controller) = CollectionStreamController::new();
        let token = self.client.get_token().await?;
        let req = self.build_req(control_rx);
        let mut client = get_client(token).await?;
        let res = client.listen(req).await?;
        let inbound = res.into_inner();
        Ok((CollectionStreamState::map_stream(inbound), controller))
    }

    pub async fn build_retry(
        mut self,
        max_retry: u64,
    ) -> Result<(CollectionStream, CollectionStreamController)> {
        let (mut control_rx, controller) = CollectionStreamController::new();

        let stream = async_stream::stream! {
            let mut retry_count = 0;

            'outer: loop {
                if retry_count > 0 {
                    tracing::debug!("delaying retry {}...", retry_count);
                    tokio::time::sleep(std::time::Duration::from_secs(retry_count)).await;
                }

                let (mut stream, mut current_controller) = match self.build().await {
                    Err(err) if retry_count < max_retry => {
                        tracing::warn!("sending streaming request for {:?} errored, retrying ({})", self.collection, err);
                        continue;
                    }
                    Err(err) => {
                        yield Err(err);
                        break;
                    },
                    Ok(req) => req,
                };


                // The stream sometimes seems to get "stuck", that is not even
                // the keep alive requests in the background happening anymore.
                // Let's have our own keep alive that will restart the stream if
                // it has not received an update for a while.
                static MAX_NO_UPDATE_SECONDS: i64 = 60 * 10;
                let mut check_interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
                let mut last_update = Utc::now();

                'inner: loop {
                    tokio::select! {
                        _ = check_interval.tick() => {
                            tracing::debug!("stream {:?} check", self.collection);
                            let last_update_duration = Utc::now() - last_update;
                            if last_update_duration.num_seconds()
                                > (MAX_NO_UPDATE_SECONDS + thread_rng().gen_range(-30..30))
                            {
                                tracing::warn!("stream {:?} has had no updates for {}, restarting", self.collection, last_update_duration);
                                if let Err(err) = current_controller.stop().await {
                                    tracing::error!("Error sending stream stop {}", err);
                                }
                                break 'inner;
                            }
                        }

                        msg = control_rx.recv() => {
                            match msg {
                                Some(StreamControlMessage::Stop) => {
                                    tracing::debug!("retriable collection stream received stop message");
                                    if let Err(err) = current_controller.stop().await {
                                        tracing::error!("Error sending stream stop {}", err);
                                    }
                                    break 'outer;
                                },
                                _ => {
                                    tracing::debug!("retriable collection stream for {:?}: control message channel closed", self.collection);
                                }
                            }
                        }

                        item = stream.next() => {
                            match item {
                                None => {
                                    tracing::debug!("retriable collection stream for {:?}: stream stopped", self.collection);
                                    break 'inner;
                                },
                                Some(Err(err)) => {
                                    if retry_count < max_retry {
                                        eprintln!(
                                            "Error receiving collection stream {:?}, retrying attempt {}: {}",
                                            self.collection, retry_count, err
                                        );
                                        retry_count += 1;
                                        break 'inner;
                                    } else {
                                        yield Err(err.into());
                                    }
                                }
                                Some(Ok(update)) => {
                                    last_update = Utc::now();
                                    retry_count = 0;
                                    self.resume_token = Some(update.resume_token.clone());
                                    yield Ok(update);
                                }
                            }
                        }
                    }
                }
            }

            tracing::debug!("stopped streaming {}", self.collection);

        };

        Ok((Box::pin(stream), controller))
    }

    fn build_req(
        &self,
        mut control_rx: mpsc::Receiver<StreamControlMessage>,
    ) -> Request<impl Stream<Item = firestore::ListenRequest> + Send + Sync> {
        let Self {
            collection,
            database,
            parent,
            resume_token,
            structured_query,
            ..
        } = self;

        let structured_query = structured_query.clone().build();

        let resume_type = resume_token.as_ref().map(|token| {
            tracing::debug!("sending listen request with resume token");
            firestore::target::ResumeType::ResumeToken(token.clone())
        });

        let req = firestore::ListenRequest {
            database: database.to_string(),
            labels: HashMap::new(),
            target_change: Some(firestore::listen_request::TargetChange::AddTarget(
                firestore::Target {
                    target_id: GRPC_TARGET_ID,
                    once: false,
                    target_type: Some(TargetType::Query(QueryTarget {
                        parent: parent.clone(),
                        query_type: Some(QueryType::StructuredQuery(structured_query)),
                    })),
                    resume_type,
                },
            )),
        };

        let stream_name = format!("db={} collection={}", database, collection);

        #[rustfmt::skip]
        let outbound = async_stream::stream! {
            tracing::debug!("Sending initial listen request for {}", stream_name);

            yield req;

            tracing::debug!("Listen request for collection {} send", stream_name);

            loop {
                if let Some(msg) = control_rx.recv().await {
                    tracing::debug!("got control message when streaming {}: {:?}", stream_name, msg);
                    match msg {
                        StreamControlMessage::Stop => break,
                    }
                }
            }

            tracing::debug!("Stopping stream for {}", stream_name);
        };

        let mut req = tonic::Request::new(outbound).into_streaming_request();
        let metadata = req.metadata_mut();
        metadata.insert(
            "google-cloud-resource-prefix",
            MetadataValue::from_str(database).unwrap(),
        );

        req
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#[derive(Debug)]
pub enum StreamControlMessage {
    Stop,
}

#[derive(Clone)]
pub struct CollectionStreamController {
    control_tx: mpsc::Sender<StreamControlMessage>,
}

impl CollectionStreamController {
    pub fn new() -> (mpsc::Receiver<StreamControlMessage>, Self) {
        let (control_tx, rx) = mpsc::channel(1);
        (rx, Self { control_tx })
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.control_tx.send(StreamControlMessage::Stop).await?;
        Ok(())
    }
}

pub type CollectionStream<E = anyhow::Error> =
    Pin<Box<dyn Stream<Item = Result<CollectionUpdate, E>>>>;

pub struct CollectionStreamState {
    pub target_ids: Rc<RefCell<Vec<i32>>>,
    pub documents: SharedDocuments,
    pub changes: Vec<CollectionChange>,
}

impl CollectionStreamState {
    pub fn map_stream(
        inbound: tonic::Streaming<firestore::ListenResponse>,
    ) -> CollectionStream<tonic::Status> {
        let target_ids = Rc::new(RefCell::new(Vec::new()));
        let documents = HashMap::new();
        let changes = Vec::new();

        let state = Rc::new(RefCell::new(Self {
            documents,
            changes,
            target_ids,
        }));

        Box::pin(inbound.filter_map(move |res| {
            let state = state.clone();
            async move {
                match res {
                    Err(err) => Some(Err(err)),
                    Ok(res) => state.borrow_mut().handle_listen_response(res).map(Ok),
                }
            }
        }))
    }

    fn next_update(&mut self, change: TargetChange, force: bool) -> Option<CollectionUpdate> {
        if self.changes.is_empty() {
            return if force {
                Some(CollectionUpdate::default())
            } else {
                None
            };
        }

        Some(CollectionUpdate {
            changes: self.changes.drain(..).collect(),
            documents: std::mem::take(&mut self.documents),
            time: change.read_time.as_ref().map(|t| {
                Utc.timestamp_opt(t.seconds, t.nanos as u32)
                    .earliest()
                    .expect("timestamp")
            }),
            resume_token: change.resume_token,
        })
    }

    fn handle_listen_response(&mut self, res: ListenResponse) -> Option<CollectionUpdate> {
        let response_type = match res.response_type {
            None => return None,
            Some(response_type) => response_type,
        };

        match response_type {
            ResponseType::TargetChange(change) => {
                match change.target_change_type() {
                    TargetChangeType::Add => {
                        tracing::trace!("TargetChangeType::Add");
                        if let Ok(mut target_ids) = self.target_ids.try_borrow_mut() {
                            target_ids.extend(change.target_ids);
                        }
                    }

                    TargetChangeType::Remove => {
                        if let Some(cause) = change.cause {
                            tracing::trace!("TargetChangeType::Remove because {:?}", cause);
                        } else {
                            tracing::trace!("TargetChangeType::Remove");
                        };

                        if let Ok(mut target_ids) = self.target_ids.try_borrow_mut() {
                            target_ids.retain(|i| !change.target_ids.contains(i));
                        }
                    }

                    TargetChangeType::Current => {
                        tracing::trace!("{:?}", change.target_change_type());
                        return self.next_update(change, true);
                    }

                    TargetChangeType::Reset | TargetChangeType::NoChange => {
                        tracing::trace!("{:?}", change.target_change_type());
                        return self.next_update(change, false);
                    }
                };
            }

            ResponseType::DocumentChange(firestore::DocumentChange { document, .. }) => {
                if let Some(document) = document {
                    tracing::trace!("document changed ({})", document.name);
                    self.changes.push(CollectionChange::Change {
                        id: document.name.clone(),
                        time: Utc::now(),
                    });
                    self.documents.insert(document.name.clone(), document);
                }
            }

            ResponseType::DocumentRemove(firestore::DocumentRemove {
                document,
                read_time,
                ..
            })
            | ResponseType::DocumentDelete(firestore::DocumentDelete {
                document,
                read_time,
                ..
            }) => {
                tracing::trace!("document removed / deleted ({})", document);
                self.changes.push(CollectionChange::Delete {
                    id: document.clone(),
                    time: Utc::now(),
                    last_read: read_time.as_ref().map(|t| {
                        Utc.timestamp_opt(t.seconds, t.nanos as u32)
                            .earliest()
                            .expect("timestamp")
                    }),
                });
                self.documents.remove(&document);
            }

            firestore::listen_response::ResponseType::Filter(firestore::ExistenceFilter {
                count,
                ..
            }) => {
                tracing::debug!("received ExistenceFilter message, count={}. TODO handle it/update the collection", count);
            }
        };

        None
    }
}
