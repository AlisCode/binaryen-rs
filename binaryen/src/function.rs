use std::ffi::{CStr, CString};

use binaryen_sys::bindings::{
    BinaryenAddFunction, BinaryenFunctionGetName, BinaryenFunctionRef, BinaryenFunctionSetBody,
};

use crate::{expression::Expression, module::Module, type_::Type};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Function(BinaryenFunctionRef);

impl Function {
    pub(crate) fn add(
        module: &Module,
        name: &str,
        params: Type,
        results: Type,
        locals: Vec<Type>,
        body: Option<&Expression>,
    ) -> Self {
        let module_ref = module.as_inner();
        // TODO: Don't panic. Maybe there's a safe way to do this?
        let name = CString::new(name).expect("No 0-byte allowed in function name");
        let params = params.into_inner();
        let results = results.into_inner();
        let mut locals: Vec<_> = locals.into_iter().map(Type::into_inner).collect();
        let locals_len = locals.len();
        let body = body.unwrap_or(&module.expr_builder().nop()).as_inner();

        // SAFETY: BinaryenAddFunction is thread-safe
        let f = unsafe {
            BinaryenAddFunction(
                module_ref,
                name.as_ptr(),
                params,
                results,
                locals.as_mut_ptr(),
                locals_len as u32,
                body,
            )
        };
        Function(f)
    }

    pub fn set_body(&mut self, expr: &Expression) {
        let func = self.as_inner();
        // SAFETY: we have exclusive access to the function
        unsafe { BinaryenFunctionSetBody(func, expr.as_inner()) }
    }

    // TODO: Reasses the safety of this.
    // Maybe this should be fallible?
    pub fn get_name(&self) -> &str {
        let func = self.as_inner();

        // SAFETY: we're returning a pointer as a CStr
        let name = unsafe {
            let name = BinaryenFunctionGetName(func);
            CStr::from_ptr(name)
        };
        name.to_str().expect("Invalid")
    }

    pub(crate) fn as_inner(&self) -> BinaryenFunctionRef {
        self.0
    }
}

impl Module {
    pub fn add_function(
        &self,
        name: &str,
        params: Type,
        results: Type,
        locals: Vec<Type>,
        body: Option<&Expression>,
    ) -> Function {
        Function::add(self, name, params, results, locals, body)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::expression::literal::Literal;

    use super::*;

    #[test]
    fn should_add_empty_function() {
        let mut module = Module::new();

        module.add_function("f", Type::none(), Type::none(), vec![], None);

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_add_function_with_body() {
        let mut module = Module::new();

        let body = module.expr_builder().const_(Literal::i32(0));
        module.add_function("f", Type::none(), Type::i32(), vec![], Some(&body));

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_get_function_name() {
        let mut module = Module::new();

        let func = module.add_function("f", Type::none(), Type::none(), vec![], None);

        assert!(module.validate());

        let fname = func.get_name();
        assert_eq!(fname, "f");
    }
}
