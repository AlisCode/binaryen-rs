//! Modules contain lists of functions, imports, exports, function types. The
//! Add* methods create them on a module. The module owns them and will free
//! their memory when the module is disposed of.
//!
//! Expressions are also allocated inside modules, and freed with the module.
//! They are not created by Add* methods, since they are not added directly on
//! the module, instead, they are arguments to other expressions (and then they
//! are the children of that AST node), or to a function (and then they are the
//! body of that function).
//!
//! A module can also contain a function table for indirect calls, a memory,
//! and a start method.
use std::{ffi::CStr, slice};

use binaryen_sys::bindings::{
    BinaryenModuleAllocateAndWrite, BinaryenModuleAllocateAndWriteResult,
    BinaryenModuleAllocateAndWriteText, BinaryenModuleCreate, BinaryenModuleDispose,
    BinaryenModulePrint, BinaryenModuleRef, BinaryenModuleValidate, BinaryenSetStart,
};

use crate::function::Function;

pub struct Module(BinaryenModuleRef);

impl Default for Module {
    fn default() -> Self {
        Module::new()
    }
}

impl Module {
    pub fn new() -> Self {
        let module_ref = unsafe { BinaryenModuleCreate() };
        Module(module_ref)
    }

    pub fn print(&mut self) {
        unsafe {
            // Safety: we have exclusive access over the module
            BinaryenModulePrint(self.0);
        }
    }

    /// Serialize a module in s-expression form
    pub fn allocate_and_write_text(&mut self) -> String {
        // Safety: we have exclusive access over the module,
        // Binaryen returns a malloc-allocated NUL-terminated buffer,
        // which we free after copying.
        //
        // Copying is fine, we mostly expect this function to be used for testing
        // purposes.
        unsafe {
            let output = BinaryenModuleAllocateAndWriteText(self.0);
            let text = CStr::from_ptr(output).to_string_lossy().into_owned();
            libc::free(output.cast());
            text
        }
    }

    /// Serializes a module into binary form, optionally including its source map if
    /// sourceMapUrl has been specified. Uses the currently set global debugInfo
    /// option. Differs from BinaryenModuleWrite in that it implicitly allocates
    /// appropriate buffers using malloc(), and expects the user to free() them
    /// manually once not needed anymore.
    ///
    /// TODO: Make a version that does not copy, if feasible.
    /// We'd need a safe variant of BinaryenModuleAllocateAndWriteResult, that calls
    /// libc::free upon dropping.
    pub fn allocate_and_write(&mut self) -> Vec<u8> {
        let module = self.as_inner();
        unsafe {
            // SAFETY: We have exlcusive access over the module
            let BinaryenModuleAllocateAndWriteResult {
                binary,
                binaryBytes,
                sourceMap,
            } = BinaryenModuleAllocateAndWrite(module, std::ptr::null());

            // SAFETY: we can only assume that whatever we got from Binaryen is valid.
            // Let's copy all those bytes to safe-land!
            let bytes = slice::from_raw_parts(binary.cast::<u8>(), binaryBytes).to_vec();
            libc::free(binary);
            libc::free(sourceMap.cast());

            bytes
        }
    }

    pub fn validate(&mut self) -> bool {
        // Safety: we have exclusive access over the module
        unsafe { BinaryenModuleValidate(self.0) }
    }

    pub fn set_start(&mut self, start: &Function) {
        let module = self.as_inner_mut();
        let start = start.as_inner();
        // Safety: we have exclusive access over the module
        unsafe { BinaryenSetStart(module, start) }
    }

    pub(crate) fn as_inner_mut(&mut self) -> BinaryenModuleRef {
        self.0
    }

    pub(crate) fn as_inner(&self) -> BinaryenModuleRef {
        self.0
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        let Module(inner) = self;
        // Safety: we have exclusive access over the module
        unsafe { BinaryenModuleDispose(*inner) }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_create_module() {
        let module = Module::default();
        drop(module);
    }

    #[test]
    fn should_allocate_and_write_text() {
        let mut module = Module::default();

        let text = module.allocate_and_write_text();
        // It's an empty module!
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_allocate_and_write_binary() {
        let mut module = Module::default();

        let _bytes = module.allocate_and_write();
    }
}
