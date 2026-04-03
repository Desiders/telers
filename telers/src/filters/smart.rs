#![allow(clippy::wrong_self_convention)]
#![allow(clippy::redundant_closure_for_method_calls)]
pub mod core;
use crate::types::Update;
use core::SmartFilterPath;
use std::sync::Arc;
pub struct SmartFilter;
impl SmartFilter {
    #[must_use]
    pub fn update() -> SmartFilterPath<Update> {
        SmartFilterPath {
            accessor: Arc::new(|update| Some(update)),
        }
    }

    #[must_use]
    pub fn actor_chat() -> SmartFilterPath<crate::types::Chat> {
        Self::update().actor_chat()
    }

    #[must_use]
    pub fn animation() -> SmartFilterPath<crate::types::Animation> {
        Self::update().animation()
    }

    #[must_use]
    pub fn audio() -> SmartFilterPath<crate::types::Audio> {
        Self::update().audio()
    }

    #[must_use]
    pub fn author_signature() -> SmartFilterPath<str> {
        Self::update().author_signature()
    }

    #[must_use]
    pub fn bio() -> SmartFilterPath<str> {
        Self::update().bio()
    }

    #[must_use]
    pub fn boost() -> SmartFilterPath<crate::types::ChatBoost> {
        Self::update().boost()
    }

    #[must_use]
    pub fn boost_added() -> SmartFilterPath<crate::types::ChatBoostAdded> {
        Self::update().boost_added()
    }

    #[must_use]
    pub fn boost_id() -> SmartFilterPath<str> {
        Self::update().boost_id()
    }

    #[must_use]
    pub fn bot() -> SmartFilterPath<crate::types::User> {
        Self::update().bot()
    }

    #[must_use]
    pub fn business_connection() -> SmartFilterPath<crate::types::BusinessConnection> {
        Self::update().business_connection()
    }

    #[must_use]
    pub fn business_connection_id() -> SmartFilterPath<str> {
        Self::update().business_connection_id()
    }

    #[must_use]
    pub fn business_message() -> SmartFilterPath<crate::types::Message> {
        Self::update().business_message()
    }

    #[must_use]
    pub fn callback_query() -> SmartFilterPath<crate::types::CallbackQuery> {
        Self::update().callback_query()
    }

    #[must_use]
    pub fn caption() -> SmartFilterPath<str> {
        Self::update().caption()
    }

    #[must_use]
    pub fn caption_entities() -> SmartFilterPath<[crate::types::MessageEntity]> {
        Self::update().caption_entities()
    }

    #[must_use]
    pub fn channel_post() -> SmartFilterPath<crate::types::Message> {
        Self::update().channel_post()
    }

    #[must_use]
    pub fn chat() -> SmartFilterPath<crate::types::Chat> {
        Self::update().chat()
    }

    #[must_use]
    pub fn chat_background_set() -> SmartFilterPath<crate::types::ChatBackground> {
        Self::update().chat_background_set()
    }

    #[must_use]
    pub fn chat_boost() -> SmartFilterPath<crate::types::ChatBoostUpdated> {
        Self::update().chat_boost()
    }

    #[must_use]
    pub fn chat_instance() -> SmartFilterPath<str> {
        Self::update().chat_instance()
    }

    #[must_use]
    pub fn chat_join_request() -> SmartFilterPath<crate::types::ChatJoinRequest> {
        Self::update().chat_join_request()
    }

    #[must_use]
    pub fn chat_member() -> SmartFilterPath<crate::types::ChatMemberUpdated> {
        Self::update().chat_member()
    }

    #[must_use]
    pub fn chat_owner_changed() -> SmartFilterPath<crate::types::ChatOwnerChanged> {
        Self::update().chat_owner_changed()
    }

    #[must_use]
    pub fn chat_owner_left() -> SmartFilterPath<crate::types::ChatOwnerLeft> {
        Self::update().chat_owner_left()
    }

    #[must_use]
    pub fn chat_shared() -> SmartFilterPath<crate::types::ChatShared> {
        Self::update().chat_shared()
    }

    #[must_use]
    pub fn chat_type() -> SmartFilterPath<str> {
        Self::update().chat_type()
    }

    #[must_use]
    pub fn checklist() -> SmartFilterPath<crate::types::Checklist> {
        Self::update().checklist()
    }

    #[must_use]
    pub fn checklist_tasks_added() -> SmartFilterPath<crate::types::ChecklistTasksAdded> {
        Self::update().checklist_tasks_added()
    }

    #[must_use]
    pub fn checklist_tasks_done() -> SmartFilterPath<crate::types::ChecklistTasksDone> {
        Self::update().checklist_tasks_done()
    }

    #[must_use]
    pub fn chosen_inline_result() -> SmartFilterPath<crate::types::ChosenInlineResult> {
        Self::update().chosen_inline_result()
    }

    #[must_use]
    pub fn connected_website() -> SmartFilterPath<str> {
        Self::update().connected_website()
    }

    #[must_use]
    pub fn contact() -> SmartFilterPath<crate::types::Contact> {
        Self::update().contact()
    }

    #[must_use]
    pub fn correct_option_ids() -> SmartFilterPath<[i64]> {
        Self::update().correct_option_ids()
    }

    #[must_use]
    pub fn currency() -> SmartFilterPath<str> {
        Self::update().currency()
    }

    #[must_use]
    pub fn data() -> SmartFilterPath<str> {
        Self::update().data()
    }

    #[must_use]
    pub fn deleted_business_messages() -> SmartFilterPath<crate::types::BusinessMessagesDeleted> {
        Self::update().deleted_business_messages()
    }

    #[must_use]
    pub fn description() -> SmartFilterPath<str> {
        Self::update().description()
    }

    #[must_use]
    pub fn description_entities() -> SmartFilterPath<[crate::types::MessageEntity]> {
        Self::update().description_entities()
    }

    #[must_use]
    pub fn dice() -> SmartFilterPath<crate::types::Dice> {
        Self::update().dice()
    }

    #[must_use]
    pub fn direct_message_price_changed() -> SmartFilterPath<crate::types::DirectMessagePriceChanged>
    {
        Self::update().direct_message_price_changed()
    }

    #[must_use]
    pub fn direct_messages_topic() -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        Self::update().direct_messages_topic()
    }

    #[must_use]
    pub fn document() -> SmartFilterPath<crate::types::Document> {
        Self::update().document()
    }

    #[must_use]
    pub fn edited_business_message() -> SmartFilterPath<crate::types::Message> {
        Self::update().edited_business_message()
    }

    #[must_use]
    pub fn edited_channel_post() -> SmartFilterPath<crate::types::Message> {
        Self::update().edited_channel_post()
    }

    #[must_use]
    pub fn edited_message() -> SmartFilterPath<crate::types::Message> {
        Self::update().edited_message()
    }

    #[must_use]
    pub fn effect_id() -> SmartFilterPath<str> {
        Self::update().effect_id()
    }

    #[must_use]
    pub fn entities() -> SmartFilterPath<[crate::types::MessageEntity]> {
        Self::update().entities()
    }

    #[must_use]
    pub fn explanation() -> SmartFilterPath<str> {
        Self::update().explanation()
    }

    #[must_use]
    pub fn explanation_entities() -> SmartFilterPath<[crate::types::MessageEntity]> {
        Self::update().explanation_entities()
    }

    #[must_use]
    pub fn external_reply() -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        Self::update().external_reply()
    }

    #[must_use]
    pub fn forum_topic_closed() -> SmartFilterPath<crate::types::ForumTopicClosed> {
        Self::update().forum_topic_closed()
    }

    #[must_use]
    pub fn forum_topic_created() -> SmartFilterPath<crate::types::ForumTopicCreated> {
        Self::update().forum_topic_created()
    }

    #[must_use]
    pub fn forum_topic_edited() -> SmartFilterPath<crate::types::ForumTopicEdited> {
        Self::update().forum_topic_edited()
    }

    #[must_use]
    pub fn forum_topic_reopened() -> SmartFilterPath<crate::types::ForumTopicReopened> {
        Self::update().forum_topic_reopened()
    }

    #[must_use]
    pub fn forward_origin() -> SmartFilterPath<crate::types::MessageOrigin> {
        Self::update().forward_origin()
    }

    #[must_use]
    pub fn from() -> SmartFilterPath<crate::types::User> {
        Self::update().from()
    }

    #[must_use]
    pub fn game() -> SmartFilterPath<crate::types::Game> {
        Self::update().game()
    }

    #[must_use]
    pub fn game_short_name() -> SmartFilterPath<str> {
        Self::update().game_short_name()
    }

    #[must_use]
    pub fn general_forum_topic_hidden() -> SmartFilterPath<crate::types::GeneralForumTopicHidden> {
        Self::update().general_forum_topic_hidden()
    }

    #[must_use]
    pub fn general_forum_topic_unhidden() -> SmartFilterPath<crate::types::GeneralForumTopicUnhidden>
    {
        Self::update().general_forum_topic_unhidden()
    }

    #[must_use]
    pub fn gift() -> SmartFilterPath<crate::types::GiftInfo> {
        Self::update().gift()
    }

    #[must_use]
    pub fn gift_upgrade_sent() -> SmartFilterPath<crate::types::GiftInfo> {
        Self::update().gift_upgrade_sent()
    }

    #[must_use]
    pub fn giveaway() -> SmartFilterPath<crate::types::Giveaway> {
        Self::update().giveaway()
    }

    #[must_use]
    pub fn giveaway_completed() -> SmartFilterPath<crate::types::GiveawayCompleted> {
        Self::update().giveaway_completed()
    }

    #[must_use]
    pub fn giveaway_created() -> SmartFilterPath<crate::types::GiveawayCreated> {
        Self::update().giveaway_created()
    }

    #[must_use]
    pub fn giveaway_winners() -> SmartFilterPath<crate::types::GiveawayWinners> {
        Self::update().giveaway_winners()
    }

    #[must_use]
    pub fn id() -> SmartFilterPath<str> {
        Self::update().id()
    }

    #[must_use]
    pub fn inline_message_id() -> SmartFilterPath<str> {
        Self::update().inline_message_id()
    }

    #[must_use]
    pub fn inline_query() -> SmartFilterPath<crate::types::InlineQuery> {
        Self::update().inline_query()
    }

    #[must_use]
    pub fn invite_link() -> SmartFilterPath<crate::types::ChatInviteLink> {
        Self::update().invite_link()
    }

    #[must_use]
    pub fn invoice() -> SmartFilterPath<crate::types::Invoice> {
        Self::update().invoice()
    }

    #[must_use]
    pub fn invoice_payload() -> SmartFilterPath<str> {
        Self::update().invoice_payload()
    }

    #[must_use]
    pub fn left_chat_member() -> SmartFilterPath<crate::types::User> {
        Self::update().left_chat_member()
    }

    #[must_use]
    pub fn link_preview_options() -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        Self::update().link_preview_options()
    }

    #[must_use]
    pub fn location() -> SmartFilterPath<crate::types::Location> {
        Self::update().location()
    }

    #[must_use]
    pub fn managed_bot() -> SmartFilterPath<crate::types::ManagedBotUpdated> {
        Self::update().managed_bot()
    }

    #[must_use]
    pub fn managed_bot_created() -> SmartFilterPath<crate::types::ManagedBotCreated> {
        Self::update().managed_bot_created()
    }

    #[must_use]
    pub fn media_group_id() -> SmartFilterPath<str> {
        Self::update().media_group_id()
    }

    #[must_use]
    pub fn message() -> SmartFilterPath<crate::types::Message> {
        Self::update().message()
    }

    #[must_use]
    pub fn message_auto_delete_timer_changed(
    ) -> SmartFilterPath<crate::types::MessageAutoDeleteTimerChanged> {
        Self::update().message_auto_delete_timer_changed()
    }

    #[must_use]
    pub fn message_ids() -> SmartFilterPath<[i64]> {
        Self::update().message_ids()
    }

    #[must_use]
    pub fn message_reaction() -> SmartFilterPath<crate::types::MessageReactionUpdated> {
        Self::update().message_reaction()
    }

    #[must_use]
    pub fn message_reaction_count() -> SmartFilterPath<crate::types::MessageReactionCountUpdated> {
        Self::update().message_reaction_count()
    }

    #[must_use]
    pub fn my_chat_member() -> SmartFilterPath<crate::types::ChatMemberUpdated> {
        Self::update().my_chat_member()
    }

    #[must_use]
    pub fn new_chat_member() -> SmartFilterPath<crate::types::ChatMember> {
        Self::update().new_chat_member()
    }

    #[must_use]
    pub fn new_chat_members() -> SmartFilterPath<[crate::types::User]> {
        Self::update().new_chat_members()
    }

    #[must_use]
    pub fn new_chat_photo() -> SmartFilterPath<[crate::types::PhotoSize]> {
        Self::update().new_chat_photo()
    }

    #[must_use]
    pub fn new_chat_title() -> SmartFilterPath<str> {
        Self::update().new_chat_title()
    }

    #[must_use]
    pub fn new_reaction() -> SmartFilterPath<[crate::types::ReactionType]> {
        Self::update().new_reaction()
    }

    #[must_use]
    pub fn offset() -> SmartFilterPath<str> {
        Self::update().offset()
    }

    #[must_use]
    pub fn old_chat_member() -> SmartFilterPath<crate::types::ChatMember> {
        Self::update().old_chat_member()
    }

    #[must_use]
    pub fn old_reaction() -> SmartFilterPath<[crate::types::ReactionType]> {
        Self::update().old_reaction()
    }

    #[must_use]
    pub fn option_ids() -> SmartFilterPath<[i64]> {
        Self::update().option_ids()
    }

    #[must_use]
    pub fn option_persistent_ids() -> SmartFilterPath<[Box<str>]> {
        Self::update().option_persistent_ids()
    }

    #[must_use]
    pub fn options() -> SmartFilterPath<[crate::types::PollOption]> {
        Self::update().options()
    }

    #[must_use]
    pub fn order_info() -> SmartFilterPath<crate::types::OrderInfo> {
        Self::update().order_info()
    }

    #[must_use]
    pub fn paid_media() -> SmartFilterPath<crate::types::PaidMediaInfo> {
        Self::update().paid_media()
    }

    #[must_use]
    pub fn paid_media_payload() -> SmartFilterPath<str> {
        Self::update().paid_media_payload()
    }

    #[must_use]
    pub fn paid_message_price_changed() -> SmartFilterPath<crate::types::PaidMessagePriceChanged> {
        Self::update().paid_message_price_changed()
    }

    #[must_use]
    pub fn passport_data() -> SmartFilterPath<crate::types::PassportData> {
        Self::update().passport_data()
    }

    #[must_use]
    pub fn photo() -> SmartFilterPath<[crate::types::PhotoSize]> {
        Self::update().photo()
    }

    #[must_use]
    pub fn pinned_message() -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        Self::update().pinned_message()
    }

    #[must_use]
    pub fn poll() -> SmartFilterPath<crate::types::Poll> {
        Self::update().poll()
    }

    #[must_use]
    pub fn poll_answer() -> SmartFilterPath<crate::types::PollAnswer> {
        Self::update().poll_answer()
    }

    #[must_use]
    pub fn poll_id() -> SmartFilterPath<str> {
        Self::update().poll_id()
    }

    #[must_use]
    pub fn poll_option_added() -> SmartFilterPath<crate::types::PollOptionAdded> {
        Self::update().poll_option_added()
    }

    #[must_use]
    pub fn poll_option_deleted() -> SmartFilterPath<crate::types::PollOptionDeleted> {
        Self::update().poll_option_deleted()
    }

    #[must_use]
    pub fn pre_checkout_query() -> SmartFilterPath<crate::types::PreCheckoutQuery> {
        Self::update().pre_checkout_query()
    }

    #[must_use]
    pub fn proximity_alert_triggered() -> SmartFilterPath<crate::types::ProximityAlertTriggered> {
        Self::update().proximity_alert_triggered()
    }

    #[must_use]
    pub fn purchased_paid_media() -> SmartFilterPath<crate::types::PaidMediaPurchased> {
        Self::update().purchased_paid_media()
    }

    #[must_use]
    pub fn query() -> SmartFilterPath<str> {
        Self::update().query()
    }

    #[must_use]
    pub fn question() -> SmartFilterPath<str> {
        Self::update().question()
    }

    #[must_use]
    pub fn question_entities() -> SmartFilterPath<[crate::types::MessageEntity]> {
        Self::update().question_entities()
    }

    #[must_use]
    pub fn quote() -> SmartFilterPath<crate::types::TextQuote> {
        Self::update().quote()
    }

    #[must_use]
    pub fn reactions() -> SmartFilterPath<[crate::types::ReactionCount]> {
        Self::update().reactions()
    }

    #[must_use]
    pub fn refunded_payment() -> SmartFilterPath<crate::types::RefundedPayment> {
        Self::update().refunded_payment()
    }

    #[must_use]
    pub fn removed_chat_boost() -> SmartFilterPath<crate::types::ChatBoostRemoved> {
        Self::update().removed_chat_boost()
    }

    #[must_use]
    pub fn reply_markup() -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        Self::update().reply_markup()
    }

    #[must_use]
    pub fn reply_to_message() -> SmartFilterPath<crate::types::Message> {
        Self::update().reply_to_message()
    }

    #[must_use]
    pub fn reply_to_poll_option_id() -> SmartFilterPath<str> {
        Self::update().reply_to_poll_option_id()
    }

    #[must_use]
    pub fn reply_to_story() -> SmartFilterPath<crate::types::Story> {
        Self::update().reply_to_story()
    }

    #[must_use]
    pub fn result_id() -> SmartFilterPath<str> {
        Self::update().result_id()
    }

    #[must_use]
    pub fn rights() -> SmartFilterPath<crate::types::BusinessBotRights> {
        Self::update().rights()
    }

    #[must_use]
    pub fn sender_business_bot() -> SmartFilterPath<crate::types::User> {
        Self::update().sender_business_bot()
    }

    #[must_use]
    pub fn sender_chat() -> SmartFilterPath<crate::types::Chat> {
        Self::update().sender_chat()
    }

    #[must_use]
    pub fn sender_tag() -> SmartFilterPath<str> {
        Self::update().sender_tag()
    }

    #[must_use]
    pub fn shipping_address() -> SmartFilterPath<crate::types::ShippingAddress> {
        Self::update().shipping_address()
    }

    #[must_use]
    pub fn shipping_option_id() -> SmartFilterPath<str> {
        Self::update().shipping_option_id()
    }

    #[must_use]
    pub fn shipping_query() -> SmartFilterPath<crate::types::ShippingQuery> {
        Self::update().shipping_query()
    }

    #[must_use]
    pub fn source() -> SmartFilterPath<crate::types::ChatBoostSource> {
        Self::update().source()
    }

    #[must_use]
    pub fn sticker() -> SmartFilterPath<crate::types::Sticker> {
        Self::update().sticker()
    }

    #[must_use]
    pub fn story() -> SmartFilterPath<crate::types::Story> {
        Self::update().story()
    }

    #[must_use]
    pub fn successful_payment() -> SmartFilterPath<crate::types::SuccessfulPayment> {
        Self::update().successful_payment()
    }

    #[must_use]
    pub fn suggested_post_approval_failed(
    ) -> SmartFilterPath<crate::types::SuggestedPostApprovalFailed> {
        Self::update().suggested_post_approval_failed()
    }

    #[must_use]
    pub fn suggested_post_approved() -> SmartFilterPath<crate::types::SuggestedPostApproved> {
        Self::update().suggested_post_approved()
    }

    #[must_use]
    pub fn suggested_post_declined() -> SmartFilterPath<crate::types::SuggestedPostDeclined> {
        Self::update().suggested_post_declined()
    }

    #[must_use]
    pub fn suggested_post_info() -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        Self::update().suggested_post_info()
    }

    #[must_use]
    pub fn suggested_post_paid() -> SmartFilterPath<crate::types::SuggestedPostPaid> {
        Self::update().suggested_post_paid()
    }

    #[must_use]
    pub fn suggested_post_refunded() -> SmartFilterPath<crate::types::SuggestedPostRefunded> {
        Self::update().suggested_post_refunded()
    }

    #[must_use]
    pub fn text() -> SmartFilterPath<str> {
        Self::update().text()
    }

    #[must_use]
    pub fn unique_gift() -> SmartFilterPath<crate::types::UniqueGiftInfo> {
        Self::update().unique_gift()
    }

    #[must_use]
    pub fn user() -> SmartFilterPath<crate::types::User> {
        Self::update().user()
    }

    #[must_use]
    pub fn users_shared() -> SmartFilterPath<crate::types::UsersShared> {
        Self::update().users_shared()
    }

    #[must_use]
    pub fn venue() -> SmartFilterPath<crate::types::Venue> {
        Self::update().venue()
    }

    #[must_use]
    pub fn via_bot() -> SmartFilterPath<crate::types::User> {
        Self::update().via_bot()
    }

    #[must_use]
    pub fn video() -> SmartFilterPath<crate::types::Video> {
        Self::update().video()
    }

    #[must_use]
    pub fn video_chat_ended() -> SmartFilterPath<crate::types::VideoChatEnded> {
        Self::update().video_chat_ended()
    }

    #[must_use]
    pub fn video_chat_participants_invited(
    ) -> SmartFilterPath<crate::types::VideoChatParticipantsInvited> {
        Self::update().video_chat_participants_invited()
    }

    #[must_use]
    pub fn video_chat_scheduled() -> SmartFilterPath<crate::types::VideoChatScheduled> {
        Self::update().video_chat_scheduled()
    }

    #[must_use]
    pub fn video_chat_started() -> SmartFilterPath<crate::types::VideoChatStarted> {
        Self::update().video_chat_started()
    }

    #[must_use]
    pub fn video_note() -> SmartFilterPath<crate::types::VideoNote> {
        Self::update().video_note()
    }

    #[must_use]
    pub fn voice() -> SmartFilterPath<crate::types::Voice> {
        Self::update().voice()
    }

    #[must_use]
    pub fn voter_chat() -> SmartFilterPath<crate::types::Chat> {
        Self::update().voter_chat()
    }

    #[must_use]
    pub fn web_app_data() -> SmartFilterPath<crate::types::WebAppData> {
        Self::update().web_app_data()
    }

    #[must_use]
    pub fn write_access_allowed() -> SmartFilterPath<crate::types::WriteAccessAllowed> {
        Self::update().write_access_allowed()
    }
}
impl SmartFilterPath<crate::types::AcceptedGiftTypes> {
    #[must_use]
    pub fn unlimited_gifts(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.unlimited_gifts)
    }

    #[must_use]
    pub fn limited_gifts(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.limited_gifts)
    }

    #[must_use]
    pub fn unique_gifts(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.unique_gifts)
    }

    #[must_use]
    pub fn premium_subscription(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.premium_subscription)
    }

    #[must_use]
    pub fn gifts_from_channels(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.gifts_from_channels)
    }
}
impl SmartFilterPath<crate::types::AffiliateInfo> {
    #[must_use]
    pub fn affiliate_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.affiliate_user.as_deref())
    }

    #[must_use]
    pub fn affiliate_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.affiliate_chat.as_deref())
    }

    #[must_use]
    pub fn commission_per_mille(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.commission_per_mille)
    }

    #[must_use]
    pub fn amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.amount)
    }

    #[must_use]
    pub fn nanostar_amount(self) -> SmartFilterPath<i32> {
        self.and_then(|value| value.nanostar_amount.as_ref())
    }
}
impl SmartFilterPath<crate::types::Animation> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.width)
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.height)
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.duration)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn file_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.file_name.as_deref())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type.as_deref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::Audio> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.duration)
    }

    #[must_use]
    pub fn performer(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.performer.as_deref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn file_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.file_name.as_deref())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type.as_deref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }
}
impl SmartFilterPath<crate::types::BackgroundFill> {
    #[must_use]
    pub fn colors(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.colors())
    }
}
impl SmartFilterPath<crate::types::BackgroundFillFreeformGradient> {
    #[must_use]
    pub fn colors(self) -> SmartFilterPath<[i64]> {
        self.map(|value| value.colors.as_ref())
    }
}
impl SmartFilterPath<crate::types::BackgroundFillGradient> {
    #[must_use]
    pub fn top_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.top_color)
    }

    #[must_use]
    pub fn bottom_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.bottom_color)
    }

    #[must_use]
    pub fn rotation_angle(self) -> SmartFilterPath<u16> {
        self.map(|value| &value.rotation_angle)
    }
}
impl SmartFilterPath<crate::types::BackgroundFillSolid> {
    #[must_use]
    pub fn color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.color)
    }
}
impl SmartFilterPath<crate::types::BackgroundType> {
    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.and_then(|value| value.document())
    }

    #[must_use]
    pub fn fill(self) -> SmartFilterPath<crate::types::BackgroundFill> {
        self.and_then(|value| value.fill())
    }

    #[must_use]
    pub fn theme_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.theme_name())
    }
}
impl SmartFilterPath<crate::types::BackgroundTypeChatTheme> {
    #[must_use]
    pub fn theme_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.theme_name.as_ref())
    }
}
impl SmartFilterPath<crate::types::BackgroundTypeFill> {
    #[must_use]
    pub fn fill(self) -> SmartFilterPath<crate::types::BackgroundFill> {
        self.map(|value| &value.fill)
    }

