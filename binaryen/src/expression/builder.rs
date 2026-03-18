use crate::module::Module;

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
}

impl Module {
    pub fn expr_builder<'module>(&'module self) -> ExpressionBuilder<'module> {
        ExpressionBuilder::new(self)
    }
}
