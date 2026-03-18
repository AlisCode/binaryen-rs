pub use binaryen_sys::bindings::BinaryenType;
use binaryen_sys::bindings::{BinaryenTypeCreate, BinaryenTypeInt32, BinaryenTypeNone};

pub struct Type(BinaryenType);

impl Type {
    pub fn none() -> Self {
        Type(unsafe { BinaryenTypeNone() })
    }

    pub fn i32() -> Self {
        Type(unsafe { BinaryenTypeInt32() })
    }

    pub(crate) fn into_inner(self) -> BinaryenType {
        self.0
    }
}

#[derive(Default)]
pub struct Types(Vec<Type>);

impl Types {
    pub fn create(&mut self, value_types: Vec<Type>) -> Type {
        let idx = self.0.len();
        let mut raw_value_types: Vec<BinaryenType> =
            value_types.into_iter().map(Type::into_inner).collect();
        let ty = unsafe { BinaryenTypeCreate(raw_value_types.as_mut_ptr(), idx as u32) };

        Type(ty)
    }
}