    #[must_use]
    pub fn dark_theme_dimming(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.dark_theme_dimming)
    }
}
impl SmartFilterPath<crate::types::BackgroundTypePattern> {
    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.map(|value| value.document.as_ref())
    }

    #[must_use]
    pub fn fill(self) -> SmartFilterPath<crate::types::BackgroundFill> {
        self.map(|value| &value.fill)
    }

    #[must_use]
    pub fn intensity(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.intensity)
    }

    #[must_use]
    pub fn is_inverted(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_inverted.as_ref())
    }

    #[must_use]
    pub fn is_moving(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_moving.as_ref())
    }
}
impl SmartFilterPath<crate::types::BackgroundTypeWallpaper> {
    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.map(|value| value.document.as_ref())
    }

    #[must_use]
    pub fn dark_theme_dimming(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.dark_theme_dimming)
    }

    #[must_use]
    pub fn is_blurred(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_blurred.as_ref())
    }

    #[must_use]
    pub fn is_moving(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_moving.as_ref())
    }
}
impl SmartFilterPath<crate::types::Birthdate> {
    #[must_use]
    pub fn day(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.day)
    }

    #[must_use]
    pub fn month(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.month)
    }

    #[must_use]
    pub fn year(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.year.as_ref())
    }
}
impl SmartFilterPath<crate::types::BotCommand> {
    #[must_use]
    pub fn command(self) -> SmartFilterPath<str> {
        self.map(|value| value.command.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.map(|value| value.description.as_ref())
    }
}
impl SmartFilterPath<crate::types::BotCommandScope> {
    #[must_use]
    pub fn chat_id(self) -> SmartFilterPath<crate::types::ChatIdKind> {
        self.and_then(|value| value.chat_id())
    }
}
impl SmartFilterPath<crate::types::BotCommandScopeAllChatAdministrators> {}
impl SmartFilterPath<crate::types::BotCommandScopeAllGroupChats> {}
impl SmartFilterPath<crate::types::BotCommandScopeAllPrivateChats> {}
impl SmartFilterPath<crate::types::BotCommandScopeChat> {
    #[must_use]
    pub fn chat_id(self) -> SmartFilterPath<crate::types::ChatIdKind> {
        self.map(|value| &value.chat_id)
    }
}
impl SmartFilterPath<crate::types::BotCommandScopeChatAdministrators> {
    #[must_use]
    pub fn chat_id(self) -> SmartFilterPath<crate::types::ChatIdKind> {
        self.map(|value| &value.chat_id)
    }
}
impl SmartFilterPath<crate::types::BotCommandScopeChatMember> {
    #[must_use]
    pub fn chat_id(self) -> SmartFilterPath<crate::types::ChatIdKind> {
        self.map(|value| &value.chat_id)
    }

    #[must_use]
    pub fn user_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.user_id)
    }
}
impl SmartFilterPath<crate::types::BotCommandScopeDefault> {}
impl SmartFilterPath<crate::types::BotDescription> {
    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.map(|value| value.description.as_ref())
    }
}
impl SmartFilterPath<crate::types::BotName> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }
}
impl SmartFilterPath<crate::types::BotShortDescription> {
    #[must_use]
    pub fn short_description(self) -> SmartFilterPath<str> {
        self.map(|value| value.short_description.as_ref())
    }
}
impl SmartFilterPath<crate::types::BusinessBotRights> {
    #[must_use]
    pub fn can_reply(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_reply.as_ref())
    }

    #[must_use]
    pub fn can_read_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_read_messages.as_ref())
    }

    #[must_use]
    pub fn can_delete_sent_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_delete_sent_messages.as_ref())
    }

    #[must_use]
    pub fn can_delete_all_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_delete_all_messages.as_ref())
    }

    #[must_use]
    pub fn can_edit_name(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_edit_name.as_ref())
    }

    #[must_use]
    pub fn can_edit_bio(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_edit_bio.as_ref())
    }

    #[must_use]
    pub fn can_edit_profile_photo(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_edit_profile_photo.as_ref())
    }

    #[must_use]
    pub fn can_edit_username(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_edit_username.as_ref())
    }

    #[must_use]
    pub fn can_change_gift_settings(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_change_gift_settings.as_ref())
    }

    #[must_use]
    pub fn can_view_gifts_and_stars(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_view_gifts_and_stars.as_ref())
    }

    #[must_use]
    pub fn can_convert_gifts_to_stars(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_convert_gifts_to_stars.as_ref())
    }

    #[must_use]
    pub fn can_transfer_and_upgrade_gifts(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_transfer_and_upgrade_gifts.as_ref())
    }

    #[must_use]
    pub fn can_transfer_stars(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_transfer_stars.as_ref())
    }

    #[must_use]
    pub fn can_manage_stories(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_stories.as_ref())
    }
}
impl SmartFilterPath<crate::types::BusinessConnection> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn user_chat_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.user_chat_id)
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn rights(self) -> SmartFilterPath<crate::types::BusinessBotRights> {
        self.and_then(|value| value.rights.as_ref())
    }

    #[must_use]
    pub fn is_enabled(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_enabled)
    }
}
impl SmartFilterPath<crate::types::BusinessIntro> {
    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.message.as_deref())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.and_then(|value| value.sticker.as_deref())
    }
}
impl SmartFilterPath<crate::types::BusinessLocation> {
    #[must_use]
    pub fn address(self) -> SmartFilterPath<str> {
        self.map(|value| value.address.as_ref())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.and_then(|value| value.location.as_ref())
    }
}
impl SmartFilterPath<crate::types::BusinessMessagesDeleted> {
    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.business_connection_id.as_ref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn message_ids(self) -> SmartFilterPath<[i64]> {
        self.map(|value| value.message_ids.as_ref())
    }
}
impl SmartFilterPath<crate::types::BusinessOpeningHours> {
    #[must_use]
    pub fn time_zone_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.time_zone_name.as_ref())
    }

    #[must_use]
    pub fn opening_hours(self) -> SmartFilterPath<[crate::types::BusinessOpeningHoursInterval]> {
        self.map(|value| value.opening_hours.as_ref())
    }
}
impl SmartFilterPath<crate::types::BusinessOpeningHoursInterval> {
    #[must_use]
    pub fn opening_minute(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.opening_minute)
    }

    #[must_use]
    pub fn closing_minute(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.closing_minute)
    }
}
impl SmartFilterPath<crate::types::CallbackGame> {}
impl SmartFilterPath<crate::types::CallbackQuery> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        self.and_then(|value| value.message.as_deref())
    }

    #[must_use]
    pub fn inline_message_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.inline_message_id.as_deref())
    }

    #[must_use]
    pub fn chat_instance(self) -> SmartFilterPath<str> {
        self.map(|value| value.chat_instance.as_ref())
    }

    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.data.as_deref())
    }

    #[must_use]
    pub fn game_short_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.game_short_name.as_deref())
    }
}
impl SmartFilterPath<crate::types::Chat> {
    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.first_name())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username())
    }
}
impl SmartFilterPath<crate::types::ChatAdministratorRights> {
    #[must_use]
    pub fn is_anonymous(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_anonymous)
    }

    #[must_use]
    pub fn can_manage_chat(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_manage_chat)
    }

    #[must_use]
    pub fn can_delete_messages(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_delete_messages)
    }

    #[must_use]
    pub fn can_manage_video_chats(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_manage_video_chats)
    }

    #[must_use]
    pub fn can_restrict_members(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_restrict_members)
    }

    #[must_use]
    pub fn can_promote_members(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_promote_members)
    }

    #[must_use]
    pub fn can_change_info(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_change_info)
    }

    #[must_use]
    pub fn can_invite_users(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_invite_users)
    }

    #[must_use]
    pub fn can_post_stories(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_post_stories)
    }

    #[must_use]
    pub fn can_edit_stories(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_edit_stories)
    }

    #[must_use]
    pub fn can_delete_stories(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_delete_stories)
    }

    #[must_use]
    pub fn can_post_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_post_messages.as_ref())
    }

    #[must_use]
    pub fn can_edit_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_edit_messages.as_ref())
    }

    #[must_use]
    pub fn can_pin_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_pin_messages.as_ref())
    }

    #[must_use]
    pub fn can_manage_topics(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_topics.as_ref())
    }

    #[must_use]
    pub fn can_manage_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_direct_messages.as_ref())
    }

    #[must_use]
    pub fn can_manage_tags(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_tags.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatBackground> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<crate::types::BackgroundType> {
        self.map(|value| value.r#type.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatBoost> {
    #[must_use]
    pub fn boost_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.boost_id.as_ref())
    }

    #[must_use]
    pub fn add_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.add_date)
    }

    #[must_use]
    pub fn expiration_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.expiration_date)
    }

    #[must_use]
    pub fn source(self) -> SmartFilterPath<crate::types::ChatBoostSource> {
        self.map(|value| &value.source)
    }
}
impl SmartFilterPath<crate::types::ChatBoostAdded> {
    #[must_use]
    pub fn boost_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.boost_count)
    }
}
impl SmartFilterPath<crate::types::ChatBoostRemoved> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn boost_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.boost_id.as_ref())
    }

    #[must_use]
    pub fn remove_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.remove_date)
    }

    #[must_use]
    pub fn source(self) -> SmartFilterPath<crate::types::ChatBoostSource> {
        self.map(|value| &value.source)
    }
}
impl SmartFilterPath<crate::types::ChatBoostSource> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user())
    }
}
impl SmartFilterPath<crate::types::ChatBoostSourceGiftCode> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatBoostSourceGiveaway> {
    #[must_use]
    pub fn giveaway_message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.giveaway_message_id)
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user.as_deref())
    }

    #[must_use]
    pub fn prize_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.prize_star_count.as_ref())
    }

    #[must_use]
    pub fn is_unclaimed(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_unclaimed.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatBoostSourcePremium> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatBoostUpdated> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn boost(self) -> SmartFilterPath<crate::types::ChatBoost> {
        self.map(|value| &value.boost)
    }
}
impl SmartFilterPath<crate::types::ChatChannel> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatFullInfo> {
    #[must_use]
    pub fn accepted_gift_types(self) -> SmartFilterPath<crate::types::AcceptedGiftTypes> {
        self.and_then(|value| value.accepted_gift_types())
    }

    #[must_use]
    pub fn active_usernames(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.active_usernames())
    }

    #[must_use]
    pub fn available_reactions(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.and_then(|value| value.available_reactions())
    }

    #[must_use]
    pub fn background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.background_custom_emoji_id())
    }

    #[must_use]
    pub fn bio(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.bio())
    }

    #[must_use]
    pub fn birthdate(self) -> SmartFilterPath<crate::types::Birthdate> {
        self.and_then(|value| value.birthdate())
    }

    #[must_use]
    pub fn business_intro(self) -> SmartFilterPath<crate::types::BusinessIntro> {
        self.and_then(|value| value.business_intro())
    }

    #[must_use]
    pub fn business_location(self) -> SmartFilterPath<crate::types::BusinessLocation> {
        self.and_then(|value| value.business_location())
    }

    #[must_use]
    pub fn business_opening_hours(self) -> SmartFilterPath<crate::types::BusinessOpeningHours> {
        self.and_then(|value| value.business_opening_hours())
    }

    #[must_use]
    pub fn custom_emoji_sticker_set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_sticker_set_name())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description())
    }

    #[must_use]
    pub fn emoji_status_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji_status_custom_emoji_id())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.first_name())
    }

    #[must_use]
    pub fn first_profile_audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.and_then(|value| value.first_profile_audio())
    }

    #[must_use]
    pub fn invite_link(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.invite_link())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::ChatLocation> {
        self.and_then(|value| value.location())
    }

    #[must_use]
    pub fn parent_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.parent_chat())
    }

    #[must_use]
    pub fn permissions(self) -> SmartFilterPath<crate::types::ChatPermissions> {
        self.and_then(|value| value.permissions())
    }

    #[must_use]
    pub fn personal_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.personal_chat())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::ChatPhoto> {
        self.and_then(|value| value.photo())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.pinned_message())
    }

    #[must_use]
    pub fn profile_background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.profile_background_custom_emoji_id())
    }

    #[must_use]
    pub fn rating(self) -> SmartFilterPath<crate::types::UserRating> {
        self.and_then(|value| value.rating())
    }

    #[must_use]
    pub fn sticker_set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sticker_set_name())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title())
    }

    #[must_use]
    pub fn unique_gift_colors(self) -> SmartFilterPath<crate::types::UniqueGiftColors> {
        self.and_then(|value| value.unique_gift_colors())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username())
    }
}
impl SmartFilterPath<crate::types::ChatFullInfoChannel> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }

    #[must_use]
    pub fn accent_color_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.accent_color_id)
    }

    #[must_use]
    pub fn max_reaction_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.max_reaction_count)
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::ChatPhoto> {
        self.and_then(|value| value.photo.as_ref())
    }

    #[must_use]
    pub fn active_usernames(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.active_usernames.as_deref())
    }

    #[must_use]
    pub fn personal_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.personal_chat.as_deref())
    }

    #[must_use]
    pub fn parent_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.parent_chat.as_deref())
    }

    #[must_use]
    pub fn available_reactions(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.and_then(|value| value.available_reactions.as_deref())
    }

    #[must_use]
    pub fn background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn profile_accent_color_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.profile_accent_color_id.as_ref())
    }

    #[must_use]
    pub fn profile_background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.profile_background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn invite_link(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.invite_link.as_deref())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.pinned_message.as_deref())
    }

    #[must_use]
    pub fn can_send_paid_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_paid_media.as_ref())
    }

    #[must_use]
    pub fn message_auto_delete_time(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_auto_delete_time.as_ref())
    }

    #[must_use]
    pub fn has_hidden_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_hidden_members.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn has_visible_history(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_visible_history.as_ref())
    }

    #[must_use]
    pub fn linked_chat_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.linked_chat_id.as_ref())
    }

    #[must_use]
    pub fn unique_gift_colors(self) -> SmartFilterPath<crate::types::UniqueGiftColors> {
        self.and_then(|value| value.unique_gift_colors.as_ref())
    }

    #[must_use]
    pub fn paid_message_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_message_star_count.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatFullInfoGroup> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }

    #[must_use]
    pub fn accent_color_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.accent_color_id)
    }

    #[must_use]
    pub fn max_reaction_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.max_reaction_count)
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::ChatPhoto> {
        self.and_then(|value| value.photo.as_ref())
    }

    #[must_use]
    pub fn available_reactions(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.and_then(|value| value.available_reactions.as_deref())
    }

    #[must_use]
    pub fn background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn profile_accent_color_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.profile_accent_color_id.as_ref())
    }

    #[must_use]
    pub fn profile_background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.profile_background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.pinned_message.as_deref())
    }

    #[must_use]
    pub fn message_auto_delete_time(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_auto_delete_time.as_ref())
    }

    #[must_use]
    pub fn has_hidden_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_hidden_members.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn has_visible_history(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_visible_history.as_ref())
    }

    #[must_use]
    pub fn can_set_sticker_set(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_set_sticker_set.as_ref())
    }

    #[must_use]
    pub fn unique_gift_colors(self) -> SmartFilterPath<crate::types::UniqueGiftColors> {
        self.and_then(|value| value.unique_gift_colors.as_ref())
    }

    #[must_use]
    pub fn paid_message_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_message_star_count.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatFullInfoPrivate> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.first_name.as_deref())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name.as_deref())
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }

    #[must_use]
    pub fn accent_color_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.accent_color_id)
    }

    #[must_use]
    pub fn max_reaction_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.max_reaction_count)
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::ChatPhoto> {
        self.and_then(|value| value.photo.as_ref())
    }

    #[must_use]
    pub fn active_usernames(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.active_usernames.as_deref())
    }

    #[must_use]
    pub fn birthdate(self) -> SmartFilterPath<crate::types::Birthdate> {
        self.and_then(|value| value.birthdate.as_ref())
    }

    #[must_use]
    pub fn business_intro(self) -> SmartFilterPath<crate::types::BusinessIntro> {
        self.and_then(|value| value.business_intro.as_ref())
    }

    #[must_use]
    pub fn business_location(self) -> SmartFilterPath<crate::types::BusinessLocation> {
        self.and_then(|value| value.business_location.as_ref())
    }

    #[must_use]
    pub fn business_opening_hours(self) -> SmartFilterPath<crate::types::BusinessOpeningHours> {
        self.and_then(|value| value.business_opening_hours.as_ref())
    }

    #[must_use]
    pub fn personal_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.personal_chat.as_deref())
    }

    #[must_use]
    pub fn available_reactions(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.and_then(|value| value.available_reactions.as_deref())
    }

    #[must_use]
    pub fn background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn profile_accent_color_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.profile_accent_color_id.as_ref())
    }

    #[must_use]
    pub fn profile_background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.profile_background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn emoji_status_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji_status_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn emoji_status_expiration_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.emoji_status_expiration_date.as_ref())
    }

    #[must_use]
    pub fn bio(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.bio.as_deref())
    }

    #[must_use]
    pub fn has_private_forwards(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_private_forwards.as_ref())
    }

    #[must_use]
    pub fn has_restricted_voice_and_video_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_restricted_voice_and_video_messages.as_ref())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.pinned_message.as_deref())
    }

    #[must_use]
    pub fn accepted_gift_types(self) -> SmartFilterPath<crate::types::AcceptedGiftTypes> {
        self.map(|value| &value.accepted_gift_types)
    }

    #[must_use]
    pub fn message_auto_delete_time(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_auto_delete_time.as_ref())
    }

    #[must_use]
    pub fn has_hidden_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_hidden_members.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn has_visible_history(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_visible_history.as_ref())
    }

    #[must_use]
    pub fn rating(self) -> SmartFilterPath<crate::types::UserRating> {
        self.and_then(|value| value.rating.as_ref())
    }

    #[must_use]
    pub fn first_profile_audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.and_then(|value| value.first_profile_audio.as_deref())
    }

    #[must_use]
    pub fn unique_gift_colors(self) -> SmartFilterPath<crate::types::UniqueGiftColors> {
        self.and_then(|value| value.unique_gift_colors.as_ref())
    }

    #[must_use]
    pub fn paid_message_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_message_star_count.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatFullInfoSupergroup> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn is_forum(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_forum.as_ref())
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }

    #[must_use]
    pub fn accent_color_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.accent_color_id)
    }

    #[must_use]
    pub fn max_reaction_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.max_reaction_count)
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::ChatPhoto> {
        self.and_then(|value| value.photo.as_ref())
    }

    #[must_use]
    pub fn active_usernames(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.active_usernames.as_deref())
    }

    #[must_use]
    pub fn available_reactions(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.and_then(|value| value.available_reactions.as_deref())
    }

    #[must_use]
    pub fn background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn profile_accent_color_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.profile_accent_color_id.as_ref())
    }

    #[must_use]
    pub fn profile_background_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.profile_background_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn join_to_send_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.join_to_send_messages.as_ref())
    }

    #[must_use]
    pub fn join_by_request(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.join_by_request.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn invite_link(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.invite_link.as_deref())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.pinned_message.as_deref())
    }

    #[must_use]
    pub fn permissions(self) -> SmartFilterPath<crate::types::ChatPermissions> {
        self.and_then(|value| value.permissions.as_ref())
    }

    #[must_use]
    pub fn slow_mode_delay(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.slow_mode_delay.as_ref())
    }

    #[must_use]
    pub fn unrestrict_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.unrestrict_boost_count.as_ref())
    }

    #[must_use]
    pub fn message_auto_delete_time(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_auto_delete_time.as_ref())
    }

    #[must_use]
    pub fn has_aggressive_anti_spam_enabled(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_aggressive_anti_spam_enabled.as_ref())
    }

    #[must_use]
    pub fn has_hidden_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_hidden_members.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn has_visible_history(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_visible_history.as_ref())
    }

    #[must_use]
    pub fn sticker_set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sticker_set_name.as_deref())
    }

    #[must_use]
    pub fn custom_emoji_sticker_set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_sticker_set_name.as_deref())
    }

    #[must_use]
    pub fn linked_chat_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.linked_chat_id.as_ref())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::ChatLocation> {
        self.and_then(|value| value.location.as_ref())
    }

    #[must_use]
    pub fn unique_gift_colors(self) -> SmartFilterPath<crate::types::UniqueGiftColors> {
        self.and_then(|value| value.unique_gift_colors.as_ref())
    }

    #[must_use]
    pub fn paid_message_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_message_star_count.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatGroup> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatInviteLink> {
    #[must_use]
    pub fn invite_link(self) -> SmartFilterPath<str> {
        self.map(|value| value.invite_link.as_ref())
    }

    #[must_use]
    pub fn creator(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.creator.as_ref())
    }

    #[must_use]
    pub fn creates_join_request(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.creates_join_request)
    }

    #[must_use]
    pub fn is_primary(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_primary)
    }

    #[must_use]
    pub fn is_revoked(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_revoked)
    }

    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.name.as_deref())
    }

    #[must_use]
    pub fn expire_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.expire_date.as_ref())
    }

    #[must_use]
    pub fn member_limit(self) -> SmartFilterPath<u32> {
        self.and_then(|value| value.member_limit.as_ref())
    }

    #[must_use]
    pub fn pending_join_request_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.pending_join_request_count.as_ref())
    }

    #[must_use]
    pub fn subscription_period(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.subscription_period.as_ref())
    }

    #[must_use]
    pub fn subscription_price(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.subscription_price.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatJoinRequest> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn user_chat_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.user_chat_id)
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn bio(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.bio.as_deref())
    }

    #[must_use]
    pub fn invite_link(self) -> SmartFilterPath<crate::types::ChatInviteLink> {
        self.and_then(|value| value.invite_link.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatLocation> {
    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.map(|value| &value.location)
    }

    #[must_use]
    pub fn address(self) -> SmartFilterPath<str> {
        self.map(|value| value.address.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatMember> {
    #[must_use]
    pub fn custom_title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_title())
    }

    #[must_use]
    pub fn tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.tag())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user())
    }
}
impl SmartFilterPath<crate::types::ChatMemberAdministrator> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn can_be_edited(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_be_edited)
    }

    #[must_use]
    pub fn is_anonymous(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_anonymous)
    }

    #[must_use]
    pub fn can_manage_chat(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_manage_chat)
    }

    #[must_use]
    pub fn can_delete_messages(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_delete_messages)
    }

    #[must_use]
    pub fn can_manage_video_chats(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_manage_video_chats)
    }

    #[must_use]
    pub fn can_restrict_members(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_restrict_members)
    }

    #[must_use]
    pub fn can_promote_members(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_promote_members)
    }

    #[must_use]
    pub fn can_change_info(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_change_info)
    }

    #[must_use]
    pub fn can_invite_users(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_invite_users)
    }

    #[must_use]
    pub fn can_post_stories(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_post_stories)
    }

    #[must_use]
    pub fn can_edit_stories(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_edit_stories)
    }

    #[must_use]
    pub fn can_delete_stories(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_delete_stories)
    }

    #[must_use]
    pub fn can_post_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_post_messages.as_ref())
    }

    #[must_use]
    pub fn can_edit_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_edit_messages.as_ref())
    }

    #[must_use]
    pub fn can_pin_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_pin_messages.as_ref())
    }

    #[must_use]
    pub fn can_manage_topics(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_topics.as_ref())
    }

    #[must_use]
    pub fn can_manage_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_direct_messages.as_ref())
    }

    #[must_use]
    pub fn can_manage_tags(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_tags.as_ref())
    }

    #[must_use]
    pub fn custom_title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_title.as_deref())
    }
}
impl SmartFilterPath<crate::types::ChatMemberBanned> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn until_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.until_date)
    }
}
impl SmartFilterPath<crate::types::ChatMemberLeft> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatMemberMember> {
    #[must_use]
    pub fn tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.tag.as_deref())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn until_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.until_date.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatMemberOwner> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn is_anonymous(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_anonymous)
    }

    #[must_use]
    pub fn custom_title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_title.as_deref())
    }
}
impl SmartFilterPath<crate::types::ChatMemberRestricted> {
    #[must_use]
    pub fn tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.tag.as_deref())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn is_member(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_member)
    }

    #[must_use]
    pub fn can_send_messages(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_messages)
    }

    #[must_use]
    pub fn can_send_audios(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_audios)
    }

    #[must_use]
    pub fn can_send_documents(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_documents)
    }

    #[must_use]
    pub fn can_send_photos(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_photos)
    }

    #[must_use]
    pub fn can_send_videos(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_videos)
    }

    #[must_use]
    pub fn can_send_video_notes(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_video_notes)
    }

    #[must_use]
    pub fn can_send_voice_notes(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_voice_notes)
    }

    #[must_use]
    pub fn can_send_polls(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_polls)
    }

    #[must_use]
    pub fn can_send_other_messages(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_send_other_messages)
    }

    #[must_use]
    pub fn can_add_web_page_previews(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_add_web_page_previews)
    }

    #[must_use]
    pub fn can_edit_tag(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_edit_tag)
    }

    #[must_use]
    pub fn can_change_info(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_change_info)
    }

    #[must_use]
    pub fn can_invite_users(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_invite_users)
    }

    #[must_use]
    pub fn can_pin_messages(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_pin_messages)
    }

    #[must_use]
    pub fn can_manage_topics(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.can_manage_topics)
    }

    #[must_use]
    pub fn until_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.until_date)
    }
}
impl SmartFilterPath<crate::types::ChatMemberUpdated> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn old_chat_member(self) -> SmartFilterPath<crate::types::ChatMember> {
        self.map(|value| &value.old_chat_member)
    }

    #[must_use]
    pub fn new_chat_member(self) -> SmartFilterPath<crate::types::ChatMember> {
        self.map(|value| &value.new_chat_member)
    }

    #[must_use]
    pub fn invite_link(self) -> SmartFilterPath<crate::types::ChatInviteLink> {
        self.and_then(|value| value.invite_link.as_ref())
    }

    #[must_use]
    pub fn via_join_request(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.via_join_request.as_ref())
    }

    #[must_use]
    pub fn via_chat_folder_invite_link(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.via_chat_folder_invite_link.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatOwnerChanged> {
    #[must_use]
    pub fn new_owner(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.new_owner.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatOwnerLeft> {
    #[must_use]
    pub fn new_owner(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.new_owner.as_deref())
    }
}
impl SmartFilterPath<crate::types::ChatPermissions> {
    #[must_use]
    pub fn can_send_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_messages.as_ref())
    }

    #[must_use]
    pub fn can_send_audios(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_audios.as_ref())
    }

    #[must_use]
    pub fn can_send_documents(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_documents.as_ref())
    }

    #[must_use]
    pub fn can_send_photos(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_photos.as_ref())
    }

    #[must_use]
    pub fn can_send_videos(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_videos.as_ref())
    }

    #[must_use]
    pub fn can_send_video_notes(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_video_notes.as_ref())
    }

    #[must_use]
    pub fn can_send_voice_notes(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_voice_notes.as_ref())
    }

    #[must_use]
    pub fn can_send_polls(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_polls.as_ref())
    }

    #[must_use]
    pub fn can_send_other_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_send_other_messages.as_ref())
    }

    #[must_use]
    pub fn can_add_web_page_previews(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_add_web_page_previews.as_ref())
    }

    #[must_use]
    pub fn can_edit_tag(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_edit_tag.as_ref())
    }

    #[must_use]
    pub fn can_change_info(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_change_info.as_ref())
    }

    #[must_use]
    pub fn can_invite_users(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_invite_users.as_ref())
    }

    #[must_use]
    pub fn can_pin_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_pin_messages.as_ref())
    }

    #[must_use]
    pub fn can_manage_topics(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_topics.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatPhoto> {
    #[must_use]
    pub fn small_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.small_file_id.as_ref())
    }

    #[must_use]
    pub fn small_file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.small_file_unique_id.as_ref())
    }

    #[must_use]
    pub fn big_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.big_file_id.as_ref())
    }

    #[must_use]
    pub fn big_file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.big_file_unique_id.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatPrivate> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.first_name.as_deref())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name.as_deref())
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChatShared> {
    #[must_use]
    pub fn request_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.request_id)
    }

    #[must_use]
    pub fn chat_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.chat_id)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.photo.as_deref())
    }
}
impl SmartFilterPath<crate::types::ChatSupergroup> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn is_forum(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_forum.as_ref())
    }

    #[must_use]
    pub fn is_direct_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_direct_messages.as_ref())
    }
}
impl SmartFilterPath<crate::types::Checklist> {
    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn title_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.title_entities.as_deref())
    }

    #[must_use]
    pub fn tasks(self) -> SmartFilterPath<[crate::types::ChecklistTask]> {
        self.map(|value| value.tasks.as_ref())
    }

    #[must_use]
    pub fn others_can_add_tasks(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.others_can_add_tasks.as_ref())
    }

    #[must_use]
    pub fn others_can_mark_tasks_as_done(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.others_can_mark_tasks_as_done.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChecklistTask> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn text_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.text_entities.as_deref())
    }

    #[must_use]
    pub fn completed_by_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.completed_by_user.as_deref())
    }

    #[must_use]
    pub fn completed_by_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.completed_by_chat.as_deref())
    }

    #[must_use]
    pub fn completion_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.completion_date.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChecklistTasksAdded> {
    #[must_use]
    pub fn checklist_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.checklist_message.as_deref())
    }

    #[must_use]
    pub fn tasks(self) -> SmartFilterPath<[crate::types::ChecklistTask]> {
        self.map(|value| value.tasks.as_ref())
    }
}
impl SmartFilterPath<crate::types::ChecklistTasksDone> {
    #[must_use]
    pub fn checklist_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.checklist_message.as_deref())
    }

    #[must_use]
    pub fn marked_as_done_task_ids(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.marked_as_done_task_ids.as_deref())
    }

    #[must_use]
    pub fn marked_as_not_done_task_ids(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.marked_as_not_done_task_ids.as_deref())
    }
}
impl SmartFilterPath<crate::types::ChosenInlineResult> {
    #[must_use]
    pub fn result_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.result_id.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.and_then(|value| value.location.as_ref())
    }

    #[must_use]
    pub fn inline_message_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.inline_message_id.as_deref())
    }

    #[must_use]
    pub fn query(self) -> SmartFilterPath<str> {
        self.map(|value| value.query.as_ref())
    }
}
impl SmartFilterPath<crate::types::Contact> {
    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.map(|value| value.phone_number.as_ref())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.first_name.as_ref())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name.as_deref())
    }

    #[must_use]
    pub fn user_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.user_id.as_ref())
    }

    #[must_use]
    pub fn vcard(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.vcard.as_deref())
    }
}
impl SmartFilterPath<crate::types::CopyTextButton> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }
}
impl SmartFilterPath<crate::types::Dice> {
    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.map(|value| value.emoji.as_ref())
    }

    #[must_use]
    pub fn value(self) -> SmartFilterPath<u8> {
        self.map(|value| &value.value)
    }
}
impl SmartFilterPath<crate::types::DirectMessagePriceChanged> {
    #[must_use]
    pub fn are_direct_messages_enabled(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.are_direct_messages_enabled)
    }

    #[must_use]
    pub fn direct_message_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.direct_message_star_count.as_ref())
    }
}
impl SmartFilterPath<crate::types::DirectMessagesTopic> {
    #[must_use]
    pub fn topic_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.topic_id)
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user.as_deref())
    }
}
impl SmartFilterPath<crate::types::Document> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn file_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.file_name.as_deref())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type.as_deref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedCredentials> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }

    #[must_use]
    pub fn secret(self) -> SmartFilterPath<str> {
        self.map(|value| value.secret.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElement> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.data())
    }

    #[must_use]
    pub fn email(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.email())
    }

    #[must_use]
    pub fn files(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.and_then(|value| value.files())
    }

    #[must_use]
    pub fn front_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.and_then(|value| value.front_side())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash())
    }

    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.phone_number())
    }

    #[must_use]
    pub fn reverse_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.and_then(|value| value.reverse_side())
    }

    #[must_use]
    pub fn selfie(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.and_then(|value| value.selfie())
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.and_then(|value| value.translation())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementAddress> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementBankStatement> {
    #[must_use]
    pub fn files(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.files.as_ref())
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementDriverLicense> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn front_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.front_side)
    }

    #[must_use]
    pub fn reverse_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.reverse_side)
    }

    #[must_use]
    pub fn selfie(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.selfie)
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementEmail> {
    #[must_use]
    pub fn email(self) -> SmartFilterPath<str> {
        self.map(|value| value.email.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementIdentityCard> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn front_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.front_side)
    }

    #[must_use]
    pub fn reverse_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.reverse_side)
    }

    #[must_use]
    pub fn selfie(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.selfie)
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementInternalPassport> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn front_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.front_side)
    }

    #[must_use]
    pub fn selfie(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.selfie)
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementPassport> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn front_side(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.front_side)
    }

    #[must_use]
    pub fn selfie(self) -> SmartFilterPath<crate::types::PassportFile> {
        self.map(|value| &value.selfie)
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementPassportRegistration> {
    #[must_use]
    pub fn files(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.files.as_ref())
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementPersonalDetails> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementPhoneNumber> {
    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.map(|value| value.phone_number.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementRentalAgreement> {
    #[must_use]
    pub fn files(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.files.as_ref())
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementTemporaryRegistration> {
    #[must_use]
    pub fn files(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.files.as_ref())
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::EncryptedPassportElementUtilityBill> {
    #[must_use]
    pub fn files(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.files.as_ref())
    }

    #[must_use]
    pub fn translation(self) -> SmartFilterPath<[crate::types::PassportFile]> {
        self.map(|value| value.translation.as_ref())
    }

    #[must_use]
    pub fn hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.hash.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfo> {
    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::Animation> {
        self.and_then(|value| value.animation())
    }

    #[must_use]
    pub fn audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.and_then(|value| value.audio())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat())
    }

    #[must_use]
    pub fn checklist(self) -> SmartFilterPath<crate::types::Checklist> {
        self.and_then(|value| value.checklist())
    }

    #[must_use]
    pub fn contact(self) -> SmartFilterPath<crate::types::Contact> {
        self.and_then(|value| value.contact())
    }

    #[must_use]
    pub fn dice(self) -> SmartFilterPath<crate::types::Dice> {
        self.and_then(|value| value.dice())
    }

    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.and_then(|value| value.document())
    }

    #[must_use]
    pub fn game(self) -> SmartFilterPath<crate::types::Game> {
        self.and_then(|value| value.game())
    }

    #[must_use]
    pub fn giveaway(self) -> SmartFilterPath<crate::types::Giveaway> {
        self.and_then(|value| value.giveaway())
    }

    #[must_use]
    pub fn giveaway_winners(self) -> SmartFilterPath<crate::types::GiveawayWinners> {
        self.and_then(|value| value.giveaway_winners())
    }

    #[must_use]
    pub fn invoice(self) -> SmartFilterPath<crate::types::Invoice> {
        self.and_then(|value| value.invoice())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.and_then(|value| value.location())
    }

    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| value.origin())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.photo())
    }

    #[must_use]
    pub fn poll(self) -> SmartFilterPath<crate::types::Poll> {
        self.and_then(|value| value.poll())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.and_then(|value| value.sticker())
    }

    #[must_use]
    pub fn story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.story())
    }

    #[must_use]
    pub fn venue(self) -> SmartFilterPath<crate::types::Venue> {
        self.and_then(|value| value.venue())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.and_then(|value| value.video())
    }

    #[must_use]
    pub fn video_note(self) -> SmartFilterPath<crate::types::VideoNote> {
        self.and_then(|value| value.video_note())
    }

    #[must_use]
    pub fn voice(self) -> SmartFilterPath<crate::types::Voice> {
        self.and_then(|value| value.voice())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoAnimation> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::Animation> {
        self.map(|value| value.animation.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoAudio> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.map(|value| value.audio.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoChecklist> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn checklist(self) -> SmartFilterPath<crate::types::Checklist> {
        self.map(|value| &value.checklist)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoContact> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn contact(self) -> SmartFilterPath<crate::types::Contact> {
        self.map(|value| &value.contact)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoDice> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn dice(self) -> SmartFilterPath<crate::types::Dice> {
        self.map(|value| &value.dice)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoDocument> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.map(|value| value.document.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoGame> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn game(self) -> SmartFilterPath<crate::types::Game> {
        self.map(|value| &value.game)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoGiveaway> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn giveaway(self) -> SmartFilterPath<crate::types::Giveaway> {
        self.map(|value| &value.giveaway)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoGiveawayWinners> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn giveaway_winners(self) -> SmartFilterPath<crate::types::GiveawayWinners> {
        self.map(|value| &value.giveaway_winners)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoInvoice> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn invoice(self) -> SmartFilterPath<crate::types::Invoice> {
        self.map(|value| &value.invoice)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoLocation> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.map(|value| &value.location)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoPhoto> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.map(|value| value.photo.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoPoll> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn poll(self) -> SmartFilterPath<crate::types::Poll> {
        self.map(|value| value.poll.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoSticker> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.map(|value| value.sticker.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoStory> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn story(self) -> SmartFilterPath<crate::types::Story> {
        self.map(|value| &value.story)
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoVenue> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn venue(self) -> SmartFilterPath<crate::types::Venue> {
        self.map(|value| value.venue.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoVideo> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.map(|value| value.video.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoVideoNote> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn video_note(self) -> SmartFilterPath<crate::types::VideoNote> {
        self.map(|value| value.video_note.as_ref())
    }
}
impl SmartFilterPath<crate::types::ExternalReplyInfoVoice> {
    #[must_use]
    pub fn origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.map(|value| &value.origin)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat.as_deref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_id.as_ref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn voice(self) -> SmartFilterPath<crate::types::Voice> {
        self.map(|value| &value.voice)
    }
}
impl SmartFilterPath<crate::types::File> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }

    #[must_use]
    pub fn file_path(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.file_path.as_deref())
    }
}
impl SmartFilterPath<crate::types::ForceReply> {
    #[must_use]
    pub fn force_reply(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.force_reply)
    }

    #[must_use]
    pub fn input_field_placeholder(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.input_field_placeholder.as_deref())
    }

    #[must_use]
    pub fn selective(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.selective.as_ref())
    }
}
impl SmartFilterPath<crate::types::ForumTopic> {
    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_thread_id)
    }

    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }

    #[must_use]
    pub fn icon_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.icon_color)
    }

    #[must_use]
    pub fn icon_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.icon_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn is_name_implicit(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_name_implicit.as_ref())
    }
}
impl SmartFilterPath<crate::types::ForumTopicClosed> {}
impl SmartFilterPath<crate::types::ForumTopicCreated> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }

    #[must_use]
    pub fn icon_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.icon_color)
    }

    #[must_use]
    pub fn icon_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.icon_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn is_name_implicit(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_name_implicit.as_ref())
    }
}
impl SmartFilterPath<crate::types::ForumTopicEdited> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.name.as_deref())
    }

    #[must_use]
    pub fn icon_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.icon_custom_emoji_id.as_deref())
    }
}
impl SmartFilterPath<crate::types::ForumTopicReopened> {}
impl SmartFilterPath<crate::types::Game> {
    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.map(|value| value.description.as_ref())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.map(|value| value.photo.as_ref())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text.as_deref())
    }

    #[must_use]
    pub fn text_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.text_entities.as_deref())
    }

    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::Animation> {
        self.and_then(|value| value.animation.as_deref())
    }
}
impl SmartFilterPath<crate::types::GameHighScore> {
    #[must_use]
    pub fn position(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.position)
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn score(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.score)
    }
}
impl SmartFilterPath<crate::types::GeneralForumTopicHidden> {}
impl SmartFilterPath<crate::types::GeneralForumTopicUnhidden> {}
impl SmartFilterPath<crate::types::Gift> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.map(|value| value.sticker.as_ref())
    }

    #[must_use]
    pub fn star_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.star_count)
    }

    #[must_use]
    pub fn upgrade_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.upgrade_star_count.as_ref())
    }

    #[must_use]
    pub fn is_premium(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_premium.as_ref())
    }

    #[must_use]
    pub fn has_colors(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_colors.as_ref())
    }

    #[must_use]
    pub fn total_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.total_count.as_ref())
    }

    #[must_use]
    pub fn remaining_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.remaining_count.as_ref())
    }

    #[must_use]
    pub fn personal_total_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.personal_total_count.as_ref())
    }

    #[must_use]
    pub fn personal_remaining_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.personal_remaining_count.as_ref())
    }

    #[must_use]
    pub fn background(self) -> SmartFilterPath<crate::types::GiftBackground> {
        self.and_then(|value| value.background.as_ref())
    }

    #[must_use]
    pub fn unique_gift_variant_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.unique_gift_variant_count.as_ref())
    }

    #[must_use]
    pub fn publisher_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.publisher_chat.as_deref())
    }
}
impl SmartFilterPath<crate::types::GiftBackground> {
    #[must_use]
    pub fn center_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.center_color)
    }

    #[must_use]
    pub fn edge_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.edge_color)
    }

    #[must_use]
    pub fn text_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.text_color)
    }
}
impl SmartFilterPath<crate::types::GiftInfo> {
    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::Gift> {
        self.map(|value| value.gift.as_ref())
    }

