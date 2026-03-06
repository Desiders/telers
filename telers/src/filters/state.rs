use super::base::Filter;
use crate::Request;

use std::{convert::Infallible, future::Future};

#[derive(Clone)]
pub enum Dummy {}

/// A filter state holding allowed states for filtering requests
#[derive(Clone)]
pub enum State<S = Dummy, const N: usize = 0> {
    /// State is equal to the specified value
    Eq([S; N]),
    /// Allow any state
    Any,
    /// Allow only no state
    None,
}

impl State {
    /// Creates a state filter that allows any state
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }

    /// Creates a state filter that allows only the absence of state
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    #[must_use]
    const fn validate(&self, is_some: bool) -> bool {
        if is_some {
            matches!(self, Self::Any)
        } else {
            matches!(self, Self::None)
        }
    }
}

impl<S> State<S, 1>
where
    for<'a> S: PartialEq<&'a str>,
{
    /// Creates a state filter with a single allowed state
    #[must_use]
    pub const fn one(state: S) -> Self {
        Self::Eq([state; 1])
    }
}

impl<S, const N: usize> State<S, N>
where
    for<'a> S: PartialEq<&'a str>,
{
    /// Creates a state filter with multiple allowed states
    #[must_use]
    pub fn many(states: impl Into<[S; N]>) -> Self {
        Self::Eq(states.into())
    }
}

impl<S, const N: usize> State<S, N>
where
    for<'a> S: PartialEq<&'a str>,
{
    #[must_use]
    fn validate_eq(&self, state: Option<&str>) -> bool {
        let Some(state) = state else {
            return false;
        };
        match self {
            State::Eq(allowed_states) => allowed_states
                .iter()
                .any(|allowed_state| *allowed_state == state),
            _ => unreachable!(),
        }
    }
}

impl<Client> Filter<Client> for State<Dummy, 0>
where
    Client: Send,
{
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        let res = self.validate(request.context.contains_key("fsm_state"));
        async move { Ok(res) }
    }
}

impl<Client, S, const N: usize> Filter<Client> for State<S, N>
where
    Client: Send,
    for<'a> S: PartialEq<&'a str> + Clone + Send + Sync + 'static,
{
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        let res = self.validate_eq(
            request
                .context
                .get::<Box<str>>("fsm_state")
                .map(|val| &**val),
        );
        async move { Ok(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let filter = State::one("state");
        assert!(filter.validate_eq(Some("state")));
        assert!(!filter.validate_eq(Some("wrong_state")));
        assert!(!filter.validate_eq(None));

        let filter = State::many(["state", "another_state"]);
        assert!(filter.validate_eq(Some("state")));
        assert!(filter.validate_eq(Some("another_state")));
        assert!(!filter.validate_eq(Some("wrong_state")));
        assert!(!filter.validate_eq(None));

        let filter = State::any();
        assert!(filter.validate(true));
        assert!(filter.validate(true));
        assert!(!filter.validate(false));

        let filter = State::none();
        assert!(!filter.validate(true));
        assert!(!filter.validate(true));
        assert!(filter.validate(false));
    }
}
