use std::sync::{Arc, Mutex, OnceLock};

use telers::{
    client::{
        session::{ClientResponse, ClientStreamResponse},
        telegram::APIServer,
        Session,
    },
    methods::{Request, TelegramMethod},
    serialization, Bot,
};

#[derive(Clone, Default)]
struct RecordingSession {
    requests: Arc<Mutex<Vec<(&'static str, String)>>>,
}

impl Session for RecordingSession {
    fn api(&self) -> &APIServer {
        static API: OnceLock<APIServer> = OnceLock::new();
        API.get_or_init(APIServer::default)
    }

    async fn send_request<Client, T>(
        &self,
        bot: &Bot<Client>,
        method: T,
        _timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let request = method.build_request(bot);
        let data = serialization::to_string(&request.data)?;
        self.requests
            .lock()
            .unwrap()
            .push((request.method_name, data));
        let response = if request.method_name == "deleteMessage" {
            r#"{"ok":true,"result":true}"#
        } else {
            r#"{"ok":true,"result":42}"#
        };
        Ok(ClientResponse::new(200, response))
    }

    async fn stream_content(
        &self,
        _url: &str,
        _timeout: Option<f32>,
    ) -> Result<ClientStreamResponse, anyhow::Error> {
        anyhow::bail!("this session only records API requests")
    }
}

struct StandaloneMethod;

impl telers_methods::TelegramMethod for StandaloneMethod {
    type Method = i64;
    type Return = i64;

    fn build_request(self) -> telers_methods::Request<Self::Method> {
        telers_methods::Request::new("standalone", 7, None)
    }

    fn build_response(
        content: &str,
    ) -> Result<telers_methods::Response<Self::Return>, serialization::Error> {
        let mut response: telers_methods::Response<i64> = serialization::from_str(content)?;
        response.result = response.result.map(|value| value + 1);
        Ok(response)
    }
}

struct LegacyMethod;

impl TelegramMethod for LegacyMethod {
    type Method = i64;
    type Return = i64;

    fn build_request<Client>(self, bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("legacy", bot.id, None)
    }
}

#[tokio::test]
async fn bot_accepts_standalone_generated_and_custom_methods_and_legacy_methods() {
    let session = RecordingSession::default();
    let requests = Arc::clone(&session.requests);
    let bot = Bot::with_client("123:token", session);

    assert!(bot
        .send(telers_methods::DeleteMessage::new(42, 5))
        .await
        .unwrap());
    assert_eq!(bot.send(StandaloneMethod).await.unwrap(), 43);
    assert_eq!(bot.send(LegacyMethod).await.unwrap(), 42);

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    assert_eq!(requests[0].0, "deleteMessage");
    let data: serialization::Value = serialization::from_str(&requests[0].1).unwrap();
    let expected: serialization::Value =
        serialization::from_str(r#"{"chat_id":42,"message_id":5}"#).unwrap();
    assert_eq!(data, expected);
    assert_eq!(requests[1], ("standalone", "7".to_owned()));
    assert_eq!(requests[2], ("legacy", "123".to_owned()));
}

#[test]
fn framework_paths_keep_shared_method_request_response_and_either_identity() {
    let method: telers_methods::methods::SendMessage =
        telers::methods::send_message::SendMessage::new(42, "hello");
    let bot = Bot::with_client("123:token", ());
    let request: telers::methods::base::Request<_> = method.build_request(&bot);
    let request: telers_methods::base::Request<_> = request;
    assert_eq!(request.method_name, "sendMessage");

    let legacy =
        <LegacyMethod as telers::methods::base::TelegramMethod>::build_request(LegacyMethod, &bot);
    assert_eq!(legacy.data, 123);

    let response: telers::methods::Response<telers::types::User> =
        telers::methods::GetMe::build_response(
            r#"{"ok":true,"result":{"id":42,"is_bot":true,"first_name":"Bot"}}"#,
        )
        .unwrap();
    let response: telers_methods::Response<telers_types::types::User> = response;
    assert_eq!(response.result.unwrap().id, 42);

    let error: telers_methods::Either<std::io::Error, std::fmt::Error> =
        telers::Either::Left(std::io::Error::other("shared error"));
    let error: anyhow::Error = error.into();
    assert_eq!(error.to_string(), "shared error");
}
