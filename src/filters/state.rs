use super::base::Filter;

use crate::{client::Bot, context::Context, types::Update};

use async_trait::async_trait;
use std::borrow::Cow;

#[derive(Default, Clone, PartialEq, Eq)]
pub enum StateType<'a, B: 'a>
where
    B: ToOwned + PartialEq<&'a str>,
{
    /// State is equal to specified
    Equal(Cow<'a, B>),
    /// Allow any state
    Any,
    /// Allow only no state
    #[default]
    None,
}

impl<'a, B: 'a> From<B> for StateType<'a, B>
where
    B: ToOwned + PartialEq<&'a str>,
{
    fn from(value: B) -> Self {
        Self::Equal(Cow::Owned(value.to_owned()))
    }
}

/// Infallible type, which can be used as [`StateType`]'s `B` generic type
///
/// This type is used to allow set type for [`StateType::Any`] and [`StateType::None`],
/// because they don't need any type and don't use equality comparisons
pub enum Infallible {}

impl ToOwned for Infallible {
    type Owned = Infallible;

    fn to_owned(&self) -> Self::Owned {
        unreachable!()
    }
}

impl PartialEq<&str> for Infallible {
    fn eq(&self, _: &&str) -> bool {
        unreachable!()
    }
}

pub struct State<'a, B: 'a = Infallible>
where
    B: ToOwned + PartialEq<&'a str>,
{
    allowed_states: Vec<StateType<'a, B>>,
}

impl State<'static> {
    /// Create new state filter, which allow any state
    pub fn any() -> Self {
        Self {
            allowed_states: vec![StateType::Any],
        }
    }

    /// Create new state filter, which allow only no state
    pub fn none() -> Self {
        Self {
            allowed_states: vec![StateType::None],
        }
    }
}

impl<'a, B: 'a> State<'a, B>
where
    B: ToOwned + PartialEq<&'a str>,
{
    /// Create new state filter
    pub fn one<T>(val: T) -> Self
    where
        T: Into<StateType<'a, B>>,
    {
        Self {
            allowed_states: vec![val.into()],
        }
    }

    /// Create new state filter with many states
    pub fn many<T, S>(val: T) -> Self
    where
        T: IntoIterator<Item = S>,
        S: Into<StateType<'a, B>>,
    {
        let mut allowed_states = Vec::new();

        for state in val {
            let state = state.into();

            // If state is `Any` or `None`, then clear all previous states and add only this one,
            // because `Any` and `None` are exclusive and can't be combined with other states
            if matches!(state, StateType::Any | StateType::None) {
                allowed_states.clear();
                allowed_states.push(state);
                break;
            }

            allowed_states.push(state);
        }

        Self { allowed_states }
    }
}

impl<'a, B: 'a> State<'a, B>
where
    B: ToOwned + PartialEq<&'a str>,
{
    fn is_allow_any(&self) -> bool {
        matches!(self.allowed_states[0], StateType::Any)
    }

    fn is_allow_only_none(&self) -> bool {
        matches!(self.allowed_states[0], StateType::None)
    }

    pub fn check(&self, state: Option<&'static str>) -> bool {
        let Some(state) = state else {
            return self.is_allow_only_none();
        };

        if self.is_allow_only_none() {
            return false;
        }

        if self.is_allow_any() {
            return true;
        }

        self.allowed_states
            .iter()
            .any(|allowed_state| match allowed_state {
                StateType::Equal(allowed_state) => *allowed_state.as_ref() == state,
                _ => unimplemented!("`StateType::Equal(_)` is only allowed here"),
            })
    }
}

#[async_trait]
impl<'a, Client, B: 'a> Filter<Client> for State<'a, B>
where
    B: ToOwned + PartialEq<&'a str> + Sync,
    B::Owned: Send + Sync,
{
    async fn check(&self, _bot: &Bot<Client>, _update: &Update, context: &Context) -> bool {
        match context.get("state") {
            Some(state) => {
                let state = state
                    .downcast_ref::<Cow<&'static str>>()
                    .expect("State isn't `Cow<str>`");

                self.check(Some(state.as_ref()))
            }
            None => self.check(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        let filter = State::one("state");
        assert!(filter.check(Some("state")));
        assert!(!filter.check(Some("wrong_state")));
        assert!(!filter.check(None));

        let filter = State::many(["state", "another_state"]);
        assert!(filter.check(Some("state")));
        assert!(filter.check(Some("another_state")));
        assert!(!filter.check(Some("wrong_state")));
        assert!(!filter.check(None));

        let filter = State::any();
        assert!(filter.check(Some("state")));
        assert!(filter.check(Some("another_state")));
        assert!(!filter.check(None));

        let filter = State::none();
        assert!(!filter.check(Some("state")));
        assert!(!filter.check(Some("another_state")));
        assert!(filter.check(None));
    }
}
