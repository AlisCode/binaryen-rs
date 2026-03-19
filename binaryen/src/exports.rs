use std::ffi::CString;

use binaryen_sys::bindings::{BinaryenAddFunctionExport, BinaryenExportRef};

use crate::module::Module;

#[allow(dead_code)]
pub struct Export(BinaryenExportRef);

impl Export {
    #[allow(dead_code)]
    pub(crate) fn as_inner(&self) -> BinaryenExportRef {
        self.0
    }
}

impl Module {
    pub fn add_function_export(&mut self, internal_name: &str, external_name: &str) -> Export {
        let module = self.as_inner();
        let internal = CString::new(internal_name).expect("No 0-byte in internal name");
        let external = CString::new(external_name).expect("No 0-byte in internal name");

        // SAFETY: We have exclusive access to the module
        let export_ref =
            unsafe { BinaryenAddFunctionExport(module, internal.as_ptr(), external.as_ptr()) };
        Export(export_ref)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{api::Binaryen, type_::Type};

    use super::*;

    #[test]
    fn should_add_export_to_module() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();

        let body = module.expr_builder().nop();
        module.add_function("f_internal", Type::none(), Type::none(), Vec::new(), &body);

        module.add_function_export("f_internal", "f_external");

        assert!(module.validate());
        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }
}
