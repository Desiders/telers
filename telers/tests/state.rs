//! Round trip of the `State` derive states through a real storage

#![cfg(feature = "memory-storage")]

use telers::{
    fsm::{MemoryStorage, StorageKey},
    FSMContext, State,
};

#[derive(Clone, State)]
enum OrderState {
    Start,
    AwaitingPayment,
}

#[test]
fn derived_states_are_stored_as_their_names() {
    let context = FSMContext::new(MemoryStorage::new(), StorageKey::new(1, 2, 3, None, None));

    tokio_test::block_on(async {
        context.set_state(OrderState::Start).await.unwrap();
        assert_eq!(context.get_state().await.unwrap().as_deref(), Some("start"));

        context
            .set_state(OrderState::AwaitingPayment)
            .await
            .unwrap();

        let stored = context.get_state().await.unwrap();
        assert_eq!(stored.as_deref(), Some("awaiting_payment"));
        assert!(OrderState::AwaitingPayment == stored.as_deref().unwrap());
    });
}
