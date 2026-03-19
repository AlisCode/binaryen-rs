//! # Binaryen
//!
//! Safe Rust bindings to Binaryen IR.
//!
//! This crate allows the creation, optimization and validation of WASM modules using the Binaryen IR.

pub mod api;
pub mod exports;
pub mod expression;
pub mod features;
pub mod function;
pub mod imports;
pub mod module;
pub mod table;
pub mod type_;
