use binaryen_sys::bindings::{BinaryenLocalGet, BinaryenLocalSet};

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

    // TODO: Replace u32 by LocalId
    pub fn local_set(&self, idx: u32, value: Expression) -> Expression {
        let module = self.module.as_inner();
        let expr_ref = value.as_inner();

        let expr_ref = unsafe { BinaryenLocalSet(module, idx, expr_ref) };
        Expression::from_inner(expr_ref)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{expression::literal::Literal, module::Module};

    use super::*;

    #[test]
    fn should_add_local_get_expr() {
        let mut module = Module::new();

        let body = module.expr_builder().local_get(0, Type::i32());
        let _f = module.add_function("f", Type::i32(), Type::i32(), vec![], Some(&body));

        assert!(module.validate());
        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_add_local_set_expr() {
        let mut module = Module::new();

        let const_zero = module.expr_builder().const_(Literal::i32(0));
        let body = module.expr_builder().local_set(0, const_zero);
        let _f = module.add_function(
            "f",
            Type::none(),
            Type::none(),
            vec![Type::i32()],
            Some(&body),
        );

        assert!(module.validate());
        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }
}
