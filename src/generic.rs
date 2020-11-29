
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

// fn coerce<A: ?Sized, B:?Sized, F: for<'a> Fn(&'a A) -> &'a B>(f: F) -> F { f }

use std::ops::Deref;
use std::marker::PhantomData;

pub struct Store<'a, S: ?Sized, Acc, A: ?Sized> {
    stored: &'a S,
    accessor: Acc,
    _marker: PhantomData<&'a A>
}

impl<'a, S: ?Sized, Acc, A: ?Sized> Store<'a, S, Acc, A>
where
    Acc: Fn(&'a S) -> &'a A,
{
    pub fn new(stored: &'a S, accessor: Acc) -> Self {
        Self { stored, accessor, _marker: PhantomData }
    }

    pub fn pos(&self) -> &S {
        self.stored
    }

    pub fn peek(&self, s: &'a S) -> &'a A {
        (self.accessor)(s)
    }

    pub fn map<F, B>(self, f: F) -> Store<'a, S, impl Fn(&'a S) -> &'a B , B>
    where
        F: Fn(&'a A) -> &'a B
    {

        Store {
            stored: self.stored,
            accessor: move |s| f((self.accessor)(s)),
            _marker: PhantomData
        }
    }

}

impl<'a, S: ?Sized, Acc, A: ?Sized> Deref for Store<'a, S, Acc, A>
where
    Acc: Fn(&S) -> &A
{
    type Target = A;

    fn deref(&self) -> &Self::Target {
        (self.accessor)(self.stored)
    }
}

impl<'a, S: ?Sized, Acc: Clone, A: ?Sized> Clone for Store<'a, S, Acc, A> {
    fn clone(&self) -> Self {
        Self {
            stored: self.stored,
            accessor: self.accessor.clone(),
            _marker: PhantomData
        }
    }
}

impl<'a, S: ?Sized, Acc: Copy, A: ?Sized> Copy for Store<'a, S, Acc, A> { }

