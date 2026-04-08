//! Stateful selection widgets.
//!
//! These widgets persist their state in `widget_data`:
//! - [`Radio`] stores one selected id
//! - [`Toggle`] stores one selected id and cycles through items
//! - [`Multiselect`] stores several selected ids

mod multiselect;
mod radio;
mod toggle;

pub use multiselect::Multiselect;
pub use radio::Radio;
pub use toggle::Toggle;

#[cfg(test)]
mod tests;
