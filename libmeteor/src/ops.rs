
pub trait __Not {
    type Output;
    fn not(self) -> Self::Output;
}

pub trait __PartialEq<RHS> {
    type Output: __Not<Output=Self::Output>;

    fn eq(&self, &RHS) -> Self::Output;
    fn ne(&self, other: &RHS) -> Self::Output { self.eq(other).not() }
}

pub trait __Assign<RHS> {
    type Output;
    fn assign(self, RHS) -> Self::Output;
}

pub trait __Add<RHS> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

pub trait __Sub<RHS> {
    type Output;
    fn sub(self, rhs: RHS) -> Self::Output;
}

pub trait __Ref<'a, T> {
    type Output;
    fn __ref(&'a self) -> Self::Output;
}

pub trait __RefMut<'a, T> {
    type Output;
    fn __mut(&'a mut self) -> Self::Output;
}
