use anyhow::Result;
use clap::{Parser, ValueEnum};
use firebase_client::firestore::{
    collection::CachedCollection, FirebaseClient, FromFirestoreDocument,
};
use futures::StreamExt;
use serde_json::Value;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    version,
    author,
    about("Read and modify values of firebase firestore.")
)]
struct Options {
    #[arg[long, help("Directory used for storing progress into.")]]
    data_dir: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value_t = Env::Dev, help("Which env to run against? Defaults to dev."))]
    env: Env,

    #[arg(value_enum, help("The method to apply."))]
    method: Method,

    #[arg(help("The path name / collection_id."),
          required_if_eq_any([("method", "list"),("method", "get"),
                              ("method", "update"),
                              ("method", "delete"),
                              ("method", "stream")]))]
    path: Option<String>,

    #[arg(help("The JSON value to send."))]
    value: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, Eq, PartialEq, PartialOrd, Ord)]
enum Env {
    Dev,
    Prod,
}

#[derive(Debug, Clone, Copy, ValueEnum, Eq, PartialEq, PartialOrd, Ord)]
enum Method {
    Get,
    List,
    Update,
    Delete,
    Stream,
    Collections,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let Options {
        data_dir,
        env,
        method,
        path,
        value,
    } = Options::parse();

    let env_file = match env {
        Env::Prod => ".env",
        Env::Dev => ".env-dev",
    };

    dotenv::from_filename(env_file).expect(".env");

    let data_dir = data_dir.as_ref().map(|data_dir| {
        let data_dir: PathBuf = data_dir.into();
        std::fs::create_dir_all(&data_dir).expect("create data dir");
        data_dir
    });

    let path = path.unwrap_or_default();

    let value = value
        .map(|val| serde_json::from_str(&val).unwrap())
        .unwrap_or(serde_json::Value::Null);

    let acct = firebase_client_auth::GoogleServiceAccount::from_default_env_var()?;
    let mut client = FirebaseClient::for_account(acct);

    match method {
        Method::List => {
            let (parent, name) = if let Some((parent, name)) = path.rsplit_once('/') {
                (parent, name)
            } else {
                ("", path.as_str())
            };

            let res = client.list_documents(name).parent(parent).fetch().await?;
            dbg!(res);
        }

        Method::Get => {
            let res = client.get_document(path).fetch().await?;
            dbg!(res);
        }

        Method::Update => {
            let res = client
                .update_document(path)
                .document(value)
                .update()
                .await?;

            dbg!(res);
        }

        Method::Delete => {
            assert!(!path.is_empty());
            client.delete_document(&path).fetch().await?;
            println!("Deleted {}", path);
        }

        Method::Stream => {
            let data_dir = match data_dir {
                None => {
                    return Err(anyhow::anyhow!("Need --data-dir for streaming"));
                }
                Some(data_dir) => data_dir,
            };
            stream_collection(client, &data_dir, path).await?;
        }

        Method::Collections => {
            let collection_ids = client.list_collections().fetch_all().await?;
            println!(
                "found {} collections:\n  {}",
                collection_ids.len(),
                collection_ids.join("\n  ")
            );
        }
    }

    Ok(())
}

async fn stream_collection(
    mut client: FirebaseClient,
    data_dir: impl AsRef<Path>,
    collection: impl ToString,
) -> Result<()> {
    let collection = collection.to_string();
    let cache_file = format!("{}-collection.json", collection);
    let mut builder = client.stream_builder(collection.to_string());

    let cache_file = data_dir.as_ref().join(cache_file);

    let mut collection = if cache_file.exists() {
        let file = std::fs::OpenOptions::new().read(true).open(cache_file)?;
        serde_json::from_reader(file)?
    } else {
        CachedCollection::<Value>::new(collection.to_string())
    };

    if let Some(resume_token) = &collection.resume_token {
        println!("got resume token, resuming stream");
        builder = builder.resume_token(resume_token.clone());
    }

    let (mut stream, mut ctrl) = builder.build_retry(3).await?;

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

    if false {
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            println!("STOPPING");
            ctrl.stop().await.unwrap();
        });
    }

    while let Some(update) = stream.next().await {
        let update = match update {
            Err(err) => {
                eprintln!("error in stream update: {}", err);
                continue;
            }
            Ok(update) => update,
        };

        for change in &update.changes {
            use firebase_client::firestore::collection::CollectionChange::*;
            match change {
                Change { id, time } => {
                    println!("changed doc={} time={}", id, time);
                }
                Delete { id, time, .. } => {
                    println!("deleted doc={} time={}", id, time);
                }
            }
        }

        for (id, doc) in &update.documents {
            let json = serde_json::Value::convert_doc(doc.clone())?;
            println!("{}\n{}\n", id, serde_json::to_string_pretty(&json)?);
        }

        println!("Got {} new/changed documents", update.documents.len());
        collection.update_from(update);
        collection.save()?;
        println!("Have {} documents in total", collection.documents.len());

        for (id, doc) in &collection.documents {
            println!("{}\n{}\n", id, serde_json::to_string_pretty(doc)?);
        }
    }

    Ok(())
}
