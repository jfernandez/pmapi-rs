use std::{ffi::CString, os::raw::c_char};

use crate::error::get_error;
use pmapi_sys::{pmID, pmLookupName};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NameError {
    #[error("Invalid metric name: {0}")]
    InvalidMetricName(String),
}

pub fn lookup_name(metric_name: &str) -> Result<u32, NameError> {
    let c_metric_name = match CString::new(metric_name) {
        Ok(name) => name,
        Err(err) => return Err(NameError::InvalidMetricName(err.to_string())),
    };
    let mut c_metric_name_ptr = c_metric_name.as_ptr();
    let mut metric_id = pmID::default();

    let sts = unsafe {
        pmLookupName(
            1,
            &mut c_metric_name_ptr as *mut *const c_char,
            &mut metric_id,
        )
    };

    if sts < 0 {
        return Err(NameError::InvalidMetricName(get_error(sts)));
    }

    Ok(metric_id)
}

#[cfg(test)]
mod tests {
    use crate::context::{Context, ContextType};

    use super::*;

    #[test]
    fn test_lookup_name() {
        let _context = Context::new(ContextType::Host, "localhost").unwrap();
        let metric_id = lookup_name("kernel.all.cpu.user").unwrap();
        assert!(metric_id > 0);
    }

    #[test]
    fn test_lookup_name_invalid() {
        let _context = Context::new(ContextType::Host, "localhost").unwrap();
        let metric_id = lookup_name("invalid.metric");
        match metric_id {
            Ok(_) => panic!("Expected an error"),
            Err(err) => assert_eq!(err.to_string(), "Invalid metric name: Unknown metric name"),
        }
    }
}
