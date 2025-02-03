wit_bindgen::generate!({
    world: "target-lexicon",
});

pub(crate) use exports::jcbhmr::target_lexicon::types as wasm;
use std::str::FromStr;
pub(crate) use target_lexicon as rust;

pub(crate) trait IntoRust {
    type RustType;
    fn into_rust(self) -> Self::RustType;
}
pub(crate) trait IntoWasm {
    type WasmType;
    fn into_wasm(self) -> Self::WasmType;
}

mod data_model;
mod parse_error;
mod targets;
mod triple;

impl IntoRust for wasm::DefaultToHost {
    type RustType = rust::DefaultToHost;
    fn into_rust(self) -> rust::DefaultToHost {
        rust::DefaultToHost(self.a.into_rust())
    }
}
impl IntoWasm for rust::DefaultToHost {
    type WasmType = wasm::DefaultToHost;
    fn into_wasm(self) -> wasm::DefaultToHost {
        wasm::DefaultToHost {
            a: self.0.into_wasm(),
        }
    }
}

impl IntoRust for wasm::DefaultToUnknown {
    type RustType = rust::DefaultToUnknown;
    fn into_rust(self) -> rust::DefaultToUnknown {
        rust::DefaultToUnknown(self.a.into_rust())
    }
}
impl IntoWasm for rust::DefaultToUnknown {
    type WasmType = wasm::DefaultToUnknown;
    fn into_wasm(self) -> wasm::DefaultToUnknown {
        wasm::DefaultToUnknown {
            a: self.0.into_wasm(),
        }
    }
}

