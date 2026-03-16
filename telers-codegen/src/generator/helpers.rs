#![allow(clippy::too_many_lines, clippy::missing_errors_doc)]

use pluralizer::pluralize;
use quote::format_ident;
use std::fmt::Display;

pub fn format_tokens(tokens: impl Display) -> syn::Result<String> {
    let syntax_tree = syn::parse_file(&tokens.to_string())?;
    Ok(prettyplease::unparse(&syntax_tree))
}

const RESERVED_KEYWORDS: &[&str] = &[
    "type", "self", "ref", "move", "use", "mod", "impl", "trait", "where",
];

#[must_use]
pub fn sanitize_field_name(name: &str) -> proc_macro2::Ident {
    if RESERVED_KEYWORDS.contains(&name) {
        format_ident!("r#{name}")
    } else {
        format_ident!("{name}")
    }
}

#[must_use]
fn sanitize_description(description: &str) -> String {
    let description = description
        .replace("Optional. ", "")
        .replace("True", "`true`")
        .replace("False", "`false`")
        .replace("None", "`null`");
    let description = {
        let mut out = String::with_capacity(description.len());
        let mut in_quotes = false;
        for ch in description.chars() {
            if ch == '"' {
                out.push('`');
                in_quotes = !in_quotes;
            } else {
                out.push(ch);
            }
        }
        if in_quotes {
            out.replace('`', "\"")
        } else {
            out
        }
    };
    let description = {
        let mut out = String::with_capacity(description.len());
        let mut chars = description.chars().peekable();
        let mut in_backticks = false;
        while let Some(ch) = chars.next() {
            if ch == '`' {
                in_backticks = !in_backticks;
                out.push(ch);
                continue;
            }
            if ch == '<' {
                let mut inner = String::new();
                let mut found_end = false;
                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    if next_ch == '>' {
                        found_end = true;
                        break;
                    }
                    inner.push(next_ch);
                }

                if found_end {
                    let is_placeholder = !in_backticks
                        && !inner.contains("://")
                        && !inner.contains('/')
                        && inner
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit());
                    if is_placeholder {
                        out.push('<');
                        out.push('`');
                        out.push_str(&inner);
                        out.push('`');
                        out.push('>');
                    } else {
                        out.push('<');
                        out.push_str(&inner);
                        out.push('>');
                    }
                    continue;
                }
                out.push('<');
                out.push_str(&inner);
                continue;
            }

            out.push(ch);
        }
        out
    };
    let description = description
        .replace("tg://user?id=<`user_id`>", "`tg://user?id=<user_id>`")
        .replace("tg://user?id=<user_id>", "`tg://user?id=<user_id>`")
        .replace(
            "<https://api.telegram.org/file/bot<`token`>/<`file_path`>>",
            "`https://api.telegram.org/file/bot<token>/<file_path>`",
        )
        .replace(
            "<https://api.telegram.org/file/bot<`token`>/<`file_path`>",
            "`https://api.telegram.org/file/bot<token>/<file_path>`",
        )
        .replace(
            "<https://api.telegram.org/file/bot<token>/<file_path>>",
            "`https://api.telegram.org/file/bot<token>/<file_path>`",
        )
        .replace(
            "<https://api.telegram.org/file/bot<token>/<file_path>",
            "`https://api.telegram.org/file/bot<token>/<file_path>`",
        )
        .replace(
            "https://api.telegram.org/file/bot<token>/<file_path>",
            "`https://api.telegram.org/file/bot<token>/<file_path>`",
        )
        .replace(
            "t.me/your_bot?start=XXXX",
            "<https://t.me/your_bot?start=XXXX>",
        )
        .replace(
            ".telegram_payment_charge_id",
            ".`telegram_payment_charge_id`",
        )
        .replace(
            "gift.prepaid_upgrade_star_count",
            "`gift.prepaid_upgrade_star_count`",
        )
        .replace("gift.upgrade_star_count", "`gift.upgrade_star_count`");

    let mut result = String::with_capacity(description.len());
    let mut chars = description.chars().peekable();
    let mut in_backticks = false;

    while let Some(ch) = chars.next() {
        if ch == '`' {
            in_backticks = !in_backticks;
            result.push(ch);
            continue;
        }
        if in_backticks {
            result.push(ch);
            continue;
        }
        if ch.is_uppercase() {
            // Don't split CamelCase fragments that are in the middle of a lower-cased identifier
            // (e.g. `getForumTopicIconStickers`).
            if result
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                result.push(ch);
                continue;
            }

            let mut word = String::from(ch);
            let mut has_camel_case = false;
            let mut prev_uppercase = true;

            while let Some(&next_c) = chars.peek() {
                if !next_c.is_alphanumeric() {
                    break;
                }
                if next_c.is_alphabetic() {
                    if next_c.is_uppercase() && !prev_uppercase {
                        has_camel_case = true;
                    }
                    prev_uppercase = next_c.is_uppercase();
                }
                word.push(next_c);
                chars.next();
            }

            if word.chars().count() > 1 && has_camel_case {
                result.push('[');
                result.push('`');
                result.push_str(&word);
                result.push('`');
                result.push(']');
            } else {
                result.push_str(&word);
            }

            continue;
        }
        result.push(ch);
    }
    let result = result
        .split_whitespace()
        .map(|token| {
            if token.contains("](") || token.starts_with('`') || token.starts_with('<') {
                return token.to_string();
            }

            let start = token
                .find(|c: char| c.is_alphanumeric() || c == '_' || c == '[' || c == 'h')
                .unwrap_or(0);
            let end = token
                .rfind(|c: char| c.is_alphanumeric() || c == '_' || c == ']' || c == '/')
                .map_or(token.len(), |idx| idx + 1);
            let (prefix, rest) = token.split_at(start);
            let (core, suffix) = rest.split_at(end.saturating_sub(start));

            let replaced = if core.starts_with("https://") || core.starts_with("http://") {
                format!("<{core}>")
            } else if core.contains('_')
                && core
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
            {
                format!("`{core}`")
            } else {
                core.to_string()
            };

            format!("{prefix}{replaced}{suffix}")
        })
        .collect::<Vec<_>>()
        .join(" ");

    // Raw schema text often contains unresolved intra-doc links like [`SomeType`]
    // that are not in local module scope. Render them as code to avoid broken links.
    let result = demote_intra_doc_links_to_code(&result);

    result
        .replace(
            "<https://api.telegram.org/file/bot<`token`>/<`file_path>`>",
            "`https://api.telegram.org/file/bot<token>/<file_path>`",
        )
        .replace(
            "<https://api.telegram.org/file/bot<`token`>/<`file_path`>>",
            "`https://api.telegram.org/file/bot<token>/<file_path>`",
        )
}

