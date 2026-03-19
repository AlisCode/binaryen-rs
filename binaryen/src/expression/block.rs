use std::ffi::CString;

use binaryen_sys::bindings::BinaryenBlock;

use crate::{
    expression::{builder::ExpressionBuilder, Expression},
    type_::Type,
};

impl<'a> ExpressionBuilder<'a> {
    pub fn block(&self, name: Option<&str>, children: Vec<Expression>, type_: Type) -> Expression {
        let module = self.module.as_inner();
        let name = name.map(|s| CString::new(s).expect("No 0-byte inside of name"));
        let name_ptr = name
            .as_ref()
            .map(|cstr| cstr.as_ptr())
            .unwrap_or(std::ptr::null());
        let num_children = children.len();
        let mut children: Vec<_> = children.into_iter().map(|expr| expr.as_inner()).collect();
        let type_ = type_.into_inner();

        // SAFETY: Expression creation is thread-safe
        let expr_ref = unsafe {
            BinaryenBlock(
                module,
                name_ptr,
                children.as_mut_ptr(),
                num_children as u32,
                type_,
            )
        };
        Expression(expr_ref)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{expression::literal::Literal, module::Module};

    use super::*;

    #[test]
    fn should_create_empty_block() {
        let mut module = Module::new();

        let children = vec![];
        module.expr_builder().block(None, children, Type::none());

        assert!(module.validate());
    }

    #[test]
    fn should_create_block_with_one_child() {
        let mut module = Module::new();

        let children = vec![module.expr_builder().const_(Literal::i32(0))];
        let block = module.expr_builder().block(None, children, Type::i32());

        module.add_function("f", Type::none(), Type::i32(), vec![], &block);

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }
}
