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
use std::{borrow::Cow, collections::HashMap, io, time::Duration};
use thiserror;

#[derive(Debug, thiserror::Error)]
pub(crate) enum BuildFormError {
    #[error(transparent)]
    Serializer(#[from] SerializerError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug, Clone)]
pub struct Reqwest {
    client: Client,
    api: Cow<'static, telegram::APIServer>,
}

impl Reqwest {
    #[must_use]
    pub fn new<T>(client: Client) -> Self
    where
        T: Into<Cow<'static, telegram::APIServer>>,
    {
        Self {
            client,
            api: Cow::Borrowed(&telegram::PRODUCTION),
        }
    }

    #[must_use]
    pub fn api<T>(self, api: T) -> Self
    where
        T: Into<Cow<'static, telegram::APIServer>>,
    {
        Self {
            api: api.into(),
            ..self
        }
    }

    async fn build_form_data<'a, T>(
        &self,
        data: &T,
        files: Option<HashMap<&str, &InputFile<'a>>>,
    ) -> Result<Form, BuildFormError>
    where
        T: Serialize + ?Sized,
    {
        let mut form = data.serialize(MultipartSerializer::new())?;

        if let Some(files) = files {
            for (value, file) in files {
                let (read_file_fut, file_name) = match file.kind() {
                    InputFileKind::FS(file) => (file.read(), file.file_name()),
                    InputFileKind::Id(_) | InputFileKind::Url(_) => continue,
                };

                match Box::pin(read_file_fut).await {
                    Ok(bytes) => {
                        let body = Body::from(bytes);
                        let mut part = Part::stream(body);

                        if let Some(file_name) = file_name {
                            part = part.file_name(file_name.to_string());
                        }

                        form = form.part((*value).to_string(), part);
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
        let url = self.api.api_url(bot.token(), request.method_name);
        let form = Box::pin(self.build_form_data(request.data, request.files))
            .await
            .map_err(|err| {
                log::error!("Cannot build a form: {err}");

                err
            })?;

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
}
