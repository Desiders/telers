use crate::{client::Bot, context::Context as AppContext, error::app::Error, types::Update};

use futures::{
    future::{ok, Ready},
    ready, Future,
};
use pin_project_lite::pin_project;
use std::{
    cell::RefCell,
    convert::Infallible,
    marker::PhantomData,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

/// Trait for extracting data from [Update] and [Context] to handlers' arguments
pub trait FromEventAndContext: Sized {
    type Error: Into<Error>;
    type Future: Future<Output = Result<Self, Self::Error>>;

    fn extract(bot: &Bot, update: &Update, context: Rc<RefCell<AppContext>>) -> Self::Future;
}

impl<T> FromEventAndContext for Option<T>
where
    T: FromEventAndContext,
{
    type Error = Infallible;
    type Future = FromEventAndContextOptFuture<T::Future>;

    #[inline]
    fn extract(bot: &Bot, update: &Update, context: Rc<RefCell<AppContext>>) -> Self::Future {
        FromEventAndContextOptFuture {
            fut: T::extract(bot, update, context),
        }
    }
}

pin_project! {
    pub struct FromEventAndContextOptFuture<Fut> {
        #[pin]
        fut: Fut,
    }
}

impl<Fut, T, E> Future for FromEventAndContextOptFuture<Fut>
where
    Fut: Future<Output = Result<T, E>>,
    E: Into<Error>,
{
    type Output = Result<Option<T>, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = ready!(this.fut.poll(cx));

        match res {
            Ok(t) => Poll::Ready(Ok(Some(t))),
            Err(_) => Poll::Ready(Ok(None)),
        }
    }
}

impl<T, E> FromEventAndContext for Result<T, E>
where
    T: FromEventAndContext,
    T::Error: Into<E>,
{
    type Error = Infallible;
    type Future = FromEventAndContextResFuture<T::Future, E>;

    #[inline]
    fn extract(bot: &Bot, update: &Update, context: Rc<RefCell<AppContext>>) -> Self::Future {
        FromEventAndContextResFuture {
            fut: T::extract(bot, update, context),
            _phantom: PhantomData,
        }
    }
}

pin_project! {
    pub struct FromEventAndContextResFuture<Fut, E> {
        #[pin]
        fut: Fut,
        _phantom: PhantomData<E>,
    }
}

impl<Fut, T, Ei, E> Future for FromEventAndContextResFuture<Fut, E>
where
    Fut: Future<Output = Result<T, Ei>>,
    Ei: Into<E>,
{
    type Output = Result<Result<T, E>, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = ready!(this.fut.poll(cx));

        Poll::Ready(Ok(res.map_err(Into::into)))
    }
}

#[allow(non_snake_case)]
mod tuple_from_req {
    use super::{
        ok, pin_project, AppContext, Bot, Context, Error, FromEventAndContext, Future, Infallible,
        Pin, Poll, Rc, Ready, RefCell, Update,
    };

    macro_rules! tuple_from_req {
        ($fut:ident; $($T:ident),*) => {
            /// `FromEventAndContext` implementation for tuple
            #[allow(unused_parens)]
            impl<$($T: FromEventAndContext + 'static),+> FromEventAndContext for ($($T,)+)
            {
                type Error = Error;
                type Future = $fut<$($T),+>;

                fn extract(bot: &Bot, update: &Update, context: Rc<RefCell<AppContext>>) -> Self::Future {
                    $fut {
                        $(
                            $T: ExtractFuture::Future {
                                fut: $T::extract(bot, update, Rc::clone(&context)),
                            },
                        )+
                    }
                }
            }

            pin_project! {
                pub struct $fut<$($T: FromEventAndContext),+> {
                    $(
                        #[pin]
                        $T: ExtractFuture<$T::Future, $T>,
                    )+
                }
            }

            impl<$($T: FromEventAndContext),+> Future for $fut<$($T),+>
            {
                type Output = Result<($($T,)+), Error>;

                fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                    let mut this = self.project();

                    let mut ready = true;
                    $(
                        match this.$T.as_mut().project() {
                            ExtractProj::Future { fut } => match fut.poll(cx) {
                                Poll::Ready(Ok(output)) => {
                                    this.$T.as_mut().project_replace(ExtractFuture::Done { output });
                                },
                                Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                                Poll::Pending => ready = false,
                            },
                            ExtractProj::Done { .. } => {},
                            ExtractProj::Empty => unreachable!("FromEventAndContext polled after finished"),
                        }
                    )+

                    if ready {
                        Poll::Ready(Ok(
                            ($(
                                match this.$T.project_replace(ExtractFuture::Empty) {
                                    ExtractReplaceProj::Done { output } => output,
                                    _ => unreachable!("FromEventAndContext polled after finished"),
                                },
                            )+)
                        ))
                    } else {
                        Poll::Pending
                    }
                }
            }
        };
    }

    pin_project! {
        #[project = ExtractProj]
        #[project_replace = ExtractReplaceProj]
        enum ExtractFuture<Fut, Res> {
            Future {
                #[pin]
                fut: Fut
            },
            Done {
                output: Res,
            },
            Empty
        }
    }

    /// To be able to use [Handler] without arguments
    impl FromEventAndContext for () {
        type Error = Infallible;
        type Future = Ready<Result<Self, Self::Error>>;

        fn extract(_: &Bot, _: &Update, _: Rc<RefCell<AppContext>>) -> Self::Future {
            ok(())
        }
    }

    tuple_from_req! { TupleFromEventAndContext1; A }
    tuple_from_req! { TupleFromEventAndContext2; A, B }
    tuple_from_req! { TupleFromEventAndContext3; A, B, C }
    tuple_from_req! { TupleFromEventAndContext4; A, B, C, D }
    tuple_from_req! { TupleFromEventAndContext5; A, B, C, D, E }
    tuple_from_req! { TupleFromEventAndContext6; A, B, C, D, E, F }
    tuple_from_req! { TupleFromEventAndContext7; A, B, C, D, E, F, G }
    tuple_from_req! { TupleFromEventAndContext8; A, B, C, D, E, F, G, H }
    tuple_from_req! { TupleFromEventAndContext9; A, B, C, D, E, F, G, H, I }
    tuple_from_req! { TupleFromEventAndContext10; A, B, C, D, E, F, G, H, I, J }
    tuple_from_req! { TupleFromEventAndContext11; A, B, C, D, E, F, G, H, I, J, K }
    tuple_from_req! { TupleFromEventAndContext12; A, B, C, D, E, F, G, H, I, J, K, L }
}
