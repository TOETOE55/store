mod ptr;
mod generic;

pub use ptr::{
    Store,
    StoreMut,
};


#[macro_export]
macro_rules! store_ref {
    {$stored:expr, self$(. $attr:tt)*} => {
       ::store::Store::new(&$stored, |s| &s$(. $attr)*)
    }
}

#[macro_export]
macro_rules! store_mut {
    {$stored:expr, self$(. $attr:tt)*} => {
        ::store::StoreMut::new(&mut $stored, |s| &s$(. $attr)*, |s| &mut s$(. $attr)*)
    }
}