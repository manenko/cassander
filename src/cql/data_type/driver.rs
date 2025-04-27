use std::marker::PhantomData;
use std::ops::Deref;

use crate::cql::ValueType;
use crate::ffi::{
    cass_data_type_add_sub_type,
    cass_data_type_add_sub_type_by_name_n,
    cass_data_type_class_name,
    cass_data_type_free,
    cass_data_type_is_frozen,
    cass_data_type_keyspace,
    cass_data_type_new,
    cass_data_type_new_from_existing,
    cass_data_type_new_tuple,
    cass_data_type_new_udt,
    cass_data_type_set_class_name_n,
    cass_data_type_set_keyspace_n,
    cass_data_type_set_type_name_n,
    cass_data_type_sub_data_type,
    cass_data_type_sub_data_type_by_name_n,
    cass_data_type_sub_type_count,
    cass_data_type_sub_type_name,
    cass_data_type_type,
    cass_data_type_type_name,
    enum_cass_bool_t_cass_false as CASS_FALSE,
    get_str,
    set_c_str,
    struct_CassDataType_,
};
use crate::{
    to_result,
    DriverError,
    DriverErrorKind,
};

/// A data type used to describe a value, collection or user-defined type.
pub struct DriverDataType {
    inner: *mut struct_CassDataType_,
    owned: bool,
}

impl DriverDataType {
    /// Creates a new owned wrapper from the given driver object.
    pub(crate) fn new_owned(data_type: *mut struct_CassDataType_) -> Self {
        Self {
            inner: data_type,
            owned: true,
        }
    }

    /// Creates a n ew non-owned wrapper from the given driver object.
    pub(crate) fn new_borrowed(data_type: *const struct_CassDataType_) -> Self {
        Self {
            inner: data_type as _,
            owned: false,
        }
    }

    /// Creates a new data type with the given value type.
    pub fn new(value_type: ValueType) -> Self {
        let value_type = value_type.into();
        let data_type = unsafe { cass_data_type_new(value_type) };

        Self::new_owned(data_type)
    }

    /// Creates a new data type from an existing one.
    pub fn from_existing<D>(other: D) -> Self
    where
        D: AsRef<DriverDataType>,
    {
        let other = other.as_ref();
        let data_type =
            unsafe { cass_data_type_new_from_existing(other.inner()) };

        Self::new_owned(data_type)
    }

    /// Creates a new tuple data type with the given number of items.
    pub fn new_tuple(items_count: usize) -> Self {
        let data_type = unsafe { cass_data_type_new_tuple(items_count) };

        Self::new_owned(data_type)
    }

    /// Creates a new user-defined type data type with the given number of
    /// fields.
    pub fn new_udt(field_count: usize) -> Self {
        let data_type = unsafe { cass_data_type_new_udt(field_count) };

        Self::new_owned(data_type)
    }

    /// Returns the value type of the data type.
    pub fn value_type(&self) -> ValueType {
        unsafe { cass_data_type_type(self.inner()) }.into()
    }

    /// Returns whether the data type is frozen.
    pub fn is_frozen(&self) -> bool {
        let frozen = unsafe { cass_data_type_is_frozen(self.inner()) };

        frozen != CASS_FALSE
    }

    /// Returns the name of the user-defined type described by this data type.
    ///
    /// Returns an error if the data type is not a user-defined type.
    pub fn udt_name(&self) -> Result<&str, DriverError> {
        get_str(|p, l| unsafe { cass_data_type_type_name(self.inner(), p, l) })
    }

    /// Sets the name of the user-defined type described by this data type.
    ///
    /// Returns an error if the data type is not a user-defined type.
    pub fn set_udt_name<S>(&mut self, name: S) -> Result<(), DriverError>
    where
        S: AsRef<str>,
    {
        set_c_str(name, |p, l| unsafe {
            cass_data_type_set_type_name_n(self.inner(), p, l)
        })
    }

