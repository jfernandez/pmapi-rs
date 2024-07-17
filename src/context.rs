use std::ffi::CString;

use pmapi_sys::{
    pmDestroyContext, pmNewContext, PM_CONTEXT_ARCHIVE, PM_CONTEXT_HOST, PM_CONTEXT_LOCAL,
};
use thiserror::Error;

use crate::error::get_error;

pub struct Context {
    handle: i32,
}

pub enum ContextType {
    Host,
    Archive,
    Local,
}

#[derive(Error, Debug)]
pub enum ContextError {
    #[error("Invalid context name: {0}")]
    InvalidContextName(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Context {
    pub fn new(context_type: ContextType, name: &str) -> Result<Self, ContextError> {
        let context_type = match context_type {
            ContextType::Host => PM_CONTEXT_HOST as i32,
            ContextType::Archive => PM_CONTEXT_ARCHIVE as i32,
            ContextType::Local => PM_CONTEXT_LOCAL as i32,
        };

        let c_name = match CString::new(name) {
            Ok(name) => name,
            Err(err) => return Err(ContextError::InvalidContextName(err.to_string())),
        };
        let c_name_ptr = c_name.as_ptr();

        let handle = unsafe { pmNewContext(context_type, c_name_ptr) };

        if handle < 0 {
            let error = get_error(handle);
            return Err(ContextError::Unknown(error));
        }

        Ok(Context { handle })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            let sts = pmDestroyContext(self.handle);
            if sts < 0 {
                let error = get_error(sts);
                panic!("Error destroying context: {}", error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_context() {
        let context = Context::new(ContextType::Host, "localhost");
        match context {
            Ok(_) => assert!(true),
            Err(err) => {
                panic!("Error creating context: {}", err);
            }
        }
    }
}
