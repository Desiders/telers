use super::base::{ClientResponse, Session, DEFAULT_TIMEOUT};

use crate::{
    client::{telegram, Bot},
    methods::{Request, TelegramMethod},
    types::InputFileKind,
};

use async_trait::async_trait;
use reqwest::{
    multipart::{Form, Part},
    Body, Client, ClientBuilder,
};
use serde::Serialize;
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
    #[must_use]
    async fn send_json<'a, T>(
        &self,
        request: Request<'a, T>,
        url: &str,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        T: Serialize + Send + Sync,
    {
        let response = if let Some(timeout) = timeout {
            self.client
                .post(url)
                .json(request.data())
                .timeout(Duration::from_secs_f32(timeout))
        } else {
            self.client.post(url).json(request.data())
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

    #[must_use]
    async fn send_multipart<'a, T>(
        &self,
        request: Request<'a, T>,
        url: &str,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        T: Serialize + Send + Sync,
    {
        let mut form = Form::new();

        // `unwrap` is safe here because we checked that there are files
        for (value, file) in request.files().unwrap() {
            let (read_file_fut, file_name) = match file.kind() {
                InputFileKind::FS(file) => (file.read(), file.file_name()),
                InputFileKind::Id(_) | InputFileKind::Url(_) => break,
            };

            match read_file_fut.await {
                Ok(bytes) => {
                    let body = Body::from(bytes);
                    let mut part = Part::stream(body);

                    if let Some(file_name) = file_name {
                        part = part.file_name(file_name);
                    }

                    form = form.part(value.clone().into_owned(), part);
                }
                Err(err) => {
                    if let Some(file_name) = file_name {
                        log::error!("Cannot read a file with name `{file_name}`: {err}");
                    } else {
                        log::error!("Cannot read a file: {err}");
                    }

                    return Err(err.into());
                }
            }
        }

        let response = if let Some(timeout) = timeout {
            self.client
                .post(url)
                .multipart(form)
                .timeout(Duration::from_secs_f32(timeout))
        } else {
            self.client.post(url).multipart(form)
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
        let url = self.api.api_url(bot.token(), request.method_name());

        if request.files().is_none() {
            self.send_json(request, &url, timeout)
        } else {
            self.send_multipart(request, &url, timeout)
        }
        .await
    }
}
