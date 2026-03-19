use binaryen_sys::bindings::{
    BinaryenGetDebugInfo, BinaryenSetColorsEnabled, BinaryenSetDebugInfo,
};

pub struct Binaryen;

impl Binaryen {
    // Enable or disable coloring for the Wasm printer
    pub fn set_colors_enabled(enabled: bool) {
        // SAFETY: No particular conditions to fulfill
        unsafe {
            BinaryenSetColorsEnabled(enabled);
        }
    }

    // Gets whether generating debug information is currently enabled or not.
    // Applies to all modules, globally.
    pub fn get_debug_info() -> bool {
        // SAFETY: No particular conditions to fulfill
        unsafe { BinaryenGetDebugInfo() }
    }

    // Enables or disables debug information in emitted binaries.
    // Applies to all modules, globally.
    pub fn set_debug_info(enabled: bool) {
        // SAFETY: No particular conditions to fulfill
        unsafe {
            BinaryenSetDebugInfo(enabled);
        }
    }
}