    #[must_use]
    pub fn owned_gift_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.owned_gift_id.as_deref())
    }

    #[must_use]
    pub fn convert_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.convert_star_count.as_ref())
    }

    #[must_use]
    pub fn prepaid_upgrade_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.prepaid_upgrade_star_count.as_ref())
    }

    #[must_use]
    pub fn is_upgrade_separate(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_upgrade_separate.as_ref())
    }

    #[must_use]
    pub fn can_be_upgraded(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_be_upgraded.as_ref())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text.as_deref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn is_private(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_private.as_ref())
    }

    #[must_use]
    pub fn unique_gift_number(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.unique_gift_number.as_ref())
    }
}
impl SmartFilterPath<crate::types::Gifts> {
    #[must_use]
    pub fn gifts(self) -> SmartFilterPath<[crate::types::Gift]> {
        self.map(|value| value.gifts.as_ref())
    }
}
impl SmartFilterPath<crate::types::Giveaway> {
    #[must_use]
    pub fn chats(self) -> SmartFilterPath<[crate::types::Chat]> {
        self.map(|value| value.chats())
    }

    #[must_use]
    pub fn country_codes(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.country_codes())
    }

    #[must_use]
    pub fn prize_description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.prize_description())
    }
}
impl SmartFilterPath<crate::types::GiveawayCompleted> {
    #[must_use]
    pub fn winner_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winner_count)
    }

    #[must_use]
    pub fn unclaimed_prize_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.unclaimed_prize_count.as_ref())
    }

    #[must_use]
    pub fn giveaway_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.giveaway_message.as_deref())
    }

    #[must_use]
    pub fn is_star_giveaway(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_star_giveaway.as_ref())
    }
}
impl SmartFilterPath<crate::types::GiveawayCreated> {
    #[must_use]
    pub fn prize_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.prize_star_count.as_ref())
    }
}
impl SmartFilterPath<crate::types::GiveawayPremium> {
    #[must_use]
    pub fn chats(self) -> SmartFilterPath<[crate::types::Chat]> {
        self.map(|value| value.chats.as_ref())
    }

    #[must_use]
    pub fn winners_selection_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winners_selection_date)
    }

    #[must_use]
    pub fn winner_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winner_count)
    }

    #[must_use]
    pub fn only_new_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.only_new_members.as_ref())
    }

    #[must_use]
    pub fn has_public_winners(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_public_winners.as_ref())
    }

    #[must_use]
    pub fn prize_description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.prize_description.as_deref())
    }

    #[must_use]
    pub fn country_codes(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.country_codes.as_deref())
    }

    #[must_use]
    pub fn premium_subscription_month_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.premium_subscription_month_count)
    }
}
impl SmartFilterPath<crate::types::GiveawayStar> {
    #[must_use]
    pub fn chats(self) -> SmartFilterPath<[crate::types::Chat]> {
        self.map(|value| value.chats.as_ref())
    }

