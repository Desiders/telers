/// Implements [`super::FromEventAndContext`] for types that impl [`From`] for [`crate::types::Update`].
/// # Syntax
/// ```ignore
/// from_update!(
///     [client_generic_or_type] |
///         [client_generic_or_type, generic1, ...] |.
///         [client_generic_or_type, generic1: generic1_bound + ..., ...] |
///         [client_generic_or_type, generic1: generic1_bound + ... + 'static, ...] // `'static` lifetime is implicitly added to generics
///     ,
///     type_to_extract |
///         type_to_extract<generic1, ...>
///     ,
/// );
/// ```
/// # Examples
/// ```ignore
/// from_update!([Client], UpdateKind);
/// ```
#[allow(clippy::module_name_repetitions)]
#[macro_export]
macro_rules! from_update {
    (
        [
            $client_generic_or_type:ident
            $(,
                $generic:tt
                $(:
                    $generic_bound:tt $(+ $generic_bounds:tt)*
                )?
            )* $(,)?
        ],
        $type_name:ty $(,)?
    ) => {
        impl<
            $client_generic_or_type,
            $(
                $generic $(:
                    $generic_bound $(+ $generic_bounds)*
                )?
            ),*
        > FromEventAndContext<$client_generic_or_type> for $type_name
        where
            $type_name: 'static,
        {
            type Error = Infallible;

            fn extract(
                _bot: Arc<Bot<$client_generic_or_type>>,
                update: Arc<Update>,
                _context: Arc<Context>,
            ) -> Result<Self, Self::Error> {
                Ok(From::from((*update).clone()))
            }
        }
    };
}

/// Implements [`super::FromEventAndContext`] for types that impl [`TryFrom`] for [`crate::types::Update`].
/// # Syntax
/// ```ignore
/// try_from_update!(
///     [client_generic_or_type] |
///         [client_generic_or_type, generic1, ...] |.
///         [client_generic_or_type, generic1: generic1_bound + ..., ...] |
///         [client_generic_or_type, generic1: generic1_bound + ... + 'static, ...] // `'static` lifetime is implicitly added to generics
///     ,
///     type_to_extract |
///         type_to_extract<generic1, ...>
///     ,
/// );
/// ```
/// # Examples
/// ```ignore
/// try_from_update!([Client], Message);
/// try_from_update!([Client], MessageText);
/// try_from_update!([Client], MessageAnimation);
/// ```
#[macro_export]
macro_rules! try_from_update {
    (
        [
            $client_generic_or_type:ident
            $(,
                $generic:tt
                $(:
                    $generic_bound:tt $(+ $generic_bounds:tt)*
                )?
            )* $(,)?
        ],
        $type_name:ty $(,)?
    ) => {
        impl<
            $client_generic_or_type,
            $(
                $generic $(:
                    $generic_bound $(+ $generic_bounds)*
                )?
            ),*
        > FromEventAndContext<$client_generic_or_type> for $type_name
        where
            $type_name: TryFrom<Update, Error = ConvertToTypeError> + 'static,
        {
            type Error = ConvertToTypeError;

            fn extract(
                _bot: Arc<Bot<$client_generic_or_type>>,
                update: Arc<Update>,
                _context: Arc<Context>,
            ) -> Result<Self, Self::Error> {
                Self::try_from((*update).clone())
            }
        }
    };
}
