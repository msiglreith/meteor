
pub trait __ExprBlock<T> {
    fn __stmnt_local(&mut self, expr: T) -> T;
}
