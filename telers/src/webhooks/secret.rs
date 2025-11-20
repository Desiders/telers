use subtle::ConstantTimeEq as _;

pub(crate) const SECRET_TOKEN_HEADER_NAME: &str = "X-Telegram-Bot-Api-Secret-Token";

#[derive(Debug)]
pub(crate) struct XTelegramBotApiSecretToken(pub(crate) Option<Box<[u8]>>);

pub(crate) fn verify(expected: Option<&[u8]>, received: &XTelegramBotApiSecretToken) -> bool {
    match (expected, received.0.as_deref()) {
        (None, None) => true,
        (Some(expected), Some(received)) => expected.ct_eq(received).into(),
        _ => false,
    }
}
