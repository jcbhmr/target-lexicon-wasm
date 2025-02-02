use crate::{rust, wasm, IntoRust, IntoWasm};

impl IntoRust for wasm::Endianness {
    type RustType = rust::Endianness;
    fn into_rust(self) -> rust::Endianness {
        match self {
            wasm::Endianness::Little => rust::Endianness::Little,
            wasm::Endianness::Big => rust::Endianness::Big,
        }
    }
}
impl IntoWasm for rust::Endianness {
    type WasmType = wasm::Endianness;
    fn into_wasm(self) -> wasm::Endianness {
        match self {
            rust::Endianness::Little => wasm::Endianness::Little,
            rust::Endianness::Big => wasm::Endianness::Big,
        }
    }
}

impl IntoRust for wasm::PointerWidth {
    type RustType = rust::PointerWidth;
    fn into_rust(self) -> rust::PointerWidth {
        match self {
            wasm::PointerWidth::U16 => rust::PointerWidth::U16,
            wasm::PointerWidth::U32 => rust::PointerWidth::U32,
            wasm::PointerWidth::U64 => rust::PointerWidth::U64,
        }
    }
}
impl IntoWasm for rust::PointerWidth {
    type WasmType = wasm::PointerWidth;
    fn into_wasm(self) -> wasm::PointerWidth {
        match self {
            rust::PointerWidth::U16 => wasm::PointerWidth::U16,
            rust::PointerWidth::U32 => wasm::PointerWidth::U32,
            rust::PointerWidth::U64 => wasm::PointerWidth::U64,
        }
    }
}

impl IntoRust for wasm::CallingConvention {
    type RustType = rust::CallingConvention;
    fn into_rust(self) -> rust::CallingConvention {
        match self {
            wasm::CallingConvention::Systemv => rust::CallingConvention::SystemV,
            wasm::CallingConvention::Wasmbasiccabi => rust::CallingConvention::WasmBasicCAbi,
            wasm::CallingConvention::Windowsfastcall => rust::CallingConvention::WindowsFastcall,
            wasm::CallingConvention::Appaleaarch64 => rust::CallingConvention::AppleAarch64,
        }
    }
}
impl IntoWasm for rust::CallingConvention {
    type WasmType = wasm::CallingConvention;
    fn into_wasm(self) -> wasm::CallingConvention {
        match self {
            rust::CallingConvention::SystemV => wasm::CallingConvention::Systemv,
            rust::CallingConvention::WasmBasicCAbi => wasm::CallingConvention::Wasmbasiccabi,
            rust::CallingConvention::WindowsFastcall => wasm::CallingConvention::Windowsfastcall,
            rust::CallingConvention::AppleAarch64 => wasm::CallingConvention::Appaleaarch64,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::Triple {
    type RustType = rust::Triple;
    fn into_rust(self) -> rust::Triple {
        rust::Triple {
            architecture: self.architecture.into_rust(),
            vendor: self.vendor.into_rust(),
            operating_system: self.operating_system.into_rust(),
            environment: self.environment.into_rust(),
            binary_format: self.binary_format.into_rust(),
        }
    }
}
impl IntoWasm for rust::Triple {
    type WasmType = wasm::Triple;
    fn into_wasm(self) -> wasm::Triple {
        wasm::Triple {
            architecture: self.architecture.into_wasm(),
            vendor: self.vendor.into_wasm(),
            operating_system: self.operating_system.into_wasm(),
            environment: self.environment.into_wasm(),
            binary_format: self.binary_format.into_wasm(),
        }
    }
}
