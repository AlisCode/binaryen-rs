use std::ffi::CString;

use binaryen_sys::bindings::{
    BinaryenCall, BinaryenCallIndirect, BinaryenReturnCall, BinaryenReturnCallIndirect,
};

use crate::{
    expression::{builder::ExpressionBuilder, Expression},
    type_::Type,
};

impl<'a> ExpressionBuilder<'a> {
    pub fn call(&self, target: &str, operands: Vec<Expression>, return_type: Type) -> Expression {
        let target = CString::new(target).expect("Should not contain 0-byte");
        let mut operands: Vec<_> = operands.into_iter().map(|e| e.as_inner()).collect();
        let num_operands = operands.len();
        let return_type = return_type.into_inner();

        let expr_ref = unsafe {
            BinaryenCall(
                self.module.as_inner(),
                target.as_ptr(),
                operands.as_mut_ptr(),
                num_operands as u32,
                return_type,
            )
        };
        Expression::from_inner(expr_ref)
    }

    pub fn call_indirect(
        &self,
        table: &str,
        target: &Expression,
        operands: Vec<Expression>,
        params: Type,
        results: Type,
    ) -> Expression {
        let table = CString::new(table).expect("Should not contain 0-byte");
        let target = target.as_inner();
        let mut operands: Vec<_> = operands.into_iter().map(|e| e.as_inner()).collect();
        let num_operands = operands.len();
        let params = params.into_inner();
        let results = results.into_inner();

        let expr_ref = unsafe {
            BinaryenCallIndirect(
                self.module.as_inner(),
                table.as_ptr(),
                target,
                operands.as_mut_ptr(),
                num_operands as u32,
                params,
                results,
            )
        };
        Expression::from_inner(expr_ref)
    }

    pub fn return_call(
        &self,
        target: &str,
        operands: Vec<Expression>,
        return_type: Type,
    ) -> Expression {
        let target = CString::new(target).expect("Should not contain 0-byte");
        let mut operands: Vec<_> = operands.into_iter().map(|e| e.as_inner()).collect();
        let num_operands = operands.len();
        let return_type = return_type.into_inner();

        let expr_ref = unsafe {
            BinaryenReturnCall(
                self.module.as_inner(),
                target.as_ptr(),
                operands.as_mut_ptr(),
                num_operands as u32,
                return_type,
            )
        };
        Expression::from_inner(expr_ref)
    }

    pub fn return_call_indirect(
        &self,
        table: &str,
        target: &Expression,
        operands: Vec<Expression>,
        params: Type,
        results: Type,
    ) -> Expression {
        let table = CString::new(table).expect("Should not contain 0-byte");
        let target = target.as_inner();
        let mut operands: Vec<_> = operands.into_iter().map(|e| e.as_inner()).collect();
        let num_operands = operands.len();
        let params = params.into_inner();
        let results = results.into_inner();

        let expr_ref = unsafe {
            BinaryenReturnCallIndirect(
                self.module.as_inner(),
                table.as_ptr(),
                target,
                operands.as_mut_ptr(),
                num_operands as u32,
                params,
                results,
            )
        };
        Expression::from_inner(expr_ref)
    }
}

#[cfg(test)]
mod tests {
    use binaryen_sys::bindings::{BinaryenFeatureTailCall, BinaryenModuleSetFeatures};

    use crate::{api::Binaryen, expression::literal::Literal, module::Module, type_::Type};

    #[test]
    fn should_validate_call_expression() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();

        let callee_body = module.expr_builder().local_get(0, Type::i32());
        let _callee = module.add_function("callee", Type::i32(), Type::i32(), vec![], &callee_body);

        let body = {
            let builder = module.expr_builder();
            let operand = builder.const_(Literal::i32(0));
            builder.call("callee", vec![operand], Type::i32())
        };
        let _caller = module.add_function("caller", Type::none(), Type::i32(), vec![], &body);

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_validate_call_indirect_expression() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();

        let callee_body = module.expr_builder().local_get(0, Type::i32());
        let _callee = module.add_function("callee", Type::i32(), Type::i32(), vec![], &callee_body);

        let _table = module.add_table("table", 1, 1, Type::funcref());

        let funcs = vec!["callee"];
        let offset = module.expr_builder().const_(Literal::i32(0));
        module.add_active_element_segment("table", "elem", funcs, &offset);

        let body = {
            let builder = module.expr_builder();
            let target = builder.const_(Literal::i32(0));
            let operand = builder.local_get(0, Type::i32());
            builder.call_indirect("table", &target, vec![operand], Type::i32(), Type::i32())
        };
        let _caller = module.add_function("caller", Type::i32(), Type::i32(), vec![], &body);

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_validate_return_call_expression() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();
        unsafe { BinaryenModuleSetFeatures(module.as_inner(), BinaryenFeatureTailCall()) };

        let callee_body = module.expr_builder().local_get(0, Type::i32());
        let _callee = module.add_function("callee", Type::i32(), Type::i32(), vec![], &callee_body);

        let body = {
            let builder = module.expr_builder();
            let operand = builder.const_(Literal::i32(0));
            builder.return_call("callee", vec![operand], Type::i32())
        };
        let _caller = module.add_function("caller", Type::none(), Type::i32(), vec![], &body);

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_validate_return_call_indirect_expression() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();
        unsafe { BinaryenModuleSetFeatures(module.as_inner(), BinaryenFeatureTailCall()) };

        let callee_body = module.expr_builder().local_get(0, Type::i32());
        let _callee = module.add_function("callee", Type::i32(), Type::i32(), vec![], &callee_body);

        let _table = module.add_table("table", 1, 1, Type::funcref());

        let funcs = vec!["callee"];
        let offset = module.expr_builder().const_(Literal::i32(0));
        module.add_active_element_segment("table", "elem", funcs, &offset);

        let body = {
            let builder = module.expr_builder();
            let target = builder.const_(Literal::i32(0));
            let operand = builder.local_get(0, Type::i32());
            builder.return_call_indirect("table", &target, vec![operand], Type::i32(), Type::i32())
        };
        let _caller = module.add_function("caller", Type::i32(), Type::i32(), vec![], &body);

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }
}
