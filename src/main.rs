pub mod proto {
    pub mod stack {
        include!(concat!(env!("OUT_DIR"), "/wcstack.proto.stack.rs"));
    }
}

use proto::stack::{JjState, JjStateStack};

fn main() {
    let stack = JjStateStack {
        stack: vec![JjState {
            change_id: vec![0u8, 1u8, 2u8],
            is_new: false,
        }],
    };
    println!("{:?}", stack.stack)
}
