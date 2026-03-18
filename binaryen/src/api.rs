use binaryen_sys::bindings::BinaryenSetColorsEnabled;

pub struct Binaryen;

impl Binaryen {
    pub fn set_colors_enabled(enabled: bool) {
        // SAFETY: No particular conditions to fulfill
        unsafe {
            BinaryenSetColorsEnabled(enabled);
        }
    }
}
