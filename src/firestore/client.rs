use anyhow::Result;
use firebase_client_auth::{scopes, GoogleAuth, GoogleServiceAccount, ServiceAccountAuthorization};
use firestore_grpc::google::firestore::v1::*;
use firestore_grpc::tonic::codegen::InterceptedService;
use firestore_grpc::tonic::Status;
use firestore_grpc::tonic::{
    metadata::MetadataValue,
    transport::{Channel, ClientTlsConfig},
    Request,
};
use firestore_grpc::v1::firestore_client::FirestoreClient;
use futures::StreamExt;
use itertools::Itertools;

use super::conversion::{IntoFirestoreDocument, IntoFirestoreDocumentValue};
use super::structured_query::{self, StructuredQueryBuilder};

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

const URL: &str = "https://firestore.googleapis.com";
const DOMAIN: &str = "firestore.googleapis.com";

pub(crate) async fn get_client(
    token: Option<String>,
) -> Result<
    FirestoreClient<
        InterceptedService<Channel, impl FnMut(Request<()>) -> Result<Request<()>, Status>>,
    >,
> {
    let auth_header = if let Some(token) = token {
        let bearer_token = format!("Bearer {}", token);
        Some(MetadataValue::from_str(&bearer_token)?)
    } else {
        None
    };

    let endpoint = Channel::from_static(URL)
        .tls_config(ClientTlsConfig::new().domain_name(DOMAIN))
        .unwrap();
    let service =
        FirestoreClient::with_interceptor(endpoint.connect().await?, move |mut req: Request<()>| {
            if let Some(auth) = &auth_header {
                req.metadata_mut().insert("authorization", auth.clone());
            }
            Ok(req)
        });
    Ok(service)
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// list documents

#[derive(Debug)]
pub struct ListDocumentsOptions<'a> {
    pub client: &'a FirebaseClient,
    pub collection_id: String,
    pub parent: Option<String>,
    pub page_size: Option<i32>,
    pub order_by: Option<String>,
    pub page_token: Option<String>,
}

