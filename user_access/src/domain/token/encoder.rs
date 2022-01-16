use crate::domain::token::{Token, TokenId};
use common::Result;
pub trait TokenEncoderExt: Send + Sync {
    fn encode(&self, token_id: &TokenId) -> Result<Token>;
    fn decode(&self, token: &Token) -> Result<TokenId>;
}
