use crate::parser::api::TypeKindInField;
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
