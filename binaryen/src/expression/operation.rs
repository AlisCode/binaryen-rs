use binaryen_sys::bindings::{BinaryenAddInt32, BinaryenBinary, BinaryenOp};

use crate::expression::{builder::ExpressionBuilder, Expression};

#[derive(Clone, Copy)]
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

impl<'a> ExpressionBuilder<'a> {
    pub fn binary(&self, op: Operation, left: &Expression, right: &Expression) -> Expression {
        let module = self.module.as_inner();
        let op = op.as_inner();
        let left = left.as_inner();
        let right = right.as_inner();

        let expr_ref = unsafe { BinaryenBinary(module, op, left, right) };
        Expression::from_inner(expr_ref)
    }
}
