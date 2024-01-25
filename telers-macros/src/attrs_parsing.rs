use syn::parse::Parse;

pub(crate) fn parse_attr<T>(ident: &str, attrs: &[syn::Attribute]) -> Result<Option<T>, syn::Error>
where
    T: Parse,
{
    for attr in attrs {
        if attr.path().is_ident(ident) {
            return attr.parse_args::<T>().map(Some);
        }
    }

    Ok(None)
}
