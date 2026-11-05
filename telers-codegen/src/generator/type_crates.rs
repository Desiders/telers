use crate::{generator::doc_utils::collect_telegram_type_names, parser::api::NormalizedSchema};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeCrate {
    Common,
    Input,
    Messages,
    Facade,
}

impl TypeCrate {
    pub const ALL: [Self; 4] = [Self::Common, Self::Input, Self::Messages, Self::Facade];
}

fn dependency_closure(name: &str, graph: &HashMap<&str, HashSet<String>>) -> HashSet<String> {
    let mut dependencies = HashSet::new();
    let mut pending = vec![name.to_owned()];
    while let Some(name) = pending.pop() {
        if dependencies.insert(name.clone())
            && let Some(children) = graph.get(name.as_str())
        {
            pending.extend(children.iter().cloned());
        }
    }
    dependencies
}

#[must_use]
pub fn assign_type_crates(schema: &NormalizedSchema) -> BTreeMap<String, TypeCrate> {
    let graph = schema
        .types
        .iter()
        .map(|(name, ty)| {
            let mut dependencies = HashSet::new();
            for field in &ty.fields {
                collect_telegram_type_names(&field.r#type, &mut dependencies);
            }
            dependencies.extend(ty.subtypes.iter().map(|subtype| subtype.ty_name.clone()));
            (name.as_str(), dependencies)
        })
        .collect::<HashMap<_, _>>();
    let message_dependencies = dependency_closure("Message", &graph);

    schema
        .types
        .keys()
        .map(|name| {
            let dependencies = dependency_closure(name, &graph);
            let owner = if dependencies.contains("Message") {
                // Keep mutually recursive message models in the same crate.
                if message_dependencies.contains(name) {
                    TypeCrate::Messages
                } else {
                    TypeCrate::Facade
                }
            } else if dependencies
                .iter()
                .any(|name| name.starts_with("Input") || name.starts_with("InlineQueryResult"))
            {
                TypeCrate::Input
            } else {
                TypeCrate::Common
            };
            (name.clone(), owner)
        })
        .collect()
}
