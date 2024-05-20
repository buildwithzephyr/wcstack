use crate::{jj_util::get_workspace_root, proto::stack::JjStateStack, stack::WcStack};
use prost::Message;
use std::{fs, io::Write, path::PathBuf};
use tempfile::NamedTempFile;

pub struct Store {
    stack_filepath: PathBuf,
}

impl Store {
    pub fn save(&self, stack: &WcStack) -> Result<(), std::io::Error> {
        let temp_file = NamedTempFile::new()?;
        let protobuf_stack: JjStateStack = stack.into();
        temp_file.as_file().write(&protobuf_stack.encode_to_vec())?;
        temp_file.persist(self.stack_filepath.clone())?;
        Ok(())
    }

    pub fn load(&self) -> Result<WcStack, std::io::Error> {
        let buf = fs::read(&self.stack_filepath)?;
        let protobuf_stack = &JjStateStack::decode(&*buf)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        Ok(protobuf_stack.into())
    }

    pub fn new_in_current_workspace() -> Result<Self, std::io::Error> {
        let mut filepath = get_workspace_root()?;
        filepath.push(".jj");
        filepath.push("wc_stack");
        Ok(Self {
            stack_filepath: filepath,
        })
    }
}
