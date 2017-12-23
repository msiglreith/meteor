
pub trait __Not<T> {
    type Output;
    fn not(&mut self, expr: T) -> Self::Output;
}

pub trait __PartialEq<LHS, RHS> {
    type Output;

    fn eq(&mut self, lhs: &LHS, rhs: &RHS) -> <Self as __PartialEq<LHS,RHS>>::Output;
    fn ne(&mut self, lhs: &LHS, rhs: &RHS) -> <Self as __PartialEq<LHS,RHS>>::Output;
}

pub trait __Assign<LHS, RHS> {
    type Output;
    fn assign(&mut self, lhs: LHS, rhs: RHS) -> Self::Output;
}

pub trait __Add<LHS, RHS> {
    type Output;
    fn add(&mut self, lhs: LHS, rhs: RHS) -> Self::Output;
}

pub trait __Sub<LHS, RHS> {
    type Output;
    fn sub(&mut self, lhs: LHS, rhs: RHS) -> Self::Output;
}

pub trait __Ref<'a, T> {
    type Output;
    fn __ref(&'a self) -> Self::Output;
}

pub trait __RefMut<'a, T> {
    type Output;
    fn __mut(&'a mut self) -> Self::Output;
}