#[must_use]
fn demote_intra_doc_links_to_code(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(start) = rest.find("[`") {
        out.push_str(&rest[..start]);
        let after_start = &rest[start + 2..];
        if let Some(end) = after_start.find("`]") {
            let content = &after_start[..end];
            if content.contains("::") || content.contains("://") {
                out.push_str("[`");
                out.push_str(content);
                out.push_str("`]");
            } else {
                out.push('`');
                out.push_str(content);
                out.push('`');
            }
            rest = &after_start[end + 2..];
        } else {
            out.push_str(&rest[start..]);
            return out;
        }
    }

    out.push_str(rest);
    out
}

#[must_use]
pub fn format_description(description: &[String], href: &str) -> Vec<String> {
    let mut lines = Vec::with_capacity(description.len() + 2);
    let mut prev_was_list = false;

    for line in description {
        let sanitized = sanitize_description(line);
        let is_list = sanitized.trim_start().starts_with("- ");
        if prev_was_list && !is_list && !sanitized.trim().is_empty() {
            lines.push(String::new());
        }
        lines.push(format!(" {sanitized}"));
        prev_was_list = is_list;
    }

    lines.push(" # Documentation".to_string());
    lines.push(format!(" <{href}>"));
    lines
}

#[must_use]
pub fn format_attr_description(description: &str) -> String {
    format!(" {}", sanitize_description(description))
}

#[must_use]
pub fn get_singular_and_plural_forms(name: &str) -> (String, String) {
    (pluralize(name, 1, false), pluralize(name, 2, false))
}

#[must_use]
pub fn camel_to_snake(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + 3);
    let chars: Vec<char> = input.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            let prev = i.checked_sub(1).map(|j| chars[j]);
            let next = chars.get(i + 1);
            if prev.is_some_and(|p| {
                p.is_lowercase()
                    || p.is_ascii_digit()
                    || (p.is_uppercase() && next.is_some_and(|n| n.is_lowercase()))
            }) {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

#[must_use]
pub fn snake_to_upper_camel(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.extend(c.to_uppercase());
            } else {
                result.push(c);
            }
            capitalize_next = false;
        }
    }

    result
}

#[must_use]
pub fn capitalize(input: &str) -> String {
    let mut chars = input.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}
