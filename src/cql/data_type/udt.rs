use crate::ffi::CassDataType;

pub struct UdtDataType {
    pub(crate) inner: *mut CassDataType,
}
