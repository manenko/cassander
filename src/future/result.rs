use crate::future::DriverFuture;
use crate::DriverError;

/// A trait that represents a successful result of a driver's future.
///
/// Every type that could be a successful result of a driver's future should
/// implement this trait.
pub(crate) trait DriverFutureResult: Sized {
    /// Gets the successful result of a driver's future.
    ///
    /// The method is called by the [`DriverFuture`] upon completion of the
    /// future only if the completion was successful.
    ///
    /// The extraction of the succesful result might fail, hence the method
    /// returns a [`Result`] with the successful result or a [`DriverError`].
    fn get_driver_future_result(
        future: &DriverFuture<Self>,
    ) -> Result<Self, DriverError>;
}
