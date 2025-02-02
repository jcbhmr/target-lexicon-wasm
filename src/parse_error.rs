use crate::{wasm, rust, IntoRust, IntoWasm};

impl IntoRust for wasm::ParseError {
    type RustType = rust::ParseError;
    fn into_rust(self) -> rust::ParseError {
        match self {
            wasm::ParseError::UnrecognizedArchitecture(s) => rust::ParseError::UnrecognizedArchitecture(s),
            wasm::ParseError::UnrecognizedVendor(s) => rust::ParseError::UnrecognizedVendor(s),
            wasm::ParseError::UnrecognizedOperatingSystem(s) => rust::ParseError::UnrecognizedOperatingSystem(s),
            wasm::ParseError::UnrecognizedEnvironment(s) => rust::ParseError::UnrecognizedEnvironment(s),
            wasm::ParseError::UnrecognizedBinaryFormat(s) => rust::ParseError::UnrecognizedBinaryFormat(s),
            wasm::ParseError::UnrecognizedField(s) => rust::ParseError::UnrecognizedField(s),
        }
    }
}
impl IntoWasm for rust::ParseError {
    type WasmType = wasm::ParseError;
    fn into_wasm(self) -> wasm::ParseError {
        match self {
            rust::ParseError::UnrecognizedArchitecture(s) => wasm::ParseError::UnrecognizedArchitecture(s),
            rust::ParseError::UnrecognizedVendor(s) => wasm::ParseError::UnrecognizedVendor(s),
            rust::ParseError::UnrecognizedOperatingSystem(s) => wasm::ParseError::UnrecognizedOperatingSystem(s),
            rust::ParseError::UnrecognizedEnvironment(s) => wasm::ParseError::UnrecognizedEnvironment(s),
            rust::ParseError::UnrecognizedBinaryFormat(s) => wasm::ParseError::UnrecognizedBinaryFormat(s),
            rust::ParseError::UnrecognizedField(s) => wasm::ParseError::UnrecognizedField(s),
        }
    }
}