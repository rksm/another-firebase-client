use anyhow::Result;
use chrono::prelude::*;
use firestore_grpc::v1 as firestore;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::{conversion::FromFirestoreDocument, FirebaseClient};

pub type SharedDocuments = HashMap<String, firestore::Document>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CollectionChange {
    Change {
        id: String,
        time: DateTime<Utc>,
    },
    Delete {
        id: String,
        time: DateTime<Utc>,
        last_read: Option<DateTime<Utc>>,
    },
}

#[derive(Debug, Default)]
pub struct CollectionUpdate {
    pub changes: Vec<CollectionChange>,
    pub documents: SharedDocuments,
    pub time: Option<DateTime<Utc>>,
    pub resume_token: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedCollection<T> {
    pub name: String,
    pub cache_file: Option<PathBuf>,
    pub documents: HashMap<String, T>,
    pub time: Option<DateTime<Utc>>,
    pub resume_token: Option<Vec<u8>>,
}

impl<T> CachedCollection<T> {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            documents: HashMap::new(),
            cache_file: None,
            resume_token: None,
            time: None,
        }
    }

    pub fn len(&self) -> usize {
        self.documents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    pub fn full_path_for_id<S: AsRef<str>, S2: AsRef<str>>(
        &self,
        id: S,
        db_base_path: S2,
    ) -> String {
        format!("{}/{}/{}", db_base_path.as_ref(), self.name, id.as_ref())
    }

    pub fn get_by_id<S: AsRef<str>, S2: AsRef<str>>(&self, id: S, db_base_path: S2) -> Option<&T> {
        self.documents.get(&self.full_path_for_id(id, db_base_path))
    }
}

impl<T> CachedCollection<T>
where
    T: DeserializeOwned,
{
    pub fn ensure<P: AsRef<Path>, S: ToString>(collection: S, data_dir: P) -> Result<Self> {
        let collection_name = collection.to_string();
        let cache_file_name = format!("{}-collection.json", collection_name);
        let cache_file = data_dir.as_ref().join(cache_file_name);

        if cache_file.exists() {
            match Self::load(&cache_file) {
                Ok(loaded) => return Ok(loaded),
                Err(err) => {
                    tracing::error!(
                        "Error loading cached collection {:?} from file {}: {}",
                        collection.to_string(),
                        cache_file.display(),
                        err
                    );
                    tracing::info!("continuing loading without cached content");
                }
            }
        };

        let mut collection = Self::new(collection);
        collection.cache_file = Some(cache_file);
        collection.name = collection_name;

        Ok(collection)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        tracing::debug!("loading from file {}", path.as_ref().display());
        let file = std::fs::OpenOptions::new().read(true).open(&path)?;
        let mut loaded: Self = serde_json::from_reader(file)?;
        loaded.cache_file = Some(path.as_ref().to_path_buf());
        Ok(loaded)
    }
}

impl<T> CachedCollection<T>
where
    T: FromFirestoreDocument + Serialize,
{
    pub fn save(&self) -> Result<()> {
        let file = match &self.cache_file {
            None => {
                return Err(anyhow::anyhow!(
                    "cannot save cached collection {}, no cache file",
                    self.name
                ))
            }
            Some(file) => file,
        };

        tracing::debug!("saving to file {}", file.display());

        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file)?;

        serde_json::to_writer(file, &self)?;
        // serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn update_from(&mut self, mut other: CollectionUpdate) -> Vec<String> {
        let changes = other.changes.drain(..);
        let mut changed_docs = Vec::new();
        for change in changes {
            match change {
                CollectionChange::Change { id, .. } => changed_docs.push(id),
                CollectionChange::Delete { id, .. } => {
                    self.documents.remove(&id);
                }
            }
        }

        let other: Self = other.into();
        self.time = other.time;
        self.resume_token = other.resume_token;
        self.documents.extend(other.documents);

        changed_docs
    }

    pub async fn fill(&mut self, client: &FirebaseClient) -> Result<usize> {
        let docs = client
            .list_documents(&self.name)
            .page_size(300)
            .fetch_all()
            .await?;
        for doc in docs {
            let name = doc.name.clone();
            match T::convert_doc(doc) {
                Ok(val) => {
                    self.documents.insert(name, val);
                }
                Err(err) => {
                    tracing::error!(
                        "[cached collection fill] unable to insert document {}: {}",
                        name,
                        err
                    );
                }
            };
        }
        Ok(self.len())
    }
}

impl<T> From<CollectionUpdate> for CachedCollection<T>
where
    T: FromFirestoreDocument,
{
    fn from(collection: CollectionUpdate) -> Self {
        let CollectionUpdate {
            documents,
            resume_token,
            time,
            ..
        } = collection;
        let documents = documents
            .into_iter()
            .filter_map(|(key, doc)| match T::convert_doc(doc) {
                Ok(val) => Some((key, val)),
                Err(err) => {
                    eprintln!("Failure when converting firestore document {}", err);
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        CachedCollection {
            name: "???".to_string(), // FIXME
            documents,
            time,
            resume_token: Some(resume_token),
            cache_file: None,
        }
    }
}
