
pub mod default;
pub mod rust;

pub trait Codegen {
    fn begin_scope(&mut self);
    fn end_scope(&mut self);
}
