pub trait CassCustomAuthenticator {
    /// The type of error that can occur when performing authentication.
    type Error;

    /// Called to set the initial response token for the authenticator.
    ///
    /// This is used to set the initial response token for the authenticator.
    /// The initial response token is used to start the authentication process.
    fn start_authentication(&mut self) -> Result<Vec<u8>, Self::Error>;

    /// Called to evaluate a challenge token for the authenticator.
    ///
    /// This is used to evaluate a challenge token for the authenticator.
    /// The challenge token is used to continue the authentication process.
    fn perform_challenge(
        &mut self,
        token: &[u8],
    ) -> Result<Vec<u8>, Self::Error>;

    fn finish_authentication(
        &mut self,
        token: &[u8],
    ) -> Result<(), Self::Error>;
}
