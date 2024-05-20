use wcstack::{
    proto::stack::{JjState, JjStateStack},
    store::Store,
};

fn main() -> Result<(), std::io::Error> {
    let stack = JjStateStack {
        stack: vec![JjState {
            change_id: vec![0u8, 1u8, 2u8],
            is_new: false,
        }],
    };

    let store = Store::new_in_current_workspace()?;
    store.save(&stack)?;

    let loaded_stack = store.load()?;
    assert_eq!(stack, loaded_stack);

    Ok(())
}
