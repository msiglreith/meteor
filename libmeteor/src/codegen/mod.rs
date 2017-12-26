
use quote::Tokens;

pub mod generic;
pub mod rust;

pub trait __Codegen {
    fn begin_scope(&mut self);
    fn end_scope(&mut self);
}

pub trait Tokenize {
    fn tokens(self) -> Tokens;
}
