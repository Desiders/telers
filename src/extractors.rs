//! This module contains functionality for extracting data from the event and context to the handler arguments.
//!
//! [`FromEventAndContext`] is the main trait for extracting data from the event and context to the handler arguments.
//!
//! By default [`FromEventAndContext`] is implemented for [`Option`] and [`Result`],
//! so you can use [`Option`] and [`Result`] to make arguments optional and handle errors of extraction.
//!
//! You can implement [`FromEventAndContext`] for your own types and use them as handler arguments.
//! [`FromEventAndContext`] is implemented for the most common filters, middlewares and types.
//!
//! Important, that limit of the number of arguments in the handler is 20.

mod extractor;
mod filters;
mod middlewares;
mod types;

pub use extractor::FromEventAndContext;
