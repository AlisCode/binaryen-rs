pub use binaryen_sys::bindings::BinaryenType;
use binaryen_sys::bindings::{
    BinaryenTypeArity, BinaryenTypeAuto, BinaryenTypeCreate, BinaryenTypeFuncref,
    BinaryenTypeInt32, BinaryenTypeNone,
};

#[derive(Clone, Copy)]
pub struct Type(BinaryenType);

impl Type {
    pub fn none() -> Self {
        Type(unsafe { BinaryenTypeNone() })
    }

    pub fn i32() -> Self {
        Type(unsafe { BinaryenTypeInt32() })
    }

    pub fn funcref() -> Self {
        Type(unsafe { BinaryenTypeFuncref() })
    }

    pub fn auto() -> Self {
        Type(unsafe { BinaryenTypeAuto() })
    }

    pub fn create(types: Vec<Type>) -> Self {
        let mut raw_value_types: Vec<BinaryenType> =
            types.into_iter().map(Type::into_inner).collect();
        let len = raw_value_types.len();

        let ty = unsafe { BinaryenTypeCreate(raw_value_types.as_mut_ptr(), len as u32) };
        Type(ty)
    }

    pub fn arity(&self) -> u32 {
        unsafe { BinaryenTypeArity(self.0) }
    }

    pub(crate) fn into_inner(self) -> BinaryenType {
        self.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn type_arity() {
        let ty = Type::create(vec![Type::i32(), Type::i32()]);
        assert_eq!(ty.arity(), 2);
    }
}