    /// Returns the name of the keyspace where this UDT is defined.
    ///
    /// Returns an error if the data type is not a user-defined type.
    pub fn udt_keyspace(&self) -> Result<&str, DriverError> {
        get_str(|p, l| unsafe { cass_data_type_keyspace(self.inner(), p, l) })
    }

    /// Returns the inner data type object.
    pub(crate) fn inner(&self) -> *mut struct_CassDataType_ {
        self.inner
    }

    /// Sets the name of the keyspace of this UDT.
    ///
    /// Returns an error of the data type is not a user-defined type.
    pub fn set_udt_keyspace<S>(&mut self, name: S) -> Result<(), DriverError>
    where
        S: AsRef<str>,
    {
        set_c_str(name, |p, l| unsafe {
            cass_data_type_set_keyspace_n(self.inner(), p, l)
        })
    }

    /// Returns the name of the custom data type.
    ///
    /// Returns an error if this data type is not [`ValueType::Custom`].
    pub fn class_name(&self) -> Result<&str, DriverError> {
        get_str(|p, l| unsafe { cass_data_type_class_name(self.inner(), p, l) })
    }

    /// Sets the name of the custom data type.
    ///
    /// Returns an error if this data type is not [`ValueType::Custom`].
    pub fn set_class_name<S>(&mut self, name: S) -> Result<(), DriverError>
    where
        S: AsRef<str>,
    {
        set_c_str(name, |p, l| unsafe {
            cass_data_type_set_class_name_n(self.inner(), p, l)
        })
    }

    /// Returns the number of sub-types of a UDT, tuple, or collection.
    ///
    /// Returns an error of the data type has not sub-types.
    pub fn sub_type_count(&self) -> Result<usize, DriverError> {
        self.ensure_sub_types_supported()?;
        let count = unsafe { cass_data_type_sub_type_count(self.inner()) };

        Ok(count)
    }

    /// Returns an immutable reference to a sub-type at the given index for a
    /// UDT, tuple, or collection.
    ///
    /// Returns an error if the index is out of range, of this data type is not
    /// one of the: UDT, tuple, collection.
    pub fn sub_data_type_by_index(
        &self,
        index: usize,
    ) -> Result<DriverDataTypeRef<'_>, DriverError> {
        self.ensure_sub_type_index_in_range(index)?;

        let data_type =
            unsafe { cass_data_type_sub_data_type(self.inner(), index) };

        // The driver returns `NULL` if the given index is out of range or the
        // data type does not support sub-types. We already checked these cases
        // but let's have an additional check in case of changes in the driver
        // behavior in the future.
        assert!(!data_type.is_null());

