pub mod proto {
    pub mod stack {
        use crate::stack::WcStack;

        include!(concat!(env!("OUT_DIR"), "/wcstack.proto.stack.rs"));

        impl From<&crate::stack::JjState> for crate::proto::stack::JjState {
            fn from(value: &crate::stack::JjState) -> Self {
                Self {
                    change_id: value.change_id.0.clone().into(),
                    is_new: value.is_new,
                }
            }
        }

        impl From<&WcStack> for crate::proto::stack::JjStateStack {
            fn from(value: &WcStack) -> Self {
                Self {
                    stack: value.0.iter().map(|state| state.into()).collect(),
                }
            }
        }
    }
}

pub mod change_id;
pub mod stack;
pub mod store;
