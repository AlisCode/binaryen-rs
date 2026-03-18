use binaryen_sys::bindings::BinaryenLocalGet;

use crate::{
    expression::{builder::ExpressionBuilder, Expression},
    type_::Type,
};

impl<'a> ExpressionBuilder<'a> {
    // TODO: Replace u32 by LocalId
    pub fn local_get(&self, idx: u32, ty: Type) -> Expression {
        let module = self.module.as_inner();

        let expr_ref = unsafe { BinaryenLocalGet(module, idx, ty.into_inner()) };
        Expression::from_inner(expr_ref)
    }
}
