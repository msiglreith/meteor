

pub trait __Not {
    type Output;
    fn not(self) -> Self::Output;
}

pub trait __PartialEq<RHS, Ret>
where
    Ret: __Not<Output=Ret>,
{
    fn eq(&self, RHS) -> Ret;
    fn ne(&self, other: RHS) -> Ret { self.eq(other).not() }
}

pub trait __Assign<RHS> {
    fn assign(self, RHS) -> Stmt;
}

pub trait __Ref<T> {
    fn __ref<'a>(&'a self) -> Expr<&'a T>;
}

pub trait __RefMut<T> {
    fn __mut<'a>(&'a mut self) -> Expr<&'a mut T>;
}