    #[must_use]
    pub fn winners_selection_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winners_selection_date)
    }

    #[must_use]
    pub fn winner_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winner_count)
    }

    #[must_use]
    pub fn only_new_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.only_new_members.as_ref())
    }

    #[must_use]
    pub fn has_public_winners(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_public_winners.as_ref())
    }

    #[must_use]
    pub fn prize_description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.prize_description.as_deref())
    }

    #[must_use]
    pub fn country_codes(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.country_codes.as_deref())
    }

    #[must_use]
    pub fn prize_star_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.prize_star_count)
    }
}
impl SmartFilterPath<crate::types::GiveawayWinners> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat())
    }

    #[must_use]
    pub fn prize_description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.prize_description())
    }

    #[must_use]
    pub fn winners(self) -> SmartFilterPath<[crate::types::User]> {
        self.map(|value| value.winners())
    }
}
impl SmartFilterPath<crate::types::GiveawayWinnersPremium> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn giveaway_message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.giveaway_message_id)
    }

    #[must_use]
    pub fn winners_selection_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winners_selection_date)
    }

    #[must_use]
    pub fn winner_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winner_count)
    }

    #[must_use]
    pub fn winners(self) -> SmartFilterPath<[crate::types::User]> {
        self.map(|value| value.winners.as_ref())
    }

    #[must_use]
    pub fn additional_chat_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.additional_chat_count.as_ref())
    }

    #[must_use]
    pub fn premium_subscription_month_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.premium_subscription_month_count)
    }

    #[must_use]
    pub fn unclaimed_prize_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.unclaimed_prize_count.as_ref())
    }

    #[must_use]
    pub fn only_new_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.only_new_members.as_ref())
    }

    #[must_use]
    pub fn was_refunded(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.was_refunded.as_ref())
    }

    #[must_use]
    pub fn prize_description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.prize_description.as_deref())
    }
}
impl SmartFilterPath<crate::types::GiveawayWinnersStar> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn giveaway_message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.giveaway_message_id)
    }

    #[must_use]
    pub fn winners_selection_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winners_selection_date)
    }

    #[must_use]
    pub fn winner_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.winner_count)
    }

    #[must_use]
    pub fn winners(self) -> SmartFilterPath<[crate::types::User]> {
        self.map(|value| value.winners.as_ref())
    }

    #[must_use]
    pub fn additional_chat_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.additional_chat_count.as_ref())
    }

    #[must_use]
    pub fn prize_star_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.prize_star_count)
    }

    #[must_use]
    pub fn unclaimed_prize_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.unclaimed_prize_count.as_ref())
    }

    #[must_use]
    pub fn only_new_members(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.only_new_members.as_ref())
    }

    #[must_use]
    pub fn was_refunded(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.was_refunded.as_ref())
    }

    #[must_use]
    pub fn prize_description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.prize_description.as_deref())
    }
}
impl SmartFilterPath<crate::types::InaccessibleMessage> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }
}
impl SmartFilterPath<crate::types::InlineKeyboardButton> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn icon_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.icon_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn style(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.style.as_deref())
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.url.as_deref())
    }

    #[must_use]
    pub fn callback_data(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.callback_data.as_deref())
    }

    #[must_use]
    pub fn web_app(self) -> SmartFilterPath<crate::types::WebAppInfo> {
        self.and_then(|value| value.web_app.as_ref())
    }

    #[must_use]
    pub fn login_url(self) -> SmartFilterPath<crate::types::LoginUrl> {
        self.and_then(|value| value.login_url.as_ref())
    }

    #[must_use]
    pub fn switch_inline_query(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.switch_inline_query.as_deref())
    }

    #[must_use]
    pub fn switch_inline_query_current_chat(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.switch_inline_query_current_chat.as_deref())
    }

    #[must_use]
    pub fn switch_inline_query_chosen_chat(
        self,
    ) -> SmartFilterPath<crate::types::SwitchInlineQueryChosenChat> {
        self.and_then(|value| value.switch_inline_query_chosen_chat.as_ref())
    }

    #[must_use]
    pub fn copy_text(self) -> SmartFilterPath<crate::types::CopyTextButton> {
        self.and_then(|value| value.copy_text.as_ref())
    }

    #[must_use]
    pub fn callback_game(self) -> SmartFilterPath<crate::types::CallbackGame> {
        self.and_then(|value| value.callback_game.as_ref())
    }

    #[must_use]
    pub fn pay(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.pay.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineKeyboardMarkup> {
    #[must_use]
    pub fn inline_keyboard(self) -> SmartFilterPath<[Box<[crate::types::InlineKeyboardButton]>]> {
        self.map(|value| value.inline_keyboard.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQuery> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn query(self) -> SmartFilterPath<str> {
        self.map(|value| value.query.as_ref())
    }

    #[must_use]
    pub fn offset(self) -> SmartFilterPath<str> {
        self.map(|value| value.offset.as_ref())
    }

    #[must_use]
    pub fn chat_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.chat_type.as_deref())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.and_then(|value| value.location.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResult> {
    #[must_use]
    pub fn address(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.address())
    }

    #[must_use]
    pub fn audio_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.audio_file_id())
    }

    #[must_use]
    pub fn audio_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.audio_url())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description())
    }

    #[must_use]
    pub fn document_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.document_file_id())
    }

    #[must_use]
    pub fn document_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.document_url())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.first_name())
    }

    #[must_use]
    pub fn foursquare_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_id())
    }

    #[must_use]
    pub fn foursquare_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_type())
    }

    #[must_use]
    pub fn game_short_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.game_short_name())
    }

    #[must_use]
    pub fn gif_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.gif_file_id())
    }

    #[must_use]
    pub fn gif_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.gif_url())
    }

    #[must_use]
    pub fn google_place_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_id())
    }

    #[must_use]
    pub fn google_place_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_type())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn performer(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.performer())
    }

    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.phone_number())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn sticker_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sticker_file_id())
    }

    #[must_use]
    pub fn thumbnail_mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_mime_type())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title())
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.url())
    }

    #[must_use]
    pub fn vcard(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.vcard())
    }

    #[must_use]
    pub fn video_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.video_file_id())
    }

    #[must_use]
    pub fn video_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.video_url())
    }

    #[must_use]
    pub fn voice_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.voice_file_id())
    }

    #[must_use]
    pub fn voice_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.voice_url())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultArticle> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.map(|value| &value.input_message_content)
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.url.as_deref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url.as_deref())
    }

    #[must_use]
    pub fn thumbnail_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_width.as_ref())
    }

    #[must_use]
    pub fn thumbnail_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_height.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultAudio> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn audio_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.audio_url.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn performer(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.performer.as_deref())
    }

    #[must_use]
    pub fn audio_duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.audio_duration.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultAudioKind> {
    #[must_use]
    pub fn audio_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.audio_file_id())
    }

    #[must_use]
    pub fn audio_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.audio_url())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn performer(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.performer())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedAudio> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn audio_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.audio_file_id.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedDocument> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn document_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.document_file_id.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedGif> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn gif_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.gif_file_id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedMpeg4Gif> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn mpeg4_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.mpeg4_file_id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedPhoto> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn photo_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.photo_file_id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedSticker> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn sticker_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.sticker_file_id.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedVideo> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn video_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.video_file_id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultCachedVoice> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn voice_file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.voice_file_id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultContact> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.map(|value| value.phone_number.as_ref())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.first_name.as_ref())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name.as_deref())
    }

    #[must_use]
    pub fn vcard(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.vcard.as_deref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url.as_deref())
    }

    #[must_use]
    pub fn thumbnail_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_width.as_ref())
    }

    #[must_use]
    pub fn thumbnail_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_height.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultDocument> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn document_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.document_url.as_ref())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.map(|value| value.mime_type.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url.as_deref())
    }

    #[must_use]
    pub fn thumbnail_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_width.as_ref())
    }

    #[must_use]
    pub fn thumbnail_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_height.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultDocumentKind> {
    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description())
    }

    #[must_use]
    pub fn document_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.document_file_id())
    }

    #[must_use]
    pub fn document_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.document_url())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultGame> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn game_short_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.game_short_name.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultGif> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn gif_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.gif_url.as_ref())
    }

    #[must_use]
    pub fn gif_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.gif_width.as_ref())
    }

    #[must_use]
    pub fn gif_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.gif_height.as_ref())
    }

    #[must_use]
    pub fn gif_duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.gif_duration.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.thumbnail_url.as_ref())
    }

    #[must_use]
    pub fn thumbnail_mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_mime_type.as_deref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultGifKind> {
    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn gif_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.gif_file_id())
    }

    #[must_use]
    pub fn gif_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.gif_url())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn thumbnail_mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_mime_type())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultLocation> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn latitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.latitude)
    }

    #[must_use]
    pub fn longitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.longitude)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn horizontal_accuracy(self) -> SmartFilterPath<f64> {
        self.and_then(|value| value.horizontal_accuracy.as_ref())
    }

    #[must_use]
    pub fn live_period(self) -> SmartFilterPath<u32> {
        self.and_then(|value| value.live_period.as_ref())
    }

    #[must_use]
    pub fn heading(self) -> SmartFilterPath<u16> {
        self.and_then(|value| value.heading.as_ref())
    }

    #[must_use]
    pub fn proximity_alert_radius(self) -> SmartFilterPath<u32> {
        self.and_then(|value| value.proximity_alert_radius.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url.as_deref())
    }

    #[must_use]
    pub fn thumbnail_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_width.as_ref())
    }

    #[must_use]
    pub fn thumbnail_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_height.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultMpeg4Gif> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn mpeg4_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.mpeg4_url.as_ref())
    }

    #[must_use]
    pub fn mpeg4_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.mpeg4_width.as_ref())
    }

    #[must_use]
    pub fn mpeg4_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.mpeg4_height.as_ref())
    }

    #[must_use]
    pub fn mpeg4_duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.mpeg4_duration.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.thumbnail_url.as_ref())
    }

    #[must_use]
    pub fn thumbnail_mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_mime_type.as_deref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultPhoto> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn photo_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.photo_url.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.thumbnail_url.as_ref())
    }

    #[must_use]
    pub fn photo_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.photo_width.as_ref())
    }

    #[must_use]
    pub fn photo_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.photo_height.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultVenue> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn latitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.latitude)
    }

    #[must_use]
    pub fn longitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.longitude)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn address(self) -> SmartFilterPath<str> {
        self.map(|value| value.address.as_ref())
    }

    #[must_use]
    pub fn foursquare_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_id.as_deref())
    }

    #[must_use]
    pub fn foursquare_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_type.as_deref())
    }

    #[must_use]
    pub fn google_place_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_id.as_deref())
    }

    #[must_use]
    pub fn google_place_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_type.as_deref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url.as_deref())
    }

    #[must_use]
    pub fn thumbnail_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_width.as_ref())
    }

    #[must_use]
    pub fn thumbnail_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.thumbnail_height.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultVideo> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn video_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.video_url.as_ref())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.map(|value| value.mime_type.as_ref())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.thumbnail_url.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn video_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.video_width.as_ref())
    }

    #[must_use]
    pub fn video_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.video_height.as_ref())
    }

    #[must_use]
    pub fn video_duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.video_duration.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultVideoKind> {
    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn thumbnail_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.thumbnail_url())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title())
    }

    #[must_use]
    pub fn video_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.video_file_id())
    }

    #[must_use]
    pub fn video_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.video_url())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultVoice> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn voice_url(self) -> SmartFilterPath<str> {
        self.map(|value| value.voice_url.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn voice_duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.voice_duration.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content.as_ref())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultVoiceKind> {
    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn input_message_content(self) -> SmartFilterPath<crate::types::InputMessageContent> {
        self.and_then(|value| value.input_message_content())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title())
    }

    #[must_use]
    pub fn voice_file_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.voice_file_id())
    }

    #[must_use]
    pub fn voice_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.voice_url())
    }
}
impl SmartFilterPath<crate::types::InlineQueryResultsButton> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn web_app(self) -> SmartFilterPath<crate::types::WebAppInfo> {
        self.and_then(|value| value.web_app.as_ref())
    }

    #[must_use]
    pub fn start_parameter(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.start_parameter.as_deref())
    }
}
impl SmartFilterPath<crate::types::InputChecklist> {
    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn title_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.title_entities.as_deref())
    }

    #[must_use]
    pub fn tasks(self) -> SmartFilterPath<[crate::types::InputChecklistTask]> {
        self.map(|value| value.tasks.as_ref())
    }

    #[must_use]
    pub fn others_can_add_tasks(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.others_can_add_tasks.as_ref())
    }

    #[must_use]
    pub fn others_can_mark_tasks_as_done(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.others_can_mark_tasks_as_done.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputChecklistTask> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn text_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.text_entities.as_deref())
    }
}
impl SmartFilterPath<crate::types::InputContactMessageContent> {
    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.map(|value| value.phone_number.as_ref())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.first_name.as_ref())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name.as_deref())
    }

    #[must_use]
    pub fn vcard(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.vcard.as_deref())
    }
}
impl SmartFilterPath<crate::types::InputInvoiceMessageContent> {
    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.map(|value| value.description.as_ref())
    }

    #[must_use]
    pub fn payload(self) -> SmartFilterPath<str> {
        self.map(|value| value.payload.as_ref())
    }

    #[must_use]
    pub fn provider_token(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.provider_token.as_deref())
    }

    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.map(|value| value.currency.as_ref())
    }

    #[must_use]
    pub fn prices(self) -> SmartFilterPath<[crate::types::LabeledPrice]> {
        self.map(|value| value.prices.as_ref())
    }

    #[must_use]
    pub fn max_tip_amount(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.max_tip_amount.as_ref())
    }

    #[must_use]
    pub fn suggested_tip_amounts(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.suggested_tip_amounts.as_deref())
    }

    #[must_use]
    pub fn provider_data(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.provider_data.as_deref())
    }

    #[must_use]
    pub fn photo_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.photo_url.as_deref())
    }

    #[must_use]
    pub fn photo_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.photo_size.as_ref())
    }

    #[must_use]
    pub fn photo_width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.photo_width.as_ref())
    }

    #[must_use]
    pub fn photo_height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.photo_height.as_ref())
    }

    #[must_use]
    pub fn need_name(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.need_name.as_ref())
    }

    #[must_use]
    pub fn need_phone_number(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.need_phone_number.as_ref())
    }

    #[must_use]
    pub fn need_email(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.need_email.as_ref())
    }

    #[must_use]
    pub fn need_shipping_address(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.need_shipping_address.as_ref())
    }

    #[must_use]
    pub fn send_phone_number_to_provider(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.send_phone_number_to_provider.as_ref())
    }

    #[must_use]
    pub fn send_email_to_provider(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.send_email_to_provider.as_ref())
    }

    #[must_use]
    pub fn is_flexible(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_flexible.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputLocationMessageContent> {
    #[must_use]
    pub fn latitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.latitude)
    }

    #[must_use]
    pub fn longitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.longitude)
    }

    #[must_use]
    pub fn horizontal_accuracy(self) -> SmartFilterPath<f64> {
        self.and_then(|value| value.horizontal_accuracy.as_ref())
    }

    #[must_use]
    pub fn live_period(self) -> SmartFilterPath<u32> {
        self.and_then(|value| value.live_period.as_ref())
    }

    #[must_use]
    pub fn heading(self) -> SmartFilterPath<u16> {
        self.and_then(|value| value.heading.as_ref())
    }

    #[must_use]
    pub fn proximity_alert_radius(self) -> SmartFilterPath<u32> {
        self.and_then(|value| value.proximity_alert_radius.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputMedia> {
    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn cover(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.cover())
    }

    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| value.media())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn performer(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.performer())
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.thumbnail())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title())
    }
}
impl SmartFilterPath<crate::types::InputMediaAnimation> {
    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.media)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.width.as_ref())
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.height.as_ref())
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.duration.as_ref())
    }

    #[must_use]
    pub fn has_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_spoiler.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputMediaAudio> {
    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.media)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.duration.as_ref())
    }

    #[must_use]
    pub fn performer(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.performer.as_deref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title.as_deref())
    }
}
impl SmartFilterPath<crate::types::InputMediaDocument> {
    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.media)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn disable_content_type_detection(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.disable_content_type_detection.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputMediaPhoto> {
    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.media)
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_spoiler.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputMediaVideo> {
    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.media)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn cover(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.cover.as_ref())
    }

    #[must_use]
    pub fn start_timestamp(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.start_timestamp.as_ref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.width.as_ref())
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.height.as_ref())
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.duration.as_ref())
    }

    #[must_use]
    pub fn supports_streaming(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.supports_streaming.as_ref())
    }

    #[must_use]
    pub fn has_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_spoiler.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputMessageContent> {
    #[must_use]
    pub fn address(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.address())
    }

    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.currency())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities())
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.first_name())
    }

    #[must_use]
    pub fn foursquare_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_id())
    }

    #[must_use]
    pub fn foursquare_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_type())
    }

    #[must_use]
    pub fn google_place_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_id())
    }

    #[must_use]
    pub fn google_place_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_type())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options())
    }

    #[must_use]
    pub fn message_text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.message_text())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode())
    }

    #[must_use]
    pub fn payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.payload())
    }

    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.phone_number())
    }

    #[must_use]
    pub fn photo_url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.photo_url())
    }

    #[must_use]
    pub fn prices(self) -> SmartFilterPath<[crate::types::LabeledPrice]> {
        self.and_then(|value| value.prices())
    }

    #[must_use]
    pub fn provider_data(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.provider_data())
    }

    #[must_use]
    pub fn provider_token(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.provider_token())
    }

    #[must_use]
    pub fn suggested_tip_amounts(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.suggested_tip_amounts())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.title())
    }

    #[must_use]
    pub fn vcard(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.vcard())
    }
}
impl SmartFilterPath<crate::types::InputPaidMedia> {
    #[must_use]
    pub fn cover(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.cover())
    }

    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| value.media())
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.thumbnail())
    }
}
impl SmartFilterPath<crate::types::InputPaidMediaPhoto> {
    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.media)
    }
}
impl SmartFilterPath<crate::types::InputPaidMediaVideo> {
    #[must_use]
    pub fn media(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.media)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn cover(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.cover.as_ref())
    }

    #[must_use]
    pub fn start_timestamp(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.start_timestamp.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.width.as_ref())
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.height.as_ref())
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.duration.as_ref())
    }

    #[must_use]
    pub fn supports_streaming(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.supports_streaming.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputPollOption> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn text_parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text_parse_mode.as_deref())
    }

    #[must_use]
    pub fn text_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.text_entities.as_deref())
    }
}
impl SmartFilterPath<crate::types::InputProfilePhoto> {
    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.animation())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.photo())
    }
}
impl SmartFilterPath<crate::types::InputProfilePhotoAnimated> {
    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.animation)
    }

    #[must_use]
    pub fn main_frame_timestamp(self) -> SmartFilterPath<f64> {
        self.and_then(|value| value.main_frame_timestamp.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputProfilePhotoStatic> {
    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.photo)
    }
}
impl SmartFilterPath<crate::types::InputSticker> {
    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.sticker)
    }

    #[must_use]
    pub fn format(self) -> SmartFilterPath<str> {
        self.map(|value| value.format.as_ref())
    }

    #[must_use]
    pub fn emoji_list(self) -> SmartFilterPath<[Box<str>]> {
        self.map(|value| value.emoji_list.as_ref())
    }

    #[must_use]
    pub fn mask_position(self) -> SmartFilterPath<crate::types::MaskPosition> {
        self.and_then(|value| value.mask_position.as_ref())
    }

    #[must_use]
    pub fn keywords(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.keywords.as_deref())
    }
}
impl SmartFilterPath<crate::types::InputStoryContent> {
    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.photo())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::InputFile> {
        self.and_then(|value| value.video())
    }
}
impl SmartFilterPath<crate::types::InputStoryContentPhoto> {
    #[must_use]
    pub fn photo(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.photo)
    }
}
impl SmartFilterPath<crate::types::InputStoryContentVideo> {
    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::InputFile> {
        self.map(|value| &value.video)
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<f64> {
        self.and_then(|value| value.duration.as_ref())
    }

    #[must_use]
    pub fn cover_frame_timestamp(self) -> SmartFilterPath<f64> {
        self.and_then(|value| value.cover_frame_timestamp.as_ref())
    }

    #[must_use]
    pub fn is_animation(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_animation.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputTextMessageContent> {
    #[must_use]
    pub fn message_text(self) -> SmartFilterPath<str> {
        self.map(|value| value.message_text.as_ref())
    }

    #[must_use]
    pub fn parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.parse_mode.as_deref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }
}
impl SmartFilterPath<crate::types::InputVenueMessageContent> {
    #[must_use]
    pub fn latitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.latitude)
    }

    #[must_use]
    pub fn longitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.longitude)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn address(self) -> SmartFilterPath<str> {
        self.map(|value| value.address.as_ref())
    }

    #[must_use]
    pub fn foursquare_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_id.as_deref())
    }

    #[must_use]
    pub fn foursquare_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_type.as_deref())
    }

    #[must_use]
    pub fn google_place_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_id.as_deref())
    }

    #[must_use]
    pub fn google_place_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_type.as_deref())
    }
}
impl SmartFilterPath<crate::types::Invoice> {
    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.map(|value| value.description.as_ref())
    }

    #[must_use]
    pub fn start_parameter(self) -> SmartFilterPath<str> {
        self.map(|value| value.start_parameter.as_ref())
    }

    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.map(|value| value.currency.as_ref())
    }

    #[must_use]
    pub fn total_amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_amount)
    }
}
impl SmartFilterPath<crate::types::KeyboardButton> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn icon_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.icon_custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn style(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.style.as_deref())
    }

    #[must_use]
    pub fn request_users(self) -> SmartFilterPath<crate::types::KeyboardButtonRequestUsers> {
        self.and_then(|value| value.request_users.as_ref())
    }

    #[must_use]
    pub fn request_chat(self) -> SmartFilterPath<crate::types::KeyboardButtonRequestChat> {
        self.and_then(|value| value.request_chat.as_ref())
    }

    #[must_use]
    pub fn request_managed_bot(
        self,
    ) -> SmartFilterPath<crate::types::KeyboardButtonRequestManagedBot> {
        self.and_then(|value| value.request_managed_bot.as_ref())
    }

    #[must_use]
    pub fn request_contact(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_contact.as_ref())
    }

    #[must_use]
    pub fn request_location(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_location.as_ref())
    }

    #[must_use]
    pub fn request_poll(self) -> SmartFilterPath<crate::types::KeyboardButtonPollType> {
        self.and_then(|value| value.request_poll.as_ref())
    }

    #[must_use]
    pub fn web_app(self) -> SmartFilterPath<crate::types::WebAppInfo> {
        self.and_then(|value| value.web_app.as_ref())
    }
}
impl SmartFilterPath<crate::types::KeyboardButtonPollType> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.r#type.as_deref())
    }
}
impl SmartFilterPath<crate::types::KeyboardButtonRequestChat> {
    #[must_use]
    pub fn request_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.request_id)
    }

    #[must_use]
    pub fn chat_is_channel(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.chat_is_channel)
    }

    #[must_use]
    pub fn chat_is_forum(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.chat_is_forum.as_ref())
    }

    #[must_use]
    pub fn chat_has_username(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.chat_has_username.as_ref())
    }

    #[must_use]
    pub fn chat_is_created(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.chat_is_created.as_ref())
    }

    #[must_use]
    pub fn user_administrator_rights(
        self,
    ) -> SmartFilterPath<crate::types::ChatAdministratorRights> {
        self.and_then(|value| value.user_administrator_rights.as_ref())
    }

    #[must_use]
    pub fn bot_administrator_rights(
        self,
    ) -> SmartFilterPath<crate::types::ChatAdministratorRights> {
        self.and_then(|value| value.bot_administrator_rights.as_ref())
    }

    #[must_use]
    pub fn bot_is_member(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.bot_is_member.as_ref())
    }

    #[must_use]
    pub fn request_title(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_title.as_ref())
    }

    #[must_use]
    pub fn request_username(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_username.as_ref())
    }

    #[must_use]
    pub fn request_photo(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_photo.as_ref())
    }
}
impl SmartFilterPath<crate::types::KeyboardButtonRequestManagedBot> {
    #[must_use]
    pub fn request_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.request_id)
    }

    #[must_use]
    pub fn suggested_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.suggested_name.as_deref())
    }

    #[must_use]
    pub fn suggested_username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.suggested_username.as_deref())
    }
}
impl SmartFilterPath<crate::types::KeyboardButtonRequestUsers> {
    #[must_use]
    pub fn request_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.request_id)
    }

    #[must_use]
    pub fn user_is_bot(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.user_is_bot.as_ref())
    }

    #[must_use]
    pub fn user_is_premium(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.user_is_premium.as_ref())
    }

    #[must_use]
    pub fn max_quantity(self) -> SmartFilterPath<u8> {
        self.and_then(|value| value.max_quantity.as_ref())
    }

    #[must_use]
    pub fn request_name(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_name.as_ref())
    }

    #[must_use]
    pub fn request_username(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_username.as_ref())
    }

    #[must_use]
    pub fn request_photo(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_photo.as_ref())
    }
}
impl SmartFilterPath<crate::types::LabeledPrice> {
    #[must_use]
    pub fn label(self) -> SmartFilterPath<str> {
        self.map(|value| value.label.as_ref())
    }

    #[must_use]
    pub fn amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.amount)
    }
}
impl SmartFilterPath<crate::types::LinkPreviewOptions> {
    #[must_use]
    pub fn is_disabled(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_disabled.as_ref())
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.url.as_deref())
    }

    #[must_use]
    pub fn prefer_small_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.prefer_small_media.as_ref())
    }

    #[must_use]
    pub fn prefer_large_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.prefer_large_media.as_ref())
    }

    #[must_use]
    pub fn show_above_text(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_above_text.as_ref())
    }
}
impl SmartFilterPath<crate::types::Location> {
    #[must_use]
    pub fn latitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.latitude)
    }

    #[must_use]
    pub fn longitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.longitude)
    }

    #[must_use]
    pub fn horizontal_accuracy(self) -> SmartFilterPath<f64> {
        self.and_then(|value| value.horizontal_accuracy.as_ref())
    }

    #[must_use]
    pub fn live_period(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.live_period.as_ref())
    }

    #[must_use]
    pub fn heading(self) -> SmartFilterPath<u16> {
        self.and_then(|value| value.heading.as_ref())
    }

    #[must_use]
    pub fn proximity_alert_radius(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.proximity_alert_radius.as_ref())
    }
}
impl SmartFilterPath<crate::types::LocationAddress> {
    #[must_use]
    pub fn country_code(self) -> SmartFilterPath<str> {
        self.map(|value| value.country_code.as_ref())
    }

    #[must_use]
    pub fn state(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.state.as_deref())
    }

    #[must_use]
    pub fn city(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.city.as_deref())
    }

    #[must_use]
    pub fn street(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.street.as_deref())
    }
}
impl SmartFilterPath<crate::types::LoginUrl> {
    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.map(|value| value.url.as_ref())
    }

    #[must_use]
    pub fn forward_text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.forward_text.as_deref())
    }

    #[must_use]
    pub fn bot_username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.bot_username.as_deref())
    }

    #[must_use]
    pub fn request_write_access(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.request_write_access.as_ref())
    }
}
impl SmartFilterPath<crate::types::ManagedBotCreated> {
    #[must_use]
    pub fn bot(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.bot.as_ref())
    }
}
impl SmartFilterPath<crate::types::ManagedBotUpdated> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn bot(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.bot.as_ref())
    }
}
impl SmartFilterPath<crate::types::MaskPosition> {
    #[must_use]
    pub fn point(self) -> SmartFilterPath<str> {
        self.map(|value| value.point.as_ref())
    }

