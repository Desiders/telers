#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivePeriod {
    SixHours,
    TwelveHours,
    OneDay,
    TwoDays,
}

impl From<ActivePeriod> for u32 {
    fn from(period: ActivePeriod) -> Self {
        match period {
            ActivePeriod::SixHours => 6 * 3600,
            ActivePeriod::TwelveHours => 12 * 3600,
            ActivePeriod::OneDay => 86400,
            ActivePeriod::TwoDays => 2 * 86400,
        }
    }
}