impl<'a> ListDocumentsOptions<'a> {
    fn new(client: &'a FirebaseClient, collection_id: String) -> Self {
        Self {
            client,
            collection_id,
            parent: None,
            page_size: None,
            order_by: None,
            page_token: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn order_by<S: ToString>(mut self, order_by: S) -> Self {
        self.order_by = Some(order_by.to_string());
        self
    }

    pub fn page_token<S: ToString>(mut self, page_token: S) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    pub fn parent<S: ToString>(mut self, parent: S) -> Self {
        self.parent = Some(parent.to_string());
        self
    }

    /// Returns pages of results
    pub async fn fetch_all(mut self) -> Result<Vec<Document>> {
        let mut result = Vec::new();

        let mut page = 0;
        loop {
            let res = self.fetch_page().await?;

            tracing::debug!("fetching page {}, {} documents", page, res.documents.len());
            page += 1;

            result.extend(res.documents);

            if res.next_page_token.is_empty() {
                break;
            }

            self.page_token = Some(res.next_page_token.to_string());
        }

        tracing::debug!("done after fetching {} pages", page);

        Ok(result)
    }

    /// Returns documents of a single "page"
    pub async fn fetch(&mut self) -> Result<Vec<Document>> {
        let res = self.fetch_page().await?;
        self.page_token = Some(res.next_page_token.to_string());
        Ok(res.documents)
    }

    /// Returns response of a single "page"
    pub async fn fetch_page(&mut self) -> Result<ListDocumentsResponse> {
        let Self {
            client,
            collection_id,
            parent,
            page_size,
            order_by,
            page_token,
        } = self;
        let token = client.get_token().await?;
        let project_id = &client.project_id;
        let parent = match parent {
            Some(parent) if !parent.is_empty() => {
                format!(
                    "projects/{}/databases/(default)/documents/{}",
                    project_id, parent
                )
            }
            _ => format!("projects/{}/databases/(default)/documents", project_id),
        };

        tracing::debug!("firestore list_documents {} {}", parent, collection_id);

        let mut req = ListDocumentsRequest {
            parent,
            collection_id: collection_id.clone(),
            ..Default::default()
        };
        if let Some(page_size) = page_size {
            req.page_size = *page_size;
        }
        if let Some(order_by) = order_by {
            req.order_by = order_by.clone();
        }
        if let Some(page_token) = page_token {
            req.page_token = page_token.clone();
        }

        let t = token.as_ref();
        let res = get_client(token).await?.list_documents(req).await?;

        Ok(res.into_inner())
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// list collections

#[derive(Debug)]
pub struct ListCollectionsOptions<'a> {
    pub client: &'a FirebaseClient,
    pub parent: Option<String>,
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
}

impl<'a> ListCollectionsOptions<'a> {
    fn new(client: &'a FirebaseClient) -> Self {
        Self {
            client,
            parent: None,
            page_size: None,
            page_token: None,
        }
    }

    #[must_use]
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    #[must_use]
    pub fn page_token<S: ToString>(mut self, page_token: S) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    #[must_use]
    pub fn parent<S: ToString>(mut self, parent: S) -> Self {
        self.parent = Some(parent.to_string());
        self
    }

    /// Returns pages of results
    pub async fn fetch_all(mut self) -> Result<Vec<String>> {
        let mut result = Vec::new();

        let mut page = 0;
        loop {
            let res = self.fetch_page().await?;

            tracing::debug!(
                "fetching page {}, {} documents",
                page,
                res.collection_ids.len()
            );
            page += 1;

            result.extend(res.collection_ids);

            if res.next_page_token.is_empty() {
                break;
            }

            self.page_token = Some(res.next_page_token.to_string());
        }

        tracing::debug!("done after fetching {} pages", page);

        Ok(result)
    }

    /// Returns documents of a single "page"
    pub async fn fetch(mut self) -> Result<Vec<String>> {
        let res = self.fetch_page().await?;
        Ok(res.collection_ids)
    }

    pub async fn fetch_page(&mut self) -> Result<ListCollectionIdsResponse> {
        let Self {
            client,
            parent,
            page_size,
            page_token,
        } = self;
        let token = client.get_token().await?;
        let project_id = &client.project_id;
        let parent = match parent {
            Some(parent) if !parent.is_empty() => {
                format!(
                    "projects/{}/databases/(default)/documents/{}",
                    project_id, parent
                )
            }
            _ => format!("projects/{}/databases/(default)/documents", project_id),
        };

        tracing::debug!("firestore list collection ids {}", parent);

        let mut req = ListCollectionIdsRequest {
            parent,
            ..Default::default()
        };
        if let Some(page_size) = page_size {
            req.page_size = *page_size;
        }
        if let Some(page_token) = page_token {
            req.page_token = page_token.clone();
        }

        let res = get_client(token).await?.list_collection_ids(req).await?;

        Ok(res.into_inner())
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// Get document

#[derive(Debug)]
pub struct GetDocumentOptions<'a> {
    pub client: &'a FirebaseClient,
    pub name: String,
}

impl<'a> GetDocumentOptions<'a> {
    fn new(client: &'a FirebaseClient, name: String) -> Self {
        Self { client, name }
    }

    pub async fn fetch(self) -> Result<Document> {
        let Self { client, name } = self;
        let token = client.get_token().await?;
        let project_id = &client.project_id;
        let name = format!(
            "projects/{}/databases/(default)/documents/{}",
            project_id, name
        );

        tracing::debug!("firestore get_document {}", name);

        let req = GetDocumentRequest {
            name,
            ..Default::default()
        };

        let res = get_client(token).await?.get_document(req).await?;
        Ok(res.into_inner())
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// Delete document

#[derive(Debug)]
pub struct DeleteDocumentOptions<'a> {
    pub client: &'a FirebaseClient,
    pub name: String,
}

impl<'a> DeleteDocumentOptions<'a> {
    fn new(client: &'a FirebaseClient, name: String) -> Self {
        Self { client, name }
    }

    pub async fn fetch(self) -> Result<()> {
        let Self { client, name } = self;
        let token = client.get_token().await?;
        let project_id = &client.project_id;
        let name = format!(
            "projects/{}/databases/(default)/documents/{}",
            project_id, name
        );

        tracing::debug!("firestore delete document {}", name);

        let req = DeleteDocumentRequest {
            name,
            current_document: None,
        };

        get_client(token).await?.delete_document(req).await?;
        Ok(())
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// Batch get documents

#[derive(Debug)]
pub struct BatchGetDocumentOptions<'a> {
    pub client: &'a FirebaseClient,
    pub names: Vec<String>,
}

impl<'a> BatchGetDocumentOptions<'a> {
    fn new(client: &'a FirebaseClient, names: Vec<String>) -> Self {
        Self { client, names }
    }

    pub async fn fetch(self) -> Result<Vec<Result<BatchGetDocumentsResponse, Status>>> {
        let Self { client, names } = self;
        let token = client.get_token().await?;
        let project_id = &client.project_id;
        let database = format!("projects/{}/databases/(default)", project_id);
        let documents = names
            .into_iter()
            .map(|name| format!("{}/documents/{}", database, name))
            .collect::<Vec<_>>();

        tracing::debug!("firestore batch get {}", documents.len());

        let req = BatchGetDocumentsRequest {
            documents,
            database,
            ..Default::default()
        };
        let res = get_client(token).await?.batch_get_documents(req).await?;
        let stream = res.into_inner();
        futures::pin_mut!(stream);
        Ok(stream.collect().await)
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// Query documents

pub struct QueryOptions<'a> {
    client: &'a FirebaseClient,
    structured_query: StructuredQueryBuilder,
    parent: Option<String>,
}

impl<'a> QueryOptions<'a> {
    fn new(client: &'a FirebaseClient) -> Self {
        Self {
            client,
            parent: None,
            structured_query: StructuredQueryBuilder::new(),
        }
    }

    pub fn from<S: ToString>(mut self, collection_id: S) -> Self {
        self.structured_query.from(collection_id);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.structured_query.limit(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.structured_query.offset(offset);
        self
    }

    pub fn order_by<S: ToString>(self, field: S) -> structured_query::OrderBuilder<Self> {
        structured_query::OrderBuilder::new(
            self,
            |me, order, start_at, end_at| {
                me.structured_query.order_by.push(order);
                me.structured_query.start_at = start_at;
                me.structured_query.end_at = end_at;
            },
            field,
        )
    }

    pub fn unary_filter<S: ToString>(
        mut self,
        field: S,
        op: structured_query::UnaryFilterOperator,
    ) -> Self {
        self.structured_query.unary_filter(field, op);
        self
    }

    pub fn field_filter<T, S>(
        mut self,
        field: S,
        op: structured_query::FieldFilterOperator,
        value: T,
    ) -> Self
    where
        T: IntoFirestoreDocumentValue,
        S: ToString,
    {
        self.structured_query.field_filter(field, op, value);
        self
    }

    pub async fn fetch(self) -> Result<Vec<Document>> {
        let responses = self.make_request().await?;
        let mut result = Vec::new();

        for res in responses {
            match res {
                Err(err) => {
                    tracing::error!("[firestore query] error querying for document {}", err);
                }
                Ok(RunQueryResponse { document, .. }) => {
                    if let Some(document) = document {
                        result.push(document);
                    } else {
                        tracing::warn!("[firestore query] no document for user");
                    }
                }
            }
        }

        Ok(result)
    }

    pub async fn make_request(self) -> Result<Vec<Result<RunQueryResponse, Status>>> {
        let Self {
            client,
            structured_query,
            parent,
        } = self;

        let project_id = &client.project_id;
        let parent = match parent {
            Some(parent) if !parent.is_empty() => {
                format!(
                    "projects/{}/databases/(default)/documents/{}",
                    project_id, parent
                )
            }
            _ => format!("projects/{}/databases/(default)/documents", project_id),
        };

        let structured_query = structured_query.build();

        let req = RunQueryRequest {
            parent,
            consistency_selector: None,
            query_type: Some(run_query_request::QueryType::StructuredQuery(
                structured_query,
            )),
        };

        let token = client.get_token().await?;
        let res = get_client(token).await?.run_query(req).await?;
        let stream = res.into_inner();
        futures::pin_mut!(stream);
        Ok(stream.collect().await)
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// Update documents

#[derive(Debug)]
pub struct UpdateDocumentOptions<'a> {
    pub client: &'a FirebaseClient,
    pub document: Document,
}

impl<'a> UpdateDocumentOptions<'a> {
    fn new(client: &'a FirebaseClient, name: String) -> Self {
        let name = format!(
            "projects/{}/databases/(default)/documents/{}",
            client.project_id, name
        );

        Self {
            client,
            document: Document {
                name,
                ..Default::default()
            },
        }
    }

    pub fn document<T: IntoFirestoreDocument>(mut self, document: T) -> Self {
        let doc = std::mem::replace(
            &mut self.document,
            document.into_document_from_fields().expect("into document"),
        );
        if self.document.name.is_empty() {
            self.document.name = doc.name;
        }
        self
    }

    pub fn field<S: ToString, T: IntoFirestoreDocumentValue>(mut self, key: S, val: T) -> Self {
        self.document
            .fields
            .insert(key.to_string(), val.into_document_value());
        self
    }

    pub async fn update(self) -> Result<Document> {
        let Self { document, client } = self;
        let token = client.get_token().await?;

        tracing::debug!("firestore update document {}", document.name);

        let req = UpdateDocumentRequest {
            document: Some(document),
            update_mask: None,
            mask: None,
            current_document: None,
        };

        let res = get_client(token).await?.update_document(req).await;

        let res = match res {
            Err(status) => {
                let details = String::from_utf8_lossy(status.details());
                return Err(anyhow::anyhow!("{} details: {}", status.message(), details));
            }
            Ok(res) => res,
        };

        Ok(res.into_inner())
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
// Batch update documents

#[derive(Debug)]
pub struct BatchUpdateDocumentOptions<'a> {
    pub client: &'a FirebaseClient,
    pub updates: Vec<Document>,
    pub deletes: Vec<String>,
}

impl<'a> BatchUpdateDocumentOptions<'a> {
    fn new(client: &'a FirebaseClient) -> Self {
        Self {
            client,
            updates: Vec::new(),
            deletes: Vec::new(),
        }
    }

    pub fn delete<S: ToString>(&mut self, id: S) {
        self.deletes.push(id.to_string());
    }

    pub fn update(&mut self, doc: Document) {
        self.updates.push(doc);
    }

    pub async fn fetch(self) -> Result<Vec<BatchWriteResponse>> {
        let Self {
            client,
            updates,
            deletes,
        } = self;

        let token = client.get_token().await?;
        let project_id = &client.project_id;
        let database = format!("projects/{}/databases/(default)", project_id);
        let mut writes = Vec::new();

        for doc in updates {
            writes.push(Write {
                update_mask: None,
                update_transforms: Vec::new(),
                current_document: None,
                operation: Some(write::Operation::Update(doc)),
            });
        }

        for id in deletes {
            writes.push(Write {
                update_mask: None,
                update_transforms: Vec::new(),
                current_document: None,
                operation: Some(write::Operation::Delete(id)),
            });
        }

        tracing::debug!("firestore batch update {}", writes.len());

        let mut responses = Vec::new();

        // the limit of documents in batch writes is 500
        for writes in &writes.into_iter().chunks(500) {
            tracing::debug!("firestore batch update commiting chunk");
            let writes = writes.collect::<Vec<_>>();
            let req = BatchWriteRequest {
                database: database.clone(),
                writes,
                ..Default::default()
            };
            let res = get_client(token.clone()).await?.batch_write(req).await?;
            responses.push(res.into_inner());
        }

        Ok(responses)
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#[derive(Debug)]
pub struct FirebaseClient {
    pub project_id: String,
    pub auth: GoogleAuth,
}

impl Clone for FirebaseClient {
    fn clone(&self) -> Self {
        Self {
            project_id: self.project_id.clone(),
            auth: self.auth.box_clone(),
        }
    }
}

impl FirebaseClient {
    pub fn for_account(account: GoogleServiceAccount) -> Self {
        FirebaseClient::new(Box::new(
            ServiceAccountAuthorization::with_account_and_scope(
                account,
                &[scopes::AUTH_DATASTORE, scopes::AUTH_CLOUD_PLATFORM],
            ),
        ))
    }

    pub fn new(auth: GoogleAuth) -> Self {
        let project_id = auth.project_id().to_string();
        FirebaseClient { project_id, auth }
    }

    pub async fn get_token(&self) -> Result<Option<String>> {
        Ok(self.auth.get_token().await?)
    }

    pub fn documents_path(&self) -> String {
        format!("projects/{}/databases/(default)/documents", self.project_id)
    }

    pub fn list_documents<S: ToString>(&self, collection_id: S) -> ListDocumentsOptions<'_> {
        ListDocumentsOptions::new(self, collection_id.to_string())
    }

    pub fn list_collections(&self) -> ListCollectionsOptions<'_> {
        ListCollectionsOptions::new(self)
    }

    pub fn get_document<S: ToString>(&self, name: S) -> GetDocumentOptions<'_> {
        GetDocumentOptions::new(self, name.to_string())
    }

    pub fn delete_document<S: ToString>(&self, name: S) -> DeleteDocumentOptions<'_> {
        DeleteDocumentOptions::new(self, name.to_string())
    }

    pub fn batch_get_documents<S: ToString>(&self, names: &[S]) -> BatchGetDocumentOptions<'_> {
        BatchGetDocumentOptions::new(self, names.iter().map(|ea| ea.to_string()).collect())
    }

    pub fn update_document<S: ToString>(&self, id: S) -> UpdateDocumentOptions<'_> {
        UpdateDocumentOptions::new(self, id.to_string())
    }

    pub fn batch_update(&self) -> BatchUpdateDocumentOptions<'_> {
        BatchUpdateDocumentOptions::new(self)
    }

    pub fn run_query(&self) -> QueryOptions {
        QueryOptions::new(self)
    }

    pub fn stream_builder<S: ToString>(
        &self,
        collection_id: S,
    ) -> super::streaming::ListenRequestBuilder {
        super::streaming::ListenRequestBuilder::new(self.clone(), &self.project_id, collection_id)
    }
}
