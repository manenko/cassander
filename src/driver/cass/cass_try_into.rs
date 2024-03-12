/// Tries to convert a value into a another type via [`TryInto::try_into`].
///
/// Returns the converted value if the conversion succeeds or a
/// [`CassError::LibBadParams`](crate::driver::cass::CassError::LibBadParams) if
/// it fails.
#[macro_export]
macro_rules! cass_try_into {
    ($v:expr) => {
        match $v.try_into() {
            Ok(val) => val,
            Err(_)  => return $crate::driver::cass::CassError::LibBadParams,
        }
    };
}
