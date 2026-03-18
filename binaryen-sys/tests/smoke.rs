use binaryen_sys::bindings;

#[test]
fn test_linking() {
    unsafe {
        let module = bindings::BinaryenModuleCreate();
        assert!(!module.is_null());
        bindings::BinaryenModuleDispose(module);
    }
}
