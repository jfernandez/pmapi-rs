use std::alloc::{alloc, dealloc, Layout};

use pmapi_sys::{pmFetch, pmResult};
use thiserror::Error;

use crate::error::get_error;

#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Fetch a metric by its ID and return the raw result from the PMAPI.
/// 
/// On success, the return value is a `Box` containing a `pmResult` struct.
/// Memory deallocatoin is handled by the `Box` when it goes out of scope.
/// See the unit tests for an example of how to use this function.
pub fn fetch_raw(metric_id: u32) -> Result<Box<pmResult>, MetricsError> {
    let layout = Layout::new::<pmResult>();
    let mut result_ptr = unsafe { alloc(layout) as *mut pmResult };
    if result_ptr.is_null() {
        return Err(MetricsError::Unknown("Failed to allocate memory".to_string()));
    }

    let mut metric_id = metric_id;
    let sts = unsafe { pmFetch(1, &mut metric_id, &mut result_ptr) };

    if sts < 0 {
        unsafe { dealloc(result_ptr as *mut u8, layout) }

        return Err(MetricsError::Unknown(get_error(sts)));
    }

    let result_box = unsafe { Box::from_raw(result_ptr) };
    Ok(result_box)
}

#[cfg(test)]
mod tests {
    use crate::{context::{Context, ContextType}, name::lookup_name};

    use super::*;

    #[test]
    fn test_fetch_raw() {
        let _context = Context::new(ContextType::Host, "localhost").unwrap();
        let metric_id = lookup_name("kernel.all.cpu.user").unwrap();
        let result = fetch_raw(metric_id);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.numpmid, 1);
        let vset = unsafe { *(result.vset[0]) };
        assert_eq!(vset.pmid, metric_id);
    }
}
