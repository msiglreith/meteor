
use quote::Tokens;

pub trait __ExprBlock {
    type Return;
    fn __stmnt_local(self, tokens: &mut Tokens) -> Self::Return;
}
