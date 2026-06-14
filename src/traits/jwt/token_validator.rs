pub trait IJwtTokenValidator {
    type Claims;
    type Error;
    type Key;

    fn new() -> Self;

    fn verify(&self, token: &str, decoding_key: &Self::Key) -> Result<bool, Self::Error>;

    fn decode(&self, token: &str, decoding_key: &Self::Key) -> Result<Self::Claims, Self::Error>;
}
