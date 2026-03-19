use binaryen_sys::bindings::{
    BinaryenFeatures, BinaryenModuleGetFeatures, BinaryenModuleSetFeatures,
};

use crate::module::Module;
use crate::utils::binaryen_enum;

// TODO: Find a way to ensure that these always stay in sync
// with the binaryen-c.h header, otherwise we might be missing some

binaryen_enum! {
    Binaryen(BinaryenFeatures):
    pub enum Feature {
        MVP => BinaryenFeatureMVP,
        Atomics => BinaryenFeatureAtomics,
        MutableGlobals => BinaryenFeatureMutableGlobals,
        NontrappingFPToInt => BinaryenFeatureNontrappingFPToInt,
        SIMD128 => BinaryenFeatureSIMD128,
        BulkMemory => BinaryenFeatureBulkMemory,
        SignExt => BinaryenFeatureSignExt,
        ExceptionHandling => BinaryenFeatureExceptionHandling,
        TailCall => BinaryenFeatureTailCall,
        ReferenceTypes => BinaryenFeatureReferenceTypes,
        Multivalue => BinaryenFeatureMultivalue,
        GC => BinaryenFeatureGC,
        Memory64 => BinaryenFeatureMemory64,
        RelaxedSIMD => BinaryenFeatureRelaxedSIMD,
        ExtendedConst => BinaryenFeatureExtendedConst,
        Strings => BinaryenFeatureStrings,
        MultiMemory => BinaryenFeatureMultiMemory,
        StackSwitching => BinaryenFeatureStackSwitching,
        SharedEverything => BinaryenFeatureSharedEverything,
        FP16 => BinaryenFeatureFP16,
        BulkMemoryOpt => BinaryenFeatureBulkMemoryOpt,
        CallIndirectOverlong => BinaryenFeatureCallIndirectOverlong,
        RelaxedAtomics => BinaryenFeatureRelaxedAtomics,
        Multibyte => BinaryenFeatureMultibyte,
        CustomPageSizes => BinaryenFeatureCustomPageSizes,
        All => BinaryenFeatureAll,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The set of features enabled on a module.
/// Refer to [`Feature`] for the typed list of values
pub struct Features(BinaryenFeatures);

impl Features {
    pub fn contains(&self, feature: Feature) -> bool {
        let Features(set) = self;
        let check = feature.to_raw();
        (set & check) == check
    }

    pub fn insert(&mut self, feature: Feature) {
        self.0 |= feature.to_raw();
    }
}

impl<const N: usize> From<[Feature; N]> for Features {
    fn from(features: [Feature; N]) -> Self {
        features.into_iter().collect()
    }
}

impl FromIterator<Feature> for Features {
    fn from_iter<T: IntoIterator<Item = Feature>>(iter: T) -> Self {
        let mut features: Features = Features(Feature::MVP.to_raw());
        for feature in iter {
            features.insert(feature);
        }
        features
    }
}

impl Module {
    /// Gets what features are allowed when validating and in passes
    pub fn get_features(&self) -> Features {
        let module = self.as_inner();

        // SAFETY: We only read the module feature bits.
        let features = unsafe { BinaryenModuleGetFeatures(module) };
        Features(features)
    }

    /// Controls what features are allowed when validating and in passes.
    pub fn set_features(&mut self, features: impl Into<Features>) {
        let module = self.as_inner();
        let Features(inner) = features.into();

        // SAFETY: We have exclusive access over the module.
        unsafe { BinaryenModuleSetFeatures(module, inner) }
    }

    /// Adds a single feature to the module.
    pub fn add_feature(&mut self, feature: Feature) {
        let module = self.as_inner();

        let Features(old) = self.get_features();
        let new = feature.to_raw();

        // SAFETY: We have exclusive access over the module.
        unsafe { BinaryenModuleSetFeatures(module, old | new) }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_set_features() {
        let mut module = Module::new();

        // By default, only the mvp feature is enabled.
        assert_eq!(module.get_features(), [Feature::MVP].into());

        // set_features replaces all features, so now only tail-call is allowed.
        module.set_features([Feature::TailCall]);

        assert_eq!(
            module.get_features(),
            [Feature::MVP, Feature::TailCall].into()
        );
    }

    #[test]
    fn should_add_features() {
        let mut module = Module::new();
        module.add_feature(Feature::TailCall);

        let features = module.get_features();

        assert!(features.contains(Feature::MVP));
        assert!(features.contains(Feature::TailCall));
        assert!(!features.contains(Feature::ReferenceTypes));
    }
}
