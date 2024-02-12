//! This module contains [`Reqwest`] struct that uses reqwest client to send requests to the Telegram Bot API.
//!
//! # Notes
//!
//! [`Reqwest`] is default implementation of [`Session`] trait in this library,
//! so it's used by default in [`Bot`] struct and trait methods that has bot as a parameter.
//!
//! This structure is cheap to clone,
//! because it contains only [`reqwest::Client`] field which is wrapped in [`Arc`] and [`APIServer`] wrapped in [`Cow`].
//!
//! [`Arc`]: std::sync::Arc
//! [`APIServer`]: crate::client::telegram::APIServer

use super::base::{ClientResponse, Session, DEFAULT_TIMEOUT};

use crate::{
    client::{telegram, Bot},
    methods::TelegramMethod,
    serializers::reqwest::{Error as SerializerError, MultipartSerializer},
    types::InputFile,
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

    /// Builds a form data from the given data and files.
    /// # Notes
    /// This method uses [`MultipartSerializer`] to serialize the data in custom format that Telegram Bot API accepts.
    /// # Warnings
    /// Be aware that build [`InputFile::Stream`] will be taken and cannot be used again without set stream again.
    /// Check its documentation for more information.
    /// # Errors
    /// Returns a [`SerializerError`] if the form cannot be built.
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

        for (index, file) in files.iter().enumerate() {
            match file {
                InputFile::FS(file) => {
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
                InputFile::Buffered(file) => {
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
                InputFile::Stream(file) => {
                    let Some(stream) = file.take_stream() else {
                        return Err(SerializerError::Custom(Cow::Owned(format!(
                            "File stream with index `{index}` already taken. \
                            Read `StreamFile::take_stream` documentation for more information."
                        ))));
                    };
                    let id = file.id().to_string();
                    let file_name = file.file_name();

                    let body = Body::wrap_stream(stream);
                    let part = if let Some(file_name) = file_name {
                        Part::stream(body).file_name(file_name.to_owned())
                    } else {
                        Part::stream(body).file_name(id.clone())
                    };

                    form = form.part(id, part);
                }
                InputFile::Id(_) | InputFile::Url(_) => continue,
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

    /// Sends a request to the Telegram Bot API and returns a response.
    /// # Arguments
    /// * `bot` - The bot instance
    /// * `method` - The method instance
    /// * `timeout` - The request timeout
    /// # Warning
    /// If the timeout is not set, the default timeout will not be used.
    ///
    /// Uses always `POST` method to send a request and `multipart/form-data` content type even if files are not provided.
    /// # Errors
    /// Returns an error if the request cannot be sent or the response cannot be received.
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