    #[must_use]
    pub fn x_shift(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.x_shift)
    }

    #[must_use]
    pub fn y_shift(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.y_shift)
    }

    #[must_use]
    pub fn scale(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.scale)
    }
}
impl SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::Animation> {
        self.and_then(|value| value.animation())
    }

    #[must_use]
    pub fn audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.and_then(|value| value.audio())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature())
    }

    #[must_use]
    pub fn boost_added(self) -> SmartFilterPath<crate::types::ChatBoostAdded> {
        self.and_then(|value| value.boost_added())
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat())
    }

    #[must_use]
    pub fn chat_background_set(self) -> SmartFilterPath<crate::types::ChatBackground> {
        self.and_then(|value| value.chat_background_set())
    }

    #[must_use]
    pub fn chat_owner_changed(self) -> SmartFilterPath<crate::types::ChatOwnerChanged> {
        self.and_then(|value| value.chat_owner_changed())
    }

    #[must_use]
    pub fn chat_owner_left(self) -> SmartFilterPath<crate::types::ChatOwnerLeft> {
        self.and_then(|value| value.chat_owner_left())
    }

    #[must_use]
    pub fn chat_shared(self) -> SmartFilterPath<crate::types::ChatShared> {
        self.and_then(|value| value.chat_shared())
    }

    #[must_use]
    pub fn checklist(self) -> SmartFilterPath<crate::types::Checklist> {
        self.and_then(|value| value.checklist())
    }

    #[must_use]
    pub fn checklist_tasks_added(self) -> SmartFilterPath<crate::types::ChecklistTasksAdded> {
        self.and_then(|value| value.checklist_tasks_added())
    }

    #[must_use]
    pub fn checklist_tasks_done(self) -> SmartFilterPath<crate::types::ChecklistTasksDone> {
        self.and_then(|value| value.checklist_tasks_done())
    }

    #[must_use]
    pub fn connected_website(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.connected_website())
    }

    #[must_use]
    pub fn contact(self) -> SmartFilterPath<crate::types::Contact> {
        self.and_then(|value| value.contact())
    }

    #[must_use]
    pub fn dice(self) -> SmartFilterPath<crate::types::Dice> {
        self.and_then(|value| value.dice())
    }

    #[must_use]
    pub fn direct_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::DirectMessagePriceChanged> {
        self.and_then(|value| value.direct_message_price_changed())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic())
    }

    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.and_then(|value| value.document())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply())
    }

    #[must_use]
    pub fn forum_topic_closed(self) -> SmartFilterPath<crate::types::ForumTopicClosed> {
        self.and_then(|value| value.forum_topic_closed())
    }

    #[must_use]
    pub fn forum_topic_created(self) -> SmartFilterPath<crate::types::ForumTopicCreated> {
        self.and_then(|value| value.forum_topic_created())
    }

    #[must_use]
    pub fn forum_topic_edited(self) -> SmartFilterPath<crate::types::ForumTopicEdited> {
        self.and_then(|value| value.forum_topic_edited())
    }

    #[must_use]
    pub fn forum_topic_reopened(self) -> SmartFilterPath<crate::types::ForumTopicReopened> {
        self.and_then(|value| value.forum_topic_reopened())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from())
    }

    #[must_use]
    pub fn game(self) -> SmartFilterPath<crate::types::Game> {
        self.and_then(|value| value.game())
    }

    #[must_use]
    pub fn general_forum_topic_hidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicHidden> {
        self.and_then(|value| value.general_forum_topic_hidden())
    }

    #[must_use]
    pub fn general_forum_topic_unhidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicUnhidden> {
        self.and_then(|value| value.general_forum_topic_unhidden())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.and_then(|value| value.gift())
    }

    #[must_use]
    pub fn gift_upgrade_sent(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.and_then(|value| value.gift_upgrade_sent())
    }

    #[must_use]
    pub fn giveaway(self) -> SmartFilterPath<crate::types::Giveaway> {
        self.and_then(|value| value.giveaway())
    }

    #[must_use]
    pub fn giveaway_completed(self) -> SmartFilterPath<crate::types::GiveawayCompleted> {
        self.and_then(|value| value.giveaway_completed())
    }

    #[must_use]
    pub fn giveaway_created(self) -> SmartFilterPath<crate::types::GiveawayCreated> {
        self.and_then(|value| value.giveaway_created())
    }

    #[must_use]
    pub fn giveaway_winners(self) -> SmartFilterPath<crate::types::GiveawayWinners> {
        self.and_then(|value| value.giveaway_winners())
    }

    #[must_use]
    pub fn invoice(self) -> SmartFilterPath<crate::types::Invoice> {
        self.and_then(|value| value.invoice())
    }

    #[must_use]
    pub fn left_chat_member(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.left_chat_member())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.and_then(|value| value.location())
    }

    #[must_use]
    pub fn managed_bot_created(self) -> SmartFilterPath<crate::types::ManagedBotCreated> {
        self.and_then(|value| value.managed_bot_created())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id())
    }

    #[must_use]
    pub fn message_auto_delete_timer_changed(
        self,
    ) -> SmartFilterPath<crate::types::MessageAutoDeleteTimerChanged> {
        self.and_then(|value| value.message_auto_delete_timer_changed())
    }

    #[must_use]
    pub fn new_chat_members(self) -> SmartFilterPath<[crate::types::User]> {
        self.and_then(|value| value.new_chat_members())
    }

    #[must_use]
    pub fn new_chat_photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.new_chat_photo())
    }

    #[must_use]
    pub fn new_chat_title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.new_chat_title())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media())
    }

    #[must_use]
    pub fn paid_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::PaidMessagePriceChanged> {
        self.and_then(|value| value.paid_message_price_changed())
    }

    #[must_use]
    pub fn passport_data(self) -> SmartFilterPath<crate::types::PassportData> {
        self.and_then(|value| value.passport_data())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.photo())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        self.and_then(|value| value.pinned_message())
    }

    #[must_use]
    pub fn poll(self) -> SmartFilterPath<crate::types::Poll> {
        self.and_then(|value| value.poll())
    }

    #[must_use]
    pub fn poll_option_added(self) -> SmartFilterPath<crate::types::PollOptionAdded> {
        self.and_then(|value| value.poll_option_added())
    }

    #[must_use]
    pub fn poll_option_deleted(self) -> SmartFilterPath<crate::types::PollOptionDeleted> {
        self.and_then(|value| value.poll_option_deleted())
    }

    #[must_use]
    pub fn proximity_alert_triggered(
        self,
    ) -> SmartFilterPath<crate::types::ProximityAlertTriggered> {
        self.and_then(|value| value.proximity_alert_triggered())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote())
    }

    #[must_use]
    pub fn refunded_payment(self) -> SmartFilterPath<crate::types::RefundedPayment> {
        self.and_then(|value| value.refunded_payment())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.and_then(|value| value.sticker())
    }

    #[must_use]
    pub fn story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.story())
    }

    #[must_use]
    pub fn successful_payment(self) -> SmartFilterPath<crate::types::SuccessfulPayment> {
        self.and_then(|value| value.successful_payment())
    }

    #[must_use]
    pub fn suggested_post_approval_failed(
        self,
    ) -> SmartFilterPath<crate::types::SuggestedPostApprovalFailed> {
        self.and_then(|value| value.suggested_post_approval_failed())
    }

    #[must_use]
    pub fn suggested_post_approved(self) -> SmartFilterPath<crate::types::SuggestedPostApproved> {
        self.and_then(|value| value.suggested_post_approved())
    }

    #[must_use]
    pub fn suggested_post_declined(self) -> SmartFilterPath<crate::types::SuggestedPostDeclined> {
        self.and_then(|value| value.suggested_post_declined())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info())
    }

    #[must_use]
    pub fn suggested_post_paid(self) -> SmartFilterPath<crate::types::SuggestedPostPaid> {
        self.and_then(|value| value.suggested_post_paid())
    }

    #[must_use]
    pub fn suggested_post_refunded(self) -> SmartFilterPath<crate::types::SuggestedPostRefunded> {
        self.and_then(|value| value.suggested_post_refunded())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text())
    }

    #[must_use]
    pub fn unique_gift(self) -> SmartFilterPath<crate::types::UniqueGiftInfo> {
        self.and_then(|value| value.unique_gift())
    }

    #[must_use]
    pub fn users_shared(self) -> SmartFilterPath<crate::types::UsersShared> {
        self.and_then(|value| value.users_shared())
    }

    #[must_use]
    pub fn venue(self) -> SmartFilterPath<crate::types::Venue> {
        self.and_then(|value| value.venue())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.and_then(|value| value.video())
    }

    #[must_use]
    pub fn video_chat_ended(self) -> SmartFilterPath<crate::types::VideoChatEnded> {
        self.and_then(|value| value.video_chat_ended())
    }

    #[must_use]
    pub fn video_chat_participants_invited(
        self,
    ) -> SmartFilterPath<crate::types::VideoChatParticipantsInvited> {
        self.and_then(|value| value.video_chat_participants_invited())
    }

    #[must_use]
    pub fn video_chat_scheduled(self) -> SmartFilterPath<crate::types::VideoChatScheduled> {
        self.and_then(|value| value.video_chat_scheduled())
    }

    #[must_use]
    pub fn video_chat_started(self) -> SmartFilterPath<crate::types::VideoChatStarted> {
        self.and_then(|value| value.video_chat_started())
    }

    #[must_use]
    pub fn video_note(self) -> SmartFilterPath<crate::types::VideoNote> {
        self.and_then(|value| value.video_note())
    }

    #[must_use]
    pub fn voice(self) -> SmartFilterPath<crate::types::Voice> {
        self.and_then(|value| value.voice())
    }

    #[must_use]
    pub fn web_app_data(self) -> SmartFilterPath<crate::types::WebAppData> {
        self.and_then(|value| value.web_app_data())
    }

    #[must_use]
    pub fn write_access_allowed(self) -> SmartFilterPath<crate::types::WriteAccessAllowed> {
        self.and_then(|value| value.write_access_allowed())
    }
}
impl SmartFilterPath<crate::types::MenuButton> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text())
    }

    #[must_use]
    pub fn web_app(self) -> SmartFilterPath<crate::types::WebAppInfo> {
        self.and_then(|value| value.web_app())
    }
}
impl SmartFilterPath<crate::types::MenuButtonCommands> {}
impl SmartFilterPath<crate::types::MenuButtonDefault> {}
impl SmartFilterPath<crate::types::MenuButtonWebApp> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn web_app(self) -> SmartFilterPath<crate::types::WebAppInfo> {
        self.map(|value| &value.web_app)
    }
}
impl SmartFilterPath<crate::types::Message> {
    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::Animation> {
        self.and_then(|value| value.animation())
    }

    #[must_use]
    pub fn audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.and_then(|value| value.audio())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature())
    }

    #[must_use]
    pub fn boost_added(self) -> SmartFilterPath<crate::types::ChatBoostAdded> {
        self.and_then(|value| value.boost_added())
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat())
    }

    #[must_use]
    pub fn chat_background_set(self) -> SmartFilterPath<crate::types::ChatBackground> {
        self.and_then(|value| value.chat_background_set())
    }

    #[must_use]
    pub fn chat_owner_changed(self) -> SmartFilterPath<crate::types::ChatOwnerChanged> {
        self.and_then(|value| value.chat_owner_changed())
    }

    #[must_use]
    pub fn chat_owner_left(self) -> SmartFilterPath<crate::types::ChatOwnerLeft> {
        self.and_then(|value| value.chat_owner_left())
    }

    #[must_use]
    pub fn chat_shared(self) -> SmartFilterPath<crate::types::ChatShared> {
        self.and_then(|value| value.chat_shared())
    }

    #[must_use]
    pub fn checklist(self) -> SmartFilterPath<crate::types::Checklist> {
        self.and_then(|value| value.checklist())
    }

    #[must_use]
    pub fn checklist_tasks_added(self) -> SmartFilterPath<crate::types::ChecklistTasksAdded> {
        self.and_then(|value| value.checklist_tasks_added())
    }

    #[must_use]
    pub fn checklist_tasks_done(self) -> SmartFilterPath<crate::types::ChecklistTasksDone> {
        self.and_then(|value| value.checklist_tasks_done())
    }

    #[must_use]
    pub fn connected_website(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.connected_website())
    }

    #[must_use]
    pub fn contact(self) -> SmartFilterPath<crate::types::Contact> {
        self.and_then(|value| value.contact())
    }

    #[must_use]
    pub fn dice(self) -> SmartFilterPath<crate::types::Dice> {
        self.and_then(|value| value.dice())
    }

    #[must_use]
    pub fn direct_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::DirectMessagePriceChanged> {
        self.and_then(|value| value.direct_message_price_changed())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic())
    }

    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.and_then(|value| value.document())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply())
    }

    #[must_use]
    pub fn forum_topic_closed(self) -> SmartFilterPath<crate::types::ForumTopicClosed> {
        self.and_then(|value| value.forum_topic_closed())
    }

    #[must_use]
    pub fn forum_topic_created(self) -> SmartFilterPath<crate::types::ForumTopicCreated> {
        self.and_then(|value| value.forum_topic_created())
    }

    #[must_use]
    pub fn forum_topic_edited(self) -> SmartFilterPath<crate::types::ForumTopicEdited> {
        self.and_then(|value| value.forum_topic_edited())
    }

    #[must_use]
    pub fn forum_topic_reopened(self) -> SmartFilterPath<crate::types::ForumTopicReopened> {
        self.and_then(|value| value.forum_topic_reopened())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from())
    }

    #[must_use]
    pub fn game(self) -> SmartFilterPath<crate::types::Game> {
        self.and_then(|value| value.game())
    }

    #[must_use]
    pub fn general_forum_topic_hidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicHidden> {
        self.and_then(|value| value.general_forum_topic_hidden())
    }

    #[must_use]
    pub fn general_forum_topic_unhidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicUnhidden> {
        self.and_then(|value| value.general_forum_topic_unhidden())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.and_then(|value| value.gift())
    }

    #[must_use]
    pub fn gift_upgrade_sent(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.and_then(|value| value.gift_upgrade_sent())
    }

    #[must_use]
    pub fn giveaway(self) -> SmartFilterPath<crate::types::Giveaway> {
        self.and_then(|value| value.giveaway())
    }

    #[must_use]
    pub fn giveaway_completed(self) -> SmartFilterPath<crate::types::GiveawayCompleted> {
        self.and_then(|value| value.giveaway_completed())
    }

    #[must_use]
    pub fn giveaway_created(self) -> SmartFilterPath<crate::types::GiveawayCreated> {
        self.and_then(|value| value.giveaway_created())
    }

    #[must_use]
    pub fn giveaway_winners(self) -> SmartFilterPath<crate::types::GiveawayWinners> {
        self.and_then(|value| value.giveaway_winners())
    }

    #[must_use]
    pub fn invoice(self) -> SmartFilterPath<crate::types::Invoice> {
        self.and_then(|value| value.invoice())
    }

    #[must_use]
    pub fn left_chat_member(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.left_chat_member())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.and_then(|value| value.location())
    }

    #[must_use]
    pub fn managed_bot_created(self) -> SmartFilterPath<crate::types::ManagedBotCreated> {
        self.and_then(|value| value.managed_bot_created())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id())
    }

    #[must_use]
    pub fn message_auto_delete_timer_changed(
        self,
    ) -> SmartFilterPath<crate::types::MessageAutoDeleteTimerChanged> {
        self.and_then(|value| value.message_auto_delete_timer_changed())
    }

    #[must_use]
    pub fn new_chat_members(self) -> SmartFilterPath<[crate::types::User]> {
        self.and_then(|value| value.new_chat_members())
    }

    #[must_use]
    pub fn new_chat_photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.new_chat_photo())
    }

    #[must_use]
    pub fn new_chat_title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.new_chat_title())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media())
    }

    #[must_use]
    pub fn paid_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::PaidMessagePriceChanged> {
        self.and_then(|value| value.paid_message_price_changed())
    }

    #[must_use]
    pub fn passport_data(self) -> SmartFilterPath<crate::types::PassportData> {
        self.and_then(|value| value.passport_data())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.photo())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        self.and_then(|value| value.pinned_message())
    }

    #[must_use]
    pub fn poll(self) -> SmartFilterPath<crate::types::Poll> {
        self.and_then(|value| value.poll())
    }

    #[must_use]
    pub fn poll_option_added(self) -> SmartFilterPath<crate::types::PollOptionAdded> {
        self.and_then(|value| value.poll_option_added())
    }

    #[must_use]
    pub fn poll_option_deleted(self) -> SmartFilterPath<crate::types::PollOptionDeleted> {
        self.and_then(|value| value.poll_option_deleted())
    }

    #[must_use]
    pub fn proximity_alert_triggered(
        self,
    ) -> SmartFilterPath<crate::types::ProximityAlertTriggered> {
        self.and_then(|value| value.proximity_alert_triggered())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote())
    }

    #[must_use]
    pub fn refunded_payment(self) -> SmartFilterPath<crate::types::RefundedPayment> {
        self.and_then(|value| value.refunded_payment())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.and_then(|value| value.sticker())
    }

    #[must_use]
    pub fn story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.story())
    }

    #[must_use]
    pub fn successful_payment(self) -> SmartFilterPath<crate::types::SuccessfulPayment> {
        self.and_then(|value| value.successful_payment())
    }

    #[must_use]
    pub fn suggested_post_approval_failed(
        self,
    ) -> SmartFilterPath<crate::types::SuggestedPostApprovalFailed> {
        self.and_then(|value| value.suggested_post_approval_failed())
    }

    #[must_use]
    pub fn suggested_post_approved(self) -> SmartFilterPath<crate::types::SuggestedPostApproved> {
        self.and_then(|value| value.suggested_post_approved())
    }

    #[must_use]
    pub fn suggested_post_declined(self) -> SmartFilterPath<crate::types::SuggestedPostDeclined> {
        self.and_then(|value| value.suggested_post_declined())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info())
    }

    #[must_use]
    pub fn suggested_post_paid(self) -> SmartFilterPath<crate::types::SuggestedPostPaid> {
        self.and_then(|value| value.suggested_post_paid())
    }

    #[must_use]
    pub fn suggested_post_refunded(self) -> SmartFilterPath<crate::types::SuggestedPostRefunded> {
        self.and_then(|value| value.suggested_post_refunded())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text())
    }

    #[must_use]
    pub fn unique_gift(self) -> SmartFilterPath<crate::types::UniqueGiftInfo> {
        self.and_then(|value| value.unique_gift())
    }

    #[must_use]
    pub fn users_shared(self) -> SmartFilterPath<crate::types::UsersShared> {
        self.and_then(|value| value.users_shared())
    }

    #[must_use]
    pub fn venue(self) -> SmartFilterPath<crate::types::Venue> {
        self.and_then(|value| value.venue())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.and_then(|value| value.video())
    }

    #[must_use]
    pub fn video_chat_ended(self) -> SmartFilterPath<crate::types::VideoChatEnded> {
        self.and_then(|value| value.video_chat_ended())
    }

    #[must_use]
    pub fn video_chat_participants_invited(
        self,
    ) -> SmartFilterPath<crate::types::VideoChatParticipantsInvited> {
        self.and_then(|value| value.video_chat_participants_invited())
    }

    #[must_use]
    pub fn video_chat_scheduled(self) -> SmartFilterPath<crate::types::VideoChatScheduled> {
        self.and_then(|value| value.video_chat_scheduled())
    }

    #[must_use]
    pub fn video_chat_started(self) -> SmartFilterPath<crate::types::VideoChatStarted> {
        self.and_then(|value| value.video_chat_started())
    }

    #[must_use]
    pub fn video_note(self) -> SmartFilterPath<crate::types::VideoNote> {
        self.and_then(|value| value.video_note())
    }

    #[must_use]
    pub fn voice(self) -> SmartFilterPath<crate::types::Voice> {
        self.and_then(|value| value.voice())
    }

    #[must_use]
    pub fn web_app_data(self) -> SmartFilterPath<crate::types::WebAppData> {
        self.and_then(|value| value.web_app_data())
    }

    #[must_use]
    pub fn write_access_allowed(self) -> SmartFilterPath<crate::types::WriteAccessAllowed> {
        self.and_then(|value| value.write_access_allowed())
    }
}
impl SmartFilterPath<crate::types::MessageAnimation> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::Animation> {
        self.map(|value| value.animation.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageAudio> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.map(|value| value.audio.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageAutoDeleteTimerChanged> {
    #[must_use]
    pub fn message_auto_delete_time(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_auto_delete_time)
    }
}
impl SmartFilterPath<crate::types::MessageBoostAdded> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn boost_added(self) -> SmartFilterPath<crate::types::ChatBoostAdded> {
        self.map(|value| &value.boost_added)
    }
}
impl SmartFilterPath<crate::types::MessageChannelChatCreated> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn channel_chat_created(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.channel_chat_created)
    }
}
impl SmartFilterPath<crate::types::MessageChatBackgroundSet> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn chat_background_set(self) -> SmartFilterPath<crate::types::ChatBackground> {
        self.map(|value| &value.chat_background_set)
    }
}
impl SmartFilterPath<crate::types::MessageChatOwnerChanged> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn chat_owner_changed(self) -> SmartFilterPath<crate::types::ChatOwnerChanged> {
        self.map(|value| &value.chat_owner_changed)
    }
}
impl SmartFilterPath<crate::types::MessageChatOwnerLeft> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn chat_owner_left(self) -> SmartFilterPath<crate::types::ChatOwnerLeft> {
        self.map(|value| &value.chat_owner_left)
    }
}
impl SmartFilterPath<crate::types::MessageChatShared> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn chat_shared(self) -> SmartFilterPath<crate::types::ChatShared> {
        self.map(|value| &value.chat_shared)
    }
}
impl SmartFilterPath<crate::types::MessageChecklist> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn checklist(self) -> SmartFilterPath<crate::types::Checklist> {
        self.map(|value| &value.checklist)
    }
}
impl SmartFilterPath<crate::types::MessageChecklistTasksAdded> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn checklist_tasks_added(self) -> SmartFilterPath<crate::types::ChecklistTasksAdded> {
        self.map(|value| &value.checklist_tasks_added)
    }
}
impl SmartFilterPath<crate::types::MessageChecklistTasksDone> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn checklist_tasks_done(self) -> SmartFilterPath<crate::types::ChecklistTasksDone> {
        self.map(|value| &value.checklist_tasks_done)
    }
}
impl SmartFilterPath<crate::types::MessageConnectedWebsite> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn connected_website(self) -> SmartFilterPath<str> {
        self.map(|value| value.connected_website.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageContact> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn contact(self) -> SmartFilterPath<crate::types::Contact> {
        self.map(|value| &value.contact)
    }
}
impl SmartFilterPath<crate::types::MessageDeleteChatPhoto> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn delete_chat_photo(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.delete_chat_photo)
    }
}
impl SmartFilterPath<crate::types::MessageDice> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn dice(self) -> SmartFilterPath<crate::types::Dice> {
        self.map(|value| &value.dice)
    }
}
impl SmartFilterPath<crate::types::MessageDirectMessagePriceChanged> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn direct_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::DirectMessagePriceChanged> {
        self.map(|value| &value.direct_message_price_changed)
    }
}
impl SmartFilterPath<crate::types::MessageDocument> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.map(|value| value.document.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageEntity> {
    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_id())
    }

    #[must_use]
    pub fn date_time_format(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.date_time_format())
    }

    #[must_use]
    pub fn language(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.language())
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.url())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user())
    }
}
impl SmartFilterPath<crate::types::MessageEntityBlockquote> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityBold> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityBotCommand> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityCashtag> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityCode> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityCustomEmoji> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }

    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.custom_emoji_id.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageEntityDateTime> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }

    #[must_use]
    pub fn unix_time(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.unix_time)
    }

    #[must_use]
    pub fn date_time_format(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.date_time_format.as_deref())
    }
}
impl SmartFilterPath<crate::types::MessageEntityEmail> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityExpandableBlockquote> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityHashtag> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityItalic> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityMention> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityPhoneNumber> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityPre> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }

    #[must_use]
    pub fn language(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.language.as_deref())
    }
}
impl SmartFilterPath<crate::types::MessageEntitySpoiler> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityStrikethrough> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityTextLink> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.map(|value| value.url.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageEntityTextMention> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageEntityUnderline> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageEntityUrl> {
    #[must_use]
    pub fn offset(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.offset)
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }
}
impl SmartFilterPath<crate::types::MessageForumTopicClosed> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn forum_topic_closed(self) -> SmartFilterPath<crate::types::ForumTopicClosed> {
        self.map(|value| &value.forum_topic_closed)
    }
}
impl SmartFilterPath<crate::types::MessageForumTopicCreated> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn forum_topic_created(self) -> SmartFilterPath<crate::types::ForumTopicCreated> {
        self.map(|value| &value.forum_topic_created)
    }
}
impl SmartFilterPath<crate::types::MessageForumTopicEdited> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn forum_topic_edited(self) -> SmartFilterPath<crate::types::ForumTopicEdited> {
        self.map(|value| &value.forum_topic_edited)
    }
}
impl SmartFilterPath<crate::types::MessageForumTopicReopened> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn forum_topic_reopened(self) -> SmartFilterPath<crate::types::ForumTopicReopened> {
        self.map(|value| &value.forum_topic_reopened)
    }
}
impl SmartFilterPath<crate::types::MessageGame> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn game(self) -> SmartFilterPath<crate::types::Game> {
        self.map(|value| &value.game)
    }
}
impl SmartFilterPath<crate::types::MessageGeneralForumTopicHidden> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn general_forum_topic_hidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicHidden> {
        self.map(|value| &value.general_forum_topic_hidden)
    }
}
impl SmartFilterPath<crate::types::MessageGeneralForumTopicUnhidden> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn general_forum_topic_unhidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicUnhidden> {
        self.map(|value| &value.general_forum_topic_unhidden)
    }
}
impl SmartFilterPath<crate::types::MessageGift> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.map(|value| &value.gift)
    }
}
impl SmartFilterPath<crate::types::MessageGiftUpgradeSent> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn gift_upgrade_sent(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.map(|value| &value.gift_upgrade_sent)
    }
}
impl SmartFilterPath<crate::types::MessageGiveaway> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn giveaway(self) -> SmartFilterPath<crate::types::Giveaway> {
        self.map(|value| &value.giveaway)
    }
}
impl SmartFilterPath<crate::types::MessageGiveawayCompleted> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn giveaway_completed(self) -> SmartFilterPath<crate::types::GiveawayCompleted> {
        self.map(|value| &value.giveaway_completed)
    }
}
impl SmartFilterPath<crate::types::MessageGiveawayCreated> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn giveaway_created(self) -> SmartFilterPath<crate::types::GiveawayCreated> {
        self.map(|value| &value.giveaway_created)
    }
}
impl SmartFilterPath<crate::types::MessageGiveawayWinners> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn giveaway_winners(self) -> SmartFilterPath<crate::types::GiveawayWinners> {
        self.map(|value| &value.giveaway_winners)
    }
}
impl SmartFilterPath<crate::types::MessageGroupChatCreated> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn group_chat_created(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.group_chat_created)
    }
}
impl SmartFilterPath<crate::types::MessageId> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }
}
impl SmartFilterPath<crate::types::MessageInvoice> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn invoice(self) -> SmartFilterPath<crate::types::Invoice> {
        self.map(|value| &value.invoice)
    }
}
impl SmartFilterPath<crate::types::MessageLeftChatMember> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn left_chat_member(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.left_chat_member.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageLocation> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.map(|value| &value.location)
    }
}
impl SmartFilterPath<crate::types::MessageManagedBotCreated> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn managed_bot_created(self) -> SmartFilterPath<crate::types::ManagedBotCreated> {
        self.map(|value| &value.managed_bot_created)
    }
}
impl SmartFilterPath<crate::types::MessageMessageAutoDeleteTimerChanged> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn message_auto_delete_timer_changed(
        self,
    ) -> SmartFilterPath<crate::types::MessageAutoDeleteTimerChanged> {
        self.map(|value| &value.message_auto_delete_timer_changed)
    }
}
impl SmartFilterPath<crate::types::MessageMigrateFromChatId> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn migrate_from_chat_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.migrate_from_chat_id)
    }
}
impl SmartFilterPath<crate::types::MessageMigrateToChatId> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn migrate_to_chat_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.migrate_to_chat_id)
    }
}
impl SmartFilterPath<crate::types::MessageNewChatMembers> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn new_chat_members(self) -> SmartFilterPath<[crate::types::User]> {
        self.map(|value| value.new_chat_members.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageNewChatPhoto> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn new_chat_photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.map(|value| value.new_chat_photo.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageNewChatTitle> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn new_chat_title(self) -> SmartFilterPath<str> {
        self.map(|value| value.new_chat_title.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageOrigin> {
    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat())
    }

    #[must_use]
    pub fn sender_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_user())
    }

    #[must_use]
    pub fn sender_user_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_user_name())
    }
}
impl SmartFilterPath<crate::types::MessageOriginChannel> {
    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }
}
impl SmartFilterPath<crate::types::MessageOriginChat> {
    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.sender_chat.as_ref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }
}
impl SmartFilterPath<crate::types::MessageOriginHiddenUser> {
    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn sender_user_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.sender_user_name.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageOriginUser> {
    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn sender_user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.sender_user.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessagePaidMedia> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.map(|value| &value.paid_media)
    }
}
impl SmartFilterPath<crate::types::MessagePaidMessagePriceChanged> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn paid_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::PaidMessagePriceChanged> {
        self.map(|value| &value.paid_message_price_changed)
    }
}
impl SmartFilterPath<crate::types::MessagePassportData> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn passport_data(self) -> SmartFilterPath<crate::types::PassportData> {
        self.map(|value| &value.passport_data)
    }
}
impl SmartFilterPath<crate::types::MessagePhoto> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.map(|value| value.photo.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessagePinnedMessage> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        self.map(|value| value.pinned_message.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessagePoll> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn poll(self) -> SmartFilterPath<crate::types::Poll> {
        self.map(|value| value.poll.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessagePollOptionAdded> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn poll_option_added(self) -> SmartFilterPath<crate::types::PollOptionAdded> {
        self.map(|value| &value.poll_option_added)
    }
}
impl SmartFilterPath<crate::types::MessagePollOptionDeleted> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn poll_option_deleted(self) -> SmartFilterPath<crate::types::PollOptionDeleted> {
        self.map(|value| &value.poll_option_deleted)
    }
}
impl SmartFilterPath<crate::types::MessageProximityAlertTriggered> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn proximity_alert_triggered(
        self,
    ) -> SmartFilterPath<crate::types::ProximityAlertTriggered> {
        self.map(|value| &value.proximity_alert_triggered)
    }
}
impl SmartFilterPath<crate::types::MessageReactionCountUpdated> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn reactions(self) -> SmartFilterPath<[crate::types::ReactionCount]> {
        self.map(|value| value.reactions.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageReactionUpdated> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user.as_deref())
    }

    #[must_use]
    pub fn actor_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.actor_chat.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn old_reaction(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.map(|value| value.old_reaction.as_ref())
    }

    #[must_use]
    pub fn new_reaction(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.map(|value| value.new_reaction.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageRefundedPayment> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn refunded_payment(self) -> SmartFilterPath<crate::types::RefundedPayment> {
        self.map(|value| &value.refunded_payment)
    }
}
impl SmartFilterPath<crate::types::MessageSticker> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.map(|value| value.sticker.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageStory> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn story(self) -> SmartFilterPath<crate::types::Story> {
        self.map(|value| &value.story)
    }
}
impl SmartFilterPath<crate::types::MessageSuccessfulPayment> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn successful_payment(self) -> SmartFilterPath<crate::types::SuccessfulPayment> {
        self.map(|value| value.successful_payment.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageSuggestedPostApprovalFailed> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn suggested_post_approval_failed(
        self,
    ) -> SmartFilterPath<crate::types::SuggestedPostApprovalFailed> {
        self.map(|value| &value.suggested_post_approval_failed)
    }
}
impl SmartFilterPath<crate::types::MessageSuggestedPostApproved> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn suggested_post_approved(self) -> SmartFilterPath<crate::types::SuggestedPostApproved> {
        self.map(|value| &value.suggested_post_approved)
    }
}
impl SmartFilterPath<crate::types::MessageSuggestedPostDeclined> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn suggested_post_declined(self) -> SmartFilterPath<crate::types::SuggestedPostDeclined> {
        self.map(|value| &value.suggested_post_declined)
    }
}
impl SmartFilterPath<crate::types::MessageSuggestedPostPaid> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn suggested_post_paid(self) -> SmartFilterPath<crate::types::SuggestedPostPaid> {
        self.map(|value| &value.suggested_post_paid)
    }
}
impl SmartFilterPath<crate::types::MessageSuggestedPostRefunded> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn suggested_post_refunded(self) -> SmartFilterPath<crate::types::SuggestedPostRefunded> {
        self.map(|value| &value.suggested_post_refunded)
    }
}
impl SmartFilterPath<crate::types::MessageSupergroupChatCreated> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn supergroup_chat_created(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.supergroup_chat_created)
    }
}
impl SmartFilterPath<crate::types::MessageText> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageUniqueGift> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn unique_gift(self) -> SmartFilterPath<crate::types::UniqueGiftInfo> {
        self.map(|value| &value.unique_gift)
    }
}
impl SmartFilterPath<crate::types::MessageUsersShared> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn users_shared(self) -> SmartFilterPath<crate::types::UsersShared> {
        self.map(|value| &value.users_shared)
    }
}
impl SmartFilterPath<crate::types::MessageVenue> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn venue(self) -> SmartFilterPath<crate::types::Venue> {
        self.map(|value| value.venue.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageVideo> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.map(|value| value.video.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageVideoChatEnded> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn video_chat_ended(self) -> SmartFilterPath<crate::types::VideoChatEnded> {
        self.map(|value| &value.video_chat_ended)
    }
}
impl SmartFilterPath<crate::types::MessageVideoChatParticipantsInvited> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn video_chat_participants_invited(
        self,
    ) -> SmartFilterPath<crate::types::VideoChatParticipantsInvited> {
        self.map(|value| &value.video_chat_participants_invited)
    }
}
impl SmartFilterPath<crate::types::MessageVideoChatScheduled> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn video_chat_scheduled(self) -> SmartFilterPath<crate::types::VideoChatScheduled> {
        self.map(|value| &value.video_chat_scheduled)
    }
}
impl SmartFilterPath<crate::types::MessageVideoChatStarted> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn video_chat_started(self) -> SmartFilterPath<crate::types::VideoChatStarted> {
        self.map(|value| &value.video_chat_started)
    }
}
impl SmartFilterPath<crate::types::MessageVideoNote> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn video_note(self) -> SmartFilterPath<crate::types::VideoNote> {
        self.map(|value| value.video_note.as_ref())
    }
}
impl SmartFilterPath<crate::types::MessageVoice> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn voice(self) -> SmartFilterPath<crate::types::Voice> {
        self.map(|value| &value.voice)
    }
}
impl SmartFilterPath<crate::types::MessageWebAppData> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn web_app_data(self) -> SmartFilterPath<crate::types::WebAppData> {
        self.map(|value| &value.web_app_data)
    }
}
impl SmartFilterPath<crate::types::MessageWriteAccessAllowed> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn message_thread_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.message_thread_id.as_ref())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from.as_deref())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat.as_deref())
    }

    #[must_use]
    pub fn sender_boost_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.sender_boost_count.as_ref())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot.as_deref())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag.as_deref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id.as_deref())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin.as_ref())
    }

    #[must_use]
    pub fn is_topic_message(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_topic_message.as_ref())
    }

    #[must_use]
    pub fn is_automatic_forward(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_automatic_forward.as_ref())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message.as_deref())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply.as_deref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote.as_ref())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story.as_ref())
    }

    #[must_use]
    pub fn reply_to_checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.reply_to_checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id.as_deref())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot.as_deref())
    }

    #[must_use]
    pub fn edit_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.edit_date.as_ref())
    }

    #[must_use]
    pub fn has_protected_content(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_protected_content.as_ref())
    }

    #[must_use]
    pub fn is_from_offline(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_offline.as_ref())
    }

    #[must_use]
    pub fn is_paid_post(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_paid_post.as_ref())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id.as_deref())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature.as_deref())
    }

    #[must_use]
    pub fn paid_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.paid_star_count.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options.as_ref())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info.as_ref())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id.as_deref())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption.as_deref())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities.as_deref())
    }

    #[must_use]
    pub fn show_caption_above_media(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.show_caption_above_media.as_ref())
    }

    #[must_use]
    pub fn has_media_spoiler(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_media_spoiler.as_ref())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup.as_ref())
    }

    #[must_use]
    pub fn write_access_allowed(self) -> SmartFilterPath<crate::types::WriteAccessAllowed> {
        self.map(|value| &value.write_access_allowed)
    }
}
impl SmartFilterPath<crate::types::OrderInfo> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.name.as_deref())
    }

    #[must_use]
    pub fn phone_number(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.phone_number.as_deref())
    }

    #[must_use]
    pub fn email(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.email.as_deref())
    }

    #[must_use]
    pub fn shipping_address(self) -> SmartFilterPath<crate::types::ShippingAddress> {
        self.and_then(|value| value.shipping_address.as_ref())
    }
}
impl SmartFilterPath<crate::types::OwnedGift> {
    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities())
    }

    #[must_use]
    pub fn owned_gift_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.owned_gift_id())
    }

    #[must_use]
    pub fn sender_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_user())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text())
    }
}
impl SmartFilterPath<crate::types::OwnedGiftRegular> {
    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::Gift> {
        self.map(|value| value.gift.as_ref())
    }

