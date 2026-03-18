use binaryen_sys::bindings::{BinaryenModuleCreate, BinaryenModuleDispose, BinaryenModuleRef};

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
