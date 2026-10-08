//! Combinations of the variants of `#[derive(State)]`, checked at runtime

use telers::filters::State as StateFilter;
use telers_macros::State;

// 1. Defaults: the state name is the variant name in `snake_case`
#[derive(Clone, State)]
enum OrderState {
    Start,
    AwaitingPayment,
    Done,
}

fn main() {
    assert_eq!(OrderState::AwaitingPayment.as_str(), "awaiting_payment");
    assert!(OrderState::Start == "start");
    assert!(OrderState::Done != "start");

    // `AsRef<str>` is what `FSMContext::set_state` asks for
    assert_eq!(std::convert::AsRef::<str>::as_ref(&OrderState::Done), "done");
    assert_eq!(OrderState::AwaitingPayment.to_string(), "awaiting_payment");

    // The states can be passed to the `State` filter as they are
    let _ = StateFilter::one(OrderState::Start);
    let _ = StateFilter::many([OrderState::Start, OrderState::Done]);
}