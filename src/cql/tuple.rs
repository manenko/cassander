use crate::ffi::{
    cass_tuple_free,
    cass_tuple_new,
    struct_CassTuple_,
};

pub struct CqlTuple(*mut struct_CassTuple_);

impl CqlTuple {
    /// Creates a new CQL tuple object from the driver's tuple.
    pub(crate) fn from_driver(tuple: *mut struct_CassTuple_) -> Self {
        CqlTuple(tuple)
    }

    /// Creates a new CQL tuple object with the given number of items.
    pub fn new(items_count: usize) -> Self {
        let tuple = unsafe { cass_tuple_new(items_count) };

        Self::from_driver(tuple)
    }

    /// Returns the inner CQL tuple object.
    pub(crate) fn inner(&self) -> *mut struct_CassTuple_ {
        self.0
    }
}

impl Drop for CqlTuple {
    /// Frees the underlying CQL tuple object.
    fn drop(&mut self) {
        unsafe { cass_tuple_free(self.inner()) }
    }
}
