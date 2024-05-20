pub mod proto {
    pub mod stack {
        include!(concat!(env!("OUT_DIR"), "/wcstack.proto.stack.rs"));
    }
}

use proto::stack::ChangeIdStack;

fn main() {
    let stack = ChangeIdStack { change_ids: vec![vec![0u8, 1u8, 2u8]]};
    println!("{:?}", stack.change_ids)
}
