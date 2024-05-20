use crate::{proto::stack::JjStateStack, stack::WcStack};
use prost::Message;
use std::{
    ffi::OsString, fs, io::Write, os::unix::ffi::OsStringExt, path::PathBuf, process::Command,
};
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
        let output = Command::new("jj").args(["workspace", "root"]).output()?;
        let workspace_root: String = OsString::from_vec(output.stdout)
            .to_string_lossy()
            .trim_end_matches('\n')
            .to_owned();
        let mut filepath: PathBuf = PathBuf::from(workspace_root);
        filepath.push(".jj");
        filepath.push("wc_stack");
        Ok(Self {
            stack_filepath: filepath,
        })
    }
}
