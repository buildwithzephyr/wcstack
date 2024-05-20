use wcstack::{
    change_id::ChangeId,
    jj_util::{get_current_change_id, get_is_discardable, get_parent_change_id},
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

    println!("{}", get_current_change_id()?);
    println!("{}", get_parent_change_id()?);
    println!("{}", get_is_discardable()?);

    Ok(())
}
