use binaryen_sys::bindings::{
    BinaryenConst, BinaryenLiteral, BinaryenLiteralFloat32, BinaryenLiteralFloat32Bits,
    BinaryenLiteralFloat64, BinaryenLiteralFloat64Bits, BinaryenLiteralInt32, BinaryenLiteralInt64,
    BinaryenLiteralVec128,
};

use crate::expression::{builder::ExpressionBuilder, Expression};

pub struct Literal(BinaryenLiteral);

impl Literal {
    pub fn i32(x: i32) -> Self {
        Literal(unsafe { BinaryenLiteralInt32(x) })
    }

    pub fn i64(x: i64) -> Self {
        Literal(unsafe { BinaryenLiteralInt64(x) })
    }

    pub fn f32(x: f32) -> Self {
        Literal(unsafe { BinaryenLiteralFloat32(x) })
    }

    pub fn f64(x: f64) -> Self {
        Literal(unsafe { BinaryenLiteralFloat64(x) })
    }

    pub fn v128(x: &[u8; 16]) -> Self {
        Literal(unsafe { BinaryenLiteralVec128(x.as_ptr()) })
    }

    pub fn f32_bits(x: i32) -> Self {
        Literal(unsafe { BinaryenLiteralFloat32Bits(x) })
    }

    pub fn f64_bits(x: i64) -> Self {
        Literal(unsafe { BinaryenLiteralFloat64Bits(x) })
    }

    pub(crate) fn into_inner(self) -> BinaryenLiteral {
        self.0
    }
}

impl<'a> ExpressionBuilder<'a> {
    pub fn const_(&self, literal: Literal) -> Expression {
        let module = self.module.as_inner();
        let value = literal.into_inner();

        let expr_ref = unsafe { BinaryenConst(module, value) };
        Expression::from_inner(expr_ref)
    }
}
