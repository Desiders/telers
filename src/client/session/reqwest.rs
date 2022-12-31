use super::base::{ClientResponse, Session, DEFAULT_TIMEOUT};

use crate::{
    client::{telegram, Bot},
    methods::TelegramMethod,
};

use async_trait::async_trait;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Reqwest {
    client: Client,
    api: telegram::APIServer,
}

impl Reqwest {
    #[must_use]
    pub fn new(client: Client, api: telegram::APIServer) -> Self {
        Self { client, api }
    }
}

impl Default for Reqwest {
    /// # Panics
    /// This method panics if the client cannot be created
    #[must_use]
    fn default() -> Self {
        Self {
            client: ClientBuilder::new()
                .timeout(Duration::from_secs_f32(DEFAULT_TIMEOUT))
                .build()
                .unwrap(),
            api: telegram::PRODUCTION.clone(),
        }
    }
}

#[async_trait]
impl Session for Reqwest {
    async fn send_request<T>(
        &self,
        bot: &Bot,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let request = method.build_request(bot);
        let url = self.api.api_url(bot.token(), request.method());

        let response = if let Some(timeout) = timeout {
            self.client
                .post(url)
                .json(request.params())
                .timeout(Duration::from_secs_f32(timeout))
        } else {
            self.client.post(url).json(request.params())
        }
        .send()
        .await
        .map_err(|err| {
            log::error!("Cannot send a request: {err}");

            err
        })?;

        let status_code = response.status().as_u16();
        let content = response.text().await.map_err(|err| {
            log::error!("Cannot decode a response: {err}");

            err
        })?;

        Ok(ClientResponse::new(status_code, content))
    }
}
