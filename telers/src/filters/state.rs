use super::base::Filter;
use crate::Request;

use async_trait::async_trait;
use std::borrow::Cow;

/// Represents the allowed state in a filter.
///
/// - `Equal`: The state must be equal to the provided value.
/// - `Any`: Any state is allowed.
/// - `None`: Only the absence of state is allowed.
#[allow(clippy::module_name_repetitions)]
pub enum StateType<'a, B>
where
    B: ToOwned + PartialEq<&'a str>,
{
    /// State is equal to the specified value.
    Equal(Cow<'a, B>),
    /// Allow any state.
    Any,
    /// Allow only no state.
    None,
}

impl<'a, B> From<B> for StateType<'a, B>
where
    B: ToOwned<Owned = B> + PartialEq<&'a str>,
{
    fn from(value: B) -> Self {
        Self::Equal(Cow::Owned(value))
    }
}

impl<'a, B> From<&'a B> for StateType<'a, B>
where
    B: ToOwned<Owned = B> + PartialEq<&'a str>,
{
    fn from(value: &'a B) -> Self {
        Self::Equal(Cow::Borrowed(value))
    }
}

impl<'a, B> Clone for StateType<'a, B>
where
    B: ToOwned + PartialEq<&'a str> + Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Equal(state) => Self::Equal(state.clone()),
            Self::Any => Self::Any,
            Self::None => Self::None,
        }
    }
}

impl Clone for StateType<'_, Dummy> {
    fn clone(&self) -> Self {
        match self {
            Self::Equal(_) => unreachable!(),
            Self::Any => Self::Any,
            Self::None => Self::None,
        }
    }
}

/// A dummy type used as the default for [`StateType`] when no state value is required.
/// This type should not be used for equality comparisons.
pub enum Dummy {}

impl ToOwned for Dummy {
    type Owned = Self;

    fn to_owned(&self) -> Self::Owned {
        unreachable!()
    }
}

impl PartialEq<&str> for Dummy {
    fn eq(&self, _: &&str) -> bool {
        unreachable!()
    }
}

/// A filter state holding allowed states for filtering requests.
///
/// The generic type parameter `B` represents the type of the state value.
pub struct State<'a, B = Dummy>
where
    B: ToOwned + PartialEq<&'a str>,
{
    allowed_states: Vec<StateType<'a, B>>,
}

impl State<'static> {
    /// Creates a state filter that allows any state.
    #[must_use]
    pub fn any() -> Self {
        Self {
            allowed_states: vec![StateType::Any],
        }
    }

    /// Creates a state filter that allows only the absence of state.
    #[must_use]
    pub fn none() -> Self {
        Self {
            allowed_states: vec![StateType::None],
        }
    }
}

impl<'a, B> State<'a, B>
where
    B: ToOwned + PartialEq<&'a str>,
{
    /// Creates a state filter with a single allowed state.
    #[must_use]
    pub fn one(state: impl Into<StateType<'a, B>>) -> Self {
        Self {
            allowed_states: vec![state.into()],
        }
    }

    /// Creates a state filter with multiple allowed states.
    ///
    /// If any state in the iterator is `Any` or `None`, then all previous states are discarded
    /// and only the exclusive state is kept.
    #[must_use]
    pub fn many<T, S>(states: T) -> Self
    where
        T: IntoIterator<Item = S>,
        S: Into<StateType<'a, B>>,
    {
        let mut allowed_states = vec![];
        for state in states {
            let state = state.into();
            if matches!(state, StateType::Any | StateType::None) {
                allowed_states.clear();
                allowed_states.push(state);
                break;
            }
            allowed_states.push(state);
        }
        Self { allowed_states }
    }

    /// Checks whether the filter is configured to allow any state.
    #[must_use]
    fn is_allow_any(&self) -> bool {
        matches!(self.allowed_states.first(), Some(StateType::Any))
    }

    /// Checks whether the filter is configured to allow only the absence of state.
    #[must_use]
    fn is_allow_only_none(&self) -> bool {
        matches!(self.allowed_states.first(), Some(StateType::None))
    }

    /// Validates the given state (provided as an `Option<&str>`) against the allowed states.
    ///
    /// If the state is `None`, validation passes only if the filter allows no state.
    /// If the filter is set to allow any state, validation always passes.
    /// Otherwise, the state must equal one of the allowed states.
    #[must_use]
    pub fn validate(&self, state: Option<&'a str>) -> bool {
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
                _ => false, // Only Equal variant is used for comparison.
            })
    }
}

impl<'a, B> Clone for State<'a, B>
where
    B: ToOwned + PartialEq<&'a str> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            allowed_states: self.allowed_states.clone(),
        }
    }
}

impl Clone for State<'_, Dummy> {
    fn clone(&self) -> Self {
        Self {
            allowed_states: self.allowed_states.clone(),
        }
    }
}

#[async_trait]
impl<Client> Filter<Client> for State<'static, Dummy>
where
    Client: Send + Sync + 'static,
{
    async fn check(&mut self, request: &mut Request<Client>) -> bool {
        self.validate(request.context.get::<Box<str>>("fsm_state").map(|v| &**v))
    }
}

#[async_trait]
impl<Client, B> Filter<Client> for State<'static, B>
where
    Client: Send + Sync + 'static,
    for<'a> B: ToOwned + PartialEq<&'a str> + Clone + Sync,
    B::Owned: Send + Sync,
{
    async fn check(&mut self, request: &mut Request<Client>) -> bool {
        self.validate(request.context.get::<Box<str>>("fsm_state").map(|v| &**v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let filter = State::one("state");
        assert!(filter.validate(Some("state")));
        assert!(!filter.validate(Some("wrong_state")));
        assert!(!filter.validate(None));

        let filter = State::many(["state", "another_state"]);
        assert!(filter.validate(Some("state")));
        assert!(filter.validate(Some("another_state")));
        assert!(!filter.validate(Some("wrong_state")));
        assert!(!filter.validate(None));

        let filter = State::any();
        assert!(filter.validate(Some("state")));
        assert!(filter.validate(Some("another_state")));
        assert!(!filter.validate(None));

        let filter = State::none();
        assert!(!filter.validate(Some("state")));
        assert!(!filter.validate(Some("another_state")));
        assert!(filter.validate(None));
    }
}
