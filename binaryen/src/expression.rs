use binaryen_sys::bindings::{
    BinaryenAddInt32, BinaryenBinary, BinaryenExpressionRef, BinaryenLocalGet, BinaryenOp,
};

use crate::{module::Module, type_::Type};

pub struct Expression(BinaryenExpressionRef);

impl Expression {
    // TODO: Replace u32 by LocalId
    pub fn local_get(module: &mut Module, idx: u32, ty: Type) -> Self {
        let module = module.as_inner_mut();

        let expr_ref = unsafe { BinaryenLocalGet(module, idx, ty.into_inner()) };
        Expression(expr_ref)
    }

    pub fn binary(
        module: &mut Module,
        op: &Operation,
        left: &Expression,
        right: &Expression,
    ) -> Self {
        let module = module.as_inner_mut();
        let op = op.as_inner();
        let left = left.as_inner();
        let right = right.as_inner();

        let expr_ref = unsafe { BinaryenBinary(module, op, left, right) };
        Expression(expr_ref)
    }

    pub(crate) fn as_inner(&self) -> BinaryenExpressionRef {
        self.0
    }
}

pub struct Operation(BinaryenOp);

impl Operation {
    pub fn i32_add() -> Operation {
        let op = unsafe { BinaryenAddInt32() };
        Operation(op)
    }

    pub(crate) fn as_inner(&self) -> BinaryenOp {
        self.0
    }
}