    #[must_use]
    pub fn owned_gift_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.owned_gift_id.as_deref())
    }

    #[must_use]
    pub fn sender_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_user.as_deref())
    }

    #[must_use]
    pub fn send_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.send_date)
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text.as_deref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn is_private(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_private.as_ref())
    }

    #[must_use]
    pub fn is_saved(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_saved.as_ref())
    }

    #[must_use]
    pub fn can_be_upgraded(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_be_upgraded.as_ref())
    }

    #[must_use]
    pub fn was_refunded(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.was_refunded.as_ref())
    }

    #[must_use]
    pub fn convert_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.convert_star_count.as_ref())
    }

    #[must_use]
    pub fn prepaid_upgrade_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.prepaid_upgrade_star_count.as_ref())
    }

    #[must_use]
    pub fn is_upgrade_separate(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_upgrade_separate.as_ref())
    }

    #[must_use]
    pub fn unique_gift_number(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.unique_gift_number.as_ref())
    }
}
impl SmartFilterPath<crate::types::OwnedGiftUnique> {
    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::UniqueGift> {
        self.map(|value| value.gift.as_ref())
    }

    #[must_use]
    pub fn owned_gift_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.owned_gift_id.as_deref())
    }

    #[must_use]
    pub fn sender_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_user.as_deref())
    }

    #[must_use]
    pub fn send_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.send_date)
    }

    #[must_use]
    pub fn is_saved(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_saved.as_ref())
    }

    #[must_use]
    pub fn can_be_transferred(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_be_transferred.as_ref())
    }

    #[must_use]
    pub fn transfer_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.transfer_star_count.as_ref())
    }

    #[must_use]
    pub fn next_transfer_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.next_transfer_date.as_ref())
    }
}
impl SmartFilterPath<crate::types::OwnedGifts> {
    #[must_use]
    pub fn total_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_count)
    }

    #[must_use]
    pub fn gifts(self) -> SmartFilterPath<[crate::types::OwnedGift]> {
        self.map(|value| value.gifts.as_ref())
    }

    #[must_use]
    pub fn next_offset(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.next_offset.as_deref())
    }
}
impl SmartFilterPath<crate::types::PaidMedia> {
    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.photo())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.and_then(|value| value.video())
    }
}
impl SmartFilterPath<crate::types::PaidMediaInfo> {
    #[must_use]
    pub fn star_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.star_count)
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<[crate::types::PaidMedia]> {
        self.map(|value| value.paid_media.as_ref())
    }
}
impl SmartFilterPath<crate::types::PaidMediaPhoto> {
    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.map(|value| value.photo.as_ref())
    }
}
impl SmartFilterPath<crate::types::PaidMediaPreview> {
    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.width.as_ref())
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.height.as_ref())
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.duration.as_ref())
    }
}
impl SmartFilterPath<crate::types::PaidMediaPurchased> {
    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn paid_media_payload(self) -> SmartFilterPath<str> {
        self.map(|value| value.paid_media_payload.as_ref())
    }
}
impl SmartFilterPath<crate::types::PaidMediaVideo> {
    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.map(|value| value.video.as_ref())
    }
}
impl SmartFilterPath<crate::types::PaidMessagePriceChanged> {
    #[must_use]
    pub fn paid_message_star_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.paid_message_star_count)
    }
}
impl SmartFilterPath<crate::types::PassportData> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<[crate::types::EncryptedPassportElement]> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn credentials(self) -> SmartFilterPath<crate::types::EncryptedCredentials> {
        self.map(|value| &value.credentials)
    }
}
impl SmartFilterPath<crate::types::PassportElementError> {
    #[must_use]
    pub fn data_hash(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.data_hash())
    }

    #[must_use]
    pub fn element_hash(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.element_hash())
    }

    #[must_use]
    pub fn field_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.field_name())
    }

    #[must_use]
    pub fn file_hash(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.file_hash())
    }

    #[must_use]
    pub fn file_hashes(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.file_hashes())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message())
    }

    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorDataField> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn field_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.field_name.as_ref())
    }

    #[must_use]
    pub fn data_hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.data_hash.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorFile> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn file_hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_hash.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorFiles> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn file_hashes(self) -> SmartFilterPath<[Box<str>]> {
        self.map(|value| value.file_hashes.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorFrontSide> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn file_hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_hash.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorReverseSide> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn file_hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_hash.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorSelfie> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn file_hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_hash.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorTranslationFile> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn file_hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_hash.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorTranslationFiles> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn file_hashes(self) -> SmartFilterPath<[Box<str>]> {
        self.map(|value| value.file_hashes.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportElementErrorUnspecified> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<str> {
        self.map(|value| value.r#type.as_ref())
    }

    #[must_use]
    pub fn element_hash(self) -> SmartFilterPath<str> {
        self.map(|value| value.element_hash.as_ref())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<str> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::PassportFile> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.file_size)
    }

    #[must_use]
    pub fn file_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.file_date)
    }
}
impl SmartFilterPath<crate::types::PhotoSize> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.width)
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.height)
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::Poll> {
    #[must_use]
    pub fn correct_option_ids(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.correct_option_ids())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description())
    }

    #[must_use]
    pub fn description_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.description_entities())
    }

    #[must_use]
    pub fn explanation(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.explanation())
    }

    #[must_use]
    pub fn explanation_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.explanation_entities())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn options(self) -> SmartFilterPath<[crate::types::PollOption]> {
        self.map(|value| value.options())
    }

    #[must_use]
    pub fn question(self) -> SmartFilterPath<str> {
        self.map(|value| value.question())
    }

    #[must_use]
    pub fn question_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.question_entities())
    }
}
impl SmartFilterPath<crate::types::PollAnswer> {
    #[must_use]
    pub fn poll_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.poll_id.as_ref())
    }

    #[must_use]
    pub fn voter_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.voter_chat.as_deref())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user.as_deref())
    }

    #[must_use]
    pub fn option_ids(self) -> SmartFilterPath<[i64]> {
        self.map(|value| value.option_ids.as_ref())
    }

    #[must_use]
    pub fn option_persistent_ids(self) -> SmartFilterPath<[Box<str>]> {
        self.map(|value| value.option_persistent_ids.as_ref())
    }
}
impl SmartFilterPath<crate::types::PollOption> {
    #[must_use]
    pub fn persistent_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.persistent_id.as_ref())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn text_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.text_entities.as_deref())
    }

    #[must_use]
    pub fn voter_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.voter_count)
    }

    #[must_use]
    pub fn added_by_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.added_by_user.as_deref())
    }

    #[must_use]
    pub fn added_by_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.added_by_chat.as_deref())
    }

    #[must_use]
    pub fn addition_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.addition_date.as_ref())
    }
}
impl SmartFilterPath<crate::types::PollOptionAdded> {
    #[must_use]
    pub fn poll_message(self) -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        self.and_then(|value| value.poll_message.as_deref())
    }

    #[must_use]
    pub fn option_persistent_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.option_persistent_id.as_ref())
    }

    #[must_use]
    pub fn option_text(self) -> SmartFilterPath<str> {
        self.map(|value| value.option_text.as_ref())
    }

    #[must_use]
    pub fn option_text_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.option_text_entities.as_deref())
    }
}
impl SmartFilterPath<crate::types::PollOptionDeleted> {
    #[must_use]
    pub fn poll_message(self) -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        self.and_then(|value| value.poll_message.as_deref())
    }

    #[must_use]
    pub fn option_persistent_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.option_persistent_id.as_ref())
    }

    #[must_use]
    pub fn option_text(self) -> SmartFilterPath<str> {
        self.map(|value| value.option_text.as_ref())
    }

    #[must_use]
    pub fn option_text_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.option_text_entities.as_deref())
    }
}
impl SmartFilterPath<crate::types::PollQuiz> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn question(self) -> SmartFilterPath<str> {
        self.map(|value| value.question.as_ref())
    }

    #[must_use]
    pub fn question_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.question_entities.as_deref())
    }

    #[must_use]
    pub fn options(self) -> SmartFilterPath<[crate::types::PollOption]> {
        self.map(|value| value.options.as_ref())
    }

    #[must_use]
    pub fn total_voter_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_voter_count)
    }

    #[must_use]
    pub fn is_closed(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_closed)
    }

    #[must_use]
    pub fn is_anonymous(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_anonymous)
    }

    #[must_use]
    pub fn allows_multiple_answers(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.allows_multiple_answers)
    }

    #[must_use]
    pub fn allows_revoting(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.allows_revoting)
    }

    #[must_use]
    pub fn correct_option_ids(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.correct_option_ids.as_deref())
    }

    #[must_use]
    pub fn explanation(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.explanation.as_deref())
    }

    #[must_use]
    pub fn explanation_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.explanation_entities.as_deref())
    }

    #[must_use]
    pub fn open_period(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.open_period.as_ref())
    }

    #[must_use]
    pub fn close_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.close_date.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn description_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.description_entities.as_deref())
    }
}
impl SmartFilterPath<crate::types::PollRegular> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn question(self) -> SmartFilterPath<str> {
        self.map(|value| value.question.as_ref())
    }

    #[must_use]
    pub fn question_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.question_entities.as_deref())
    }

    #[must_use]
    pub fn options(self) -> SmartFilterPath<[crate::types::PollOption]> {
        self.map(|value| value.options.as_ref())
    }

    #[must_use]
    pub fn total_voter_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_voter_count)
    }

    #[must_use]
    pub fn is_closed(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_closed)
    }

    #[must_use]
    pub fn is_anonymous(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_anonymous)
    }

    #[must_use]
    pub fn allows_multiple_answers(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.allows_multiple_answers)
    }

    #[must_use]
    pub fn allows_revoting(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.allows_revoting)
    }

    #[must_use]
    pub fn explanation_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.explanation_entities.as_deref())
    }

    #[must_use]
    pub fn open_period(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.open_period.as_ref())
    }

    #[must_use]
    pub fn close_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.close_date.as_ref())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description.as_deref())
    }

    #[must_use]
    pub fn description_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.description_entities.as_deref())
    }
}
impl SmartFilterPath<crate::types::PreCheckoutQuery> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.map(|value| value.currency.as_ref())
    }

    #[must_use]
    pub fn total_amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_amount)
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.map(|value| value.invoice_payload.as_ref())
    }

    #[must_use]
    pub fn shipping_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.shipping_option_id.as_deref())
    }

    #[must_use]
    pub fn order_info(self) -> SmartFilterPath<crate::types::OrderInfo> {
        self.and_then(|value| value.order_info.as_ref())
    }
}
impl SmartFilterPath<crate::types::PreparedInlineMessage> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn expiration_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.expiration_date)
    }
}
impl SmartFilterPath<crate::types::PreparedKeyboardButton> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }
}
impl SmartFilterPath<crate::types::ProximityAlertTriggered> {
    #[must_use]
    pub fn traveler(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.traveler.as_ref())
    }

    #[must_use]
    pub fn watcher(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.watcher.as_ref())
    }

    #[must_use]
    pub fn distance(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.distance)
    }
}
impl SmartFilterPath<crate::types::ReactionCount> {
    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<crate::types::ReactionType> {
        self.map(|value| &value.r#type)
    }

    #[must_use]
    pub fn total_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_count)
    }
}
impl SmartFilterPath<crate::types::ReactionType> {
    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_id())
    }

    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji())
    }
}
impl SmartFilterPath<crate::types::ReactionTypeCustomEmoji> {
    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.custom_emoji_id.as_ref())
    }
}
impl SmartFilterPath<crate::types::ReactionTypeEmoji> {
    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.map(|value| value.emoji.as_ref())
    }
}
impl SmartFilterPath<crate::types::ReactionTypePaid> {}
impl SmartFilterPath<crate::types::RefundedPayment> {
    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.map(|value| value.currency.as_ref())
    }

    #[must_use]
    pub fn total_amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_amount)
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.map(|value| value.invoice_payload.as_ref())
    }

    #[must_use]
    pub fn telegram_payment_charge_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.telegram_payment_charge_id.as_ref())
    }

    #[must_use]
    pub fn provider_payment_charge_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.provider_payment_charge_id.as_deref())
    }
}
impl SmartFilterPath<crate::types::ReplyKeyboardMarkup> {
    #[must_use]
    pub fn keyboard(self) -> SmartFilterPath<[Box<[crate::types::KeyboardButton]>]> {
        self.map(|value| value.keyboard.as_ref())
    }

    #[must_use]
    pub fn is_persistent(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_persistent.as_ref())
    }

    #[must_use]
    pub fn resize_keyboard(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.resize_keyboard.as_ref())
    }

    #[must_use]
    pub fn one_time_keyboard(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.one_time_keyboard.as_ref())
    }

    #[must_use]
    pub fn input_field_placeholder(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.input_field_placeholder.as_deref())
    }

    #[must_use]
    pub fn selective(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.selective.as_ref())
    }
}
impl SmartFilterPath<crate::types::ReplyKeyboardRemove> {
    #[must_use]
    pub fn remove_keyboard(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.remove_keyboard)
    }

    #[must_use]
    pub fn selective(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.selective.as_ref())
    }
}
impl SmartFilterPath<crate::types::ReplyMarkup> {
    #[must_use]
    pub fn inline_keyboard(self) -> SmartFilterPath<[Box<[crate::types::InlineKeyboardButton]>]> {
        self.and_then(|value| value.inline_keyboard())
    }

    #[must_use]
    pub fn input_field_placeholder(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.input_field_placeholder())
    }

    #[must_use]
    pub fn keyboard(self) -> SmartFilterPath<[Box<[crate::types::KeyboardButton]>]> {
        self.and_then(|value| value.keyboard())
    }
}
impl SmartFilterPath<crate::types::ReplyParameters> {
    #[must_use]
    pub fn message_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.message_id)
    }

    #[must_use]
    pub fn chat_id(self) -> SmartFilterPath<crate::types::ChatIdKind> {
        self.and_then(|value| value.chat_id.as_ref())
    }

    #[must_use]
    pub fn allow_sending_without_reply(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.allow_sending_without_reply.as_ref())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.quote.as_deref())
    }

    #[must_use]
    pub fn quote_parse_mode(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.quote_parse_mode.as_deref())
    }

    #[must_use]
    pub fn quote_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.quote_entities.as_deref())
    }

    #[must_use]
    pub fn quote_position(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.quote_position.as_ref())
    }

    #[must_use]
    pub fn checklist_task_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.checklist_task_id.as_ref())
    }

    #[must_use]
    pub fn poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.poll_option_id.as_deref())
    }
}
impl SmartFilterPath<crate::types::ResponseParameters> {
    #[must_use]
    pub fn migrate_to_chat_id(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.migrate_to_chat_id.as_ref())
    }

    #[must_use]
    pub fn retry_after(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.retry_after.as_ref())
    }
}
impl SmartFilterPath<crate::types::RevenueWithdrawalState> {
    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.url())
    }
}
impl SmartFilterPath<crate::types::RevenueWithdrawalStateFailed> {}
impl SmartFilterPath<crate::types::RevenueWithdrawalStatePending> {}
impl SmartFilterPath<crate::types::RevenueWithdrawalStateSucceeded> {
    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.map(|value| value.url.as_ref())
    }
}
impl SmartFilterPath<crate::types::SentWebAppMessage> {
    #[must_use]
    pub fn inline_message_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.inline_message_id.as_deref())
    }
}
impl SmartFilterPath<crate::types::SharedUser> {
    #[must_use]
    pub fn user_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.user_id)
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.first_name.as_deref())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name.as_deref())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.photo.as_deref())
    }
}
impl SmartFilterPath<crate::types::ShippingAddress> {
    #[must_use]
    pub fn country_code(self) -> SmartFilterPath<str> {
        self.map(|value| value.country_code.as_ref())
    }

    #[must_use]
    pub fn state(self) -> SmartFilterPath<str> {
        self.map(|value| value.state.as_ref())
    }

    #[must_use]
    pub fn city(self) -> SmartFilterPath<str> {
        self.map(|value| value.city.as_ref())
    }

    #[must_use]
    pub fn street_line1(self) -> SmartFilterPath<str> {
        self.map(|value| value.street_line1.as_ref())
    }

    #[must_use]
    pub fn street_line2(self) -> SmartFilterPath<str> {
        self.map(|value| value.street_line2.as_ref())
    }

