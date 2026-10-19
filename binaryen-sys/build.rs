use std::{env, path::{Path, PathBuf}};

fn main() {
    if !Path::new("binaryen/.git").exists() {
        panic!("binaryen submodule not found. Please run `git submodule update --init` first.");
    }

    println!("cargo:rerun-if-changed=wrapper.h");

    let mut config = cmake::Config::new("binaryen");
    config
        .define("BUILD_STATIC_LIB", "ON")
        .define("ENABLE_WERROR", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_TOOLS", "OFF");

    println!("cargo:rerun-if-env-changed=CMAKE_GENERATOR");
    let generator = env::var("CMAKE_GENERATOR").ok().or_else(|| {
        find_in_path("ninja").map(|_| "Ninja".to_string())
    });

    if let Some(generator) = &generator {
        config.generator(generator);

        // If the user has already built the library, CMake refuses to switch generators (existing build directory)
        // This can happen when:
        // - the user's PATH has changed
        // - rust-analyzer running with a different PATH
        // - nix or devenv shells
        discard_cmake_cache_if_generator_changed(generator);
    }

    // Every new OUT_DIR rebuilds Binaryen from scratch...
    // Let's enable caching when it is available.
    if let Some(ccache) = find_in_path("ccache") {
        config
            .define("CMAKE_C_COMPILER_LAUNCHER", &ccache)
            .define("CMAKE_CXX_COMPILER_LAUNCHER", &ccache);
    }

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=binaryen");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Binaryen is a C++ library, so we need to link against the C++ standard library.
    // The cmake crate doesn't handle this automatically for static libraries.
    if let Some(cpp_stdlib) = get_cpp_stdlib() {
        println!("cargo:rustc-link-lib={}", cpp_stdlib);
    }
}

fn find_in_path(program: &str) -> Option<PathBuf> {
    println!("cargo:rerun-if-env-changed=PATH");
    env::split_paths(&env::var_os("PATH")?)
        .map(|dir| dir.join(program))
        .find(|path| path.is_file())
}

fn discard_cmake_cache_if_generator_changed(generator: &str) {
    let build_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"))
        .join("build");
    let Ok(cache) = std::fs::read_to_string(build_dir.join("CMakeCache.txt")) else {
        return;
    };
    let previous = cache
        .lines()
        .find_map(|line| line.strip_prefix("CMAKE_GENERATOR:INTERNAL="));
    if previous.is_some_and(|previous| previous != generator) {
        let _ = std::fs::remove_file(build_dir.join("CMakeCache.txt"));
        let _ = std::fs::remove_dir_all(build_dir.join("CMakeFiles"));
    }
}

fn get_cpp_stdlib() -> Option<String> {
    env::var("TARGET").ok().and_then(|target| {
        if target.contains("msvc") {
            None
        } else if target.contains("darwin") || target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("musl") {
            Some("static=stdc++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    })
}
