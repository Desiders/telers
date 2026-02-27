use crate::{
    file::camel_to_filename,
    generator::helpers::{
        camel_to_snake, format_attr_description, format_description, get_singular_and_plural_forms,
        sanitize_field_name,
    },
    parser::api::{
        IntegerKind, NormalizedField, NormalizedSchema, NormalizedSubtypeVariant, NormalizedType,
        SubtypeKind, TypeKindInField,
    },
};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::{HashMap, HashSet};

impl ToTokens for TypeKindInField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = match self {
            TypeKindInField::String => quote! { Box<str> },
            TypeKindInField::Integer(kind) => match kind {
                IntegerKind::Int8 => quote! { i8 },
                IntegerKind::Int16 => quote! { i16 },
                IntegerKind::Int32 => quote! { i32 },
                IntegerKind::Int64 => quote! { i64 },
                IntegerKind::UInt8 => quote! { u8 },
                IntegerKind::UInt16 => quote! { u16 },
                IntegerKind::UInt32 => quote! { u32 },
                IntegerKind::UInt64 => quote! { u64 },
                IntegerKind::Float32 => quote! { f32 },
                IntegerKind::Float64 => quote! { f64 },
            },
            TypeKindInField::Boolean(_) => quote! { bool },
            TypeKindInField::Telegram(name) => {
                let ident = format_ident!("{name}");
                quote! { #ident }
            }
            TypeKindInField::InputFile => quote! { InputFile },
            TypeKindInField::ChatId => quote! { ChatIdKind },
            TypeKindInField::Array(inner) => quote! { Box<[#inner]> },
            TypeKindInField::Either(left, right) => quote! { crate::Either<#left, #right> },
        };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedSubtypeVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (variant, name) = (
            format_ident!("{}", self.variant),
            format_ident!("{}", self.ty_name),
        );
        tokens.extend(quote! { #variant(#name), });
    }
}

impl ToTokens for NormalizedField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = sanitize_field_name(&self.name);
        let doc = format_attr_description(&self.description);
        let raw_ty = &self.r#type;
        let ty = if self.is_recursive || self.is_boxed {
            quote! { Box<#raw_ty> }
        } else {
            quote! { #raw_ty }
        };

        let ts = if self.required {
            quote! { #[doc = #doc] pub #name: #ty, }
        } else {
            quote! {
                #[doc = #doc]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #name: Option<#ty>,
            }
        };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.as_str());
        let doc_lines = format_description(&self.description, &self.href);

        let (tag_field, parent_tag_field) = self
            .subtype_kind
            .as_ref()
            .map(|kind| kind.get_tags())
            .unwrap_or_default();

        let derive_quotes = get_derives_for_types(self);
        let ts = if self.subtypes.is_empty() {
            let fields = self
                .fields
                .iter()
                .filter(|f| !f.is_tagged(tag_field, parent_tag_field));
            let extra_field = if self.has_extra_fields {
                quote! {
                    #[serde(flatten)]
                    pub _extra: BTreeMap<Box<str>, serde_json::Value>,
                }
            } else {
                quote! {}
            };
            quote! {
                #( #[doc = #doc_lines] )*
                #( #derive_quotes )*
                pub struct #name {
                    #( #fields )*
                    #extra_field
                }
            }
        } else {
            let serde_attr = match &self.subtype_kind {
                Some(SubtypeKind::Tagged { tag_field, .. }) => {
                    quote! { #[serde(tag = #tag_field, rename_all = "snake_case")] }
                }
                Some(SubtypeKind::Untagged | SubtypeKind::UntaggedInTagged { .. }) => {
                    quote! { #[serde(untagged)] }
                }
                None => quote! {},
            };
            let subtypes = self.subtypes.iter();
            quote! {
                #( #[doc = #doc_lines] )*
                #( #derive_quotes )*
                #serde_attr
                pub enum #name {
                    #( #subtypes )*
                }
            }
        };

        tokens.extend(ts);
    }
}

pub fn get_from_impls_for_subtypes(type_quote: &NormalizedType) -> Vec<TokenStream> {
    let name = format_ident!("{}", type_quote.name);

    let mut impl_quotes = vec![];
    for subtype in &type_quote.subtypes {
        let subtype_name = format_ident!("{}", subtype.ty_name);
        let subtype_variant = format_ident!("{}", subtype.variant);
        impl_quotes.push(quote! {
            impl From<#subtype_name> for #name {
                #[inline]
                fn from(val: #subtype_name) -> Self {
                    Self::#subtype_variant(val)
                }
            }
            impl TryFrom<#name> for #subtype_name {
                type Error = crate::errors::ConvertToTypeError;
                #[inline]
                fn try_from(val: #name) -> Result<Self, Self::Error> {
                    if let #name::#subtype_variant(inner) = val {
                        Ok(inner)
                    } else {
                        Err(Self::Error::new(stringify!(#name), stringify!(#subtype_name)))
                    }
                }
            }
        });
    }
    impl_quotes
}

pub fn get_impls_for_types(
    type_quote: &NormalizedType,
    schema: &NormalizedSchema,
) -> Vec<TokenStream> {
    let name = format_ident!("{}", type_quote.name);

    let mut impl_quotes = vec![];

    if type_quote.is_update_variant() {
        let ty_field = type_quote
            .update_variant_ty_field()
            .expect("Update variant doesn't have type field");
        let field_name = format_ident!("{}", ty_field.name);
        let field_ty = &ty_field.r#type;

        let body = if ty_field.is_boxed || ty_field.is_recursive {
            quote! { *val.#field_name }
        } else {
            quote! { val.#field_name }
        };

        impl_quotes.push(quote! {
            impl From<#name> for #field_ty {
                #[inline]
                fn from(val: #name) -> Self {
                    #body
                }
            }
        });
        impl_quotes.push(quote! {
            impl<Client> crate::Extractor<Client> for #name
            {
                type Error = crate::errors::ConvertToTypeError;

                #[inline]
                fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                    let val = TryFrom::try_from((*request.update).clone());
                    async move { val }
                }
            }
        });
    }

    if type_quote.is_update() {
        impl_quotes.push(quote! {
            impl<Client> crate::Extractor<Client> for Update
            {
                type Error = std::convert::Infallible;

                #[inline]
                fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                    let val = (*request.update).clone();
                    async move { Ok(val) }
                }
            }
        });
        impl_quotes.push(quote! {
            impl<Client> crate::Extractor<Client> for std::sync::Arc<Update>
            {
                type Error = std::convert::Infallible;

                #[inline]
                fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                    let val = request.update.clone();
                    async move { Ok(val) }
                }
            }
        });

        // We need to collect all types that can be got from update variants,
        // for example `Message` from `UpdateMessage` and `UpdateBusinessMessage`,
        // so that we can generate `impl From<Update> for Message` for each of variants.
        let mut types_update_variants_with_field: HashMap<&str, Vec<_>> = HashMap::new();

        for subtype in &type_quote.subtypes {
            let variant_ty = schema.types.get(&subtype.ty_name).unwrap();
            let variant_field_ty = variant_ty
                .update_variant_ty_field()
                .expect("Update variant doesn't have type field");
            let TypeKindInField::Telegram(variant_field_ty_name) = &variant_field_ty.r#type else {
                panic!("Update variant type field must be Telegram type");
            };

            types_update_variants_with_field
                .entry(variant_field_ty_name.as_str())
                .or_default()
                .push((subtype.variant.as_str(), variant_field_ty));
        }

        for (variant_field_ty_name_str, variants_with_field) in types_update_variants_with_field {
            let variant_field_ty_name = format_ident!("{}", variant_field_ty_name_str);

            let mut match_arms = vec![];
            for (variant, ty_field) in variants_with_field {
                let variant = format_ident!("{}", variant);
                let field_name = format_ident!("{}", ty_field.name);

                let body = if ty_field.is_boxed || ty_field.is_recursive {
                    quote! { *val.#field_name }
                } else {
                    quote! { val.#field_name }
                };

                match_arms.push(quote! {
                    Update::#variant(val) => Ok(#body)
                });
            }
            match_arms.push(quote! {
                _ => Err(crate::errors::ConvertToTypeError::new(stringify!(Update), stringify!(#variant_field_ty_name)))
            });

            impl_quotes.push(quote! {
                impl TryFrom<Update> for #variant_field_ty_name {
                    type Error = crate::errors::ConvertToTypeError;
                    #[inline]
                    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
                        match val {
                            #(#match_arms),*
                        }
                    }
                }
            });
            impl_quotes.push(quote! {
                impl<Client> crate::Extractor<Client> for #variant_field_ty_name
                {
                    type Error = crate::errors::ConvertToTypeError;

                    #[inline]
                    fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                        let val = (*request.update).clone().try_into();
                        async move { val }
                    }
                }
            });

            // If update inner type is an enum (for example `Message`), also expose
            // extractors for each of its variants (for example `MessageText`).
            let Some(inner_ty) = schema.types.get(variant_field_ty_name_str) else {
                continue;
            };
            if inner_ty.subtypes.is_empty() {
                continue;
            }

            let mut seen_subtype_names = HashSet::new();
            for subtype in &inner_ty.subtypes {
                if !seen_subtype_names.insert(subtype.ty_name.as_str()) {
                    continue;
                }

                let subtype_ty_name = format_ident!("{}", subtype.ty_name);
                impl_quotes.push(quote! {
                    impl TryFrom<Update> for #subtype_ty_name {
                        type Error = crate::errors::ConvertToTypeError;
                        #[inline]
                        fn try_from(val: Update) -> Result<Self, Self::Error> {
                            let parent: #variant_field_ty_name = val.try_into()?;
                            parent.try_into()
                        }
                    }
                });
                impl_quotes.push(quote! {
                    impl<Client> crate::Extractor<Client> for #subtype_ty_name
                    {
                        type Error = crate::errors::ConvertToTypeError;
                        #[inline]
                        fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                            let val = (*request.update).clone().try_into();
                            async move { val }
                        }
                    }
                });
            }
        }
    }

    impl_quotes
}

pub fn get_derives_for_types(_type_quote: &NormalizedType) -> Vec<TokenStream> {
    let derive_quotes = vec![
        quote! { #[derive(Clone, Debug)] },
        quote! { #[derive(Serialize, Deserialize)] },
    ];

    derive_quotes
}

pub fn builder_impl_for_type(type_quote: &NormalizedType) -> TokenStream {
    if !type_quote.subtypes.is_empty() {
        return quote! {};
    }

    let type_name = format_ident!("{}", type_quote.name);

    let (tag_field, parent_tag_field) = type_quote
        .subtype_kind
        .as_ref()
        .map(|kind| kind.get_tags())
        .unwrap_or_default();

    let fields: Box<[_]> = type_quote
        .fields
        .iter()
        .filter(|&f| !f.is_tagged(tag_field, parent_tag_field))
        .collect();
    let required_fields: Box<[_]> = fields.iter().filter(|&&f| f.required).copied().collect();
    let optional_fields: Box<[_]> = fields.iter().filter(|&&f| !f.required).copied().collect();

    let new_method_ts = {
        let doc_creates = format_attr_description(&format!("Creates a new {}.", type_quote.name));
        let mut doc_lines: Vec<TokenStream> = vec![quote! { #[doc = #doc_creates] }];

        if !required_fields.is_empty() {
            let doc_args = format_attr_description("# Arguments");
            doc_lines.push(quote! { #[doc = ""] #[doc = #doc_args] });
            for &field in &required_fields {
                let doc =
                    format_attr_description(&format!("* {} - {}", field.name, field.description));
                doc_lines.push(quote! { #[doc = #doc] });
            }
        }
        if !optional_fields.is_empty() {
            let doc_notes = format_attr_description("# Notes");
            let doc_opt = format_attr_description("Use builder methods to set optional fields.");
            doc_lines.push(quote! { #[doc = ""] #[doc = #doc_notes] #[doc = #doc_opt] });
        }

        if fields.is_empty() {
            let extra_field = if type_quote.has_extra_fields {
                quote! {
                    _extra: BTreeMap::new(),
                }
            } else {
                quote! {}
            };
            quote! {
                #( #doc_lines )*
                #[must_use]
                pub const fn new() -> Self {
                    Self {
                        #extra_field
                    }
                }
            }
        } else {
            let new_generics: Vec<_> = required_fields
                .iter()
                .enumerate()
                .flat_map(|(i, field)| {
                    let ty = &field.r#type;
                    let t = format_ident!("T{i}");
                    if let TypeKindInField::Array(inner) = ty {
                        let t_item = format_ident!("T{i}Item");
                        vec![
                            quote! { #t_item: Into<#inner> },
                            quote! { #t: IntoIterator<Item = #t_item> },
                        ]
                    } else {
                        vec![quote! { #t: Into<#ty> }]
                    }
                })
                .collect();
            let new_args: Vec<_> = required_fields
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let name = sanitize_field_name(&field.name);
                    let t = format_ident!("T{i}");
                    quote! { #name: #t }
                })
                .collect();
            let new_init = fields.iter().map(|field| {
                let name = sanitize_field_name(&field.name);
                if field.required {
                    if let TypeKindInField::Array(_) = &field.r#type {
                        quote! { #name: #name.into_iter().map(Into::into).collect() }
                    } else if field.is_recursive || field.is_boxed {
                        quote! { #name: Box::new(#name.into()) }
                    } else {
                        quote! { #name: #name.into() }
                    }
                } else {
                    quote! { #name: None }
                }
            });
            let extra_field = if type_quote.has_extra_fields {
                quote! {
                    _extra: BTreeMap::new(),
                }
            } else {
                quote! {}
            };
            quote! {
                #( #doc_lines )*
                #[must_use]
                pub fn new<#( #new_generics ),*>(#( #new_args ),*) -> Self {
                    Self {
                        #( #new_init, )*
                        #extra_field
                    }
                }
            }
        }
    };

    let default_impl_ts = if required_fields.is_empty() {
        quote! {
            impl Default for #type_name {
                fn default() -> Self { Self::new() }
            }
        }
    } else {
        quote! {}
    };

    let builder_methods_ts: Vec<_> = fields
        .iter()
        .flat_map(|field| {
            let mut methods = vec![];
            let name = sanitize_field_name(&field.name);
            let ty = &field.r#type;

            if let TypeKindInField::Array(inner) = ty {
                let (singular, plural) = get_singular_and_plural_forms(&field.name);
                let singular_name = sanitize_field_name(&singular);
                let plural_name = sanitize_field_name(&plural);

                let make_doc = |note: &str| {
                    let desc = format_attr_description(&field.description);
                    let notes = format_attr_description("# Notes");
                    let note = format_attr_description(note);
                    quote! { #[doc = #desc] #[doc = ""] #[doc = #notes] #[doc = #note] }
                };
                let make_body = |is_many: bool| -> TokenStream {
                    let op = if is_many {
                        quote! { val.into() }
                    } else {
                        quote! { Some(val.into()) }
                    };
                    if field.required {
                        quote! {
                            Self { #name: self.#name.into_vec().into_iter().chain(#op).collect(), ..self }
                        }
                    } else {
                        quote! {
                            Self { #name: Some(self.#name.unwrap_or_default().into_vec().into_iter().chain(#op).collect()), ..self }
                        }
                    }
                };

                let (plural_doc, plural_body) = (make_doc("Adds multiple elements."), make_body(true));
                methods.push(quote! {
                    #plural_doc
                    #[must_use]
                    pub fn #plural_name<T: Into<#ty>>(self, val: T) -> Self {
                        #plural_body
                    }
                });

                if singular_name != plural_name {
                    let (singular_doc, singular_body) = (make_doc("Adds a single element."), make_body(false));
                    methods.push(quote! {
                        #singular_doc
                        #[must_use]
                        pub fn #singular_name<T: Into<#inner>>(self, val: T) -> Self {
                            #singular_body
                        }
                    });
                }

                if !field.required {
                    let option_doc = make_doc("Adds a single element.");
                    let option_name = format_ident!("{}_option", field.name);
                    methods.push(quote! {
                        #option_doc
                        #[must_use]
                        pub fn #option_name<T: Into<#ty>>(self, val: Option<T>) -> Self {
                            Self { #name: val.map(Into::into), ..self }
                        }
                    });
                }
            } else {
                let doc = format_attr_description(&field.description);
                let boxed = field.is_recursive || field.is_boxed;
                let inner_value = if boxed { quote! { Box::new(val.into()) } } else { quote! { val.into() } };
                let value = if field.required { inner_value.clone() } else { quote! { Some(#inner_value) } };

                methods.push(quote! {
                    #[doc = #doc]
                    #[must_use]
                    pub fn #name<T: Into<#ty>>(self, val: T) -> Self {
                        Self { #name: #value, ..self }
                    }
                });

                if !field.required {
                    let opt_value = if boxed {
                        quote! { val.map(|val| Box::new(val.into())) }
                    } else {
                        quote! { val.map(Into::into) }
                    };
                    let method_name = format_ident!("{}_option", field.name);
                    methods.push(quote! {
                        #[doc = #doc]
                        #[must_use]
                        pub fn #method_name<T: Into<#ty>>(self, val: Option<T>) -> Self {
                            Self { #name: #opt_value, ..self }
                        }
                    });
                }
            }
            methods
        })
        .collect();

    quote! {
        impl #type_name {
            #new_method_ts
            #( #builder_methods_ts )*
        }
        #default_impl_ts
    }
}

fn collect_common_fields<'a>(
    ty: &'a NormalizedType,
    schema: &'a NormalizedSchema,
) -> HashMap<&'a str, (&'a NormalizedField, bool, bool)> {
    let (tag_field, parent_tag_field) = ty
        .subtype_kind
        .as_ref()
        .map(|k| k.get_tags())
        .unwrap_or_default();

    if ty.subtypes.is_empty() {
        ty.fields
            .iter()
            .filter(|f| !f.is_tagged(tag_field, parent_tag_field))
            .map(|f| (f.name.as_str(), (f, f.required, true)))
            .collect()
    } else {
        let mut map: HashMap<&str, Vec<&NormalizedField>> = HashMap::new();
        for subtype in &ty.subtypes {
            let sub_ty = schema.types.get(&subtype.ty_name).unwrap();
            // ↓ use the subtype's own tag context, not the parent's
            let (sub_tag, sub_parent_tag) = sub_ty
                .subtype_kind
                .as_ref()
                .map(|k| k.get_tags())
                .unwrap_or_default();
            for field in &sub_ty.fields {
                if !field.is_tagged(tag_field, parent_tag_field)
                    && !field.is_tagged(sub_tag, sub_parent_tag)
                {
                    map.entry(field.name.as_str()).or_default().push(field);
                }
            }
        }
        map.into_iter()
            .filter(|(_, fields)| {
                let first_ty = &fields[0].r#type;
                fields.iter().all(|f| &f.r#type == first_ty)
            })
            .map(|(name, fields)| {
                let is_common = fields.len() == ty.subtypes.len();
                let is_fully_required = is_common && fields.iter().all(|f| f.required);
                (name, (fields[0], is_fully_required, is_common))
            })
            .collect()
    }
}

fn helper_method_return_type(field_ty: &TypeKindInField, fully_required: bool) -> TokenStream {
    match field_ty {
        TypeKindInField::Array(inner) if fully_required => quote! { &[#inner] },
        TypeKindInField::Array(inner) => quote! { Option<&[#inner]> },
        TypeKindInField::String if fully_required => quote! { &str },
        TypeKindInField::String => quote! { Option<&str> },
        _ if field_ty.is_copy() && fully_required => quote! { #field_ty },
        _ if field_ty.is_copy() => quote! { Option<#field_ty> },
        _ if fully_required => quote! { &#field_ty },
        _ => quote! { Option<&#field_ty> },
    }
}

fn helper_field_accessor_expr(field: &NormalizedField) -> TokenStream {
    let field_ident = sanitize_field_name(&field.name);
    let field_ty = &field.r#type;
    let is_required = field.required;

    if matches!(field_ty, TypeKindInField::Array(_)) {
        if is_required {
            quote! { &*val.#field_ident }
        } else {
            quote! { val.#field_ident.as_deref() }
        }
    } else if field_ty.is_copy() {
        quote! { val.#field_ident }
    } else if field.is_recursive || field.is_boxed || matches!(field_ty, TypeKindInField::String) {
        if is_required {
            quote! { &*val.#field_ident }
        } else {
            quote! { val.#field_ident.as_deref() }
        }
    } else if is_required {
        quote! { &val.#field_ident }
    } else {
        quote! { val.#field_ident.as_ref() }
    }
}

fn nested_inner_accessor_expr(
    inner_ident: &Ident,
    inner_field: &NormalizedField,
    inner_ty: &TypeKindInField,
    inner_is_enum: bool,
    inner_field_fully_required: bool,
    fully_required: bool,
) -> TokenStream {
    if inner_is_enum {
        if inner_field_fully_required && !fully_required {
            quote! { Some(inner.#inner_ident()) }
        } else {
            quote! { inner.#inner_ident() }
        }
    } else if matches!(inner_ty, TypeKindInField::Array(_)) {
        if inner_field.required {
            quote! { Some(&*inner.#inner_ident) }
        } else {
            quote! { inner.#inner_ident.as_deref() }
        }
    } else {
        let is_inner_copy = inner_ty.is_copy();
        let is_inner_boxed = inner_field.is_recursive
            || inner_field.is_boxed
            || matches!(inner_ty, TypeKindInField::String);

        match (
            inner_field.required,
            is_inner_copy,
            is_inner_boxed,
            fully_required,
        ) {
            (true, true, _, true) => quote! { inner.#inner_ident },
            (true, true, _, false) => quote! { Some(inner.#inner_ident) },
            (false, true, ..) => quote! { inner.#inner_ident },
            (true, false, true, true) => quote! { &*inner.#inner_ident },
            (true, false, true, false) => quote! { Some(&*inner.#inner_ident) },
            (false, _, true, _) => quote! { inner.#inner_ident.as_deref() },
            (true, false, false, true) => quote! { &inner.#inner_ident },
            (true, false, false, false) => quote! { Some(&inner.#inner_ident) },
            (false, false, false, _) => quote! { inner.#inner_ident.as_ref() },
        }
    }
}

fn nested_outer_accessor_expr(
    outer_ident: &Ident,
    outer_field: &NormalizedField,
    inner_access: TokenStream,
) -> TokenStream {
    let is_outer_boxed = outer_field.is_recursive || outer_field.is_boxed;
    if outer_field.required {
        let get_inner = if is_outer_boxed {
            quote! { &*val.#outer_ident }
        } else {
            quote! { &val.#outer_ident }
        };
        quote! { { let inner = #get_inner; #inner_access } }
    } else {
        let as_opt = if is_outer_boxed {
            quote! { val.#outer_ident.as_deref() }
        } else {
            quote! { val.#outer_ident.as_ref() }
        };
        quote! { #as_opt.and_then(|inner| #inner_access) }
    }
}

pub fn get_helper_impls_for_type(
    type_quote: &NormalizedType,
    schema: &NormalizedSchema,
) -> TokenStream {
    if type_quote.subtypes.is_empty() {
        return quote! {};
    }

    let type_name = format_ident!("{}", type_quote.name);
    let mut methods = vec![];

    let (tag_field, parent_tag_field) = type_quote
        .subtype_kind
        .as_ref()
        .map(|kind| kind.get_tags())
        .unwrap_or_default();
    let mut fields_subtypes_map: HashMap<&str, Vec<(&NormalizedSubtypeVariant, &NormalizedField)>> =
        HashMap::new();
    for subtype in &type_quote.subtypes {
        let variant = format_ident!("{}", subtype.variant);
        let name = format_ident!("{}", subtype.ty_name);
        let snake = camel_to_snake(&subtype.variant);
        let is_name = format_ident!("is_{snake}_variant");
        let into_name = format_ident!("into_{snake}_variant");

        methods.extend([
            quote! {
                #[must_use]
                pub const fn #is_name(&self) -> bool {
                    if let Self::#variant(_) = self { true } else { false }
                }
            },
            quote! {
                #[must_use]
                pub fn #into_name(self) -> Option<#name> {
                    if let Self::#variant(val) = self { Some(val) } else { None }
                }
            },
        ]);

        let ty = schema.types.get(&subtype.ty_name).unwrap();
        for field in &ty.fields {
            if field.is_tagged(tag_field, parent_tag_field) {
                continue;
            }
            fields_subtypes_map
                .entry(&field.name)
                .or_default()
                .push((subtype, field));
        }
    }

    for (&field_name, subtypes) in &fields_subtypes_map {
        let method_name = sanitize_field_name(&field_name);
        let field = &subtypes[0].1;
        let field_ty = &field.r#type;

        let is_identical_field_type = subtypes.iter().all(|(_, f)| f.r#type == *field_ty);
        if !is_identical_field_type {
            continue;
        }
        let is_common = subtypes.len() == type_quote.subtypes.len();
        let is_required_for_all = is_common && subtypes.iter().all(|(_, f)| f.required);
        let return_ty = helper_method_return_type(field_ty, is_required_for_all);

        let doc_helper = format_attr_description(&format!("Helper method for field {field_name}."));
        let doc_variants = format_attr_description("# Variants");
        let mut doc_lines: Vec<TokenStream> = vec![
            quote! { #[doc = #doc_helper] },
            quote! { #[doc = ""] },
            quote! { #[doc = #doc_variants] },
        ];
        let mut match_arms = vec![];
        for (subtype, field) in subtypes {
            let variant = format_ident!("{}", subtype.variant);

            let doc_field =
                format_attr_description(&format!("- {}. {}", subtype.ty_name, field.description));
            doc_lines.push(quote! { #[doc = #doc_field] });

            let body = helper_field_accessor_expr(field);
            let body = if field.required && !is_required_for_all {
                quote! { Some(#body) }
            } else {
                quote! { #body }
            };

            match_arms.push(quote! {
                Self::#variant(val) => #body
            });
        }
        if !is_common {
            match_arms.push(quote! { _ => None });
        }

        let method = quote! {
            #( #doc_lines )*
            #[must_use]
            pub fn #method_name(&self) -> #return_ty {
                match self { #( #match_arms ),* }
            }
        };

        methods.push(method);
    }

    let mut nested_map: HashMap<
        String,
        Vec<(
            &NormalizedSubtypeVariant,
            &NormalizedField, // outer field
            &NormalizedField, // inner field (representative)
            bool,             // inner_is_enum
            bool,             // inner_field_fully_required (within inner type)
        )>,
    > = HashMap::new();

    for (_, outer_subtypes) in &fields_subtypes_map {
        let first_outer = outer_subtypes[0].1;
        let outer_ty = &first_outer.r#type;

        let TypeKindInField::Telegram(inner_type_name) = outer_ty else {
            continue;
        };
        if !outer_subtypes.iter().all(|(_, f)| &f.r#type == outer_ty) {
            continue;
        }

        let Some(inner_ty) = schema.types.get(inner_type_name.as_str()) else {
            continue;
        };

        let inner_is_enum = !inner_ty.subtypes.is_empty();
        let inner_common = collect_common_fields(inner_ty, schema);

        let (inner_tag, inner_parent_tag) = inner_ty
            .subtype_kind
            .as_ref()
            .map(|k| k.get_tags())
            .unwrap_or_default();

        for (inner_field_name, (inner_field, inner_field_fully_required, _)) in &inner_common {
            if !inner_is_enum && inner_field.is_tagged(inner_tag, inner_parent_tag) {
                continue;
            }
            for (subtype, outer_field) in outer_subtypes {
                nested_map
                    .entry(inner_field_name.to_string())
                    .or_default()
                    .push((
                        subtype,
                        outer_field,
                        inner_field,
                        inner_is_enum,
                        *inner_field_fully_required,
                    ));
            }
        }
    }

    for (inner_field_name, entries) in nested_map {
        let inner_ty = &entries[0].2.r#type;
        if !entries.iter().all(|(_, _, f, ..)| &f.r#type == inner_ty) {
            continue;
        }

        let mut seen = HashSet::new();
        if entries
            .iter()
            .any(|(s, ..)| !seen.insert(s.variant.as_str()))
        {
            continue;
        }

        if fields_subtypes_map.contains_key(inner_field_name.as_str()) {
            continue;
        }

        let method_ident = sanitize_field_name(&inner_field_name);

        let is_all_covered = entries.len() == type_quote.subtypes.len();
        let is_inner_req_all =
            entries
                .iter()
                .all(|(_, _, f, inner_is_enum, inner_field_fully_required)| {
                    if *inner_is_enum {
                        *inner_field_fully_required
                    } else {
                        f.required
                    }
                });
        let fully_required = is_all_covered
            && is_inner_req_all
            && entries.iter().all(|(_, outer, ..)| outer.required);

        let return_ty = helper_method_return_type(inner_ty, fully_required);

        let mut match_arms = vec![];
        for (subtype, outer_field, inner_field, inner_is_enum, inner_field_fully_required) in
            &entries
        {
            let variant = format_ident!("{}", subtype.variant);
            let outer_ident = sanitize_field_name(&outer_field.name);
            let inner_ident = sanitize_field_name(&inner_field.name);
            let inner_access = nested_inner_accessor_expr(
                &inner_ident,
                inner_field,
                inner_ty,
                *inner_is_enum,
                *inner_field_fully_required,
                fully_required,
            );
            let body = nested_outer_accessor_expr(&outer_ident, outer_field, inner_access);

            match_arms.push(quote! { Self::#variant(val) => #body });
        }
        if !is_all_covered {
            match_arms.push(quote! { _ => None });
        }

        let doc = format_attr_description(&format!(
            "Helper method for nested field {inner_field_name}."
        ));
        methods.push(quote! {
            #[doc = #doc]
            #[must_use]
            pub fn #method_ident(&self) -> #return_ty {
                match self { #( #match_arms ),* }
            }
        });
    }

    quote! {
        impl #type_name {
            #( #methods )*
        }
    }
}

pub fn tokenize_type(type_quote: &NormalizedType, schema: &NormalizedSchema) -> TokenStream {
    let mut import_quotes = vec![];
    if type_quote.get_paths_count() > 0 {
        import_quotes.push(quote! { use super::*; });
    }
    if type_quote.has_extra_fields && type_quote.subtypes.is_empty() {
        import_quotes.push(quote! { use std::collections::BTreeMap; });
    }
    import_quotes.push(quote! { use serde::{Serialize, Deserialize}; });

    let type_impls = get_impls_for_types(type_quote, schema);
    let subtype_impls = get_from_impls_for_subtypes(type_quote);
    let builder_impls = builder_impl_for_type(type_quote);
    let helper_impls = get_helper_impls_for_type(type_quote, schema);

    quote! {
        #( #import_quotes )*
        #type_quote
        #builder_impls
        #helper_impls
        #( #type_impls )*
        #( #subtype_impls )*
    }
}

pub fn tokenize_types_mod(type_names: &[&String]) -> TokenStream {
    let mods_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        quote! { pub mod #mod_name; }
    });
    let uses_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        let type_name = format_ident!("{name}");
        quote! { pub use #mod_name::#type_name; }
    });

    quote! {
        pub(crate) mod non_telegram;
        pub use non_telegram::*;
        #( #mods_quote )*
        #( #uses_quote )*
    }
}
