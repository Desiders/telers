//! Reqwest session implementation
//!
//! This module contains [`Reqwest`] struct, which is default implementation of [`Session`].

use super::base::{ClientResponse, Session, DEFAULT_TIMEOUT};

use crate::{
    client::{telegram, Bot},
    methods::TelegramMethod,
    serializers::reqwest::{Error as SerializerError, MultipartSerializer},
    types::{InputFile, InputFileKind},
};

use async_trait::async_trait;
use reqwest::{
    multipart::{Form, Part},
    Body, Client, ClientBuilder,
};
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
use tracing::{event, field, instrument, Level, Span};

#[derive(Debug, Clone)]
pub struct Reqwest {
    client: Client,
    api: Cow<'static, telegram::APIServer>,
}

impl Reqwest {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            api: Cow::Borrowed(&telegram::PRODUCTION),
        }
    }

    #[must_use]
    pub fn with_api_server(self, api: impl Into<Cow<'static, telegram::APIServer>>) -> Self {
        Self {
            api: api.into(),
            ..self
        }
    }

    #[instrument(skip(self, data))]
    async fn build_form_data<'a, Data: ?Sized>(
        &self,
        data: &Data,
        files: Option<&[&InputFile<'a>]>,
    ) -> Result<Form, SerializerError>
    where
        Data: Serialize,
    {
        let mut form = data.serialize(MultipartSerializer::new())?;

        let Some(files) = files else {
            return Ok(form);
        };

        for file in files {
            match file.kind() {
                InputFileKind::FS(file) => {
                    let id = file.id().to_string();
                    let file_name = file.file_name();
                    let stream = file.clone().stream();

                    let body = Body::wrap_stream(stream);
                    let part = if let Some(file_name) = file_name {
                        Part::stream(body).file_name(file_name.to_owned())
                    } else {
                        Part::stream(body).file_name(id.clone())
                    };

                    form = form.part(id, part);
                }
                InputFileKind::Buffered(file) => {
                    let id = file.id().to_string();
                    let file_name = file.file_name();
                    let bytes = file.bytes();

                    let part = if let Some(file_name) = file_name {
                        Part::bytes(bytes.to_vec()).file_name(file_name.to_string())
                    } else {
                        Part::bytes(bytes.to_vec()).file_name(id.clone())
                    };

                    form = form.part(id, part);
                }
                InputFileKind::Id(_) | InputFileKind::Url(_) => continue,
            };
        }

        Ok(form)
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
            api: Cow::Borrowed(&telegram::PRODUCTION),
        }
    }
}

#[async_trait]
impl Session for Reqwest {
    #[must_use]
    fn api(&self) -> &telegram::APIServer {
        &self.api
    }

    #[instrument(skip(self, bot, method, timeout), fields(files, method_name, timeout))]
    async fn send_request<Client, T>(
        &self,
        bot: &Bot<Client>,
        method: &T,
        timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let request = method.build_request(bot);

        Span::current()
            .record("files", field::debug(&request.files))
            .record("method_name", request.method_name);

        let form = self
            .build_form_data(request.data, request.files.as_deref())
            .await
            .map_err(|err| {
                event!(
                    Level::ERROR,
                    error = %err,
                    "Cannot build a form",
                );

                err
            })?;

        let url = self.api.api_url(&bot.token, request.method_name);

        let response = if let Some(timeout) = timeout {
            Span::current().record("timeout", timeout);

            self.client
                .post(url.as_ref())
                .multipart(form)
                .timeout(Duration::from_secs_f32(timeout))
        } else {
            self.client.post(url.as_ref()).multipart(form)
        }
        .send()
        .await
        .map_err(|err| {
            event!(
                Level::ERROR,
                error = %err,
                "Cannot send a request",
            );

            err
        })?;

        let status_code = response.status().as_u16();

        let content = response.text().await.map_err(|err| {
            event!(
                Level::ERROR,
                error = %err,
                status_code,
                "Cannot get a response content",
            );

            err
        })?;

        Ok(ClientResponse::new(status_code, content))
    }
}
