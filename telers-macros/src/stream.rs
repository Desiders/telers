use proc_macro2::{TokenStream, TokenTree};

/// Trim chars from the stream.
/// # Arguments
/// * `stream` - Stream to trim chars from
/// * `start` - Char to trim from the start of the stream
/// * `end` - Char to trim from the end of the stream
/// # Returns
/// * If `start` and `end` chars are not passed, then returns `stream` as is without any changes
/// * If `start` char is passed, then returns `stream` without first token if it's `start` char
/// * If `end` char is passed, then returns `stream` without last token if it's `end` char
/// * If `start` and `end` chars are passed, then returns `stream` without first and last tokens if they are `start` and `end` chars
/// # Notes
/// This can be used to trim `<` and `>` chars from the stream for adding generic type to it.
/// Example: `<T, E>, OUR_GENERIC` => `T, E, OUR_GENERIC`.
///
/// This can be use to trim `,` char from the end to avoid multiple commas.
/// Example: `T, E, OUR_GENERIC,` => `T, E, OUR_GENERIC`.
/// # Examples
/// For examples check `tests` module, because I don't want to copy them here.
pub(crate) fn trim_chars(
    stream: TokenStream,
    start: Option<char>,
    end: Option<char>,
) -> TokenStream {
    let mut iter = stream.into_iter().peekable();

    let first = iter.peek();

    // If `start` char is not passed, then we ignore it
    if let Some(start) = start {
        if let Some(TokenTree::Punct(punct)) = first {
            // If the first token is `start` char, then we need to remove it
            if punct.as_char() == start {
                // We need to remove first token
                iter.next();
            }
        }
    }

    // If `end` char is not passed, then we ignore it
    if let Some(end) = end {
        let mut last_token_pos = None;
        let mut last_pos = 0;

        // Iterate over all tokens and find last token with `end` char and it's position, and last position
        for (pos, token) in iter.clone().enumerate() {
            if let TokenTree::Punct(punct) = token {
                if punct.as_char() == end {
                    last_token_pos = Some(pos);
                }
            }

            last_pos = pos;
        }

        if let Some(last_token_pos) = last_token_pos {
            // If last token is `end` char, then we need to remove it
            if last_token_pos == last_pos {
                return iter.take(last_pos).collect();
            }
        }
    }

    iter.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_start_char() {
        use quote::quote;

        let stream = quote! { <T, E> };

        assert_eq!(
            trim_chars(stream, Some('<'), None).to_string(),
            quote! { T, E> }.to_string()
        );

        let stream = quote! { T, E> };

        assert_eq!(
            trim_chars(stream, Some('<'), None).to_string(),
            quote! { T, E> }.to_string()
        );

        let stream = quote! { <T, E };

        assert_eq!(
            trim_chars(stream, Some('<'), None).to_string(),
            quote! { T, E }.to_string()
        );

        let stream = quote! { T, E };

        assert_eq!(
            trim_chars(stream, Some('<'), None).to_string(),
            quote! { T, E }.to_string()
        );
    }

    #[test]
    fn trim_last_char() {
        use quote::quote;

        let stream = quote! { <T, E> };

        assert_eq!(
            trim_chars(stream, None, Some('>')).to_string(),
            quote! { <T, E }.to_string()
        );

        let stream = quote! { T, E> };

        assert_eq!(
            trim_chars(stream, None, Some('>')).to_string(),
            quote! { T, E }.to_string()
        );

        let stream = quote! { <T, E };

        assert_eq!(
            trim_chars(stream, None, Some('>')).to_string(),
            quote! { <T, E }.to_string()
        );

        let stream = quote! { T, E };

        assert_eq!(
            trim_chars(stream, None, Some('>')).to_string(),
            quote! { T, E }.to_string()
        );
    }

    #[test]
    fn trim_start_and_end_chars() {
        use quote::quote;

        let stream = quote! { <T, E> };

        assert_eq!(
            trim_chars(stream, Some('<'), Some('>')).to_string(),
            quote! { T, E }.to_string()
        );

        let stream = quote! { T, E> };

        assert_eq!(
            trim_chars(stream, Some('<'), Some('>')).to_string(),
            quote! { T, E }.to_string()
        );

        let stream = quote! { <T, E };

        assert_eq!(
            trim_chars(stream, Some('<'), Some('>')).to_string(),
            quote! { T, E }.to_string()
        );

        let stream = quote! { T, E };

        assert_eq!(
            trim_chars(stream, Some('<'), Some('>')).to_string(),
            quote! { T, E }.to_string()
        );
    }
}