        Ok(DriverDataTypeRef::new(data_type))
    }

    /// Returns an immutable reference to a sub-type for a field of this UDT.
    ///
    /// Returns an error if this is not a UDT or if it has no field with such
    /// name.
    pub fn sub_data_type_by_name<S>(
        &self,
        name: S,
    ) -> Result<DriverDataTypeRef<'_>, DriverError>
    where
        S: AsRef<str>,
    {
        self.ensure_this_is_udt()?;

        let name = name.as_ref();
        let p = name.as_ptr() as _;
        let l = name.len();

        let data_type = unsafe {
            cass_data_type_sub_data_type_by_name_n(self.inner(), p, l)
        };

        // The driver returns `NULL` when there is no field with the given name
        // found for this UDT.
        if data_type.is_null() {
            Err(DriverError::with_message(
                DriverErrorKind::LibBadParams,
                format!(
                    "there is no field '{}' in this user-defined type",
                    name
                ),
            ))
        } else {
            Ok(DriverDataTypeRef::new(data_type))
        }
    }

    /// Gets the sub-type name of a UDT (user defined type) at the specified
    /// index.
    ///
    /// Returns an error if this is not a UDT or if the index is out of range.
    pub fn sub_data_type_name(
        &self,
        index: usize,
    ) -> Result<&str, DriverError> {
        self.ensure_this_is_udt()?;
        self.ensure_sub_type_index_in_range(index)?;

        get_str(|p, l| unsafe {
            cass_data_type_sub_type_name(self.inner(), index, p, l)
        })
    }

    /// Adds a sub-type to a tuple or collection.
    ///
    /// Returns an error if this data type is not a tuple or collection.
    pub fn add_sub_type<D>(&mut self, date_type: D) -> Result<(), DriverError>
    where
        D: AsRef<DriverDataType>,
    {
        self.ensure_sub_types_supported()?;

        let data_type = date_type.as_ref();
        let error = unsafe {
            cass_data_type_add_sub_type(self.inner(), data_type.inner())
        };

        to_result(error)
    }

    /// Adds a sub-type to a UDT by name.
    ///
    /// Returns an error if this is not a UDT.
    pub fn add_sub_type_by_name<S, D>(
        &mut self,
        name: S,
        data_type: D,
    ) -> Result<(), DriverError>
    where
        S: AsRef<str>,
        D: AsRef<DriverDataType>,
    {
        self.ensure_this_is_udt()?;

        let name = name.as_ref();
        let data_type = data_type.as_ref();

        set_c_str(name, |p, l| unsafe {
            cass_data_type_add_sub_type_by_name_n(
                self.inner(),
                p,
                l,
                data_type.inner(),
            )
        })
    }

    fn ensure_sub_types_supported(&self) -> Result<(), DriverError> {
        let value_type = self.value_type();
        if supports_sub_types(value_type) {
            Ok(())
        } else {
            Err(sub_types_are_not_supported(value_type))
        }
    }

    fn ensure_sub_type_index_in_range(
        &self,
        index: usize,
    ) -> Result<(), DriverError> {
        let sub_type_count = self.sub_type_count()?;
        if index >= sub_type_count {
            Err(DriverError::with_kind(DriverErrorKind::LibIndexOutOfBounds))
        } else {
            Ok(())
        }
    }

    fn ensure_this_is_udt(&self) -> Result<(), DriverError> {
        let value_type = self.value_type();
        match value_type {
            ValueType::Udt => Err(DriverError::with_message(
                DriverErrorKind::LibBadParams,
                format!(
                    "expected a user-defined type but got {:?}",
                    value_type
                ),
            )),
            _ => Ok(()),
        }
    }
}

/// Returns `true` if the given data type supports sub-types.
fn supports_sub_types(value_type: ValueType) -> bool {
    use ValueType::*;

    matches!(value_type, Udt | Tuple | List | Map | Set)
}

/// Returns an error indicating that the given data type does not support
/// sub-types.
fn sub_types_are_not_supported(value_type: ValueType) -> DriverError {
    DriverError::with_message(
        DriverErrorKind::LibBadParams,
        format!("the {:?} data type does not support sub types", value_type),
    )
}

impl Drop for DriverDataType {
    /// Frees the memory allocated for the data type object.
    fn drop(&mut self) {
        if self.owned {
            unsafe { cass_data_type_free(self.inner()) }
        }
    }
}

/// An immutable reference to a [`DataType`].
#[repr(transparent)]
pub struct DriverDataTypeRef<'a> {
    inner:     DriverDataType,
    _lifetime: PhantomData<&'a DriverDataType>,
}

impl<'a> DriverDataTypeRef<'a> {
    /// Creates a new reference to the given data type.
    pub(crate) fn new(data_type: *const struct_CassDataType_) -> Self {
        Self {
            inner:     DriverDataType::new_borrowed(data_type),
            _lifetime: PhantomData,
        }
    }

    pub(crate) fn into_inner(self) -> DriverDataType {
        self.inner
    }
}

impl<'a> Deref for DriverDataTypeRef<'a> {
    type Target = DriverDataType;

    /// Returns a reference to the inner data type object.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<DriverDataType> for DriverDataTypeRef<'_> {
    /// Returns a reference to the inner data type object.
    fn as_ref(&self) -> &DriverDataType {
        &self.inner
    }
}
