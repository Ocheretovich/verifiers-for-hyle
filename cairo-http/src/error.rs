#[derive(Debug)]
pub struct VerifierError(pub String);

impl<T: std::error::Error> From<T> for VerifierError {
    fn from(error: T) -> Self {
        VerifierError(error.to_string())
    }
}
