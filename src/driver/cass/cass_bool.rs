use crate::driver::ffi::{
    enum_cass_bool_t,
    enum_cass_bool_t_cass_false as CASS_FALSE,
    enum_cass_bool_t_cass_true as CASS_TRUE,
};

/// A boolean value used by the driver.
#[repr(transparent)]
pub struct CassBool(enum_cass_bool_t);

impl CassBool {
    /// Wraps a raw `enum_cass_bool_t` value.
    pub fn new(value: enum_cass_bool_t) -> Self {
        Self(value)
    }

    /// Returns the wrapped `enum_cass_bool_t` value.
    pub fn as_raw(&self) -> enum_cass_bool_t {
        self.0
    }
}

impl From<CassBool> for bool {
    #[rustfmt::skip]
    fn from(value: CassBool) -> Self {
        match value.as_raw() {
            CASS_TRUE  => true,
            CASS_FALSE => false,
            raw        => unreachable!("unknown enum_cass_bool_t variant {}", raw),
        }
    }
}

impl From<bool> for CassBool {
    #[rustfmt::skip]
    fn from(value: bool) -> Self {
        match value {
            true  => CassBool(CASS_TRUE),
            false => CassBool(CASS_FALSE),
        }
    }
}
