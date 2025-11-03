#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PremiumSubscriptionStarCount {
    Thousand,
    OneThousandFiveHundred,
    TwoFiveHundred,
}

impl From<PremiumSubscriptionStarCount> for u16 {
    fn from(star_count: PremiumSubscriptionStarCount) -> Self {
        match star_count {
            PremiumSubscriptionStarCount::Thousand => 1000,
            PremiumSubscriptionStarCount::OneThousandFiveHundred => 1500,
            PremiumSubscriptionStarCount::TwoFiveHundred => 2500,
        }
    }
}
