
pub trait __Not<T> {
    type Output;
    fn __not(&mut self, expr: T) -> Self::Output;
}

pub trait __PartialEq<LHS, RHS> {
    type Output;

    fn __eq(&mut self, lhs: &LHS, rhs: &RHS) -> <Self as __PartialEq<LHS,RHS>>::Output;
    fn __ne(&mut self, lhs: &LHS, rhs: &RHS) -> <Self as __PartialEq<LHS,RHS>>::Output;
}

pub trait __Assign<LHS, RHS> {
    type Output;
    fn __assign(&mut self, lhs: LHS, rhs: RHS) -> Self::Output;
}

pub trait __Add<LHS, RHS> {
    type Output;
    fn __add(&mut self, lhs: LHS, rhs: RHS) -> Self::Output;
}

pub trait __Sub<LHS, RHS> {
    type Output;
    fn __sub(&mut self, lhs: LHS, rhs: RHS) -> Self::Output;
}
