#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PremiumSubscriptionMonthCount {
    ThreeMonths,
    SixMonths,
    TwelveMonths,
}

impl From<PremiumSubscriptionMonthCount> for u8 {
    fn from(period: PremiumSubscriptionMonthCount) -> Self {
        match period {
            PremiumSubscriptionMonthCount::ThreeMonths => 3,
            PremiumSubscriptionMonthCount::SixMonths => 6,
            PremiumSubscriptionMonthCount::TwelveMonths => 12,
        }
    }
}
