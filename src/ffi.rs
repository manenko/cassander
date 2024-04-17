//! FFI bindings for DataStax C++ driver for Apache Cassandra.

mod bindings;

use core::slice;
use std::ffi::{
    c_char,
    CStr,
    FromBytesWithNulError,
};
use std::str::Utf8Error;

pub(crate) use bindings::*;

use crate::{
    to_result,
    DriverError,
    DriverErrorKind,
};

/// Given a Rust string slices, calls the setter with the slice pointer and
/// length, then converts the returned driver status code to a `Result`.
pub(crate) fn set_c_str<S, F>(s: S, setter: F) -> Result<(), DriverError>
where
    S: AsRef<str>,
    F: Fn(*const c_char, usize) -> enum_CassError_,
{
    let s = s.as_ref();
    let error = setter(s.as_ptr() as _, s.len());

    to_result(error)
}

/// Calls the getter which extracts a C string pointer and its lengths, then
/// converts the C string into a Rust string slice.
pub(crate) fn get_str<'a, F>(getter: F) -> Result<&'a str, DriverError>
where
    F: Fn(*mut *const c_char, *mut usize) -> enum_CassError_,
{
    let mut ptr = std::ptr::null();
    let mut len = 0;

    let error = getter(&mut ptr, &mut len);
    to_result(error)?;

    c_str_n_to_str(ptr, len)
}

/// Converts a C string to a Rust string slice.
pub(crate) fn c_str_n_to_str<'a>(
    c_str: *const c_char,
    len: usize,
) -> Result<&'a str, DriverError> {
    if c_str.is_null() || len == 0 {
        return Ok("");
    }

    let slice = unsafe { slice::from_raw_parts(c_str as _, len) };
    let c_str = CStr::from_bytes_with_nul(slice)
        .map_err(|e| invalid_c_string_error(slice, e))?;

    c_str
        .to_str()
        .map_err(|error| invalid_utf8_error(c_str, error))
}

fn invalid_c_string_error(
    slice: &[u8],
    error: FromBytesWithNulError,
) -> DriverError {
    DriverError::with_message(
        DriverErrorKind::LibInternalError,
        format!(
            "the slice {:?} cannot be converted into a C string: {:?}",
            slice, error
        ),
    )
}

fn invalid_utf8_error(c_str: &CStr, error: Utf8Error) -> DriverError {
    DriverError::with_message(
        DriverErrorKind::LibInternalError,
        format!(
            "the C string {:?} cannot be converted into a UTF-8 string: {:?}",
            c_str, error
        ),
    )
}

#[cfg(test)]
mod tests {
    mod c_str_n_to_str {
        use crate::ffi::c_str_n_to_str;

        #[test]
        fn test_null_ptr() {
            let ptr = std::ptr::null();
            let size = 5;

            let result = c_str_n_to_str(ptr, size);

            match result {
                Ok(s) => assert!(s.is_empty()),
                Err(e) => panic!(
                    "should have returned an empty string but got an error \
                     {:?}",
                    e
                ),
            }
        }

        #[test]
        fn test_zero_len() {
            let c_str = b"hello, world!\x00";
            let ptr = c_str.as_ptr() as _;
            let size = 0;

            let result = c_str_n_to_str(ptr, size);

            match result {
                Ok(s) => assert!(s.is_empty()),
                Err(e) => panic!(
                    "should have returned an empty string but got an error \
                     {:?}",
                    e
                ),
            }
        }

        #[test]
        fn test_interior_nul() {
            let c_str = b"hello, \x00 world!\x00";
            let ptr = c_str.as_ptr() as _;
            let size = 16;

            let result = c_str_n_to_str(ptr, size);

            match result {
                Ok(s) => {
                    panic!("should have returned an error but got '{}'", s)
                }
                Err(e) => assert!(e.message.contains("InteriorNul")),
            }
        }

        #[test]
        fn test_excludes_trailing_nul() {
            let c_str = b"hello, world!\x00";
            let ptr = c_str.as_ptr() as _;
            let size = 14;

            let result = c_str_n_to_str(ptr, size);

            match result {
                Ok(s) => assert_eq!("hello, world!", s),
                Err(e) => panic!(
                    "should have returned a string but got an error {:?}",
                    e
                ),
            }
        }
    }
}
