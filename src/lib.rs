// #![feature(type_alias_impl_trait)]
#![allow(unused)]

mod module {
    use prudent::unsafe_method;
    //
    const _: u8 = unsafe_method!( 1u8 =>.unchecked_add => 0 );

    //const _: u8 = unsafe_method!( {1u8} .unchecked  _add @ 0 );
    //                            literal|ident|path
}

pub const AFTER_PATH: &[char] = &['{', '[', ',', '>', '=', ':', ';', '|']; // and: "=>"

#[macro_export]
macro_rules! unsafe_fn {
    ( $fn:path | $( $arg:expr ),+ ) => {
        unsafe { /**/ $fn( $( $arg ),+ ) }
    };
    // some::path:fn_x
    ( $path:path : $fn:ident $( $arg:expr ),+ ) => {
        ({
          use $path::{$fn};
          unsafe { /* * */ $fn( $( $arg ),+ ) }
        })
    };
    // some::path::type>method_x
    /*( $path:path : $type:ident > $method:ident $( $arg:expr ),+ ) => {
        ({
          use $path::{$type};
          unsafe { /* ****** */ $type::$method( $( $arg ),+ ) }
        })
    };*/
    ( $type:ty > $method:ident $( $arg:expr ),+ ) => {
          unsafe { /* ****** */ <$type>::$method( $( $arg ),+ ) }
    };
    ( $fn:expr, $( $arg:expr ),+ ) => {
        unsafe { /* **** */$fn( $( $arg ),+ ) }
    };
    // both expressions within {}, and blocks (blocks don't need an extra inner pair of {})
    ( { $( $fn_part:tt )+ } $( $arg:expr ),+ ) => {
        unsafe { /* ****** */ ({ $( $fn_part )+ })( $( $arg ),+ ) }
    };
    // this has to be **after** alternatives where $fn:expr. Otherwise it would (surprisingly)
    // attempt to match `unsafe_fn!(crate::f, 1, 2);` and (of course) it would fail.
    ( $fn:ident $( $arg:expr ),+ ) => {
        unsafe { /* ** */ $fn( $( $arg ),+ ) }
    };
}
#[macro_export]
macro_rules! unsafe_md {
    ( $self:ident . $method:ident $( $arg:expr ),+ ) => {
        unsafe { $self.$method( $( $arg ),+ ) }
    };
    ( $self:literal . $method:ident $( $arg:expr ),+ ) => {
        unsafe { $self.$method( $( $arg ),+ ) }
    };
    // some::path:CONST_X
    ( $path:path : $self:ident . $method:ident $( $arg:expr ),+ ) => {
        ({
            use $path::{$self};
            unsafe { $self.$method( $( $arg ),+ ) }
        })
    };
    ( $self:path > $method:ident $( $arg:expr ),+ ) => {
        unsafe { $self.$method( $( $arg ),+ ) }
    };
    ( $self:expr => $method:ident $( $arg:expr ),+ ) => {
        unsafe { $self.$method( $( $arg ),+ ) }
    };
    ( { $self:expr }. $method:ident $( $arg:expr ),+ ) => {
        unsafe { $self.$method( $( $arg ),+ ) }
    };
}
pub const UNCHECKED_ADD: unsafe fn(u8, u8) -> u8 = u8::unchecked_add;
const unsafe fn f(_: u8, _: u8) {}
const _: () = {
    //unsafe { u8::{unchecked_add}(1, 2); }
};
#[rustfmt::skip]
const _FN: () = {
    use core::primitive;
    let unchecked_add = u8::unchecked_add;
    // - both functions and methods:
    // - no autocomplete!
    unsafe_fn!(                  crate::f, 1, 2);
    unsafe_fn!(         u8::unchecked_add, 1, 2);
    /*
    unsafe_fn!(              crate::f; 1, 2);
    */
    // - limited: the arguments must not use any existing `f` from outside
    // - functions only - not for methods
    unsafe_fn!(                   crate: f 1, 2);
    // NOT for methods:
    #[cfg(false)]
    unsafe_fn!(    core::u8: unchecked_add 1, 2);

    // method with a type (in scope, or with a path)
    unsafe_fn!(           u8>unchecked_add 1, 2);
    unsafe_fn!(primitive::u8>unchecked_add 1, 2);

    // - function name (identifier) in scope; not for methods
    // - autocomplete after the first letter:
    unsafe_fn!(              unchecked_add 1, 2);
    /*
    // NOT for methods:
    unsafe_fn!(      u8: unchecked_add 1, 2);
    unsafe_fn!(core::u8: unchecked_add 1, 2);
    */
    // - Accepting either fn or method:
    // - autocomplete after the second colon AND the first letter
    // - REMOVED:
    //unsafe_fn!(    u8::unchecked_add=> 1, 2);

    /*
    unsafe_fn!(    u8::unchecked_add;  1, 2);
    unsafe_fn!(    u8::unchecked_add,  1, 2);
    */
    unsafe_fn!(        {u8::unchecked_add} 1, 2);
    unsafe_fn!( {let m = u8::unchecked_add;
                                        m} 1, 2);
    // path to both functions AND methods:
    unsafe_fn!(                  crate::f| 1, 2);
    unsafe_fn!(         u8::unchecked_add| 1, 2);
};
#[rustfmt::skip]
fn _fn_pointers_non_const() {
    // autocomplete after the second colon AND the first letter
    unsafe_fn!(  crate::UNCHECKED_ADD| 1, 2);
}
const ZERO: u8 = 0;
struct Strc;
impl Strc {
    const fn method(&self, _: u8, _: u8) {}
}
const S: Strc = Strc;
#[rustfmt::skip]
const _MD: () = {
    // - receiver is a literal, or an identifier (const/variable) in scope
    // - autocomplete after the dot AND the first letter
    unsafe_md!(      1u8  .unchecked_add 1   );
    unsafe_md!(        S  .method        1, 2);

    // path::to::module :type .method
    unsafe_md!( crate: S  .method        1, 2);
    // path::to::type >method
    unsafe_md!( crate::S  >method        1, 2);

    // receiver is an expression
    unsafe_md!(      1u8 =>unchecked_add 2   );
    unsafe_md!(    *&1u8 =>unchecked_add 2   );
    unsafe_md!(      *&S =>method        1, 2);

    // - receiver is an expression. But **unlike** unsafe_fn where the function parameter may be a
    //   result of a block, here it can't be a block. Otherwise it would move, or it would require
    //   Copy and it would make an unnecessary copy (if the receiver is not by value, but by
    //   reference).
    unsafe_md!( { *&S }.method 1, 2);

};

#[macro_export]
macro_rules! expr_accept_path {
    ( $fn:ident $( $arg:expr ),+ ) => {
        //unsafe { $fn( $( $arg ),+ ) }
    };
    ($e:expr) => {};
}

expr_accept_path!(crate::f);
