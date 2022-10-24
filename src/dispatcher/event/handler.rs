use crate::{client::Bot, extract::FromEventAndContext, types::Update};

use std::{future::Future, pin::Pin, rc::Rc};

#[allow(clippy::module_name_repetitions)]
pub type HandlerFut<H, Args> = <H as Handler<Args>>::Future;
pub type BoxFutHandlerFut<H, Args> = Pin<Box<dyn Future<Output = HandlerFut<H, Args>>>>;

pub trait Handler<Args>: Clone {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

async fn extract_fut_with_args<H, Args>(
    handler: H,
    bot: Rc<Bot>,
    update: Rc<Update>,
) -> HandlerFut<H, Args>
where
    H: Handler<Args>,
    Args: FromEventAndContext,
{
    match Args::extract(bot.as_ref(), update.as_ref()).await {
        Ok(args) => handler.call(args),
        Err(err) => panic!("Extract error: {}", err.into()),
    }
}

pub fn fut_wrap<H, Args>(handler: H) -> impl Fn(Rc<Bot>, Rc<Update>) -> BoxFutHandlerFut<H, Args>
where
    H: Handler<Args> + 'static,
    Args: FromEventAndContext + 'static,
{
    move |bot: Rc<Bot>, update: Rc<Update>| {
        Box::pin(extract_fut_with_args(handler.clone(), bot, update))
    }
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Fut + Clone,
        Fut: Future,
    {
        type Output = Fut::Output;
        type Future = Fut;

        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
            (self)($($param,)*)
        }
    }
});

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
factory_tuple! { A B C D E F G H I }
factory_tuple! { A B C D E F G H I J }
factory_tuple! { A B C D E F G H I J K }
factory_tuple! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use crate::types::Message;

    use super::*;

    fn assert_impl_handler<T: FromEventAndContext>(_: impl Handler<T>) {}

    #[test]
    fn test_arg_number() {
        async fn handler_min() {}
        async fn handler_max(
            _01: (),
            _02: (),
            _03: (),
            _04: (),
            _05: (),
            _06: (),
            _07: (),
            _08: (),
            _09: (),
            _10: (),
            _11: (),
            _12: (),
        ) {
        }

        assert_impl_handler(handler_min);
        assert_impl_handler(handler_max);
    }

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_call_handler() {
        async fn handler(message: Message) -> Message {
            message
        }

        let message = Message::default();
        let fut = fut_wrap(handler);
        let fut_with_args = r#await!(fut(
            Rc::new(Bot::new()),
            Rc::new(Update {
                message: Some(message.clone()),
                ..Update::default()
            }),
        ));
        let result = r#await!(fut_with_args);

        assert_eq!(result, message);
    }
}
