
pub trait __Not {
    type Output;
    fn not(self) -> Self::Output;
}

pub trait __PartialEq<RHS> {
    type Output: __Not<Output=Self::Output>;

    fn eq(&self, &RHS) -> Self::Output;
    fn ne(&self, other: &RHS) -> Self::Output { self.eq(other).not() }
}

pub trait __Assign<RHS, Ret> {
    fn assign(self, RHS) -> Ret;
}

pub trait __Ref<'a, T, Ret> {
    fn __ref(&'a self) -> Ret;
}

pub trait __RefMut<'a, T, Ret> {
    fn __mut(&'a mut self) -> Ret;
}
