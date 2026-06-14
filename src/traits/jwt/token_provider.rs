pub trait IJwtTokenProvider {
    type Claims: Send + Sync;
    type Error;
    type Key;

    fn generate(&self, claims: &Self::Claims, encoding_key: &Self::Key) -> Result<String, Self::Error>;
}
