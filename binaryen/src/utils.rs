macro_rules! binaryen_enum {
    (
        Binaryen($binaryen_ty:ty):
        $vis:vis enum $name:ident {
            $(
                $variant:ident => $ffi:ident,
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $name {
            $(
                $variant,
            )*
        }

        $(
            pastey::paste!(
                #[allow(non_upper_case_globals)]
                static [<__static $variant>] : std::sync::LazyLock<binaryen_sys::bindings::$binaryen_ty> =
                    std::sync::LazyLock::new(||
                        // SAFETY: These are variables that are OK to cache
                        unsafe { binaryen_sys::bindings::$ffi() }
                    );
            );
        )*

        pastey::paste!(
            impl $name {
                fn to_raw(self) -> binaryen_sys::bindings::$binaryen_ty {
                    match self {
                        $(
                            Self::$variant => *[< __static $variant >],
                        )*
                    }
                }
            }
        );
    };
}

pub(crate) use binaryen_enum;