    #[must_use]
    pub fn post_code(self) -> SmartFilterPath<str> {
        self.map(|value| value.post_code.as_ref())
    }
}
impl SmartFilterPath<crate::types::ShippingOption> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn prices(self) -> SmartFilterPath<[crate::types::LabeledPrice]> {
        self.map(|value| value.prices.as_ref())
    }
}
impl SmartFilterPath<crate::types::ShippingQuery> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.from.as_ref())
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.map(|value| value.invoice_payload.as_ref())
    }

    #[must_use]
    pub fn shipping_address(self) -> SmartFilterPath<crate::types::ShippingAddress> {
        self.map(|value| &value.shipping_address)
    }
}
impl SmartFilterPath<crate::types::StarAmount> {
    #[must_use]
    pub fn amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.amount)
    }

    #[must_use]
    pub fn nanostar_amount(self) -> SmartFilterPath<i32> {
        self.and_then(|value| value.nanostar_amount.as_ref())
    }
}
impl SmartFilterPath<crate::types::StarTransaction> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id())
    }

    #[must_use]
    pub fn receiver(self) -> SmartFilterPath<crate::types::TransactionPartner> {
        self.and_then(|value| value.receiver())
    }

    #[must_use]
    pub fn source(self) -> SmartFilterPath<crate::types::TransactionPartner> {
        self.and_then(|value| value.source())
    }
}
impl SmartFilterPath<crate::types::StarTransactionIncoming> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.amount)
    }

    #[must_use]
    pub fn nanostar_amount(self) -> SmartFilterPath<u32> {
        self.and_then(|value| value.nanostar_amount.as_ref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn source(self) -> SmartFilterPath<crate::types::TransactionPartner> {
        self.map(|value| &value.source)
    }
}
impl SmartFilterPath<crate::types::StarTransactionOutgoing> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.map(|value| value.id.as_ref())
    }

    #[must_use]
    pub fn amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.amount)
    }

    #[must_use]
    pub fn nanostar_amount(self) -> SmartFilterPath<u32> {
        self.and_then(|value| value.nanostar_amount.as_ref())
    }

    #[must_use]
    pub fn date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.date)
    }

    #[must_use]
    pub fn receiver(self) -> SmartFilterPath<crate::types::TransactionPartner> {
        self.map(|value| &value.receiver)
    }
}
impl SmartFilterPath<crate::types::StarTransactions> {
    #[must_use]
    pub fn transactions(self) -> SmartFilterPath<[crate::types::StarTransaction]> {
        self.map(|value| value.transactions.as_ref())
    }
}
impl SmartFilterPath<crate::types::Sticker> {
    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_id())
    }

    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji())
    }

    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id())
    }

    #[must_use]
    pub fn mask_position(self) -> SmartFilterPath<crate::types::MaskPosition> {
        self.and_then(|value| value.mask_position())
    }

    #[must_use]
    pub fn premium_animation(self) -> SmartFilterPath<crate::types::File> {
        self.and_then(|value| value.premium_animation())
    }

    #[must_use]
    pub fn set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.set_name())
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail())
    }
}
impl SmartFilterPath<crate::types::StickerCustomEmoji> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.width)
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.height)
    }

    #[must_use]
    pub fn is_animated(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_animated)
    }

    #[must_use]
    pub fn is_video(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_video)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji.as_deref())
    }

    #[must_use]
    pub fn set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.set_name.as_deref())
    }

    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn needs_repainting(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.needs_repainting.as_ref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::StickerMask> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.width)
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.height)
    }

    #[must_use]
    pub fn is_animated(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_animated)
    }

    #[must_use]
    pub fn is_video(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_video)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji.as_deref())
    }

    #[must_use]
    pub fn set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.set_name.as_deref())
    }

    #[must_use]
    pub fn mask_position(self) -> SmartFilterPath<crate::types::MaskPosition> {
        self.and_then(|value| value.mask_position.as_ref())
    }

    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn needs_repainting(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.needs_repainting.as_ref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::StickerRegular> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.width)
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.height)
    }

    #[must_use]
    pub fn is_animated(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_animated)
    }

    #[must_use]
    pub fn is_video(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_video)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji.as_deref())
    }

    #[must_use]
    pub fn set_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.set_name.as_deref())
    }

    #[must_use]
    pub fn premium_animation(self) -> SmartFilterPath<crate::types::File> {
        self.and_then(|value| value.premium_animation.as_ref())
    }

    #[must_use]
    pub fn custom_emoji_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.custom_emoji_id.as_deref())
    }

    #[must_use]
    pub fn needs_repainting(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.needs_repainting.as_ref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::StickerSet> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn sticker_type(self) -> SmartFilterPath<str> {
        self.map(|value| value.sticker_type.as_ref())
    }

    #[must_use]
    pub fn stickers(self) -> SmartFilterPath<[crate::types::Sticker]> {
        self.map(|value| value.stickers.as_ref())
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }
}
impl SmartFilterPath<crate::types::Story> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }
}
impl SmartFilterPath<crate::types::StoryArea> {
    #[must_use]
    pub fn position(self) -> SmartFilterPath<crate::types::StoryAreaPosition> {
        self.map(|value| &value.position)
    }

    #[must_use]
    pub fn r#type(self) -> SmartFilterPath<crate::types::StoryAreaType> {
        self.map(|value| &value.r#type)
    }
}
impl SmartFilterPath<crate::types::StoryAreaPosition> {
    #[must_use]
    pub fn x_percentage(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.x_percentage)
    }

    #[must_use]
    pub fn y_percentage(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.y_percentage)
    }

    #[must_use]
    pub fn width_percentage(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.width_percentage)
    }

    #[must_use]
    pub fn height_percentage(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.height_percentage)
    }

    #[must_use]
    pub fn rotation_angle(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.rotation_angle)
    }

    #[must_use]
    pub fn corner_radius_percentage(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.corner_radius_percentage)
    }
}
impl SmartFilterPath<crate::types::StoryAreaType> {
    #[must_use]
    pub fn address(self) -> SmartFilterPath<crate::types::LocationAddress> {
        self.and_then(|value| value.address())
    }

    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.emoji())
    }

    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.name())
    }

    #[must_use]
    pub fn reaction_type(self) -> SmartFilterPath<crate::types::ReactionType> {
        self.and_then(|value| value.reaction_type())
    }

    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.url())
    }
}
impl SmartFilterPath<crate::types::StoryAreaTypeLink> {
    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.map(|value| value.url.as_ref())
    }
}
impl SmartFilterPath<crate::types::StoryAreaTypeLocation> {
    #[must_use]
    pub fn latitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.latitude)
    }

    #[must_use]
    pub fn longitude(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.longitude)
    }

    #[must_use]
    pub fn address(self) -> SmartFilterPath<crate::types::LocationAddress> {
        self.and_then(|value| value.address.as_ref())
    }
}
impl SmartFilterPath<crate::types::StoryAreaTypeSuggestedReaction> {
    #[must_use]
    pub fn reaction_type(self) -> SmartFilterPath<crate::types::ReactionType> {
        self.map(|value| &value.reaction_type)
    }

    #[must_use]
    pub fn is_dark(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_dark.as_ref())
    }

    #[must_use]
    pub fn is_flipped(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_flipped.as_ref())
    }
}
impl SmartFilterPath<crate::types::StoryAreaTypeUniqueGift> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }
}
impl SmartFilterPath<crate::types::StoryAreaTypeWeather> {
    #[must_use]
    pub fn temperature(self) -> SmartFilterPath<f64> {
        self.map(|value| &value.temperature)
    }

    #[must_use]
    pub fn emoji(self) -> SmartFilterPath<str> {
        self.map(|value| value.emoji.as_ref())
    }

    #[must_use]
    pub fn background_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.background_color)
    }
}
impl SmartFilterPath<crate::types::SuccessfulPayment> {
    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.map(|value| value.currency.as_ref())
    }

    #[must_use]
    pub fn total_amount(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_amount)
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.map(|value| value.invoice_payload.as_ref())
    }

    #[must_use]
    pub fn subscription_expiration_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.subscription_expiration_date.as_ref())
    }

    #[must_use]
    pub fn is_recurring(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_recurring.as_ref())
    }

    #[must_use]
    pub fn is_first_recurring(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_first_recurring.as_ref())
    }

    #[must_use]
    pub fn shipping_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.shipping_option_id.as_deref())
    }

    #[must_use]
    pub fn order_info(self) -> SmartFilterPath<crate::types::OrderInfo> {
        self.and_then(|value| value.order_info.as_ref())
    }

    #[must_use]
    pub fn telegram_payment_charge_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.telegram_payment_charge_id.as_ref())
    }

    #[must_use]
    pub fn provider_payment_charge_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.provider_payment_charge_id.as_ref())
    }
}
impl SmartFilterPath<crate::types::SuggestedPostApprovalFailed> {
    #[must_use]
    pub fn suggested_post_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.suggested_post_message.as_deref())
    }

    #[must_use]
    pub fn price(self) -> SmartFilterPath<crate::types::SuggestedPostPrice> {
        self.map(|value| &value.price)
    }
}
impl SmartFilterPath<crate::types::SuggestedPostApproved> {
    #[must_use]
    pub fn suggested_post_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.suggested_post_message.as_deref())
    }

    #[must_use]
    pub fn price(self) -> SmartFilterPath<crate::types::SuggestedPostPrice> {
        self.and_then(|value| value.price.as_ref())
    }

    #[must_use]
    pub fn send_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.send_date)
    }
}
impl SmartFilterPath<crate::types::SuggestedPostDeclined> {
    #[must_use]
    pub fn suggested_post_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.suggested_post_message.as_deref())
    }

    #[must_use]
    pub fn comment(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.comment.as_deref())
    }
}
impl SmartFilterPath<crate::types::SuggestedPostInfo> {
    #[must_use]
    pub fn state(self) -> SmartFilterPath<str> {
        self.map(|value| value.state.as_ref())
    }

    #[must_use]
    pub fn price(self) -> SmartFilterPath<crate::types::SuggestedPostPrice> {
        self.and_then(|value| value.price.as_ref())
    }

    #[must_use]
    pub fn send_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.send_date.as_ref())
    }
}
impl SmartFilterPath<crate::types::SuggestedPostPaid> {
    #[must_use]
    pub fn suggested_post_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.suggested_post_message.as_deref())
    }

    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.map(|value| value.currency.as_ref())
    }

    #[must_use]
    pub fn amount(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.amount.as_ref())
    }

    #[must_use]
    pub fn star_amount(self) -> SmartFilterPath<crate::types::StarAmount> {
        self.and_then(|value| value.star_amount.as_ref())
    }
}
impl SmartFilterPath<crate::types::SuggestedPostParameters> {
    #[must_use]
    pub fn price(self) -> SmartFilterPath<crate::types::SuggestedPostPrice> {
        self.and_then(|value| value.price.as_ref())
    }

    #[must_use]
    pub fn send_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.send_date.as_ref())
    }
}
impl SmartFilterPath<crate::types::SuggestedPostPrice> {
    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.map(|value| value.currency.as_ref())
    }

    #[must_use]
    pub fn amount(self) -> SmartFilterPath<u32> {
        self.map(|value| &value.amount)
    }
}
impl SmartFilterPath<crate::types::SuggestedPostRefunded> {
    #[must_use]
    pub fn suggested_post_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.suggested_post_message.as_deref())
    }

    #[must_use]
    pub fn reason(self) -> SmartFilterPath<str> {
        self.map(|value| value.reason.as_ref())
    }
}
impl SmartFilterPath<crate::types::SwitchInlineQueryChosenChat> {
    #[must_use]
    pub fn query(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.query.as_deref())
    }

    #[must_use]
    pub fn allow_user_chats(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.allow_user_chats.as_ref())
    }

    #[must_use]
    pub fn allow_bot_chats(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.allow_bot_chats.as_ref())
    }

    #[must_use]
    pub fn allow_group_chats(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.allow_group_chats.as_ref())
    }

    #[must_use]
    pub fn allow_channel_chats(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.allow_channel_chats.as_ref())
    }
}
impl SmartFilterPath<crate::types::TextQuote> {
    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.map(|value| value.text.as_ref())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities.as_deref())
    }

    #[must_use]
    pub fn position(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.position)
    }

    #[must_use]
    pub fn is_manual(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_manual.as_ref())
    }
}
impl SmartFilterPath<crate::types::TransactionPartner> {
    #[must_use]
    pub fn affiliate(self) -> SmartFilterPath<crate::types::AffiliateInfo> {
        self.and_then(|value| value.affiliate())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::Gift> {
        self.and_then(|value| value.gift())
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.invoice_payload())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<[crate::types::PaidMedia]> {
        self.and_then(|value| value.paid_media())
    }

    #[must_use]
    pub fn paid_media_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.paid_media_payload())
    }

    #[must_use]
    pub fn sponsor_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sponsor_user())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user())
    }

    #[must_use]
    pub fn withdrawal_state(self) -> SmartFilterPath<crate::types::RevenueWithdrawalState> {
        self.and_then(|value| value.withdrawal_state())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerAffiliateProgram> {
    #[must_use]
    pub fn sponsor_user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sponsor_user.as_deref())
    }

    #[must_use]
    pub fn commission_per_mille(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.commission_per_mille)
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerChat> {
    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.map(|value| value.chat.as_ref())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::Gift> {
        self.and_then(|value| value.gift.as_deref())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerFragment> {
    #[must_use]
    pub fn withdrawal_state(self) -> SmartFilterPath<crate::types::RevenueWithdrawalState> {
        self.and_then(|value| value.withdrawal_state.as_ref())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerOther> {}
impl SmartFilterPath<crate::types::TransactionPartnerTelegramAds> {}
impl SmartFilterPath<crate::types::TransactionPartnerTelegramApi> {
    #[must_use]
    pub fn request_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.request_count)
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerUser> {
    #[must_use]
    pub fn affiliate(self) -> SmartFilterPath<crate::types::AffiliateInfo> {
        self.and_then(|value| value.affiliate())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::Gift> {
        self.and_then(|value| value.gift())
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.invoice_payload())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<[crate::types::PaidMedia]> {
        self.and_then(|value| value.paid_media())
    }

    #[must_use]
    pub fn paid_media_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.paid_media_payload())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerUserBusinessAccountTransfer> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerUserGiftPurchase> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::Gift> {
        self.map(|value| value.gift.as_ref())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerUserInvoicePayment> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn affiliate(self) -> SmartFilterPath<crate::types::AffiliateInfo> {
        self.and_then(|value| value.affiliate.as_ref())
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.invoice_payload.as_deref())
    }

    #[must_use]
    pub fn subscription_period(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.subscription_period.as_ref())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerUserPaidMediaPayment> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn affiliate(self) -> SmartFilterPath<crate::types::AffiliateInfo> {
        self.and_then(|value| value.affiliate.as_ref())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<[crate::types::PaidMedia]> {
        self.map(|value| value.paid_media.as_ref())
    }

    #[must_use]
    pub fn paid_media_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.paid_media_payload.as_deref())
    }
}
impl SmartFilterPath<crate::types::TransactionPartnerUserPremiumPurchase> {
    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.map(|value| value.user.as_ref())
    }

    #[must_use]
    pub fn premium_subscription_duration(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.premium_subscription_duration)
    }
}
impl SmartFilterPath<crate::types::UniqueGift> {
    #[must_use]
    pub fn gift_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.gift_id.as_ref())
    }

    #[must_use]
    pub fn base_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.base_name.as_ref())
    }

    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }

    #[must_use]
    pub fn number(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.number)
    }

    #[must_use]
    pub fn model(self) -> SmartFilterPath<crate::types::UniqueGiftModel> {
        self.map(|value| &value.model)
    }

    #[must_use]
    pub fn symbol(self) -> SmartFilterPath<crate::types::UniqueGiftSymbol> {
        self.map(|value| &value.symbol)
    }

    #[must_use]
    pub fn backdrop(self) -> SmartFilterPath<crate::types::UniqueGiftBackdrop> {
        self.map(|value| &value.backdrop)
    }

    #[must_use]
    pub fn is_premium(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_premium.as_ref())
    }

    #[must_use]
    pub fn is_burned(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_burned.as_ref())
    }

    #[must_use]
    pub fn is_from_blockchain(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_from_blockchain.as_ref())
    }

    #[must_use]
    pub fn colors(self) -> SmartFilterPath<crate::types::UniqueGiftColors> {
        self.and_then(|value| value.colors.as_ref())
    }

    #[must_use]
    pub fn publisher_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.publisher_chat.as_deref())
    }
}
impl SmartFilterPath<crate::types::UniqueGiftBackdrop> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }

    #[must_use]
    pub fn colors(self) -> SmartFilterPath<crate::types::UniqueGiftBackdropColors> {
        self.map(|value| &value.colors)
    }

    #[must_use]
    pub fn rarity_per_mille(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.rarity_per_mille)
    }
}
impl SmartFilterPath<crate::types::UniqueGiftBackdropColors> {
    #[must_use]
    pub fn center_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.center_color)
    }

    #[must_use]
    pub fn edge_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.edge_color)
    }

    #[must_use]
    pub fn symbol_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.symbol_color)
    }

    #[must_use]
    pub fn text_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.text_color)
    }
}
impl SmartFilterPath<crate::types::UniqueGiftColors> {
    #[must_use]
    pub fn model_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.model_custom_emoji_id.as_ref())
    }

    #[must_use]
    pub fn symbol_custom_emoji_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.symbol_custom_emoji_id.as_ref())
    }

    #[must_use]
    pub fn light_theme_main_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.light_theme_main_color)
    }

    #[must_use]
    pub fn light_theme_other_colors(self) -> SmartFilterPath<[u8]> {
        self.map(|value| value.light_theme_other_colors.as_ref())
    }

    #[must_use]
    pub fn dark_theme_main_color(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.dark_theme_main_color)
    }

    #[must_use]
    pub fn dark_theme_other_colors(self) -> SmartFilterPath<[u8]> {
        self.map(|value| value.dark_theme_other_colors.as_ref())
    }
}
impl SmartFilterPath<crate::types::UniqueGiftInfo> {
    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::UniqueGift> {
        self.map(|value| value.gift.as_ref())
    }

    #[must_use]
    pub fn origin(self) -> SmartFilterPath<str> {
        self.map(|value| value.origin.as_ref())
    }

    #[must_use]
    pub fn last_resale_currency(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_resale_currency.as_deref())
    }

    #[must_use]
    pub fn last_resale_amount(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.last_resale_amount.as_ref())
    }

    #[must_use]
    pub fn owned_gift_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.owned_gift_id.as_deref())
    }

    #[must_use]
    pub fn transfer_star_count(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.transfer_star_count.as_ref())
    }

    #[must_use]
    pub fn next_transfer_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.next_transfer_date.as_ref())
    }
}
impl SmartFilterPath<crate::types::UniqueGiftModel> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.map(|value| value.sticker.as_ref())
    }

    #[must_use]
    pub fn rarity_per_mille(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.rarity_per_mille)
    }

    #[must_use]
    pub fn rarity(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.rarity.as_deref())
    }
}
impl SmartFilterPath<crate::types::UniqueGiftSymbol> {
    #[must_use]
    pub fn name(self) -> SmartFilterPath<str> {
        self.map(|value| value.name.as_ref())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.map(|value| value.sticker.as_ref())
    }

