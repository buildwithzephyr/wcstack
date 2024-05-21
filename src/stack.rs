use std::collections::VecDeque;

use crate::change_id::ChangeId;

#[derive(Debug, PartialEq, Clone)]
pub struct JjState {
    pub change_id: ChangeId,
    pub is_new: bool,
}

impl From<&crate::proto::stack::JjState> for JjState {
    fn from(value: &crate::proto::stack::JjState) -> Self {
        Self {
            change_id: ChangeId(value.change_id.clone()),
            is_new: value.is_new,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WcStack(pub VecDeque<JjState>);

impl From<&crate::proto::stack::JjStateStack> for WcStack {
    fn from(value: &crate::proto::stack::JjStateStack) -> Self {
        Self(value.stack.iter().map(|state| state.into()).collect())
    }
}

impl WcStack {
    pub fn push(&mut self, new_state: JjState) {
        self.0.push_front(new_state);
    }

    pub fn pop(&mut self) -> Option<JjState> {
        self.0.pop_front()
    }

    pub fn iter(&self) -> impl Iterator<Item = &JjState> {
        self.0.iter()
    }

    pub fn empty() -> Self {
        Self(VecDeque::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
