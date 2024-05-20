pub mod proto {
    pub mod stack {
        include!(concat!(env!("OUT_DIR"), "/wcstack.proto.stack.rs"));
    }
}

mod change_id;
pub mod store;
