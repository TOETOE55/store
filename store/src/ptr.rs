use std::ops::{Deref, DerefMut};

/// The share pointer to A can refer to its parent S
pub struct Store<'a, S: ?Sized, A: ?Sized>
{
    stored: &'a S,
    accessor: fn(&S) -> &A,
}

/// The mutable pointer to A can refer to its parent S
pub struct StoreMut<'a, S: ?Sized, A: ?Sized>
{
    stored: &'a mut S,
    accessor: fn(&S) -> &A,
    accessor_mut: fn(&mut S) -> &mut A,
}

impl<'a, S: ?Sized, A: ?Sized> Store<'a, S, A> {
    pub fn new(stored: &'a S, accessor: fn(&S) -> &A) -> Self {
        Self { stored, accessor }
    }

    pub fn pos(&self) -> &S {
        self.stored
    }

    pub fn peek<'s>(&self, s: &'s S) -> &'s A {
        (self.accessor)(s)
    }
}


impl<'a, S: ?Sized, A: ?Sized> StoreMut<'a, S, A> {
    pub fn new(stored: &'a mut S, accessor: fn(&S) -> &A, accessor_mut: fn(&mut S) -> &mut A) -> Self {
        Self { stored, accessor, accessor_mut }
    }

    pub fn pos(&self) -> &S {
        self.stored
    }

    pub fn pos_mut(&mut self) -> &mut S {
        self.stored
    }

    pub fn peek<'s>(&self, s: &'s S) -> &'s A {
        (self.accessor)(s)
    }

    pub fn peek_mut<'s>(&self, s: &'s mut S) -> &'s mut A {
        (self.accessor_mut)(s)
    }
}


impl<'a, S: ?Sized, A: ?Sized> Clone for Store<'a, S, A> {
    fn clone(&self) -> Self {
        Self {
            stored: self.stored,
            accessor: self.accessor,
        }
    }
}

impl<'a, S: ?Sized, A: ?Sized> Copy for Store<'a, S, A> {}

impl<'a, S: ?Sized, A: ?Sized> Deref for Store<'a, S, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        (self.accessor)(self.stored)
    }
}

impl<'a, S: ?Sized, A: ?Sized> Deref for StoreMut<'a, S, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        (self.accessor)(self.stored)
    }
}

impl<'a, S: ?Sized, A: ?Sized> DerefMut for StoreMut<'a, S, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (self.accessor_mut)(self.stored)
    }
}



