use std::ffi::CString;

use binaryen_sys::bindings::{BinaryenAddFunction, BinaryenFunctionRef};

use crate::{expression::Expression, module::Module, type_::Type};

pub struct Function(BinaryenFunctionRef);

impl Function {
    pub fn add(
        module: &mut Module,
        name: &str,
        params: Type,
        results: Type,
        body: &Expression,
    ) -> Self {
        let module = module.as_inner_mut();
        // TODO: Don't panic. Maybe there's a safe way to do this?
        let name = CString::new(name).expect("No 0-byte allowed in function name");
        let params = params.into_inner();
        let results = results.into_inner();
        let body = body.as_inner();

        let f = unsafe {
            BinaryenAddFunction(
                module,
                name.as_ptr(),
                params,
                results,
                std::ptr::null_mut(),
                0,
                body,
            )
        };
        Function(f)
    }

    pub(crate) fn as_inner(&self) -> BinaryenFunctionRef {
        self.0
    }
}
