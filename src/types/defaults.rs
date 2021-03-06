use std::fmt::{Display};

use super::abc::*;


// --- [ default implementations ] ------------------------------------------------------------------------------------

impl <T> ToIString for T where T: Type {
    default fn to_istring(&self) -> Option<String> { None }
}

impl <T> ToIString for T where T: Type + Display {
    default fn to_istring(&self) -> Option<String> { Some( ToString::to_string(self)) }
}

impl <T> ToINumeric for T where T: Type {
    default fn to_real(&self) -> Option<f64> { None }
    default fn to_int(&self) -> Option<isize> { None }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl <S> IArithm for S where S: Type {
    default fn iadd(self: Box<Self>, _other: BType) -> Option<BType> { None }
    default fn isub(self: Box<Self>, _other: BType) -> Option<BType> { None }
    default fn imul(self: Box<Self>, _other: BType) -> Option<BType> { None }
    default fn idiv(self: Box<Self>, _other: BType) -> Option<BType> { None }
}


impl <T> AsIterable for T where T: Type {
    default fn as_iterable(&self) -> Option<&IIterable> { None }
}

impl <T> AsComposable for T where T: Type {
    default fn as_composable(&self) -> Option<&IComposable> { None }
}

//impl <'a, T> AsComposable for T where T: Type + IComposable<'a> {
//    default fn as_composable(&self) -> Option<&IComposable> { Some(self) }
//}

impl <T> AsInvocable for T where T: Type {
    default fn as_invocable(&self) -> Option<&IInvocable> { None }
}

