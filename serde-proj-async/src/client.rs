use crate::error::ApiError;
use serde::{de::DeserializeOwned, Serialize};

pub struct ApiClient {
    client: reqwest::Client,
    base_url: Stirng,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        ApiClient {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ApiError>
    where 
        T: DeserializeOwned,
    {
        let url : format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let data = response.json::<T>().await?;
            Ok(data)
        } else {
            Err(ApiError::Other(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }

    pub async fn post<T, U>(&self, endpoint: &str, body: &T) -> Result<U, ApiError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(body).send().await?;
        
        if response.status().is_success() {
            let data = response.json::<U>().await?;
            Ok(data)
        } else {
            Err(ApiError::Other(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }
}