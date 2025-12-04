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
    ( $path:path : $type:ident > $method:ident $( $arg:expr ),+ ) => {
        ({
          use $path::{$type};
          unsafe { /* ****** */ $type::$method( $( $arg ),+ ) }
        })
    };
    ( $fn:expr => $( $arg:expr ),+ ) => {
        unsafe { /* *** */$fn( $( $arg ),+ ) }
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
    ( $self:path >. $method:ident $( $arg:expr ),+ ) => {
        unsafe { $self.$method( $( $arg ),+ ) }
    };
    ( $self:expr =>. $method:ident $( $arg:expr ),+ ) => {
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
    let unchecked_add = u8::unchecked_add;
    // - no autocomplete!
    // - both functions and methods:
    unsafe_fn!(              crate::f, 1, 2);
    unsafe_fn!(     u8::unchecked_add, 1, 2);
    /*
    unsafe_fn!(              crate::f; 1, 2);
    */

    // - limited: the arguments must not use any existing `f` from outside
    // - functions only - not for methods
    unsafe_fn!(              crate: f  1, 2);
    // no:
    //
    //unsafe_fn!(     core::u8:unchecked_add 1, 2);
    unsafe_fn!(              core:u8>unchecked_add  1, 2);

    /**/
    // autocomplete after the first leter:
    unsafe_fn!(          unchecked_add 1, 2);
    /*
    // NOT for methods:
    unsafe_fn!(      u8: unchecked_add 1, 2);
    unsafe_fn!(core::u8: unchecked_add 1, 2);
    */
    // Accepting either fn or method:
    //
    // autocomplete after the second colon AND the first letter
    unsafe_fn!(    u8::unchecked_add=> 1, 2);
    /*
    unsafe_fn!(    u8::unchecked_add;  1, 2);
    unsafe_fn!(    u8::unchecked_add,  1, 2);
    */
    unsafe_fn!(   {u8::unchecked_add}  1, 2);
    unsafe_fn!(   {let _=0;
                   u8::unchecked_add}  1, 2);
    // both functions AND methods:
    unsafe_fn!(              crate::f| 1, 2);
    unsafe_fn!(     u8::unchecked_add| 1, 2);
};
#[rustfmt::skip]
fn _fn_pointers_non_const() {
    // autocomplete after the second colon AND the first letter
    unsafe_fn!(  crate::UNCHECKED_ADD| 1, 2);
}
const ZERO: u8 = 0;
struct Struct;
impl Struct {
    const fn method(&self, _: u8, _: u8) {}
}
const S: Struct = Struct;
#[rustfmt::skip]
const _MD: () = {
    // autocomplete after the dot AND the first letter
    unsafe_md!(      1u8.unchecked_add    1   );
    unsafe_md!(        S.method           1, 2);

    // autocomplete after the dot AND the first letter
    unsafe_md!( crate: S   .method        1, 2);
    unsafe_md!( crate::S  >.method        1, 2);
    unsafe_md!( 0u8+1    =>.unchecked_add 1   );
    // autocomplete after the first letter; sometimes only after several letters!!
    unsafe_md!(      { S } .method        1, 2);
};

#[macro_export]
macro_rules! expr_accept_path {
    ( $fn:ident $( $arg:expr ),+ ) => {
        //unsafe { $fn( $( $arg ),+ ) }
    };
    ($e:expr) => {};
}

expr_accept_path!(crate::f);
