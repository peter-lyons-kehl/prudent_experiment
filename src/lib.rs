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
          use $path::{$fn as prudent_unique_local_function_alias};
          unsafe { /* * */ prudent_unique_local_function_alias( $( $arg ),+ ) }
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
const unsafe fn f(_: u8, _: u8) {}
struct Gen<T> {
    t: T,
}
impl<T> Gen<T> {
    const unsafe fn method(&self, _: u16, _: u16) {}
}
const GEN: Gen<u8> = Gen { t: 0 };
struct Strc;
impl Strc {
    const fn method(&self, _: u8, _: u8) {}
    const fn associated_fn(_: i8, _: u64) {}
}
#[rustfmt::skip]
const _FN: () = {
    use core::primitive;
    let unchecked_add = u8::unchecked_add;
    // - functions, methods, associated functions
    // - paths only; not generic qualifications of methods/associated functions
    unsafe_fn!(                     crate::f|      1, 2);
    unsafe_fn!(            u8::unchecked_add|      1, 2);
    // - Same, but a comma instead of the pipe. TODO Remove?
    unsafe_fn!(                     crate::f,      1, 2);
    unsafe_fn!(            u8::unchecked_add,      1, 2);
    unsafe_fn!(          Strc::associated_fn,      1, 2);
    unsafe_fn!(   crate::Strc::associated_fn,      1, 2);
    // - functions only (not for methods/associated functions)
    unsafe_fn!(                          crate: f  1, 2);

    // qualified methods and associated functions only, with a type (the type is in scope, or with a
    // path)
    unsafe_fn!(           u8  >unchecked_add       1, 2);
    unsafe_fn!(primitive::u8  >unchecked_add       1, 2);
    unsafe_fn!(       Gen<u8> >method        &GEN, 1, 2);
    unsafe_fn!(crate::Gen<u8> >method        &GEN, 1, 2);
    unsafe_fn!(          Strc >associated_fn       1, 2);
    unsafe_fn!(   crate::Strc >associated_fn       1, 2);

    // - function name (identifier) in scope
    // - functions; methods/associated functions only if passed stored in a const/variable
    unsafe_fn!(                unchecked_add       1, 2);

    // - function is a result of a block
    // - both functions and methods
    unsafe_fn!(           {u8::unchecked_add}      1, 2);
    unsafe_fn!( {let m = u8::unchecked_add;
                                           m}      1, 2);
};
const S: Strc = Strc;
#[rustfmt::skip]
const _MD: () = {
    // - receiver passed in as a literal, or an identifier (const/variable), **and** in scope
    unsafe_md!(      1u8  .unchecked_add 1   );
    unsafe_md!(        S  .method        1, 2);

    // path::to::module :CONST_OR_STATIC
    unsafe_md!( crate: S  .method        1, 2);

    // path::to::CONST_OR_STATIC
    unsafe_md!( crate::S  >method        1, 2);

    // receiver is passed as an expression
    unsafe_md!(      1u8 =>unchecked_add 2   );
    unsafe_md!(      *&S =>method        1, 2);

    // - receiver is an expression passed in between `{...}`. But, **unlike** unsafe_fn, here the
    //   receiver expression **cannot** be preceded by any number of statements (inside that
    //   `{...}`).
    //
    // Of course, you can put the expression in an extra pair of brackets `{...}`. But then the
    // result will move, or it will have to be Copy and it will be copied.
    unsafe_md!(    {*&S}  .method        1, 2);
};

#[macro_export]
macro_rules! expr_accept_path {
    ( $fn:ident $( $arg:expr ),+ ) => {
        //unsafe { $fn( $( $arg ),+ ) }
    };
    ($e:expr) => {};
}

expr_accept_path!(crate::f);
