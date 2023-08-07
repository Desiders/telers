/// Implements [`super::FromEventAndContext`] for types that can be extracts from the context by key
/// # Syntax
/// ```ignore
/// from_context_impl!(
///     [client_generic_or_type] |
///         [client_generic_or_type, generic1, ...] |.
///         [client_generic_or_type, generic1: generic1_bound + ..., ...] |
///         [client_generic_or_type, generic1: generic1_bound + ... + 'static, ...] // `'static` lifetime is implicitly added to generics
///     ,
///     type_to_extract |
///         type_to_extract<generic1, ...>
///     ,
///     key_in_context (optional) // if not specified, search will be by type (expensive operation)
///     ,
/// );
/// ```
/// # Notes
/// - If you want to search data in the context by key, you must specify it in the third argument or search will be by type (expensive operation)
/// - This macro throws a runtime extraction error if the data in the context by key has wrong type or no found
/// - Lifetimes except `'static` aren't supported. `'static` lifetime is implicitly adding to generics and types that are passes to extract
/// - Data to extract must be cloneable, it's cloned from reference to context
/// - First generic or type must be a client
/// - Trailing comma is optional
/// # Variants
/// * Variant with client generic or type and generics
/// * Variant with client generic or type and no generics
/// * Variant with client generic or type, generics and no key
/// * Variant with client generic or type, no generics and no key
/// # Examples
/// ```ignore
/// use std::sync::Arc;
/// use telers::{
///     errors::ExtractionError,
///     event::{telegram::HandlerResult, EventReturn},
///     extractors::{from_context_impl, FromEventAndContext},
///     types::Update,
///     Bot, Context,
/// };
///
/// #[derive(Clone)]
/// struct A;
///
/// #[derive(Debug, Clone)]
/// struct B<T, U>(T, U);
///
/// from_context_impl!([Client], A, "a");
/// from_context_impl!([Client, T: Clone, U: Clone], B<T, U>, "b");
///
/// async fn handler<T: Clone, U: Clone>(_bot: Bot, _a: A, _b: B<T, U>) -> HandlerResult {
///    todo!()
/// }
/// ```
#[allow(clippy::module_name_repetitions)]
#[macro_export]
macro_rules! from_context_impl {
    // Variant with client generic or type and generics
    (
        [
            $client_generic_or_type:ident,
            $(
                $generic:tt
                $(:
                    $generic_bound:tt $(+ $generic_bounds:tt)*
                )?
            ),* $(,)?
        ],
        $(
            $type_name:ty
        ),+,
        $key:literal $(,)?
    ) => {
        impl<
            $client_generic_or_type,
            $(
                $generic $(:
                    $generic_bound $(+ $generic_bounds)*
                )?
            ),*
        > FromEventAndContext<$client_generic_or_type> for $($type_name),+
        where
            $($type_name: 'static),+
        {
            type Error = ExtractionError;

            fn extract(
                _bot: Arc<Bot<$client_generic_or_type>>,
                _update: Arc<Update>,
                context: Arc<Context>,
            ) -> Result<Self, Self::Error> {
                match context.get($key) {
                    Some(data) => match data.downcast_ref::<Self>() {
                        Some(data) => Ok(data.clone()),
                        None => Err(ExtractionError::new(concat!(
                            "Data in context by key `",
                            $key,
                            "` has wrong type. Expected `",
                            stringify!($($type_name),+),
                            "`, found `",
                            stringify!(any::type_name_of_val(data)),
                        ))),
                    },
                    None => Err(ExtractionError::new(concat!(
                        "No found data in context by key `",
                        $key,
                        '`'
                    ))),
                }
            }
        }
    };

    // Variant with client generic or type and no generics
    (
        [
            $client_generic_or_type:ident $(,)?
        ],
        $(
            $type_name:ty
        ),+,
        $key:literal $(,)?
    ) => {
        from_context_impl!(
            [
                $client_generic_or_type,
            ],
            $(
                $type_name
            ),+,
            $key
        );
    };

    // Variant with client generic or type, generics and no key
    (
        [
            $client_generic_or_type:ident,
            $(
                $generic:tt
                $(:
                    $generic_bound:tt $(+ $generic_bounds:tt)*
                )?
            ),* $(,)?
        ],
        $(
            $type_name:ty
        ),+ $(,)?
    ) => {
        impl<
            $client_generic_or_type,
            $(
                $generic $(:
                    $generic_bound $(+ $generic_bounds)*
                )?
            ),*
        > FromEventAndContext<$client_generic_or_type> for $($type_name),+
        where
            $($type_name: 'static),+
        {
            type Error = ExtractionError;

            fn extract(
                _bot: Arc<Bot<$client_generic_or_type>>,
                _update: Arc<Update>,
                context: Arc<Context>,
            ) -> Result<Self, Self::Error> {
                for ref_multi in context.iter() {
                    if let Some(data) = ref_multi.value().downcast_ref::<Self>() {
                        return Ok(data.clone());
                    };
                }

                Err(ExtractionError::new(concat!(
                    "No found data in context with type `",
                    stringify!($($type_name),+),
                    '`'
                )))
            }
        }
    };

    // Variant with client generic or type, no generics and no key
    (
        [
            $client_generic_or_type:ident $(,)?
        ],
        $(
            $type_name:ty
        ),+ $(,)?
    ) => {
        from_context_impl!(
            [
                $client_generic_or_type,
            ],
            $(
                $type_name
            ),+,
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        errors::ExtractionError,
        extractors::FromEventAndContext,
        types::Update,
    };

    use std::{
        fmt::{self, Debug, Formatter},
        marker::PhantomData,
        sync::Arc,
    };

    #[tokio::test]
    async fn test_extract() {
        #[derive(Debug, Clone, PartialEq)]
        struct A;

        #[derive(Debug, PartialEq)]
        struct B<T> {
            _phantom: PhantomData<T>,
        }

        impl<T> Clone for B<T> {
            fn clone(&self) -> Self {
                Self {
                    _phantom: PhantomData,
                }
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        struct C<T, U>(T, U);

        trait Trait: Send + Sync {}

        impl Trait for A {}

        #[derive(Clone)]
        struct D(Arc<Box<dyn Trait>>);

        impl Debug for D {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_struct("D").finish()
            }
        }

        impl PartialEq for D {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }

        from_context_impl!([Client], A, "a");
        from_context_impl!([Client, T], B<T>, "b");
        from_context_impl!([Client, T: Clone, U: Clone], C<T, U>, "c");
        from_context_impl!([Client], D);

        let bot = Arc::new(Bot::<Reqwest>::default());
        let update = Arc::new(Update::default());
        let context = Arc::new(Context::default());

        context.insert("a", Box::new(A));
        context.insert(
            "b",
            Box::new(B::<i32> {
                _phantom: PhantomData,
            }),
        );
        context.insert("c", Box::new(C(1i32, 2i64)));
        context.insert("no_key", Box::new(D(Arc::new(Box::new(A)))));

        assert_eq!(
            A,
            A::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context)).unwrap()
        );
        assert_eq!(
            B::<i32> {
                _phantom: PhantomData,
            },
            B::<i32>::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context)).unwrap()
        );
        assert_eq!(
            C(1i32, 2i64),
            C::<i32, i64>::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
                .unwrap()
        );
        assert_eq!(
            D(Arc::new(Box::new(A))),
            D::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context)).unwrap()
        );
    }
}
