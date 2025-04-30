use crate::ffi::CassDataType;

pub struct TupleDataType {
    pub(crate) inner: *mut CassDataType,
}