struct MyHost;
impl exports::jcbhmr::target_lexicon::types::Guest for MyHost {
    fn size_bits(self_: wasm::Size) -> u8 {
        self_.into_rust().bits()
    }
    fn size_bytes(self_: wasm::Size) -> u8 {
        self_.into_rust().bytes()
    }
    fn c_data_model_pointer_width(self_: wasm::CDataModel) -> wasm::Size {
        self_.into_rust().pointer_width().into_wasm()
    }
    fn c_data_model_short_size(self_: wasm::CDataModel) -> wasm::Size {
        self_.into_rust().short_size().into_wasm()
    }
    fn c_data_model_int_size(self_: wasm::CDataModel) -> wasm::Size {
        self_.into_rust().int_size().into_wasm()
    }
    fn c_data_model_long_size(self_: wasm::CDataModel) -> wasm::Size {
        self_.into_rust().long_size().into_wasm()
    }
    fn c_data_model_long_long_size(self_: wasm::CDataModel) -> wasm::Size {
        self_.into_rust().long_long_size().into_wasm()
    }
    fn c_data_model_float_size(self_: wasm::CDataModel) -> wasm::Size {
        self_.into_rust().float_size().into_wasm()
    }
    fn c_data_model_double_size(self_: wasm::CDataModel) -> wasm::Size {
        self_.into_rust().double_size().into_wasm()
    }
    fn default_to_host_default() -> wasm::DefaultToHost {
        rust::DefaultToHost::default().into_wasm()
    }
    fn default_to_unknown_default() -> wasm::DefaultToUnknown {
        rust::DefaultToUnknown::default().into_wasm()
    }
    fn triple_serialize_json(triple: wasm::Triple) -> Result<String, String> {
        serde_json::to_string(&triple.into_rust()).map_err(|e| e.to_string())
    }
    fn triple_deserialize_json(json: String) -> Result<wasm::Triple, String> {
        serde_json::from_str(&json)
            .map(|t: rust::Triple| t.into_wasm())
            .map_err(|e| e.to_string())
    }
    fn parse_error_to_string(err: wasm::ParseError) -> String {
        err.into_rust().to_string()
    }
    fn arm_architecture_is_thumb(arch: wasm::ArmArchitecture) -> bool {
        arch.into_rust().is_thumb()
    }
    fn arm_architecture_pointer_width(arch: wasm::ArmArchitecture) -> wasm::PointerWidth {
        arch.into_rust().pointer_width().into_wasm()
    }
    fn arm_architecture_endianness(arch: wasm::ArmArchitecture) -> wasm::Endianness {
        arch.into_rust().endianness().into_wasm()
    }
    fn arm_architecture_into_str(arch: wasm::ArmArchitecture) -> String {
        arch.into_rust().into_str().into()
    }
    fn aarch64_architecture_is_thumb(arch: wasm::Aarch64Architecture) -> bool {
        arch.into_rust().is_thumb()
    }
    fn aarch64_architecture_pointer_width(arch: wasm::Aarch64Architecture) -> wasm::PointerWidth {
        arch.into_rust().pointer_width().into_wasm()
    }
    fn aarch64_architecture_endianness(arch: wasm::Aarch64Architecture) -> wasm::Endianness {
        arch.into_rust().endianness().into_wasm()
    }
    fn aarch64_architecture_into_str(arch: wasm::Aarch64Architecture) -> String {
        arch.into_rust().into_str().into()
    }
    fn riscv32_architecture_into_str(self_: wasm::Riscv32Architecture) -> String {
        self_.into_rust().into_str().into()
    }
    fn riscv64_architecture_into_str(self_: wasm::Riscv64Architecture) -> String {
        self_.into_rust().into_str().into()
    }
    fn x8632_architecture_into_str(self_: wasm::X8632Architecture) -> String {
        self_.into_rust().into_str().into()
    }
    fn mips32_architecture_into_str(self_: wasm::Mips32Architecture) -> String {
        self_.into_rust().into_str().into()
    }
    fn mips64_architecture_into_str(self_: wasm::Mips64Architecture) -> String {
        self_.into_rust().into_str().into()
    }
    fn custom_vendor_as_str(self_: wasm::CustomVendor) -> String {
        self_.into_rust().as_str().into()
    }
    fn custom_vendor_eq(self_: wasm::CustomVendor, other: wasm::CustomVendor) -> bool {
        self_.into_rust().eq(&other.into_rust())
    }
    fn vendor_as_str(self_: wasm::Vendor) -> String {
        self_.into_rust().as_str().into()
    }
    fn operating_system_into_str(self_: wasm::OperatingSystem) -> String {
        self_.into_rust().into_str().into()
    }
    fn operating_system_is_like_darwin(self_: wasm::OperatingSystem) -> bool {
        self_.into_rust().is_like_darwin()
    }
    fn environment_into_str(self_: wasm::Environment) -> String {
        self_.into_rust().into_str().into()
    }
    fn binary_format_into_str(self_: wasm::BinaryFormat) -> String {
        self_.into_rust().into_str().into()
    }
    fn architecture_endianness(self_: wasm::Architecture) -> Result<wasm::Endianness, bool> {
        self_
            .into_rust()
            .endianness()
            .map(|e| e.into_wasm())
            .map_err(|_| false)
    }
    fn architecture_pointer_width(self_: wasm::Architecture) -> Result<wasm::PointerWidth, bool> {
        self_
            .into_rust()
            .pointer_width()
            .map(|w| w.into_wasm())
            .map_err(|_| false)
    }
    fn architecture_is_clever(self_: wasm::Architecture) -> bool {
        self_.into_rust().is_clever()
    }
    fn architecture_into_str(self_: wasm::Architecture) -> String {
        self_.into_rust().into_str().into()
    }
    fn arm_architecture_to_string(self_: wasm::ArmArchitecture) -> String {
        self_.into_rust().to_string()
    }
    fn aarch64_architecture_to_string(self_: wasm::Aarch64Architecture) -> String {
        self_.into_rust().to_string()
    }
    fn riscv32_architecture_to_string(self_: wasm::Riscv32Architecture) -> String {
        self_.into_rust().to_string()
    }
    fn riscv64_architecture_to_string(self_: wasm::Riscv64Architecture) -> String {
        self_.into_rust().to_string()
    }
    fn x8632_architecture_to_string(self_: wasm::X8632Architecture) -> String {
        self_.into_rust().to_string()
    }
    fn mips32_architecture_to_string(self_: wasm::Mips32Architecture) -> String {
        self_.into_rust().to_string()
    }
    fn mips64_architecture_to_string(self_: wasm::Mips64Architecture) -> String {
        self_.into_rust().to_string()
    }
    fn architecture_to_string(self_: wasm::Architecture) -> String {
        self_.into_rust().to_string()
    }
    fn aarch64_architecture_from_str(s: String) -> Result<wasm::Aarch64Architecture, bool> {
        rust::Aarch64Architecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn arm_architecture_from_str(s: String) -> Result<wasm::ArmArchitecture, bool> {
        rust::ArmArchitecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn riscv32_architecture_from_str(s: String) -> Result<wasm::Riscv32Architecture, bool> {
        rust::Riscv32Architecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn riscv64_architecture_from_str(s: String) -> Result<wasm::Riscv64Architecture, bool> {
        rust::Riscv64Architecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn x8632_architecture_from_str(s: String) -> Result<wasm::X8632Architecture, bool> {
        rust::X86_32Architecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn mips32_architecture_from_str(s: String) -> Result<wasm::Mips32Architecture, bool> {
        rust::Mips32Architecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn mips64_architecture_from_str(s: String) -> Result<wasm::Mips64Architecture, bool> {
        rust::Mips64Architecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn architecture_from_str(s: String) -> Result<wasm::Architecture, bool> {
        rust::Architecture::from_str(&s)
            .map(|a| a.into_wasm())
            .map_err(|_| false)
    }
    fn vendor_to_string(self_: wasm::Vendor) -> String {
        self_.into_rust().to_string()
    }
    fn vendor_from_str(s: String) -> Result<wasm::Vendor, bool> {
        rust::Vendor::from_str(&s)
            .map(|v| v.into_wasm())
            .map_err(|_| false)
    }
    fn operating_system_to_string(self_: wasm::OperatingSystem) -> String {
        self_.into_rust().to_string()
    }
    fn operating_system_from_str(s: String) -> Result<wasm::OperatingSystem, bool> {
        rust::OperatingSystem::from_str(&s)
            .map(|os| os.into_wasm())
            .map_err(|_| false)
    }
    fn environment_to_string(self_: wasm::Environment) -> String {
        self_.into_rust().to_string()
    }
    fn environment_from_str(s: String) -> Result<wasm::Environment, bool> {
        rust::Environment::from_str(&s)
            .map(|env| env.into_wasm())
            .map_err(|_| false)
    }
    fn binary_format_to_string(self_: wasm::BinaryFormat) -> String {
        self_.into_rust().to_string()
    }
    fn binary_format_from_str(s: String) -> Result<wasm::BinaryFormat, bool> {
        rust::BinaryFormat::from_str(&s)
            .map(|bf| bf.into_wasm())
            .map_err(|_| false)
    }
    fn pointer_width_bits(self_: wasm::PointerWidth) -> u8 {
        self_.into_rust().bits()
    }
    fn pointer_width_bytes(self_: wasm::PointerWidth) -> u8 {
        self_.into_rust().bytes()
    }
    fn triple_endianness(self_: wasm::Triple) -> Result<wasm::Endianness, bool> {
        self_
            .into_rust()
            .endianness()
            .map(|e| e.into_wasm())
            .map_err(|_| false)
    }
    fn triple_pointer_width(self_: wasm::Triple) -> Result<wasm::PointerWidth, bool> {
        self_
            .into_rust()
            .pointer_width()
            .map(|w| w.into_wasm())
            .map_err(|_| false)
    }
    fn triple_default_calling_convention(
        self_: wasm::Triple,
    ) -> Result<wasm::CallingConvention, bool> {
        self_
            .into_rust()
            .default_calling_convention()
            .map(|cc| cc.into_wasm())
            .map_err(|_| false)
    }
    fn triple_data_model(self_: wasm::Triple) -> Result<wasm::CDataModel, bool> {
        self_
            .into_rust()
            .data_model()
            .map(|dm| dm.into_wasm())
            .map_err(|_| false)
    }
    fn triple_unknown() -> wasm::Triple {
        rust::Triple::unknown().into_wasm()
    }
    fn triple_to_string(self_: wasm::Triple) -> String {
        self_.into_rust().to_string()
    }
    fn triple_from_str(s: String) -> Result<wasm::Triple, wasm::ParseError> {
        rust::Triple::from_str(&s)
            .map(|t| t.into_wasm())
            .map_err(|e| e.into_wasm())
    }
    fn host() -> wasm::Triple {
        rust::HOST.into_wasm()
    }
    fn architecture_host() -> wasm::Architecture {
        rust::Architecture::host().into_wasm()
    }
    fn vendor_host() -> wasm::Vendor {
        rust::Vendor::host().into_wasm()
    }
    fn operating_system_host() -> wasm::OperatingSystem {
        rust::OperatingSystem::host().into_wasm()
    }
    fn environment_host() -> wasm::Environment {
        rust::Environment::host().into_wasm()
    }
    fn binary_format_host() -> wasm::BinaryFormat {
        rust::BinaryFormat::host().into_wasm()
    }
    fn triple_host() -> wasm::Triple {
        rust::Triple::host().into_wasm()
    }
}

export!(MyHost);
