use crate::{wasm, rust, IntoRust, IntoWasm};

impl IntoRust for wasm::Size {
    type RustType = rust::Size;
    fn into_rust(self) -> rust::Size {
        match self {
            wasm::Size::U8 => rust::Size::U8,
            wasm::Size::U16 => rust::Size::U16,
            wasm::Size::U32 => rust::Size::U32,
            wasm::Size::U64 => rust::Size::U64,
        }
    }
}
impl IntoWasm for rust::Size {
    type WasmType = wasm::Size;
    fn into_wasm(self) -> wasm::Size {
        match self {
            rust::Size::U8 => wasm::Size::U8,
            rust::Size::U16 => wasm::Size::U16,
            rust::Size::U32 => wasm::Size::U32,
            rust::Size::U64 => wasm::Size::U64,
        }
    }
}

impl IntoRust for wasm::CDataModel {
    type RustType = rust::CDataModel;
    fn into_rust(self) -> rust::CDataModel {
        match self {
            wasm::CDataModel::Lp32 => rust::CDataModel::LP32,
            wasm::CDataModel::Ilp32 => rust::CDataModel::ILP32,
            wasm::CDataModel::Llp64 => rust::CDataModel::LLP64,
            wasm::CDataModel::Lp64 => rust::CDataModel::LP64,
            wasm::CDataModel::Ilp64 => rust::CDataModel::ILP64,
        }
    }
}
impl IntoWasm for rust::CDataModel {
    type WasmType = wasm::CDataModel;
    fn into_wasm(self) -> wasm::CDataModel {
        match self {
            rust::CDataModel::LP32 => wasm::CDataModel::Lp32,
            rust::CDataModel::ILP32 => wasm::CDataModel::Ilp32,
            rust::CDataModel::LLP64 => wasm::CDataModel::Llp64,
            rust::CDataModel::LP64 => wasm::CDataModel::Lp64,
            rust::CDataModel::ILP64 => wasm::CDataModel::Ilp64,
            _ => todo!(),
        }
    }
}