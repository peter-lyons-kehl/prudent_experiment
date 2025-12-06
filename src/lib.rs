// #![feature(type_alias_impl_trait)]
#![allow(unused)]

/*mod module {
    use prudent::unsafe_method;
    //
    const _: u8 = unsafe_method!( 1u8 =>.unchecked_add => 0 );

    //const _: u8 = unsafe_method!( {1u8} .unchecked  _add @ 0 );
    //                            literal|ident|path
}

pub const AFTER_PATH: &[char] = &['{', '[', ',', '>', '=', ':', ';', '|']; // and: "=>"
*/

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
    ( $type_and_method:ty | $( $arg:expr ),+ ) => {
          unsafe { /* ****** */ $type_and_method( $( $arg ),+ ) }
    };
    ( $type:ty > $method:ident $( $arg:expr ),+ ) => {
          unsafe { /* ****** */ <$type>::$method( $( $arg ),+ ) }
    };
    // both expressions within {}, and blocks (blocks don't need an extra inner pair of {})
    ( { $( $fn_part:tt )+ } $( $arg:expr ),+ ) => {
        unsafe { /* ****** */ ({ $( $fn_part )+ })( $( $arg ),+ ) }
    };
    ( $fn:expr, $( $arg:expr ),+ ) => {
        unsafe { /* **** */$fn( $( $arg ),+ ) }
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
    const fn method_gen<T>(&self, _: u8, _: u8) {}
    const fn associated_fn(_: i8, _: u64) {}
    const fn associated_gen<T>(_: u8, _: u8) {}
}
const S: Strc = Strc;
#[rustfmt::skip]
const _FN: () = {
    use core::primitive;
    let unchecked_add = u8::unchecked_add;
    // - functions/methods/associated functions
    // - paths only; not specified generic qualifications of the receiver types and/or their
    //   methods/associated functions
    unsafe_fn!(                     crate::f|       1, 2);
    unsafe_fn!(            u8::unchecked_add|       1, 2);

    // - functions/methods/associated functions
    // - function/method/associated function is a result of any well-formed expression (including
    //   generic qualifications of the receiver types and/or their methods/associated functions)
    unsafe_fn!(                     crate::f,       1, 2);
    unsafe_fn!(            u8::unchecked_add,       1, 2);
    unsafe_fn!(       Strc::method_gen::<i8>, &S,   1, 2);
    unsafe_fn!(   crate::Strc::associated_fn,       1, 2);
    unsafe_fn!(   Strc::associated_gen::<()>,       1, 2);
    unsafe_fn!(   Strc::associated_gen::<u8>,       1, 2);
    unsafe_fn!(            Gen::<u8>::method, &GEN, 1, 2);

    // - paths + function name only;
    // - functions only (not for methods/associated functions)
    unsafe_fn!(                     crate: f        1, 2);

    // - qualified methods/associated functions only
    // - with a specified receiver type (the type must be in scope, or with a path)
    // - the receiver type may have generic qualifications, but
    // - the method/associated function can't have any generic qualifications specified
    unsafe_fn!(            u8  >unchecked_add        1, 2);
    unsafe_fn!( primitive::u8  >unchecked_add        1, 2);
    unsafe_fn!(        Gen<u8> >method         &GEN, 1, 2);
    unsafe_fn!(    crate::Strc >associated_fn        1, 2);

    // - qualified functions/methods/associated functions only
    // - with a specified receiver type (the type must be in scope, or with a path), and/or with
    //   specified method's/associated function's generic parameters
    unsafe_fn!(        Gen<u8>::method| &GEN, 1, 2);
    unsafe_fn!(    crate::Strc::associated_gen<()>|        1, 2);
    type _T = (Gen<u8>);
    type _U = Gen::<u8>;
    type _V = (Gen::<u8>);

    // - function name (identifier) in scope - no path; no specified generic qualifications
    // - functions; methods/associated functions only if passed in via a const/variable
    unsafe_fn!(                 unchecked_add        1, 2);

    // - function is a result of a block (its last expression; any well-formed expression)
    // - functions/methods/associated functions
    // - for methods/associated functions both the receiver type and the methods/associated
    //   functions may have specified generic qualifications
    unsafe_fn!(            {u8::unchecked_add}       1, 2);
    unsafe_fn!(  {let m = u8::unchecked_add;
                                            m}       1, 2);
    unsafe_fn!(            {<Gen<u8>>::method} &GEN, 1, 2);
    unsafe_fn!(          {Strc::associated_fn}       1, 2);
    unsafe_fn!(   {Strc::associated_gen::<u8>}       1, 2);
};

#[rustfmt::skip]
const _MD: () = {
    // - receiver passed in as a literal/identifier (const/variable), **and** in scope - no path;
    // - no specified generic qualifications
    unsafe_md!(      1u8  .unchecked_add 1   );
    unsafe_md!(        S  .method        1, 2);

    // - path::to::module :CONST_OR_STATIC
    // - no specified generic qualifications
    unsafe_md!( crate: S  .method        1, 2);

    // - path::to::CONST_OR_STATIC
    // - no specified generic qualifications
    unsafe_md!( crate::S  >method        1, 2);

    // - receiver is passed as an expression
    // - no specified generic qualifications
    unsafe_md!(      1u8 =>unchecked_add 2   );
    unsafe_md!(      *&S =>method        1, 2);

    // - receiver is an expression passed in between `{...}`. But, **unlike** unsafe_fn, here the
    //   receiver expression **cannot** be preceded by any number of statements (inside that
    //   `{...}`). The whole block must consist of exactly one expression.
    //
    // Of course, you can put any block in an extra pair of brackets `{...}`. But then the result
    // will move, or it will have to be Copy and it will be copied.
    unsafe_md!(    {*&S}  .method        1, 2);
};
