use binaryen_sys::bindings::BinaryenExpressionRef;

pub mod builder;
pub mod literal;
pub mod local;
pub mod operation;

// TODO: Expressions should have a lifetime since they're tied to a module
pub struct Expression(BinaryenExpressionRef);

impl Expression {
    pub(crate) fn from_inner(inner: BinaryenExpressionRef) -> Self {
        Self(inner)
    }

    pub(crate) fn as_inner(&self) -> BinaryenExpressionRef {
        self.0
    }
}
