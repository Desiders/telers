use super::base::{Session, DEFAULT_TIMEOUT};

use crate::{
    client::{telegram, Bot},
    error::session,
    methods::TelegramMethod,
};

use async_trait::async_trait;
use reqwest;
use std::time::Duration;

#[derive(Clone)]
pub struct Reqwest {
    client: reqwest::Client,
    api: telegram::APIServer,
}

impl Reqwest {
    /// Creates a new session with custom parameters
    /// # Arguments
    /// * `client` - HTTP client
    /// * `api` - Telegram API server
    #[must_use]
    pub fn new(client: reqwest::Client, api: telegram::APIServer) -> Self {
        Self { client, api }
    }
}

impl Default for Reqwest {
    /// Creates a new session with default parameters
    /// # Panics
    /// This method panics if the client cannot be created
    #[must_use]
    fn default() -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
                .timeout(Duration::from_secs_f32(DEFAULT_TIMEOUT))
                .build()
                .unwrap(),
            api: telegram::PRODUCTION.clone(),
        }
    }
}

#[async_trait]
impl Session for Reqwest {
    async fn make_request<T>(
        &self,
        bot: &Bot,
        method: T,
        timeout: Option<f32>,
    ) -> Result<T::Return, session::ErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let request = method.build_request(bot);
        let url = self.api.api_url(bot.token(), request.method());

        let response = if let Some(timeout) = timeout {
            self.client
                .post(url)
                .form(request.params())
                .timeout(Duration::from_secs_f32(timeout))
        } else {
            self.client.post(url).form(request.params())
        }
        .send()
        .await
        .map_err(|err| {
            log::error!("Cannot send a request: {err}");

            session::ErrorKind::Request(err.into())
        })?;

        let status_code = response.status().as_u16();
        let content = response.text().await.map_err(|err| {
            log::error!("Cannot decode a response: {err}");

            session::ErrorKind::Decode(err.into())
        })?;

        let response = <Self as Session>::check_response(self, method, status_code, &content)?;

        // Unwrap is safe here, because we checked it in `check_response` method
        Ok(response.result.unwrap())
    }
}
