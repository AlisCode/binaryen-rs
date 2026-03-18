use std::ffi::CString;

use binaryen_sys::bindings::{BinaryenAddFunction, BinaryenFunctionRef};

use crate::{expression::Expression, module::Module, type_::Type};

pub struct Function(BinaryenFunctionRef);

impl Function {
    fn add(
        module: &Module,
        name: &str,
        params: Type,
        results: Type,
        locals: Vec<Type>,
        body: &Expression,
    ) -> Self {
        let module = module.as_inner();
        // TODO: Don't panic. Maybe there's a safe way to do this?
        let name = CString::new(name).expect("No 0-byte allowed in function name");
        let params = params.into_inner();
        let results = results.into_inner();
        let mut locals: Vec<_> = locals.into_iter().map(Type::into_inner).collect();
        let locals_len = locals.len();
        let body = body.as_inner();

        // Safety: BinaryenAddFunction is thread-safe
        let f = unsafe {
            BinaryenAddFunction(
                module,
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
        body: &Expression,
    ) -> Function {
        Function::add(self, name, params, results, locals, body)
    }
}
