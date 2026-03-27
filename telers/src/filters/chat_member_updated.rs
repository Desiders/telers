use super::{Filter, FilterResult};
use crate::{enums::ChatMemberType, Request};

use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct ChatMemberUpdated {
    new: ChatMemberType,
    old: Option<ChatMemberType>,
}

impl ChatMemberUpdated {
    #[inline]
    #[must_use]
    pub const fn new(new: ChatMemberType) -> Self {
        Self {
            new,
            old: None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn old(mut self, old: ChatMemberType) -> Self {
        self.old = Some(old);
        self
    }

    #[inline]
    #[must_use]
    pub fn validate(&self, new: ChatMemberType, old: ChatMemberType) -> bool {
        (self.new == new) & self.old.map_or(true, |o| o == old)
    }
}

impl<Client> Filter<Client> for ChatMemberUpdated
where
    Client: Send + Sync,
{
    type Error = Infallible;

    async fn check(&mut self, request: &mut Request<Client>) -> FilterResult<Self::Error> {
        // Use or_else instead of or for lazy evaluation
        let member_update = request
            .update
            .chat_member()
            .or_else(|| request.update.my_chat_member());

        let Some(m) = member_update else {
            return Ok(false);
        };

        Ok(self.validate((&m.new_chat_member).into(), (&m.old_chat_member).into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member_filter() {
        let filter =
            ChatMemberUpdated::new(ChatMemberType::Administrator).old(ChatMemberType::Member);

        // true
        assert!(filter.validate(ChatMemberType::Administrator, ChatMemberType::Member));

        // false
        assert!(!filter.validate(ChatMemberType::Member, ChatMemberType::Member));
        assert!(!filter.validate(ChatMemberType::Administrator, ChatMemberType::Kicked));
    }

    #[test]
    fn test_member_filter_any() {
        let filter = ChatMemberUpdated::new(ChatMemberType::Kicked);

        // any
        assert!(filter.validate(ChatMemberType::Kicked, ChatMemberType::Member));
        assert!(filter.validate(ChatMemberType::Kicked, ChatMemberType::Administrator));
    }
}
