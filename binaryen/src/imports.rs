use std::ffi::CString;

use binaryen_sys::bindings::BinaryenAddFunctionImport;

use crate::{module::Module, type_::Type};

impl Module {
    pub fn add_function_import(
        &mut self,
        internal_name: &str,
        external_module_name: &str,
        external_base_name: &str,
        params: Type,
        results: Type,
    ) {
        let module = self.as_inner();
        let internal_name = CString::new(internal_name).expect("No 0-byte in internal name");
        let external_module_name =
            CString::new(external_module_name).expect("No 0-byte in external module name");
        let external_base_name =
            CString::new(external_base_name).expect("No 0-byte in external base name");
        let params = params.into_inner();
        let results = results.into_inner();

        // SAFETY: We have exclusive access to the module
        unsafe {
            BinaryenAddFunctionImport(
                module,
                internal_name.as_ptr(),
                external_module_name.as_ptr(),
                external_base_name.as_ptr(),
                params,
                results,
            )
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::api::Binaryen;

    use super::*;

    #[test]
    fn should_add_import_to_module() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();

        module.add_function_import(
            "f_internal",
            "external_module",
            "f_external",
            Type::none(),
            Type::none(),
        );

        assert!(module.validate());
        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }
}
