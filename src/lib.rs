mod ptr;
pub mod generic;

pub use ptr::{
    Store,
    StoreMut,
};

use replace::replace;

#[macro_export]
macro_rules! store_ref {
    {$stored:expr, $($path:tt)*} => {{
        use replace::replace;
       ::store::Store::new(&$stored, |s| &replace!(s, self, $($path)*))
    }}
}

#[macro_export]
macro_rules! store_mut {
    {$stored:expr, $($path:tt)*} => {{
        use replace::replace;
        ::store::StoreMut::new(&mut $stored, |s| &replace!(s, self, $($path)*), |s| &mut replace!(s, self, $($path)*))
    }}
}