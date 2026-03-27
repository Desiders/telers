use super::{Filter, FilterResult};
use crate::{enums::ChatMemberType, Request};

use std::convert::Infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemberFilter {
    new: ChatMemberType,
    old: Option<ChatMemberType>,
}

impl MemberFilter {
    #[inline]
    #[must_use]
    pub const fn new(new: ChatMemberType) -> Self {
        Self { new, old: None }
    }

    #[inline]
    #[must_use]
    pub const fn old(self, old: ChatMemberType) -> Self {
        let mut this = self;
        this.old = Some(old);
        this
    }

    #[inline]
    #[must_use]
    pub fn validate(&self, new: ChatMemberType, old: Option<ChatMemberType>) -> bool {
        self.new == new && (self.old.is_none() || self.old == old)
    }
}

impl<Client> Filter<Client> for MemberFilter 
where 
    Client: Send + Sync 
{
    type Error = Infallible;

    async fn check(&mut self, request: &mut Request<Client>) -> FilterResult<Self::Error> {
        // Use or_else instead of or for lazy evaluation
        let member_update = request.update.chat_member()
            .or_else(|| request.update.my_chat_member());

        let Some(m) = member_update else { return Ok(false) };

        Ok(self.validate(
            (&m.new_chat_member).into(), 
            Some((&m.old_chat_member).into())
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member_filter() {
        let filter = MemberFilter::new(ChatMemberType::Administrator).old(ChatMemberType::Member);

        // true
        assert!(filter.validate(ChatMemberType::Administrator, Some(ChatMemberType::Member)));

        // false
        assert!(!filter.validate(ChatMemberType::Member, Some(ChatMemberType::Member)));
        assert!(!filter.validate(ChatMemberType::Administrator, Some(ChatMemberType::Kicked)));
        assert!(!filter.validate(ChatMemberType::Administrator, None));
    }

    #[test]
    fn test_member_filter_any() {
        let filter = MemberFilter::new(ChatMemberType::Kicked);

        // any
        assert!(filter.validate(ChatMemberType::Kicked, Some(ChatMemberType::Member)));
        assert!(filter.validate(ChatMemberType::Kicked, Some(ChatMemberType::Administrator)));
        assert!(filter.validate(ChatMemberType::Kicked, None));
    }
}