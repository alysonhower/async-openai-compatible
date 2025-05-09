use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{Batch, BatchRequest, ListBatchesResponse},
    Client,
};

/// Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.
///
/// Related guide: [Batch](https://platform.openai.com/docs/guides/batch)
pub struct Batches<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Batches<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Creates and executes a batch from an uploaded file of requests
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn create(&self, request: BatchRequest) -> Result<Batch, OpenAIError> {
        let path = self
            .client
            .config()
            .custom_batch_url()
            .unwrap_or("/batches");
        self.client.post(path, request).await
    }

    /// List your organization's batches.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<ListBatchesResponse, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        let path = self
            .client
            .config()
            .custom_batch_url()
            .unwrap_or("/batches");
        self.client.get_with_query(path, &query).await
    }

    /// Retrieves a batch.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, batch_id: &str) -> Result<Batch, OpenAIError> {
        let base_path = self
            .client
            .config()
            .custom_batch_url()
            .unwrap_or("/batches");
        self.client
            .get(&format!("{base_path}/{batch_id}"))
            .await
    }

    /// Cancels an in-progress batch. The batch will be in status `cancelling` for up to 10 minutes, before changing to `cancelled`, where it will have partial results (if any) available in the output file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn cancel(&self, batch_id: &str) -> Result<Batch, OpenAIError> {
        let base_path = self
            .client
            .config()
            .custom_batch_url()
            .unwrap_or("/batches");
        self.client
            .post(
                &format!("{base_path}/{batch_id}/cancel"),
                serde_json::json!({}),
            )
            .await
    }
}
