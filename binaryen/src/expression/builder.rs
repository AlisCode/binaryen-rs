use binaryen_sys::bindings::BinaryenNop;

use crate::{expression::Expression, module::Module};

pub struct ExpressionBuilder<'a> {
    pub(crate) module: &'a Module,
}

// SAFETY: Binaryen documents expression creation as thread-safe.
unsafe impl<'a> Send for ExpressionBuilder<'a> {}
// SAFETY: Binaryen documents expression creation as thread-safe.
unsafe impl<'a> Sync for ExpressionBuilder<'a> {}

impl<'a> ExpressionBuilder<'a> {
    pub(crate) fn new(module: &'a Module) -> Self {
        Self { module }
    }

    pub fn nop(&self) -> Expression {
        // SAFETY: Creating an expression is thread-safe
        let expr_ref = unsafe { BinaryenNop(self.module.as_inner()) };
        Expression(expr_ref)
    }
}

impl Module {
    pub fn expr_builder<'module>(&'module self) -> ExpressionBuilder<'module> {
        ExpressionBuilder::new(self)
    }
}
