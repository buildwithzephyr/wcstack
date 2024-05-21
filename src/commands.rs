use clap::{command, Parser, Subcommand};

use crate::{
    change_id::ChangeId,
    errors::WcStackError,
    jj_util::{
        edit_change, get_current_change_id, get_is_discardable, get_jj_status,
        get_parent_change_id, new_from_change,
    },
    stack::JjState,
    store::Store,
};

#[derive(Subcommand)]
pub enum Commands {
    Push,
    List,
    Pop,
    Drop,
}

fn display_state(state: &JjState) -> String {
    // TODO Call JJ and print some change information
    format!(
        "{}{}",
        state.change_id,
        if state.is_new { "+ (new)" } else { "  (edit)" }
    )
}

impl Commands {
    pub fn execute(&self, store: Store) -> Result<(), WcStackError> {
        match self {
            Self::Push => {
                let discardable = get_is_discardable()?;
                let change_id: ChangeId = if discardable {
                    get_parent_change_id()?.as_str().try_into()?
                } else {
                    get_current_change_id()?.as_str().try_into()?
                };
                let state = JjState {
                    change_id: change_id.clone(),
                    is_new: discardable,
                };
                let mut stack = store.load()?;
                stack.push(state.clone());
                store.save(&stack)?;
                println!("Saved working copy: {}", display_state(&state));
                Ok(())
            }
            Self::List => {
                let stack = store.load()?;
                let log: Vec<String> = stack.iter().map(display_state).collect();
                if log.len() > 0 {
                    println!("{}", log.join("\n"));
                } else {
                    println!("{}", "(Stack is empty)");
                }
                Ok(())
            }
            Self::Pop => {
                let mut stack = store.load()?;
                let maybe_state = stack.pop();
                if let Some(state) = maybe_state {
                    if state.is_new {
                        new_from_change(state.change_id.to_string())?
                    } else {
                        edit_change(state.change_id.to_string())?
                    };
                    store.save(&stack)?;
                    println!("{}", "State restored, new jj status:");
                    println!("{}", get_jj_status()?);

                    Ok(())
                } else {
                    println!("{}", "Nothing to pop; stack is empty");
                    Ok(())
                }
            }
            Self::Drop => {
                let mut stack = store.load()?;
                let maybe_state = stack.pop();
                if let Some(state) = maybe_state {
                    store.save(&stack)?;
                    println!("Dropped state: {}", display_state(&state));
                    Ok(())
                } else {
                    println!("{}", "Nothing to drop; stack is empty");
                    Ok(())
                }
            }
        }
    }
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
