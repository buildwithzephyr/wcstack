use wcstack::{
    change_id::ChangeId,
    stack::{JjState, WcStack},
    store::Store,
};

fn main() -> Result<(), std::io::Error> {
    let stack = WcStack(
        vec![JjState {
            change_id: ChangeId(vec![0u8, 1u8, 2u8]),
            is_new: false,
        }]
        .into(),
    );

    let store = Store::new_in_current_workspace()?;
    store.save(&stack)?;

    let loaded_stack = store.load()?;
    assert_eq!(stack, loaded_stack);

    Ok(())
}
