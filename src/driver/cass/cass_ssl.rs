use std::ffi::c_char;

use crate::driver::cass::{
    CassError,
    CassSslVerifyFlags,
};
use crate::driver::ffi::{
    cass_ssl_free,
    cass_ssl_new,
    cass_ssl_new_no_lib_init,
    cass_ssl_set_cert_n,
    cass_ssl_set_private_key_n,
    cass_ssl_set_verify_flags,
    struct_CassSsl_,
};

// TODO: Check if `cass_ssl_new` and `cass_ssl_new_no_lib_init` return NULL
//       pointers in case of an error.

/// Describes the SSL configuration of a cluster.
pub struct CassSsl(*mut struct_CassSsl_);

impl CassSsl {
    /// Creates a new SSL configuration.
    pub fn new() -> Self {
        unsafe { CassSsl(cass_ssl_new()) }
    }

    /// Creates a new SSL context without initializing the underlying library
    /// implementation.
    ///
    /// <div class="warning">
    /// The integrating application is responsible for initializing the
    /// underlying SSL implementation.The driver uses the SSL implmentation from
    /// several threads concurrently so it’s important that it’s properly setup
    /// for multithreaded use e.g. lock callbacks for OpenSSL.
    ///
    /// The SSL library must be initialized before calling this function. When
    /// using OpenSSL the following components need to be initialized:
    ///
    /// - [`SSL_library_init`](https://linux.die.net/man/3/ssl_library_init)
    /// - [`SSL_load_error_strings`](https://linux.die.net/man/3/ssl_load_error_strings)
    /// - [`OpenSSL_add_all_algorithms`](https://linux.die.net/man/3/openssl_add_all_algorithms)
    /// - [`CRYPTO_set_locking_callback`](https://linux.die.net/man/3/crypto_set_locking_callback)
    /// - [`CRYPTO_set_id_callback`](https://linux.die.net/man/3/crypto_set_id_callback)
    /// </div>
    pub fn new_no_lib_init() -> Self {
        unsafe { CassSsl(cass_ssl_new_no_lib_init()) }
    }

    /// Returns a raw pointer to the SSL configuration.
    pub fn as_raw(&self) -> *mut struct_CassSsl_ {
        self.0
    }

    /// Adds a trusted certificate in the PEM format.
    ///
    /// This is used to verify the peer's certificate.
    pub fn add_trusted_cert<T>(&mut self, cert: T) -> CassError
    where
        T: AsRef<str>,
    {
        let cert = cert.as_ref();
        let len = cert.len();
        let ptr = cert.as_ptr() as *const c_char;
        unsafe { cass_ssl_set_cert_n(self.as_raw(), ptr, len) }.into()
    }

    /// Sets the flags used to verify the peer's certificate.
    pub fn set_verify_flags(&mut self, flags: CassSslVerifyFlags) -> CassError {
        if let Ok(flags) = u32::from(flags).try_into() {
            unsafe { cass_ssl_set_verify_flags(self.as_raw(), flags) };
            CassError::Ok
        } else {
            // Should never happen, though.
            CassError::LibBadParams
        }
    }

    /// Sets the client-side certificate chain in the PEM format.
    ///
    /// This is used to authenticate the client on the server-side. This should
    /// contain the entire certificate chain starting with the certificate
    /// itself.
    pub fn set_cert<T>(&mut self, cert: T) -> CassError
    where
        T: AsRef<str>,
    {
        let cert = cert.as_ref();
        let len = cert.len();
        let ptr = cert.as_ptr() as *const c_char;
        unsafe { cass_ssl_set_cert_n(self.as_raw(), ptr, len) }.into()
    }

    /// Sets the client-side private key in the PEM format.
    ///
    /// This is used to authenticate the client on the server-side.
    pub fn set_private_key<A, B>(&mut self, key: A, password: B) -> CassError
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let key = key.as_ref();
        let key_len = key.len();
        let key_ptr = key.as_ptr() as *const c_char;

        let password = password.as_ref();
        let password_len = password.len();
        let password_ptr = password.as_ptr() as *const c_char;

        unsafe {
            cass_ssl_set_private_key_n(
                self.as_raw(),
                key_ptr,
                key_len,
                password_ptr,
                password_len,
            )
        }
        .into()
    }
}

impl Default for CassSsl {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for CassSsl {}
unsafe impl Sync for CassSsl {}

impl Drop for CassSsl {
    fn drop(&mut self) {
        unsafe { cass_ssl_free(self.as_raw()) }
    }
}
