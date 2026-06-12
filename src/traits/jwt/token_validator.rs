use jsonwebtoken::DecodingKey;

pub trait IJwtTokenValidator {
    type Claims;
    type Error;

    fn new() -> Self;

    fn verify(&self, token: &str, decoding_key: &DecodingKey) -> Result<bool, Self::Error>;

    fn decode(&self, token: &str, decoding_key: &DecodingKey) -> Result<Self::Claims, Self::Error>;
}
