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
use binaryen_sys::bindings::{
    BinaryenModuleCreate, BinaryenModuleDispose, BinaryenModulePrint, BinaryenModuleRef,
    BinaryenSetStart,
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

    pub fn print(&self) {
        unsafe {
            BinaryenModulePrint(self.0);
        }
    }

    pub fn set_start(&mut self, start: &Function) {
        let module = self.as_inner_mut();
        let start = start.as_inner();
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
}
