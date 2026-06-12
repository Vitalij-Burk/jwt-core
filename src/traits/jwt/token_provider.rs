use jsonwebtoken::EncodingKey;

pub trait IJwtTokenProvider {
    type Claims: Send + Sync;
    type Error;

    fn generate(&self, claims: &Self::Claims, encoding_key: &EncodingKey) -> Result<String, Self::Error>;
}
