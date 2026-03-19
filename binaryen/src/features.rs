use binaryen_sys::bindings::{
    BinaryenFeatureMVP, BinaryenFeatureTailCall, BinaryenFeatures, BinaryenModuleGetFeatures,
    BinaryenModuleSetFeatures,
};

use crate::module::Module;

#[derive(Debug, PartialEq, Eq)]
pub struct Features(BinaryenFeatures);

impl Features {
    pub fn mvp() -> Features {
        // SAFETY: No condition to run this
        let feature = unsafe { BinaryenFeatureMVP() };
        Features(feature)
    }

    pub fn tail_call() -> Features {
        // SAFETY: No condition to run this
        let feature = unsafe { BinaryenFeatureTailCall() };
        Features(feature)
    }
}

impl Module {
    /// Gets what features are allowed when validating and in passes
    pub fn get_features(&mut self) -> Features {
        let module = self.as_inner();

        // SAFETY: We have exclusive access over the module (this might not be required?)
        let features = unsafe { BinaryenModuleGetFeatures(module) };
        Features(features)
    }

    /// Control what features are allowed when validating and in passes
    pub fn set_features(&mut self, features: Features) {
        let module = self.as_inner();
        let Features(inner) = features;

        // SAFETY: We have exclusive access over the module
        unsafe { BinaryenModuleSetFeatures(module, inner) }
    }

    /// Adds a feature to the module that is allowed when validating and in passes
    pub fn add_features(&mut self, features: Features) {
        let Features(old) = self.get_features();

        let module = self.as_inner();
        let Features(new) = features;

        // SAFETY: We have exclusive access over the module
        unsafe { BinaryenModuleSetFeatures(module, old | new) }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_enable_features() {
        let mut module = Module::new();

        // By default, only the mvp feature is enabled
        assert_eq!(module.get_features(), Features::mvp());

        module.set_features(Features::tail_call());

        // set_features replaces all features, so now only tail_call is allowed
        // (which is kinda pointless outside of testing)
        assert_eq!(module.get_features(), Features::tail_call());
    }
}
