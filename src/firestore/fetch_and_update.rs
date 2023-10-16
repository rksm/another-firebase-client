use anyhow::{anyhow, Result};

use super::{FirebaseClient, FromFirestoreDocument};

pub struct FetchAndUpdate<'a> {
    col_name: &'a str,
    id: &'a str,
    client: &'a FirebaseClient,
    add_updated_timestamp: Option<String>,
}

impl<'a> FetchAndUpdate<'a> {
    pub(crate) fn new(client: &'a FirebaseClient, col_name: &'a str, id: &'a str) -> Self {
        Self {
            col_name,
            id,
            client,
            add_updated_timestamp: None,
        }
    }

    #[must_use]
    pub fn add_updated_timestamp(mut self, add_updated_timestamp: impl ToString) -> Self {
        self.add_updated_timestamp = Some(add_updated_timestamp.to_string());
        self
    }

    pub async fn modify<T>(&self, modify_fn: impl FnOnce(&mut T)) -> Result<()>
    where
        T: FromFirestoreDocument + serde::Serialize,
        T::Err: Into<anyhow::Error>,
    {
        let mut doc = self
            .client
            .get_document_of_collection(self.col_name, self.id)
            .await?;
        modify_fn(&mut doc);
        self.store(doc).await
    }

    pub async fn store<T>(&self, doc: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let Self {
            col_name,
            id,
            client,
            add_updated_timestamp,
        } = self;

        let mut doc = serde_json::to_value(doc)
            .map_err(|e| anyhow!("unable to serialize transcription doc: {}", e))?;

        if let Some(add_updated_timestamp) = add_updated_timestamp {
            // FIXME better have a trait for this with T?
            doc[add_updated_timestamp] = chrono::Utc::now().to_rfc3339().into();
        }

        client
            .update_document(&format!("{col_name}/{id}"))
            .document(doc)
            .update()
            .await?;

        Ok(())
    }
}
