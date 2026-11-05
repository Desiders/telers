use crate::types::*;
impl<Client> crate::Extractor<Client> for Update {
    type Error = std::convert::Infallible;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone();
        async move { Ok(val) }
    }
}
impl<Client> crate::Extractor<Client> for std::sync::Arc<Update> {
    type Error = std::convert::Infallible;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = request.update.clone();
        async move { Ok(val) }
    }
}
impl<Client> crate::Extractor<Client> for BusinessConnection {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for Message {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageBoostAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChannelChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChatBackgroundSet {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChatOwnerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChatOwnerLeft {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChatShared {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChecklist {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChecklistTasksAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageChecklistTasksDone {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageCommunityChatAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageCommunityChatJoined {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageCommunityChatRemoved {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageConnectedWebsite {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageContact {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageDeleteChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageDice {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageDirectMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageForumTopicClosed {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageForumTopicCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageForumTopicEdited {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageForumTopicReopened {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGame {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGeneralForumTopicHidden {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGeneralForumTopicUnhidden {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGift {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGiftUpgradeSent {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGiveaway {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGiveawayCompleted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGiveawayCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGiveawayWinners {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageInvoice {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageLeftChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageManagedBotCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageMessageAutoDeleteTimerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageMigrateFromChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageMigrateToChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageNewChatMembers {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageNewChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageNewChatTitle {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePaidMedia {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePaidMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePassportData {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePinnedMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePoll {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePollOptionAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessagePollOptionDeleted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageProximityAlertTriggered {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageRefundedPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageRichMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageStory {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSuccessfulPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSuggestedPostApprovalFailed {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSuggestedPostApproved {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSuggestedPostDeclined {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSuggestedPostPaid {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSuggestedPostRefunded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageSupergroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageText {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageUniqueGift {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageUsersShared {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVideoChatEnded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVideoChatParticipantsInvited {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVideoChatScheduled {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVideoChatStarted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVideoNote {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageVoice {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageWebAppData {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageWriteAccessAllowed {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for CallbackQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for ChatBoostUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for ChatJoinRequest {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for ChatMemberUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for ChosenInlineResult {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for BusinessMessagesDeleted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for InlineQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for ManagedBotUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageReactionUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageReactionCountUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for Poll {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for PollRegular {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for PollQuiz {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for PollUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for PollAnswer {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for PreCheckoutQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for PaidMediaPurchased {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for ChatBoostRemoved {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for ShippingQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for MessageGenerationStopped {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for BotSubscriptionUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateBusinessConnection {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateBusinessMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateCallbackQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateChannelPost {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateChatBoost {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateChatJoinRequest {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateChosenInlineResult {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateDeletedBusinessMessages {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateEditedBusinessMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateEditedChannelPost {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateEditedMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateGuestMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateInlineQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateManagedBot {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateMessageReaction {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateMessageReactionCount {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateMyChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdatePoll {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdatePollAnswer {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdatePreCheckoutQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdatePurchasedPaidMedia {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateRemovedChatBoost {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateShippingQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateStoppedMessageGeneration {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl<Client> crate::Extractor<Client> for UpdateSubscription {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
