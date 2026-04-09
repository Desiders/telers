//! Stateful selection widgets.
//!
//! These widgets persist their state in `widget_data`:
//! - [`Checkbox`] stores one boolean flag
//! - [`Counter`] stores one numeric value
//! - [`Radio`] stores one selected id
//! - [`Toggle`] stores one selected id and cycles through items
//! - [`Multiselect`] stores several selected ids

mod checkbox;
mod counter;
mod multiselect;
mod radio;
mod toggle;

pub use checkbox::Checkbox;
pub use counter::Counter;
pub use multiselect::Multiselect;
pub use radio::Radio;
pub use toggle::Toggle;

#[cfg(test)]
mod tests;
