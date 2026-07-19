use crate::parser::api::{NormalizedSubtypeVariant, TypeKindInField};
use std::collections::HashSet;

pub fn collect_telegram_type_names(kind: &TypeKindInField, out: &mut HashSet<String>) {
    match kind {
        TypeKindInField::Telegram(name) => {
            out.insert(name.clone());
        }
        TypeKindInField::Array(inner) => collect_telegram_type_names(inner, out),
        TypeKindInField::Either(left, right) => {
            collect_telegram_type_names(left, out);
            collect_telegram_type_names(right, out);
        }
        _ => {}
    }
}

#[must_use]
pub fn link_known_type_mentions(doc: &str, names: &HashSet<String>) -> String {
    let mut out = String::with_capacity(doc.len() + 32);
    let mut rest = doc;

    while let Some(pos) = rest.find('`') {
        out.push_str(&rest[..pos]);

        if pos > 0 && rest.as_bytes()[pos - 1] == b'[' {
            out.pop();
            let after_tick = &rest[pos + 1..];
            if let Some(end_tick) = after_tick.find('`') {
                let token = &after_tick[..end_tick];
                let after_end_tick = &after_tick[end_tick + 1..];
                if let Some(after_bracket) = after_end_tick.strip_prefix(']') {
                    if names.contains(token) {
                        out.push_str("[`crate::types::");
                        out.push_str(token);
                        out.push_str("`]");
                    } else {
                        out.push_str("[`");
                        out.push_str(token);
                        out.push_str("`]");
                    }
                    rest = after_bracket;
                    continue;
                }
            }
            out.push('[');
            out.push('`');
            rest = &rest[pos + 1..];
            continue;
        }

        let after_tick = &rest[pos + 1..];
        if let Some(end_tick) = after_tick.find('`') {
            let token = &after_tick[..end_tick];
            if names.contains(token) {
                out.push_str("[`crate::types::");
                out.push_str(token);
                out.push_str("`]");
            } else {
                out.push('`');
                out.push_str(token);
                out.push('`');
            }
            rest = &after_tick[end_tick + 1..];
        } else {
            out.push('`');
            out.push_str(after_tick);
            rest = "";
            break;
        }
    }

    out.push_str(rest);
    out
}

#[must_use]
pub fn normalize_doc_line_prefix(doc: &str) -> String {
    format!(" {}", doc.trim_start())
}

pub fn link_subtype_mentions(doc_lines: &mut [String], subtypes: &[NormalizedSubtypeVariant]) {
    for subtype in subtypes {
        let type_name = &subtype.ty_name;
        let code = format!("`{type_name}`");
        let bare_link = format!("[`{type_name}`]");
        let path_link = format!("[`crate::types::{type_name}`]");
        for line in doc_lines.iter_mut() {
            if line.contains(&code) {
                *line = line.replace(&code, &path_link);
            }
            if line.contains(&bare_link) {
                *line = line.replace(&bare_link, &path_link);
            }
        }
    }
}

#[must_use]
pub fn link_prefixed_type_mentions(lines: Vec<String>, prefix: &str) -> Vec<String> {
    lines
        .into_iter()
        .map(|line| {
            let mut out = String::with_capacity(line.len() + 32);
            let mut rest = line.as_str();

            while let Some(start) = rest.find('`') {
                out.push_str(&rest[..start]);
                let after_start = &rest[start + 1..];
                if let Some(end_rel) = after_start.find('`') {
                    let token = &after_start[..end_rel];
                    if token.starts_with(prefix)
                        && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        out.push_str("[`crate::types::");
                        out.push_str(token);
                        out.push_str("`]");
                    } else {
                        out.push('`');
                        out.push_str(token);
                        out.push('`');
                    }
                    rest = &after_start[end_rel + 1..];
                } else {
                    // Unmatched backtick: keep the remainder as is. `rest` must be cleared,
                    // otherwise the trailing push below would append it a second time.
                    out.push_str(&rest[start..]);
                    rest = "";
                    break;
                }
            }

            out.push_str(rest);
            out
        })
        .collect()
}
