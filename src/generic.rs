
// trait Accessor<S: ?Sized> {
//     type Target: ?Sized;
//     fn access<'a>(&self, stored: &'a S) -> &'a Self::Target;
// }
//
// struct Map<F, Acc> {
//     f: F,
//     accessor: Acc,
// }
//
// impl<S: ?Sized, A: ?Sized, B: ?Sized, F, Acc> Map<F, Acc> for Accessor<>{
//

use std::ops::Deref;

pub struct Store<'a, S: ?Sized, Acc, A: ?Sized + 'a>
where
    Acc: Fn(&'a S) -> &'a A,
{
    stored: &'a S,
    accessor: Acc,
}

impl<'a, S: ?Sized, Acc, A: ?Sized + 'a> Store<'a, S, Acc, A>
where
    Acc: Fn(&'a S) -> &'a A,
{
    pub fn new(stored: &'a S, accessor: Acc) -> Self {
        Self { stored, accessor }
    }

    pub fn pos(&self) -> &S {
        self.stored
    }

    pub fn peek(&self, s: &'a S) -> &'a A {
        (self.accessor)(s)
    }

    pub fn map<F, B: 'a>(self, f: F) -> Store<'a, S, impl Fn(&'a S) -> &'a B, B>
    where
        F: Fn(&A) -> &B
    {

        Store {
            stored: self.stored,
            accessor: move |s| f((self.accessor)(s)),
        }
    }

}

impl<'a, S: ?Sized, Acc, A: ?Sized + 'a> Deref for Store<'a, S, Acc, A>
where
    Acc: Fn(&'a S) -> &'a A
{
    type Target = A;

    fn deref(&self) -> &Self::Target {
        (self.accessor)(self.stored)
    }
}
