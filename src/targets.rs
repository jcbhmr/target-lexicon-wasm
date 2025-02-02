use crate::{rust, wasm, IntoRust, IntoWasm};

impl IntoRust for wasm::Architecture {
    type RustType = rust::Architecture;
    fn into_rust(self) -> rust::Architecture {
        match self {
            wasm::Architecture::Unknown => rust::Architecture::Unknown,
            wasm::Architecture::Arm(a) => rust::Architecture::Arm(a.into_rust()),
            wasm::Architecture::Amdgcn => rust::Architecture::AmdGcn,
            wasm::Architecture::Aarch64(a) => rust::Architecture::Aarch64(a.into_rust()),
            wasm::Architecture::Asmjs => rust::Architecture::Asmjs,
            wasm::Architecture::Avr => rust::Architecture::Avr,
            wasm::Architecture::Bpfeb => rust::Architecture::Bpfeb,
            wasm::Architecture::Bpfel => rust::Architecture::Bpfel,
            wasm::Architecture::Hexagon => rust::Architecture::Hexagon,
            wasm::Architecture::X8632(a) => rust::Architecture::X86_32(a.into_rust()),
            wasm::Architecture::M68k => rust::Architecture::M68k,
            wasm::Architecture::Loongarch64 => rust::Architecture::LoongArch64,
            wasm::Architecture::Mips32(a) => rust::Architecture::Mips32(a.into_rust()),
            wasm::Architecture::Mips64(a) => rust::Architecture::Mips64(a.into_rust()),
            wasm::Architecture::Msp430 => rust::Architecture::Msp430,
            wasm::Architecture::Nvptx64 => rust::Architecture::Nvptx64,
            wasm::Architecture::Pulley32 => rust::Architecture::Pulley32,
            wasm::Architecture::Pulley64 => rust::Architecture::Pulley64,
            wasm::Architecture::Pulley32be => rust::Architecture::Pulley32be,
            wasm::Architecture::Pulley64be => rust::Architecture::Pulley64be,
            wasm::Architecture::Powerpc => rust::Architecture::Powerpc,
            wasm::Architecture::Powerpc64 => rust::Architecture::Powerpc64,
            wasm::Architecture::Powerpc64le => rust::Architecture::Powerpc64le,
            wasm::Architecture::Riscv32(a) => rust::Architecture::Riscv32(a.into_rust()),
            wasm::Architecture::Riscv64(a) => rust::Architecture::Riscv64(a.into_rust()),
            wasm::Architecture::S390x => rust::Architecture::S390x,
            wasm::Architecture::Sparc => rust::Architecture::Sparc,
            wasm::Architecture::Sparc64 => rust::Architecture::Sparc64,
            wasm::Architecture::Sparcv9 => rust::Architecture::Sparcv9,
            wasm::Architecture::Wasm32 => rust::Architecture::Wasm32,
            wasm::Architecture::Wasm64 => rust::Architecture::Wasm64,
            wasm::Architecture::X8664 => rust::Architecture::X86_64,
            wasm::Architecture::X8664h => rust::Architecture::X86_64h,
            wasm::Architecture::Xtensa => rust::Architecture::XTensa,
            wasm::Architecture::Clever(_a) => todo!(),
            wasm::Architecture::Zkasm => rust::Architecture::ZkAsm,
        }
    }
}
impl IntoWasm for rust::Architecture {
    type WasmType = wasm::Architecture;
    fn into_wasm(self) -> wasm::Architecture {
        match self {
            rust::Architecture::Unknown => wasm::Architecture::Unknown,
            rust::Architecture::Arm(a) => wasm::Architecture::Arm(a.into_wasm()),
            rust::Architecture::AmdGcn => wasm::Architecture::Amdgcn,
            rust::Architecture::Aarch64(a) => wasm::Architecture::Aarch64(a.into_wasm()),
            rust::Architecture::Asmjs => wasm::Architecture::Asmjs,
            rust::Architecture::Avr => wasm::Architecture::Avr,
            rust::Architecture::Bpfeb => wasm::Architecture::Bpfeb,
            rust::Architecture::Bpfel => wasm::Architecture::Bpfel,
            rust::Architecture::Hexagon => wasm::Architecture::Hexagon,
            rust::Architecture::X86_32(a) => wasm::Architecture::X8632(a.into_wasm()),
            rust::Architecture::M68k => wasm::Architecture::M68k,
            rust::Architecture::LoongArch64 => wasm::Architecture::Loongarch64,
            rust::Architecture::Mips32(a) => wasm::Architecture::Mips32(a.into_wasm()),
            rust::Architecture::Mips64(a) => wasm::Architecture::Mips64(a.into_wasm()),
            rust::Architecture::Msp430 => wasm::Architecture::Msp430,
            rust::Architecture::Nvptx64 => wasm::Architecture::Nvptx64,
            rust::Architecture::Pulley32 => wasm::Architecture::Pulley32,
            rust::Architecture::Pulley64 => wasm::Architecture::Pulley64,
            rust::Architecture::Pulley32be => wasm::Architecture::Pulley32be,
            rust::Architecture::Pulley64be => wasm::Architecture::Pulley64be,
            rust::Architecture::Powerpc => wasm::Architecture::Powerpc,
            rust::Architecture::Powerpc64 => wasm::Architecture::Powerpc64,
            rust::Architecture::Powerpc64le => wasm::Architecture::Powerpc64le,
            rust::Architecture::Riscv32(a) => wasm::Architecture::Riscv32(a.into_wasm()),
            rust::Architecture::Riscv64(a) => wasm::Architecture::Riscv64(a.into_wasm()),
            rust::Architecture::S390x => wasm::Architecture::S390x,
            rust::Architecture::Sparc => wasm::Architecture::Sparc,
            rust::Architecture::Sparc64 => wasm::Architecture::Sparc64,
            rust::Architecture::Sparcv9 => wasm::Architecture::Sparcv9,
            rust::Architecture::Wasm32 => wasm::Architecture::Wasm32,
            rust::Architecture::Wasm64 => wasm::Architecture::Wasm64,
            rust::Architecture::X86_64 => wasm::Architecture::X8664,
            rust::Architecture::X86_64h => wasm::Architecture::X8664h,
            rust::Architecture::XTensa => wasm::Architecture::Xtensa,
            rust::Architecture::Clever(_a) => wasm::Architecture::Clever(false),
            rust::Architecture::ZkAsm => wasm::Architecture::Zkasm,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::ArmArchitecture {
    type RustType = rust::ArmArchitecture;
    fn into_rust(self) -> rust::ArmArchitecture {
        match self {
            wasm::ArmArchitecture::Arm => rust::ArmArchitecture::Arm,
            wasm::ArmArchitecture::Armeb => rust::ArmArchitecture::Armeb,
            wasm::ArmArchitecture::Armv4 => rust::ArmArchitecture::Armv4,
            wasm::ArmArchitecture::Armv4t => rust::ArmArchitecture::Armv4t,
            wasm::ArmArchitecture::Armv5t => rust::ArmArchitecture::Armv5t,
            wasm::ArmArchitecture::Armv5te => rust::ArmArchitecture::Armv5te,
            wasm::ArmArchitecture::Armv5tej => rust::ArmArchitecture::Armv5tej,
            wasm::ArmArchitecture::Armv6 => rust::ArmArchitecture::Armv6,
            wasm::ArmArchitecture::Armv6j => rust::ArmArchitecture::Armv6j,
            wasm::ArmArchitecture::Armv6k => rust::ArmArchitecture::Armv6k,
            wasm::ArmArchitecture::Armv6z => rust::ArmArchitecture::Armv6z,
            wasm::ArmArchitecture::Armv6kz => rust::ArmArchitecture::Armv6kz,
            wasm::ArmArchitecture::Armv6t2 => rust::ArmArchitecture::Armv6t2,
            wasm::ArmArchitecture::Armv6m => rust::ArmArchitecture::Armv6m,
            wasm::ArmArchitecture::Armv7 => rust::ArmArchitecture::Armv7,
            wasm::ArmArchitecture::Armv7a => rust::ArmArchitecture::Armv7a,
            wasm::ArmArchitecture::Armv7k => rust::ArmArchitecture::Armv7k,
            wasm::ArmArchitecture::Armv7ve => rust::ArmArchitecture::Armv7ve,
            wasm::ArmArchitecture::Armv7m => rust::ArmArchitecture::Armv7m,
            wasm::ArmArchitecture::Armv7r => rust::ArmArchitecture::Armv7r,
            wasm::ArmArchitecture::Armv7s => rust::ArmArchitecture::Armv7s,
            wasm::ArmArchitecture::Armv8 => rust::ArmArchitecture::Armv8,
            wasm::ArmArchitecture::Armv8a => rust::ArmArchitecture::Armv8a,
            wasm::ArmArchitecture::Armv81a => rust::ArmArchitecture::Armv8_1a,
            wasm::ArmArchitecture::Armv82a => rust::ArmArchitecture::Armv8_2a,
            wasm::ArmArchitecture::Armv83a => rust::ArmArchitecture::Armv8_3a,
            wasm::ArmArchitecture::Armv84a => rust::ArmArchitecture::Armv8_4a,
            wasm::ArmArchitecture::Armv85a => rust::ArmArchitecture::Armv8_5a,
            wasm::ArmArchitecture::Armv8mbase => rust::ArmArchitecture::Armv8mBase,
            wasm::ArmArchitecture::Armv8mmain => rust::ArmArchitecture::Armv8mMain,
            wasm::ArmArchitecture::Armv8r => rust::ArmArchitecture::Armv8r,
            wasm::ArmArchitecture::Armebv7r => rust::ArmArchitecture::Armebv7r,
            wasm::ArmArchitecture::Thumbeb => rust::ArmArchitecture::Thumbeb,
            wasm::ArmArchitecture::Thumbv4t => rust::ArmArchitecture::Thumbv4t,
            wasm::ArmArchitecture::Thumbv5te => rust::ArmArchitecture::Thumbv5te,
            wasm::ArmArchitecture::Thumbv6m => rust::ArmArchitecture::Thumbv6m,
            wasm::ArmArchitecture::Thumbv7a => rust::ArmArchitecture::Thumbv7a,
            wasm::ArmArchitecture::Thumbv7em => rust::ArmArchitecture::Thumbv7em,
            wasm::ArmArchitecture::Thumbv7m => rust::ArmArchitecture::Thumbv7m,
            wasm::ArmArchitecture::Thumbv7neon => rust::ArmArchitecture::Thumbv7neon,
            wasm::ArmArchitecture::Thumbv8mbase => rust::ArmArchitecture::Thumbv8mBase,
            wasm::ArmArchitecture::Thumbv8mmain => rust::ArmArchitecture::Thumbv8mMain,
        }
    }
}
impl IntoWasm for rust::ArmArchitecture {
    type WasmType = wasm::ArmArchitecture;
    fn into_wasm(self) -> wasm::ArmArchitecture {
        match self {
            rust::ArmArchitecture::Arm => wasm::ArmArchitecture::Arm,
            rust::ArmArchitecture::Armeb => wasm::ArmArchitecture::Armeb,
            rust::ArmArchitecture::Armv4 => wasm::ArmArchitecture::Armv4,
            rust::ArmArchitecture::Armv4t => wasm::ArmArchitecture::Armv4t,
            rust::ArmArchitecture::Armv5t => wasm::ArmArchitecture::Armv5t,
            rust::ArmArchitecture::Armv5te => wasm::ArmArchitecture::Armv5te,
            rust::ArmArchitecture::Armv5tej => wasm::ArmArchitecture::Armv5tej,
            rust::ArmArchitecture::Armv6 => wasm::ArmArchitecture::Armv6,
            rust::ArmArchitecture::Armv6j => wasm::ArmArchitecture::Armv6j,
            rust::ArmArchitecture::Armv6k => wasm::ArmArchitecture::Armv6k,
            rust::ArmArchitecture::Armv6z => wasm::ArmArchitecture::Armv6z,
            rust::ArmArchitecture::Armv6kz => wasm::ArmArchitecture::Armv6kz,
            rust::ArmArchitecture::Armv6t2 => wasm::ArmArchitecture::Armv6t2,
            rust::ArmArchitecture::Armv6m => wasm::ArmArchitecture::Armv6m,
            rust::ArmArchitecture::Armv7 => wasm::ArmArchitecture::Armv7,
            rust::ArmArchitecture::Armv7a => wasm::ArmArchitecture::Armv7a,
            rust::ArmArchitecture::Armv7k => wasm::ArmArchitecture::Armv7k,
            rust::ArmArchitecture::Armv7ve => wasm::ArmArchitecture::Armv7ve,
            rust::ArmArchitecture::Armv7m => wasm::ArmArchitecture::Armv7m,
            rust::ArmArchitecture::Armv7r => wasm::ArmArchitecture::Armv7r,
            rust::ArmArchitecture::Armv7s => wasm::ArmArchitecture::Armv7s,
            rust::ArmArchitecture::Armv8 => wasm::ArmArchitecture::Armv8,
            rust::ArmArchitecture::Armv8a => wasm::ArmArchitecture::Armv8a,
            rust::ArmArchitecture::Armv8_1a => wasm::ArmArchitecture::Armv81a,
            rust::ArmArchitecture::Armv8_2a => wasm::ArmArchitecture::Armv82a,
            rust::ArmArchitecture::Armv8_3a => wasm::ArmArchitecture::Armv83a,
            rust::ArmArchitecture::Armv8_4a => wasm::ArmArchitecture::Armv84a,
            rust::ArmArchitecture::Armv8_5a => wasm::ArmArchitecture::Armv85a,
            rust::ArmArchitecture::Armv8mBase => wasm::ArmArchitecture::Armv8mbase,
            rust::ArmArchitecture::Armv8mMain => wasm::ArmArchitecture::Armv8mmain,
            rust::ArmArchitecture::Armv8r => wasm::ArmArchitecture::Armv8r,
            rust::ArmArchitecture::Armebv7r => wasm::ArmArchitecture::Armebv7r,
            rust::ArmArchitecture::Thumbeb => wasm::ArmArchitecture::Thumbeb,
            rust::ArmArchitecture::Thumbv4t => wasm::ArmArchitecture::Thumbv4t,
            rust::ArmArchitecture::Thumbv5te => wasm::ArmArchitecture::Thumbv5te,
            rust::ArmArchitecture::Thumbv6m => wasm::ArmArchitecture::Thumbv6m,
            rust::ArmArchitecture::Thumbv7a => wasm::ArmArchitecture::Thumbv7a,
            rust::ArmArchitecture::Thumbv7em => wasm::ArmArchitecture::Thumbv7em,
            rust::ArmArchitecture::Thumbv7m => wasm::ArmArchitecture::Thumbv7m,
            rust::ArmArchitecture::Thumbv7neon => wasm::ArmArchitecture::Thumbv7neon,
            rust::ArmArchitecture::Thumbv8mBase => wasm::ArmArchitecture::Thumbv8mbase,
            rust::ArmArchitecture::Thumbv8mMain => wasm::ArmArchitecture::Thumbv8mmain,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::Aarch64Architecture {
    type RustType = rust::Aarch64Architecture;
    fn into_rust(self) -> rust::Aarch64Architecture {
        match self {
            wasm::Aarch64Architecture::Aarch64 => rust::Aarch64Architecture::Aarch64,
            wasm::Aarch64Architecture::Aarch64be => rust::Aarch64Architecture::Aarch64be,
        }
    }
}
impl IntoWasm for rust::Aarch64Architecture {
    type WasmType = wasm::Aarch64Architecture;
    fn into_wasm(self) -> wasm::Aarch64Architecture {
        match self {
            rust::Aarch64Architecture::Aarch64 => wasm::Aarch64Architecture::Aarch64,
            rust::Aarch64Architecture::Aarch64be => wasm::Aarch64Architecture::Aarch64be,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::Riscv32Architecture {
    type RustType = rust::Riscv32Architecture;
    fn into_rust(self) -> rust::Riscv32Architecture {
        match self {
            wasm::Riscv32Architecture::Riscv32 => rust::Riscv32Architecture::Riscv32,
            wasm::Riscv32Architecture::Riscv32gc => rust::Riscv32Architecture::Riscv32gc,
            wasm::Riscv32Architecture::Riscv32i => rust::Riscv32Architecture::Riscv32i,
            wasm::Riscv32Architecture::Riscv32im => rust::Riscv32Architecture::Riscv32im,
            wasm::Riscv32Architecture::Riscv32ima => rust::Riscv32Architecture::Riscv32ima,
            wasm::Riscv32Architecture::Riscv32imac => rust::Riscv32Architecture::Riscv32imac,
            wasm::Riscv32Architecture::Riscv32imafc => rust::Riscv32Architecture::Riscv32imafc,
            wasm::Riscv32Architecture::Riscv32imc => rust::Riscv32Architecture::Riscv32imc,
        }
    }
}
impl IntoWasm for rust::Riscv32Architecture {
    type WasmType = wasm::Riscv32Architecture;
    fn into_wasm(self) -> wasm::Riscv32Architecture {
        match self {
            rust::Riscv32Architecture::Riscv32 => wasm::Riscv32Architecture::Riscv32,
            rust::Riscv32Architecture::Riscv32gc => wasm::Riscv32Architecture::Riscv32gc,
            rust::Riscv32Architecture::Riscv32i => wasm::Riscv32Architecture::Riscv32i,
            rust::Riscv32Architecture::Riscv32im => wasm::Riscv32Architecture::Riscv32im,
            rust::Riscv32Architecture::Riscv32ima => wasm::Riscv32Architecture::Riscv32ima,
            rust::Riscv32Architecture::Riscv32imac => wasm::Riscv32Architecture::Riscv32imac,
            rust::Riscv32Architecture::Riscv32imafc => wasm::Riscv32Architecture::Riscv32imafc,
            rust::Riscv32Architecture::Riscv32imc => wasm::Riscv32Architecture::Riscv32imc,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::Riscv64Architecture {
    type RustType = rust::Riscv64Architecture;
    fn into_rust(self) -> rust::Riscv64Architecture {
        match self {
            wasm::Riscv64Architecture::Riscv64 => rust::Riscv64Architecture::Riscv64,
            wasm::Riscv64Architecture::Riscv64gc => rust::Riscv64Architecture::Riscv64gc,
            wasm::Riscv64Architecture::Riscv64imac => rust::Riscv64Architecture::Riscv64imac,
        }
    }
}
impl IntoWasm for rust::Riscv64Architecture {
    type WasmType = wasm::Riscv64Architecture;
    fn into_wasm(self) -> wasm::Riscv64Architecture {
        match self {
            rust::Riscv64Architecture::Riscv64 => wasm::Riscv64Architecture::Riscv64,
            rust::Riscv64Architecture::Riscv64gc => wasm::Riscv64Architecture::Riscv64gc,
            rust::Riscv64Architecture::Riscv64imac => wasm::Riscv64Architecture::Riscv64imac,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::X8632Architecture {
    type RustType = rust::X86_32Architecture;
    fn into_rust(self) -> rust::X86_32Architecture {
        match self {
            wasm::X8632Architecture::I386 => rust::X86_32Architecture::I386,
            wasm::X8632Architecture::I586 => rust::X86_32Architecture::I586,
            wasm::X8632Architecture::I686 => rust::X86_32Architecture::I686,
        }
    }
}
impl IntoWasm for rust::X86_32Architecture {
    type WasmType = wasm::X8632Architecture;
    fn into_wasm(self) -> wasm::X8632Architecture {
        match self {
            rust::X86_32Architecture::I386 => wasm::X8632Architecture::I386,
            rust::X86_32Architecture::I586 => wasm::X8632Architecture::I586,
            rust::X86_32Architecture::I686 => wasm::X8632Architecture::I686,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::Mips32Architecture {
    type RustType = rust::Mips32Architecture;
    fn into_rust(self) -> rust::Mips32Architecture {
        match self {
            wasm::Mips32Architecture::Mips => rust::Mips32Architecture::Mips,
            wasm::Mips32Architecture::Mipsel => rust::Mips32Architecture::Mipsel,
            wasm::Mips32Architecture::Mipsisa32r6 => rust::Mips32Architecture::Mipsisa32r6,
            wasm::Mips32Architecture::Mipsisa32r6el => rust::Mips32Architecture::Mipsisa32r6el,
        }
    }
}
impl IntoWasm for rust::Mips32Architecture {
    type WasmType = wasm::Mips32Architecture;
    fn into_wasm(self) -> wasm::Mips32Architecture {
        match self {
            rust::Mips32Architecture::Mips => wasm::Mips32Architecture::Mips,
            rust::Mips32Architecture::Mipsel => wasm::Mips32Architecture::Mipsel,
            rust::Mips32Architecture::Mipsisa32r6 => wasm::Mips32Architecture::Mipsisa32r6,
            rust::Mips32Architecture::Mipsisa32r6el => wasm::Mips32Architecture::Mipsisa32r6el,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::Mips64Architecture {
    type RustType = rust::Mips64Architecture;
    fn into_rust(self) -> rust::Mips64Architecture {
        match self {
            wasm::Mips64Architecture::Mips64 => rust::Mips64Architecture::Mips64,
            wasm::Mips64Architecture::Mips64el => rust::Mips64Architecture::Mips64el,
            wasm::Mips64Architecture::Mipsisa64r6 => rust::Mips64Architecture::Mipsisa64r6,
            wasm::Mips64Architecture::Mipsisa64r6el => rust::Mips64Architecture::Mipsisa64r6el,
        }
    }
}
impl IntoWasm for rust::Mips64Architecture {
    type WasmType = wasm::Mips64Architecture;
    fn into_wasm(self) -> wasm::Mips64Architecture {
        match self {
            rust::Mips64Architecture::Mips64 => wasm::Mips64Architecture::Mips64,
            rust::Mips64Architecture::Mips64el => wasm::Mips64Architecture::Mips64el,
            rust::Mips64Architecture::Mipsisa64r6 => wasm::Mips64Architecture::Mipsisa64r6,
            rust::Mips64Architecture::Mipsisa64r6el => wasm::Mips64Architecture::Mipsisa64r6el,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::CustomVendor {
    type RustType = rust::CustomVendor;
    fn into_rust(self) -> rust::CustomVendor {
        match self {
            wasm::CustomVendor::Owned(s) => rust::CustomVendor::Owned(Box::new(s)),
            wasm::CustomVendor::Static(s) => rust::CustomVendor::Static(Box::leak(Box::new(s))),
        }
    }
}
impl IntoWasm for rust::CustomVendor {
    type WasmType = wasm::CustomVendor;
    fn into_wasm(self) -> wasm::CustomVendor {
        match self {
            rust::CustomVendor::Owned(s) => wasm::CustomVendor::Owned(*s),
            rust::CustomVendor::Static(s) => wasm::CustomVendor::Static(s.into()),
        }
    }
}

impl IntoRust for wasm::Vendor {
    type RustType = rust::Vendor;
    fn into_rust(self) -> rust::Vendor {
        match self {
            wasm::Vendor::Unknown => rust::Vendor::Unknown,
            wasm::Vendor::Amd => rust::Vendor::Amd,
            wasm::Vendor::Apple => rust::Vendor::Apple,
            wasm::Vendor::Espressif => rust::Vendor::Espressif,
            wasm::Vendor::Experimental => rust::Vendor::Experimental,
            wasm::Vendor::Fortanix => rust::Vendor::Fortanix,
            wasm::Vendor::Ibm => rust::Vendor::Ibm,
            wasm::Vendor::Kmc => rust::Vendor::Kmc,
            wasm::Vendor::Nintendo => rust::Vendor::Nintendo,
            wasm::Vendor::Nvidia => rust::Vendor::Nvidia,
            wasm::Vendor::Pc => rust::Vendor::Pc,
            wasm::Vendor::Rumprun => rust::Vendor::Rumprun,
            wasm::Vendor::Sun => rust::Vendor::Sun,
            wasm::Vendor::Uwp => rust::Vendor::Uwp,
            wasm::Vendor::Wrs => rust::Vendor::Wrs,
            wasm::Vendor::Custom(s) => rust::Vendor::Custom(s.into_rust()),
        }
    }
}
impl IntoWasm for rust::Vendor {
    type WasmType = wasm::Vendor;
    fn into_wasm(self) -> wasm::Vendor {
        match self {
            rust::Vendor::Unknown => wasm::Vendor::Unknown,
            rust::Vendor::Amd => wasm::Vendor::Amd,
            rust::Vendor::Apple => wasm::Vendor::Apple,
            rust::Vendor::Espressif => wasm::Vendor::Espressif,
            rust::Vendor::Experimental => wasm::Vendor::Experimental,
            rust::Vendor::Fortanix => wasm::Vendor::Fortanix,
            rust::Vendor::Ibm => wasm::Vendor::Ibm,
            rust::Vendor::Kmc => wasm::Vendor::Kmc,
            rust::Vendor::Nintendo => wasm::Vendor::Nintendo,
            rust::Vendor::Nvidia => wasm::Vendor::Nvidia,
            rust::Vendor::Pc => wasm::Vendor::Pc,
            rust::Vendor::Rumprun => wasm::Vendor::Rumprun,
            rust::Vendor::Sun => wasm::Vendor::Sun,
            rust::Vendor::Uwp => wasm::Vendor::Uwp,
            rust::Vendor::Wrs => wasm::Vendor::Wrs,
            rust::Vendor::Custom(s) => wasm::Vendor::Custom(s.into_wasm()),
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::DeploymentTarget {
    type RustType = rust::DeploymentTarget;
    fn into_rust(self) -> rust::DeploymentTarget {
        rust::DeploymentTarget {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
        }
    }
}
impl IntoWasm for rust::DeploymentTarget {
    type WasmType = wasm::DeploymentTarget;
    fn into_wasm(self) -> wasm::DeploymentTarget {
        wasm::DeploymentTarget {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
        }
    }
}

impl IntoRust for wasm::OperatingSystem {
    type RustType = rust::OperatingSystem;
    fn into_rust(self) -> rust::OperatingSystem {
        match self {
            wasm::OperatingSystem::Unknown => rust::OperatingSystem::Unknown,
            wasm::OperatingSystem::Aix => rust::OperatingSystem::Aix,
            wasm::OperatingSystem::Amdhsa => rust::OperatingSystem::AmdHsa,
            wasm::OperatingSystem::Bitrig => rust::OperatingSystem::Bitrig,
            wasm::OperatingSystem::Cloudabi => rust::OperatingSystem::Cloudabi,
            wasm::OperatingSystem::Cuda => rust::OperatingSystem::Cuda,
            wasm::OperatingSystem::Darwin(o) => {
                rust::OperatingSystem::Darwin(o.map(|d| d.into_rust()))
            }
            wasm::OperatingSystem::Dragonfly => rust::OperatingSystem::Dragonfly,
            wasm::OperatingSystem::Emscripten => rust::OperatingSystem::Emscripten,
            wasm::OperatingSystem::Espidf => rust::OperatingSystem::Espidf,
            wasm::OperatingSystem::Freebsd => rust::OperatingSystem::Freebsd,
            wasm::OperatingSystem::Fuchsia => rust::OperatingSystem::Fuchsia,
            wasm::OperatingSystem::Haiku => rust::OperatingSystem::Haiku,
            wasm::OperatingSystem::Hermit => rust::OperatingSystem::Hermit,
            wasm::OperatingSystem::Horizon => rust::OperatingSystem::Horizon,
            wasm::OperatingSystem::Hurd => rust::OperatingSystem::Hurd,
            wasm::OperatingSystem::Illumos => rust::OperatingSystem::Illumos,
            wasm::OperatingSystem::Ios(o) => rust::OperatingSystem::IOS(o.map(|d| d.into_rust())),
            wasm::OperatingSystem::L4re => rust::OperatingSystem::L4re,
            wasm::OperatingSystem::Linux => rust::OperatingSystem::Linux,
            wasm::OperatingSystem::Macosx(o) => {
                rust::OperatingSystem::MacOSX(o.map(|d| d.into_rust()))
            }
            wasm::OperatingSystem::Nebulet => rust::OperatingSystem::Nebulet,
            wasm::OperatingSystem::Netbsd => rust::OperatingSystem::Netbsd,
            wasm::OperatingSystem::None => rust::OperatingSystem::None_,
            wasm::OperatingSystem::Openbsd => rust::OperatingSystem::Openbsd,
            wasm::OperatingSystem::Psp => rust::OperatingSystem::Psp,
            wasm::OperatingSystem::Redox => rust::OperatingSystem::Redox,
            wasm::OperatingSystem::Solaris => rust::OperatingSystem::Solaris,
            wasm::OperatingSystem::Solidasp3 => rust::OperatingSystem::SolidAsp3,
            wasm::OperatingSystem::Tvos(o) => rust::OperatingSystem::TvOS(o.map(|d| d.into_rust())),
            wasm::OperatingSystem::Uefi => rust::OperatingSystem::Uefi,
            wasm::OperatingSystem::Visionos(o) => {
                rust::OperatingSystem::VisionOS(o.map(|d| d.into_rust()))
            }
            wasm::OperatingSystem::Vxworks => rust::OperatingSystem::VxWorks,
            wasm::OperatingSystem::Wasi => rust::OperatingSystem::Wasi,
            wasm::OperatingSystem::Wasip1 => rust::OperatingSystem::WasiP1,
            wasm::OperatingSystem::Wasip2 => rust::OperatingSystem::WasiP2,
            wasm::OperatingSystem::Watchos(o) => {
                rust::OperatingSystem::WatchOS(o.map(|d| d.into_rust()))
            }
            wasm::OperatingSystem::Windows => rust::OperatingSystem::Windows,
            wasm::OperatingSystem::Xros(o) => rust::OperatingSystem::XROS(o.map(|d| d.into_rust())),
        }
    }
}
impl IntoWasm for rust::OperatingSystem {
    type WasmType = wasm::OperatingSystem;
    fn into_wasm(self) -> wasm::OperatingSystem {
        match self {
            rust::OperatingSystem::Unknown => wasm::OperatingSystem::Unknown,
            rust::OperatingSystem::Aix => wasm::OperatingSystem::Aix,
            rust::OperatingSystem::AmdHsa => wasm::OperatingSystem::Amdhsa,
            rust::OperatingSystem::Bitrig => wasm::OperatingSystem::Bitrig,
            rust::OperatingSystem::Cloudabi => wasm::OperatingSystem::Cloudabi,
            rust::OperatingSystem::Cuda => wasm::OperatingSystem::Cuda,
            rust::OperatingSystem::Darwin(o) => {
                wasm::OperatingSystem::Darwin(o.map(|d| d.into_wasm()))
            }
            rust::OperatingSystem::Dragonfly => wasm::OperatingSystem::Dragonfly,
            rust::OperatingSystem::Emscripten => wasm::OperatingSystem::Emscripten,
            rust::OperatingSystem::Espidf => wasm::OperatingSystem::Espidf,
            rust::OperatingSystem::Freebsd => wasm::OperatingSystem::Freebsd,
            rust::OperatingSystem::Fuchsia => wasm::OperatingSystem::Fuchsia,
            rust::OperatingSystem::Haiku => wasm::OperatingSystem::Haiku,
            rust::OperatingSystem::Hermit => wasm::OperatingSystem::Hermit,
            rust::OperatingSystem::Horizon => wasm::OperatingSystem::Horizon,
            rust::OperatingSystem::Hurd => wasm::OperatingSystem::Hurd,
            rust::OperatingSystem::Illumos => wasm::OperatingSystem::Illumos,
            rust::OperatingSystem::IOS(o) => wasm::OperatingSystem::Ios(o.map(|d| d.into_wasm())),
            rust::OperatingSystem::L4re => wasm::OperatingSystem::L4re,
            rust::OperatingSystem::Linux => wasm::OperatingSystem::Linux,
            rust::OperatingSystem::MacOSX(o) => {
                wasm::OperatingSystem::Macosx(o.map(|d| d.into_wasm()))
            }
            rust::OperatingSystem::Nebulet => wasm::OperatingSystem::Nebulet,
            rust::OperatingSystem::Netbsd => wasm::OperatingSystem::Netbsd,
            rust::OperatingSystem::None_ => wasm::OperatingSystem::None,
            rust::OperatingSystem::Openbsd => wasm::OperatingSystem::Openbsd,
            rust::OperatingSystem::Psp => wasm::OperatingSystem::Psp,
            rust::OperatingSystem::Redox => wasm::OperatingSystem::Redox,
            rust::OperatingSystem::Solaris => wasm::OperatingSystem::Solaris,
            rust::OperatingSystem::SolidAsp3 => wasm::OperatingSystem::Solidasp3,
            rust::OperatingSystem::TvOS(o) => wasm::OperatingSystem::Tvos(o.map(|d| d.into_wasm())),
            rust::OperatingSystem::Uefi => wasm::OperatingSystem::Uefi,
            rust::OperatingSystem::VisionOS(o) => {
                wasm::OperatingSystem::Visionos(o.map(|d| d.into_wasm()))
            }
            rust::OperatingSystem::VxWorks => wasm::OperatingSystem::Vxworks,
            rust::OperatingSystem::Wasi => wasm::OperatingSystem::Wasi,
            rust::OperatingSystem::WasiP1 => wasm::OperatingSystem::Wasip1,
            rust::OperatingSystem::WasiP2 => wasm::OperatingSystem::Wasip2,
            rust::OperatingSystem::WatchOS(o) => {
                wasm::OperatingSystem::Watchos(o.map(|d| d.into_wasm()))
            }
            rust::OperatingSystem::Windows => wasm::OperatingSystem::Windows,
            rust::OperatingSystem::XROS(o) => wasm::OperatingSystem::Xros(o.map(|d| d.into_wasm())),
            _ => todo!(),
        }
    }
}

// unknown,
// amdgiz,
// android,
// androideabi,
// eabi,
// eabihf,
// gnu,
// gnuabi64,
// gnueabi,
// gnueabihf,
// gnuspe,
// gnux32,
// gnuilp32,
// gnullvm,
// hermitkernel,
// hurdkernel,
// linuxkernel,
// macabi,
// musl,
// musleabi,
// musleabihf,
// muslabi64,
// nsvc,
// newlib,
// none,
// kernel,
// uclibc,
// uclibceabi,
// uclibceabihf,
// sgx,
// sim,
// softfloat,
// spe,
// threads,
// ohos,
impl IntoRust for wasm::Environment {
    type RustType = rust::Environment;
    fn into_rust(self) -> rust::Environment {
        match self {
            wasm::Environment::Unknown => rust::Environment::Unknown,
            wasm::Environment::Amdgiz => rust::Environment::AmdGiz,
            wasm::Environment::Android => rust::Environment::Android,
            wasm::Environment::Androideabi => rust::Environment::Androideabi,
            wasm::Environment::Eabi => rust::Environment::Eabi,
            wasm::Environment::Eabihf => rust::Environment::Eabihf,
            wasm::Environment::Gnu => rust::Environment::Gnu,
            wasm::Environment::Gnuabi64 => rust::Environment::Gnuabi64,
            wasm::Environment::Gnueabi => rust::Environment::Gnueabi,
            wasm::Environment::Gnueabihf => rust::Environment::Gnueabihf,
            wasm::Environment::Gnuspe => rust::Environment::Gnuspe,
            wasm::Environment::Gnux32 => rust::Environment::Gnux32,
            wasm::Environment::Gnuilp32 => rust::Environment::GnuIlp32,
            wasm::Environment::Gnullvm => rust::Environment::GnuLlvm,
            wasm::Environment::Hermitkernel => rust::Environment::HermitKernel,
            wasm::Environment::Hurdkernel => rust::Environment::HurdKernel,
            wasm::Environment::Linuxkernel => rust::Environment::LinuxKernel,
            wasm::Environment::Macabi => rust::Environment::Macabi,
            wasm::Environment::Musl => rust::Environment::Musl,
            wasm::Environment::Musleabi => rust::Environment::Musleabi,
            wasm::Environment::Musleabihf => rust::Environment::Musleabihf,
            wasm::Environment::Muslabi64 => rust::Environment::Muslabi64,
            wasm::Environment::Msvc => rust::Environment::Msvc,
            wasm::Environment::Newlib => rust::Environment::Newlib,
            wasm::Environment::None => rust::Environment::None,
            wasm::Environment::Kernel => rust::Environment::Kernel,
            wasm::Environment::Uclibc => rust::Environment::Uclibc,
            wasm::Environment::Uclibceabi => rust::Environment::Uclibceabi,
            wasm::Environment::Uclibceabihf => rust::Environment::Uclibceabihf,
            wasm::Environment::Sgx => rust::Environment::Sgx,
            wasm::Environment::Sim => rust::Environment::Sim,
            wasm::Environment::Softfloat => rust::Environment::Softfloat,
            wasm::Environment::Spe => rust::Environment::Spe,
            wasm::Environment::Threads => rust::Environment::Threads,
            wasm::Environment::Ohos => rust::Environment::Ohos,
        }
    }
}
impl IntoWasm for rust::Environment {
    type WasmType = wasm::Environment;
    fn into_wasm(self) -> wasm::Environment {
        match self {
            rust::Environment::Unknown => wasm::Environment::Unknown,
            rust::Environment::AmdGiz => wasm::Environment::Amdgiz,
            rust::Environment::Android => wasm::Environment::Android,
            rust::Environment::Androideabi => wasm::Environment::Androideabi,
            rust::Environment::Eabi => wasm::Environment::Eabi,
            rust::Environment::Eabihf => wasm::Environment::Eabihf,
            rust::Environment::Gnu => wasm::Environment::Gnu,
            rust::Environment::Gnuabi64 => wasm::Environment::Gnuabi64,
            rust::Environment::Gnueabi => wasm::Environment::Gnueabi,
            rust::Environment::Gnueabihf => wasm::Environment::Gnueabihf,
            rust::Environment::Gnuspe => wasm::Environment::Gnuspe,
            rust::Environment::Gnux32 => wasm::Environment::Gnux32,
            rust::Environment::GnuIlp32 => wasm::Environment::Gnuilp32,
            rust::Environment::GnuLlvm => wasm::Environment::Gnullvm,
            rust::Environment::HermitKernel => wasm::Environment::Hermitkernel,
            rust::Environment::HurdKernel => wasm::Environment::Hurdkernel,
            rust::Environment::LinuxKernel => wasm::Environment::Linuxkernel,
            rust::Environment::Macabi => wasm::Environment::Macabi,
            rust::Environment::Musl => wasm::Environment::Musl,
            rust::Environment::Musleabi => wasm::Environment::Musleabi,
            rust::Environment::Musleabihf => wasm::Environment::Musleabihf,
            rust::Environment::Muslabi64 => wasm::Environment::Muslabi64,
            rust::Environment::Msvc => wasm::Environment::Msvc,
            rust::Environment::Newlib => wasm::Environment::Newlib,
            rust::Environment::None => wasm::Environment::None,
            rust::Environment::Kernel => wasm::Environment::Kernel,
            rust::Environment::Uclibc => wasm::Environment::Uclibc,
            rust::Environment::Uclibceabi => wasm::Environment::Uclibceabi,
            rust::Environment::Uclibceabihf => wasm::Environment::Uclibceabihf,
            rust::Environment::Sgx => wasm::Environment::Sgx,
            rust::Environment::Sim => wasm::Environment::Sim,
            rust::Environment::Softfloat => wasm::Environment::Softfloat,
            rust::Environment::Spe => wasm::Environment::Spe,
            rust::Environment::Threads => wasm::Environment::Threads,
            rust::Environment::Ohos => wasm::Environment::Ohos,
            _ => todo!(),
        }
    }
}

impl IntoRust for wasm::BinaryFormat {
    type RustType = rust::BinaryFormat;
    fn into_rust(self) -> rust::BinaryFormat {
        match self {
            wasm::BinaryFormat::Unknown => rust::BinaryFormat::Unknown,
            wasm::BinaryFormat::Elf => rust::BinaryFormat::Elf,
            wasm::BinaryFormat::Coff => rust::BinaryFormat::Coff,
            wasm::BinaryFormat::Macho => rust::BinaryFormat::Macho,
            wasm::BinaryFormat::Wasm => rust::BinaryFormat::Wasm,
            wasm::BinaryFormat::Xcoff => rust::BinaryFormat::Xcoff,
        }
    }
}
impl IntoWasm for rust::BinaryFormat {
    type WasmType = wasm::BinaryFormat;
    fn into_wasm(self) -> wasm::BinaryFormat {
        match self {
            rust::BinaryFormat::Unknown => wasm::BinaryFormat::Unknown,
            rust::BinaryFormat::Elf => wasm::BinaryFormat::Elf,
            rust::BinaryFormat::Coff => wasm::BinaryFormat::Coff,
            rust::BinaryFormat::Macho => wasm::BinaryFormat::Macho,
            rust::BinaryFormat::Wasm => wasm::BinaryFormat::Wasm,
            rust::BinaryFormat::Xcoff => wasm::BinaryFormat::Xcoff,
            _ => todo!(),
        }
    }
}