    #[must_use]
    pub fn rarity_per_mille(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.rarity_per_mille)
    }
}
impl SmartFilterPath<crate::types::Update> {
    #[must_use]
    pub fn actor_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.actor_chat())
    }

    #[must_use]
    pub fn animation(self) -> SmartFilterPath<crate::types::Animation> {
        self.and_then(|value| value.animation())
    }

    #[must_use]
    pub fn audio(self) -> SmartFilterPath<crate::types::Audio> {
        self.and_then(|value| value.audio())
    }

    #[must_use]
    pub fn author_signature(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.author_signature())
    }

    #[must_use]
    pub fn bio(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.bio())
    }

    #[must_use]
    pub fn boost(self) -> SmartFilterPath<crate::types::ChatBoost> {
        self.and_then(|value| value.boost())
    }

    #[must_use]
    pub fn boost_added(self) -> SmartFilterPath<crate::types::ChatBoostAdded> {
        self.and_then(|value| value.boost_added())
    }

    #[must_use]
    pub fn boost_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.boost_id())
    }

    #[must_use]
    pub fn bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.bot())
    }

    #[must_use]
    pub fn business_connection(self) -> SmartFilterPath<crate::types::BusinessConnection> {
        self.and_then(|value| value.business_connection())
    }

    #[must_use]
    pub fn business_connection_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.business_connection_id())
    }

    #[must_use]
    pub fn business_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.business_message())
    }

    #[must_use]
    pub fn callback_query(self) -> SmartFilterPath<crate::types::CallbackQuery> {
        self.and_then(|value| value.callback_query())
    }

    #[must_use]
    pub fn caption(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.caption())
    }

    #[must_use]
    pub fn caption_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.caption_entities())
    }

    #[must_use]
    pub fn channel_post(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.channel_post())
    }

    #[must_use]
    pub fn chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.chat())
    }

    #[must_use]
    pub fn chat_background_set(self) -> SmartFilterPath<crate::types::ChatBackground> {
        self.and_then(|value| value.chat_background_set())
    }

    #[must_use]
    pub fn chat_boost(self) -> SmartFilterPath<crate::types::ChatBoostUpdated> {
        self.and_then(|value| value.chat_boost())
    }

    #[must_use]
    pub fn chat_instance(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.chat_instance())
    }

    #[must_use]
    pub fn chat_join_request(self) -> SmartFilterPath<crate::types::ChatJoinRequest> {
        self.and_then(|value| value.chat_join_request())
    }

    #[must_use]
    pub fn chat_member(self) -> SmartFilterPath<crate::types::ChatMemberUpdated> {
        self.and_then(|value| value.chat_member())
    }

    #[must_use]
    pub fn chat_owner_changed(self) -> SmartFilterPath<crate::types::ChatOwnerChanged> {
        self.and_then(|value| value.chat_owner_changed())
    }

    #[must_use]
    pub fn chat_owner_left(self) -> SmartFilterPath<crate::types::ChatOwnerLeft> {
        self.and_then(|value| value.chat_owner_left())
    }

    #[must_use]
    pub fn chat_shared(self) -> SmartFilterPath<crate::types::ChatShared> {
        self.and_then(|value| value.chat_shared())
    }

    #[must_use]
    pub fn chat_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.chat_type())
    }

    #[must_use]
    pub fn checklist(self) -> SmartFilterPath<crate::types::Checklist> {
        self.and_then(|value| value.checklist())
    }

    #[must_use]
    pub fn checklist_tasks_added(self) -> SmartFilterPath<crate::types::ChecklistTasksAdded> {
        self.and_then(|value| value.checklist_tasks_added())
    }

    #[must_use]
    pub fn checklist_tasks_done(self) -> SmartFilterPath<crate::types::ChecklistTasksDone> {
        self.and_then(|value| value.checklist_tasks_done())
    }

    #[must_use]
    pub fn chosen_inline_result(self) -> SmartFilterPath<crate::types::ChosenInlineResult> {
        self.and_then(|value| value.chosen_inline_result())
    }

    #[must_use]
    pub fn connected_website(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.connected_website())
    }

    #[must_use]
    pub fn contact(self) -> SmartFilterPath<crate::types::Contact> {
        self.and_then(|value| value.contact())
    }

    #[must_use]
    pub fn correct_option_ids(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.correct_option_ids())
    }

    #[must_use]
    pub fn currency(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.currency())
    }

    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.data())
    }

    #[must_use]
    pub fn deleted_business_messages(
        self,
    ) -> SmartFilterPath<crate::types::BusinessMessagesDeleted> {
        self.and_then(|value| value.deleted_business_messages())
    }

    #[must_use]
    pub fn description(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.description())
    }

    #[must_use]
    pub fn description_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.description_entities())
    }

    #[must_use]
    pub fn dice(self) -> SmartFilterPath<crate::types::Dice> {
        self.and_then(|value| value.dice())
    }

    #[must_use]
    pub fn direct_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::DirectMessagePriceChanged> {
        self.and_then(|value| value.direct_message_price_changed())
    }

    #[must_use]
    pub fn direct_messages_topic(self) -> SmartFilterPath<crate::types::DirectMessagesTopic> {
        self.and_then(|value| value.direct_messages_topic())
    }

    #[must_use]
    pub fn document(self) -> SmartFilterPath<crate::types::Document> {
        self.and_then(|value| value.document())
    }

    #[must_use]
    pub fn edited_business_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.edited_business_message())
    }

    #[must_use]
    pub fn edited_channel_post(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.edited_channel_post())
    }

    #[must_use]
    pub fn edited_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.edited_message())
    }

    #[must_use]
    pub fn effect_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.effect_id())
    }

    #[must_use]
    pub fn entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.entities())
    }

    #[must_use]
    pub fn explanation(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.explanation())
    }

    #[must_use]
    pub fn explanation_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.explanation_entities())
    }

    #[must_use]
    pub fn external_reply(self) -> SmartFilterPath<crate::types::ExternalReplyInfo> {
        self.and_then(|value| value.external_reply())
    }

    #[must_use]
    pub fn forum_topic_closed(self) -> SmartFilterPath<crate::types::ForumTopicClosed> {
        self.and_then(|value| value.forum_topic_closed())
    }

    #[must_use]
    pub fn forum_topic_created(self) -> SmartFilterPath<crate::types::ForumTopicCreated> {
        self.and_then(|value| value.forum_topic_created())
    }

    #[must_use]
    pub fn forum_topic_edited(self) -> SmartFilterPath<crate::types::ForumTopicEdited> {
        self.and_then(|value| value.forum_topic_edited())
    }

    #[must_use]
    pub fn forum_topic_reopened(self) -> SmartFilterPath<crate::types::ForumTopicReopened> {
        self.and_then(|value| value.forum_topic_reopened())
    }

    #[must_use]
    pub fn forward_origin(self) -> SmartFilterPath<crate::types::MessageOrigin> {
        self.and_then(|value| value.forward_origin())
    }

    #[must_use]
    pub fn from(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.from())
    }

    #[must_use]
    pub fn game(self) -> SmartFilterPath<crate::types::Game> {
        self.and_then(|value| value.game())
    }

    #[must_use]
    pub fn game_short_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.game_short_name())
    }

    #[must_use]
    pub fn general_forum_topic_hidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicHidden> {
        self.and_then(|value| value.general_forum_topic_hidden())
    }

    #[must_use]
    pub fn general_forum_topic_unhidden(
        self,
    ) -> SmartFilterPath<crate::types::GeneralForumTopicUnhidden> {
        self.and_then(|value| value.general_forum_topic_unhidden())
    }

    #[must_use]
    pub fn gift(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.and_then(|value| value.gift())
    }

    #[must_use]
    pub fn gift_upgrade_sent(self) -> SmartFilterPath<crate::types::GiftInfo> {
        self.and_then(|value| value.gift_upgrade_sent())
    }

    #[must_use]
    pub fn giveaway(self) -> SmartFilterPath<crate::types::Giveaway> {
        self.and_then(|value| value.giveaway())
    }

    #[must_use]
    pub fn giveaway_completed(self) -> SmartFilterPath<crate::types::GiveawayCompleted> {
        self.and_then(|value| value.giveaway_completed())
    }

    #[must_use]
    pub fn giveaway_created(self) -> SmartFilterPath<crate::types::GiveawayCreated> {
        self.and_then(|value| value.giveaway_created())
    }

    #[must_use]
    pub fn giveaway_winners(self) -> SmartFilterPath<crate::types::GiveawayWinners> {
        self.and_then(|value| value.giveaway_winners())
    }

    #[must_use]
    pub fn id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.id())
    }

    #[must_use]
    pub fn inline_message_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.inline_message_id())
    }

    #[must_use]
    pub fn inline_query(self) -> SmartFilterPath<crate::types::InlineQuery> {
        self.and_then(|value| value.inline_query())
    }

    #[must_use]
    pub fn invite_link(self) -> SmartFilterPath<crate::types::ChatInviteLink> {
        self.and_then(|value| value.invite_link())
    }

    #[must_use]
    pub fn invoice(self) -> SmartFilterPath<crate::types::Invoice> {
        self.and_then(|value| value.invoice())
    }

    #[must_use]
    pub fn invoice_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.invoice_payload())
    }

    #[must_use]
    pub fn left_chat_member(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.left_chat_member())
    }

    #[must_use]
    pub fn link_preview_options(self) -> SmartFilterPath<crate::types::LinkPreviewOptions> {
        self.and_then(|value| value.link_preview_options())
    }

    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.and_then(|value| value.location())
    }

    #[must_use]
    pub fn managed_bot(self) -> SmartFilterPath<crate::types::ManagedBotUpdated> {
        self.and_then(|value| value.managed_bot())
    }

    #[must_use]
    pub fn managed_bot_created(self) -> SmartFilterPath<crate::types::ManagedBotCreated> {
        self.and_then(|value| value.managed_bot_created())
    }

    #[must_use]
    pub fn media_group_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.media_group_id())
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.message())
    }

    #[must_use]
    pub fn message_auto_delete_timer_changed(
        self,
    ) -> SmartFilterPath<crate::types::MessageAutoDeleteTimerChanged> {
        self.and_then(|value| value.message_auto_delete_timer_changed())
    }

    #[must_use]
    pub fn message_ids(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.message_ids())
    }

    #[must_use]
    pub fn message_reaction(self) -> SmartFilterPath<crate::types::MessageReactionUpdated> {
        self.and_then(|value| value.message_reaction())
    }

    #[must_use]
    pub fn message_reaction_count(
        self,
    ) -> SmartFilterPath<crate::types::MessageReactionCountUpdated> {
        self.and_then(|value| value.message_reaction_count())
    }

    #[must_use]
    pub fn my_chat_member(self) -> SmartFilterPath<crate::types::ChatMemberUpdated> {
        self.and_then(|value| value.my_chat_member())
    }

    #[must_use]
    pub fn new_chat_member(self) -> SmartFilterPath<crate::types::ChatMember> {
        self.and_then(|value| value.new_chat_member())
    }

    #[must_use]
    pub fn new_chat_members(self) -> SmartFilterPath<[crate::types::User]> {
        self.and_then(|value| value.new_chat_members())
    }

    #[must_use]
    pub fn new_chat_photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.new_chat_photo())
    }

    #[must_use]
    pub fn new_chat_title(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.new_chat_title())
    }

    #[must_use]
    pub fn new_reaction(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.and_then(|value| value.new_reaction())
    }

    #[must_use]
    pub fn offset(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.offset())
    }

    #[must_use]
    pub fn old_chat_member(self) -> SmartFilterPath<crate::types::ChatMember> {
        self.and_then(|value| value.old_chat_member())
    }

    #[must_use]
    pub fn old_reaction(self) -> SmartFilterPath<[crate::types::ReactionType]> {
        self.and_then(|value| value.old_reaction())
    }

    #[must_use]
    pub fn option_ids(self) -> SmartFilterPath<[i64]> {
        self.and_then(|value| value.option_ids())
    }

    #[must_use]
    pub fn option_persistent_ids(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.option_persistent_ids())
    }

    #[must_use]
    pub fn options(self) -> SmartFilterPath<[crate::types::PollOption]> {
        self.and_then(|value| value.options())
    }

    #[must_use]
    pub fn order_info(self) -> SmartFilterPath<crate::types::OrderInfo> {
        self.and_then(|value| value.order_info())
    }

    #[must_use]
    pub fn paid_media(self) -> SmartFilterPath<crate::types::PaidMediaInfo> {
        self.and_then(|value| value.paid_media())
    }

    #[must_use]
    pub fn paid_media_payload(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.paid_media_payload())
    }

    #[must_use]
    pub fn paid_message_price_changed(
        self,
    ) -> SmartFilterPath<crate::types::PaidMessagePriceChanged> {
        self.and_then(|value| value.paid_message_price_changed())
    }

    #[must_use]
    pub fn passport_data(self) -> SmartFilterPath<crate::types::PassportData> {
        self.and_then(|value| value.passport_data())
    }

    #[must_use]
    pub fn photo(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.photo())
    }

    #[must_use]
    pub fn pinned_message(self) -> SmartFilterPath<crate::types::MaybeInaccessibleMessage> {
        self.and_then(|value| value.pinned_message())
    }

    #[must_use]
    pub fn poll(self) -> SmartFilterPath<crate::types::Poll> {
        self.and_then(|value| value.poll())
    }

    #[must_use]
    pub fn poll_answer(self) -> SmartFilterPath<crate::types::PollAnswer> {
        self.and_then(|value| value.poll_answer())
    }

    #[must_use]
    pub fn poll_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.poll_id())
    }

    #[must_use]
    pub fn poll_option_added(self) -> SmartFilterPath<crate::types::PollOptionAdded> {
        self.and_then(|value| value.poll_option_added())
    }

    #[must_use]
    pub fn poll_option_deleted(self) -> SmartFilterPath<crate::types::PollOptionDeleted> {
        self.and_then(|value| value.poll_option_deleted())
    }

    #[must_use]
    pub fn pre_checkout_query(self) -> SmartFilterPath<crate::types::PreCheckoutQuery> {
        self.and_then(|value| value.pre_checkout_query())
    }

    #[must_use]
    pub fn proximity_alert_triggered(
        self,
    ) -> SmartFilterPath<crate::types::ProximityAlertTriggered> {
        self.and_then(|value| value.proximity_alert_triggered())
    }

    #[must_use]
    pub fn purchased_paid_media(self) -> SmartFilterPath<crate::types::PaidMediaPurchased> {
        self.and_then(|value| value.purchased_paid_media())
    }

    #[must_use]
    pub fn query(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.query())
    }

    #[must_use]
    pub fn question(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.question())
    }

    #[must_use]
    pub fn question_entities(self) -> SmartFilterPath<[crate::types::MessageEntity]> {
        self.and_then(|value| value.question_entities())
    }

    #[must_use]
    pub fn quote(self) -> SmartFilterPath<crate::types::TextQuote> {
        self.and_then(|value| value.quote())
    }

    #[must_use]
    pub fn reactions(self) -> SmartFilterPath<[crate::types::ReactionCount]> {
        self.and_then(|value| value.reactions())
    }

    #[must_use]
    pub fn refunded_payment(self) -> SmartFilterPath<crate::types::RefundedPayment> {
        self.and_then(|value| value.refunded_payment())
    }

    #[must_use]
    pub fn removed_chat_boost(self) -> SmartFilterPath<crate::types::ChatBoostRemoved> {
        self.and_then(|value| value.removed_chat_boost())
    }

    #[must_use]
    pub fn reply_markup(self) -> SmartFilterPath<crate::types::InlineKeyboardMarkup> {
        self.and_then(|value| value.reply_markup())
    }

    #[must_use]
    pub fn reply_to_message(self) -> SmartFilterPath<crate::types::Message> {
        self.and_then(|value| value.reply_to_message())
    }

    #[must_use]
    pub fn reply_to_poll_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.reply_to_poll_option_id())
    }

    #[must_use]
    pub fn reply_to_story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.reply_to_story())
    }

    #[must_use]
    pub fn result_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.result_id())
    }

    #[must_use]
    pub fn rights(self) -> SmartFilterPath<crate::types::BusinessBotRights> {
        self.and_then(|value| value.rights())
    }

    #[must_use]
    pub fn sender_business_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.sender_business_bot())
    }

    #[must_use]
    pub fn sender_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.sender_chat())
    }

    #[must_use]
    pub fn sender_tag(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.sender_tag())
    }

    #[must_use]
    pub fn shipping_address(self) -> SmartFilterPath<crate::types::ShippingAddress> {
        self.and_then(|value| value.shipping_address())
    }

    #[must_use]
    pub fn shipping_option_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.shipping_option_id())
    }

    #[must_use]
    pub fn shipping_query(self) -> SmartFilterPath<crate::types::ShippingQuery> {
        self.and_then(|value| value.shipping_query())
    }

    #[must_use]
    pub fn source(self) -> SmartFilterPath<crate::types::ChatBoostSource> {
        self.and_then(|value| value.source())
    }

    #[must_use]
    pub fn sticker(self) -> SmartFilterPath<crate::types::Sticker> {
        self.and_then(|value| value.sticker())
    }

    #[must_use]
    pub fn story(self) -> SmartFilterPath<crate::types::Story> {
        self.and_then(|value| value.story())
    }

    #[must_use]
    pub fn successful_payment(self) -> SmartFilterPath<crate::types::SuccessfulPayment> {
        self.and_then(|value| value.successful_payment())
    }

    #[must_use]
    pub fn suggested_post_approval_failed(
        self,
    ) -> SmartFilterPath<crate::types::SuggestedPostApprovalFailed> {
        self.and_then(|value| value.suggested_post_approval_failed())
    }

    #[must_use]
    pub fn suggested_post_approved(self) -> SmartFilterPath<crate::types::SuggestedPostApproved> {
        self.and_then(|value| value.suggested_post_approved())
    }

    #[must_use]
    pub fn suggested_post_declined(self) -> SmartFilterPath<crate::types::SuggestedPostDeclined> {
        self.and_then(|value| value.suggested_post_declined())
    }

    #[must_use]
    pub fn suggested_post_info(self) -> SmartFilterPath<crate::types::SuggestedPostInfo> {
        self.and_then(|value| value.suggested_post_info())
    }

    #[must_use]
    pub fn suggested_post_paid(self) -> SmartFilterPath<crate::types::SuggestedPostPaid> {
        self.and_then(|value| value.suggested_post_paid())
    }

    #[must_use]
    pub fn suggested_post_refunded(self) -> SmartFilterPath<crate::types::SuggestedPostRefunded> {
        self.and_then(|value| value.suggested_post_refunded())
    }

    #[must_use]
    pub fn text(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.text())
    }

    #[must_use]
    pub fn unique_gift(self) -> SmartFilterPath<crate::types::UniqueGiftInfo> {
        self.and_then(|value| value.unique_gift())
    }

    #[must_use]
    pub fn user(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.user())
    }

    #[must_use]
    pub fn users_shared(self) -> SmartFilterPath<crate::types::UsersShared> {
        self.and_then(|value| value.users_shared())
    }

    #[must_use]
    pub fn venue(self) -> SmartFilterPath<crate::types::Venue> {
        self.and_then(|value| value.venue())
    }

    #[must_use]
    pub fn via_bot(self) -> SmartFilterPath<crate::types::User> {
        self.and_then(|value| value.via_bot())
    }

    #[must_use]
    pub fn video(self) -> SmartFilterPath<crate::types::Video> {
        self.and_then(|value| value.video())
    }

    #[must_use]
    pub fn video_chat_ended(self) -> SmartFilterPath<crate::types::VideoChatEnded> {
        self.and_then(|value| value.video_chat_ended())
    }

    #[must_use]
    pub fn video_chat_participants_invited(
        self,
    ) -> SmartFilterPath<crate::types::VideoChatParticipantsInvited> {
        self.and_then(|value| value.video_chat_participants_invited())
    }

    #[must_use]
    pub fn video_chat_scheduled(self) -> SmartFilterPath<crate::types::VideoChatScheduled> {
        self.and_then(|value| value.video_chat_scheduled())
    }

    #[must_use]
    pub fn video_chat_started(self) -> SmartFilterPath<crate::types::VideoChatStarted> {
        self.and_then(|value| value.video_chat_started())
    }

    #[must_use]
    pub fn video_note(self) -> SmartFilterPath<crate::types::VideoNote> {
        self.and_then(|value| value.video_note())
    }

    #[must_use]
    pub fn voice(self) -> SmartFilterPath<crate::types::Voice> {
        self.and_then(|value| value.voice())
    }

    #[must_use]
    pub fn voter_chat(self) -> SmartFilterPath<crate::types::Chat> {
        self.and_then(|value| value.voter_chat())
    }

    #[must_use]
    pub fn web_app_data(self) -> SmartFilterPath<crate::types::WebAppData> {
        self.and_then(|value| value.web_app_data())
    }

    #[must_use]
    pub fn write_access_allowed(self) -> SmartFilterPath<crate::types::WriteAccessAllowed> {
        self.and_then(|value| value.write_access_allowed())
    }
}
impl SmartFilterPath<crate::types::UpdateBusinessConnection> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn business_connection(self) -> SmartFilterPath<crate::types::BusinessConnection> {
        self.map(|value| &value.business_connection)
    }
}
impl SmartFilterPath<crate::types::UpdateBusinessMessage> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn business_message(self) -> SmartFilterPath<crate::types::Message> {
        self.map(|value| value.business_message.as_ref())
    }
}
impl SmartFilterPath<crate::types::UpdateCallbackQuery> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn callback_query(self) -> SmartFilterPath<crate::types::CallbackQuery> {
        self.map(|value| &value.callback_query)
    }
}
impl SmartFilterPath<crate::types::UpdateChannelPost> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn channel_post(self) -> SmartFilterPath<crate::types::Message> {
        self.map(|value| value.channel_post.as_ref())
    }
}
impl SmartFilterPath<crate::types::UpdateChatBoost> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn chat_boost(self) -> SmartFilterPath<crate::types::ChatBoostUpdated> {
        self.map(|value| &value.chat_boost)
    }
}
impl SmartFilterPath<crate::types::UpdateChatJoinRequest> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn chat_join_request(self) -> SmartFilterPath<crate::types::ChatJoinRequest> {
        self.map(|value| &value.chat_join_request)
    }
}
impl SmartFilterPath<crate::types::UpdateChatMember> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn chat_member(self) -> SmartFilterPath<crate::types::ChatMemberUpdated> {
        self.map(|value| &value.chat_member)
    }
}
impl SmartFilterPath<crate::types::UpdateChosenInlineResult> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn chosen_inline_result(self) -> SmartFilterPath<crate::types::ChosenInlineResult> {
        self.map(|value| &value.chosen_inline_result)
    }
}
impl SmartFilterPath<crate::types::UpdateDeletedBusinessMessages> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn deleted_business_messages(
        self,
    ) -> SmartFilterPath<crate::types::BusinessMessagesDeleted> {
        self.map(|value| &value.deleted_business_messages)
    }
}
impl SmartFilterPath<crate::types::UpdateEditedBusinessMessage> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn edited_business_message(self) -> SmartFilterPath<crate::types::Message> {
        self.map(|value| value.edited_business_message.as_ref())
    }
}
impl SmartFilterPath<crate::types::UpdateEditedChannelPost> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn edited_channel_post(self) -> SmartFilterPath<crate::types::Message> {
        self.map(|value| value.edited_channel_post.as_ref())
    }
}
impl SmartFilterPath<crate::types::UpdateEditedMessage> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn edited_message(self) -> SmartFilterPath<crate::types::Message> {
        self.map(|value| value.edited_message.as_ref())
    }
}
impl SmartFilterPath<crate::types::UpdateInlineQuery> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn inline_query(self) -> SmartFilterPath<crate::types::InlineQuery> {
        self.map(|value| &value.inline_query)
    }
}
impl SmartFilterPath<crate::types::UpdateManagedBot> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn managed_bot(self) -> SmartFilterPath<crate::types::ManagedBotUpdated> {
        self.map(|value| &value.managed_bot)
    }
}
impl SmartFilterPath<crate::types::UpdateMessage> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn message(self) -> SmartFilterPath<crate::types::Message> {
        self.map(|value| value.message.as_ref())
    }
}
impl SmartFilterPath<crate::types::UpdateMessageReaction> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn message_reaction(self) -> SmartFilterPath<crate::types::MessageReactionUpdated> {
        self.map(|value| &value.message_reaction)
    }
}
impl SmartFilterPath<crate::types::UpdateMessageReactionCount> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn message_reaction_count(
        self,
    ) -> SmartFilterPath<crate::types::MessageReactionCountUpdated> {
        self.map(|value| &value.message_reaction_count)
    }
}
impl SmartFilterPath<crate::types::UpdateMyChatMember> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn my_chat_member(self) -> SmartFilterPath<crate::types::ChatMemberUpdated> {
        self.map(|value| &value.my_chat_member)
    }
}
impl SmartFilterPath<crate::types::UpdatePoll> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn poll(self) -> SmartFilterPath<crate::types::Poll> {
        self.map(|value| value.poll.as_ref())
    }
}
impl SmartFilterPath<crate::types::UpdatePollAnswer> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn poll_answer(self) -> SmartFilterPath<crate::types::PollAnswer> {
        self.map(|value| &value.poll_answer)
    }
}
impl SmartFilterPath<crate::types::UpdatePreCheckoutQuery> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn pre_checkout_query(self) -> SmartFilterPath<crate::types::PreCheckoutQuery> {
        self.map(|value| &value.pre_checkout_query)
    }
}
impl SmartFilterPath<crate::types::UpdatePurchasedPaidMedia> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn purchased_paid_media(self) -> SmartFilterPath<crate::types::PaidMediaPurchased> {
        self.map(|value| &value.purchased_paid_media)
    }
}
impl SmartFilterPath<crate::types::UpdateRemovedChatBoost> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn removed_chat_boost(self) -> SmartFilterPath<crate::types::ChatBoostRemoved> {
        self.map(|value| &value.removed_chat_boost)
    }
}
impl SmartFilterPath<crate::types::UpdateShippingQuery> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }

    #[must_use]
    pub fn shipping_query(self) -> SmartFilterPath<crate::types::ShippingQuery> {
        self.map(|value| &value.shipping_query)
    }
}
impl SmartFilterPath<crate::types::UpdateUnparsed> {
    #[must_use]
    pub fn update_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.update_id)
    }
}
impl SmartFilterPath<crate::types::User> {
    #[must_use]
    pub fn id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.id)
    }

    #[must_use]
    pub fn is_bot(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.is_bot)
    }

    #[must_use]
    pub fn first_name(self) -> SmartFilterPath<str> {
        self.map(|value| value.first_name.as_ref())
    }

    #[must_use]
    pub fn last_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_name.as_deref())
    }

    #[must_use]
    pub fn username(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.username.as_deref())
    }

    #[must_use]
    pub fn language_code(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.language_code.as_deref())
    }

    #[must_use]
    pub fn is_premium(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.is_premium.as_ref())
    }

    #[must_use]
    pub fn added_to_attachment_menu(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.added_to_attachment_menu.as_ref())
    }

    #[must_use]
    pub fn can_join_groups(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_join_groups.as_ref())
    }

    #[must_use]
    pub fn can_read_all_group_messages(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_read_all_group_messages.as_ref())
    }

    #[must_use]
    pub fn supports_inline_queries(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.supports_inline_queries.as_ref())
    }

    #[must_use]
    pub fn can_connect_to_business(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_connect_to_business.as_ref())
    }

    #[must_use]
    pub fn has_main_web_app(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_main_web_app.as_ref())
    }

    #[must_use]
    pub fn has_topics_enabled(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.has_topics_enabled.as_ref())
    }

    #[must_use]
    pub fn allows_users_to_create_topics(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.allows_users_to_create_topics.as_ref())
    }

    #[must_use]
    pub fn can_manage_bots(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.can_manage_bots.as_ref())
    }
}
impl SmartFilterPath<crate::types::UserChatBoosts> {
    #[must_use]
    pub fn boosts(self) -> SmartFilterPath<[crate::types::ChatBoost]> {
        self.map(|value| value.boosts.as_ref())
    }
}
impl SmartFilterPath<crate::types::UserProfileAudios> {
    #[must_use]
    pub fn total_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_count)
    }

    #[must_use]
    pub fn audios(self) -> SmartFilterPath<[crate::types::Audio]> {
        self.map(|value| value.audios.as_ref())
    }
}
impl SmartFilterPath<crate::types::UserProfilePhotos> {
    #[must_use]
    pub fn total_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.total_count)
    }

    #[must_use]
    pub fn photos(self) -> SmartFilterPath<[Box<[crate::types::PhotoSize]>]> {
        self.map(|value| value.photos.as_ref())
    }
}
impl SmartFilterPath<crate::types::UserRating> {
    #[must_use]
    pub fn level(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.level)
    }

    #[must_use]
    pub fn rating(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.rating)
    }

    #[must_use]
    pub fn current_level_rating(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.current_level_rating)
    }

    #[must_use]
    pub fn next_level_rating(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.next_level_rating.as_ref())
    }
}
impl SmartFilterPath<crate::types::UsersShared> {
    #[must_use]
    pub fn request_id(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.request_id)
    }

    #[must_use]
    pub fn users(self) -> SmartFilterPath<[crate::types::SharedUser]> {
        self.map(|value| value.users.as_ref())
    }
}
impl SmartFilterPath<crate::types::Venue> {
    #[must_use]
    pub fn location(self) -> SmartFilterPath<crate::types::Location> {
        self.map(|value| &value.location)
    }

    #[must_use]
    pub fn title(self) -> SmartFilterPath<str> {
        self.map(|value| value.title.as_ref())
    }

    #[must_use]
    pub fn address(self) -> SmartFilterPath<str> {
        self.map(|value| value.address.as_ref())
    }

    #[must_use]
    pub fn foursquare_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_id.as_deref())
    }

    #[must_use]
    pub fn foursquare_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.foursquare_type.as_deref())
    }

    #[must_use]
    pub fn google_place_id(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_id.as_deref())
    }

    #[must_use]
    pub fn google_place_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.google_place_type.as_deref())
    }
}
impl SmartFilterPath<crate::types::Video> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.width)
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.height)
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.duration)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn cover(self) -> SmartFilterPath<[crate::types::PhotoSize]> {
        self.and_then(|value| value.cover.as_deref())
    }

    #[must_use]
    pub fn start_timestamp(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.start_timestamp.as_ref())
    }

    #[must_use]
    pub fn qualities(self) -> SmartFilterPath<[crate::types::VideoQuality]> {
        self.and_then(|value| value.qualities.as_deref())
    }

    #[must_use]
    pub fn file_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.file_name.as_deref())
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type.as_deref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::VideoChatEnded> {
    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.duration)
    }
}
impl SmartFilterPath<crate::types::VideoChatParticipantsInvited> {
    #[must_use]
    pub fn users(self) -> SmartFilterPath<[crate::types::User]> {
        self.map(|value| value.users.as_ref())
    }
}
impl SmartFilterPath<crate::types::VideoChatScheduled> {
    #[must_use]
    pub fn start_date(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.start_date)
    }
}
impl SmartFilterPath<crate::types::VideoChatStarted> {}
impl SmartFilterPath<crate::types::VideoNote> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn length(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.length)
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.duration)
    }

    #[must_use]
    pub fn thumbnail(self) -> SmartFilterPath<crate::types::PhotoSize> {
        self.and_then(|value| value.thumbnail.as_ref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::VideoQuality> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn width(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.width)
    }

    #[must_use]
    pub fn height(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.height)
    }

    #[must_use]
    pub fn codec(self) -> SmartFilterPath<str> {
        self.map(|value| value.codec.as_ref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::Voice> {
    #[must_use]
    pub fn file_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_id.as_ref())
    }

    #[must_use]
    pub fn file_unique_id(self) -> SmartFilterPath<str> {
        self.map(|value| value.file_unique_id.as_ref())
    }

    #[must_use]
    pub fn duration(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.duration)
    }

    #[must_use]
    pub fn mime_type(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.mime_type.as_deref())
    }

    #[must_use]
    pub fn file_size(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.file_size.as_ref())
    }
}
impl SmartFilterPath<crate::types::WebAppData> {
    #[must_use]
    pub fn data(self) -> SmartFilterPath<str> {
        self.map(|value| value.data.as_ref())
    }

    #[must_use]
    pub fn button_text(self) -> SmartFilterPath<str> {
        self.map(|value| value.button_text.as_ref())
    }
}
impl SmartFilterPath<crate::types::WebAppInfo> {
    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.map(|value| value.url.as_ref())
    }
}
impl SmartFilterPath<crate::types::WebhookInfo> {
    #[must_use]
    pub fn url(self) -> SmartFilterPath<str> {
        self.map(|value| value.url.as_ref())
    }

    #[must_use]
    pub fn has_custom_certificate(self) -> SmartFilterPath<bool> {
        self.map(|value| &value.has_custom_certificate)
    }

    #[must_use]
    pub fn pending_update_count(self) -> SmartFilterPath<i64> {
        self.map(|value| &value.pending_update_count)
    }

    #[must_use]
    pub fn ip_address(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.ip_address.as_deref())
    }

    #[must_use]
    pub fn last_error_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.last_error_date.as_ref())
    }

    #[must_use]
    pub fn last_error_message(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.last_error_message.as_deref())
    }

    #[must_use]
    pub fn last_synchronization_error_date(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.last_synchronization_error_date.as_ref())
    }

    #[must_use]
    pub fn max_connections(self) -> SmartFilterPath<i64> {
        self.and_then(|value| value.max_connections.as_ref())
    }

    #[must_use]
    pub fn allowed_updates(self) -> SmartFilterPath<[Box<str>]> {
        self.and_then(|value| value.allowed_updates.as_deref())
    }
}
impl SmartFilterPath<crate::types::WriteAccessAllowed> {
    #[must_use]
    pub fn from_request(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.from_request.as_ref())
    }

    #[must_use]
    pub fn web_app_name(self) -> SmartFilterPath<str> {
        self.and_then(|value| value.web_app_name.as_deref())
    }

    #[must_use]
    pub fn from_attachment_menu(self) -> SmartFilterPath<bool> {
        self.and_then(|value| value.from_attachment_menu.as_ref())
    }
}
