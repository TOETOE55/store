mod ptr;

pub use ptr::{
    Store,
    StoreMut,
};


#[macro_export]
macro_rules! store_ref {
    ({$stored:expr}) => {{
       store::Store::new(&$stored, |s| s)
    }};
    ({$stored:expr}$($path:tt)*) => {{
       store::Store::new(&$stored, |s| &(s $($path)*))
    }}
}

#[macro_export]
macro_rules! store_mut {
    ({$stored:expr}) => {{
       store::StoreMut::new(&$stored, |s| s)
    }};
    ({$stored:expr}$($path:tt)*) => {{
        store::StoreMut::new(&mut $stored, |s| &(s $($path)*), |s| &mut (s $($path)*))
    }}
}