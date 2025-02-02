# target-lexicon for WebAssembly

🎯 Target "triple" support for WASM

<table align=center><td>

```wit
record triple {
    architecture: architecture,
    vendor: vendor,
    operating-system: operating-system,
    environment: environment,
    binary-format: binary-format,
}
triple-from-str: func(s: string) -> result<triple, parse-error>;
```

</table>

🦀 Wraps the Rust [target-lexicon](https://docs.rs/target-lexicon/latest/target_lexicon/) \
💻 Parse LLVM triples with ease \
🦄 Supports all the obscure platforms and CPUs \
🔥 A portable WebAssembly component

## Installation

![curl](https://img.shields.io/badge/curl-073551?style=for-the-badge&logo=curl&logoColor=FFFFFF)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=FFFFFF)

Download the `target-lexicon.wasm` artifact from [the GitHub releases page](https://github.com/jcbhmr/target-lexicon-wasm/releases).

```sh
curl --remote-name https://github.com/jcbhmr/target-lexicon-wasm/releases/latest/download/target-lexicon.wasm
```

You'll probably end up either a) downloading the `target-lexicon.wasm` WASM component at compile-time or b) vendoring the `target-lexicon.wasm` WASM component in your Git repository.

## Usage

![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=FFFFFF)
![JavaScript](https://img.shields.io/badge/JavaScript-222222?style=for-the-badge&logo=JavaScript&logoColor=F7DF1E)
![Go](https://img.shields.io/badge/Go-00ADD8?style=for-the-badge&logo=Go&logoColor=FFFFFF)

<div align=center><b>You're probably looking for premade ergonomic bindings for your language of choice.</b></div>
<table align=center><td>

- JavaScript: https://www.npmjs.com/package/target-lexicon
- Go: https://pkg.go.dev/github.com/jcbhmr/go-targetlexicon

</table>

[📚 See the full WIT API documentation](https://github.com/jcbhmr/target-lexicon-wasm/blob/main/wit/target-lexicon.wit)

You can use tools like [Jco](https://github.com/bytecodealliance/jco) to generate bindings for your favorite language to the WebAssembly component. Read the development section below for more information on how some of the Rust ideas have been ported to WIT. You may wish to extend or wrap the generated bindings in more language-specific idioms.

**`host()` and other host-related functions return WebAssembly characteristics,** not the underlying native platform characteristics. The WebAssembly component is compiled for the `wasm32-wasi` target, and as such `host()` will return `wasm32-wasi` as a target triple struct with no idea of the underlying host platform which could be Windows x86-64, macOS ARM64, Linux RISCV musl, or something else. It is recommended that you wrap or replace `host()` and friends to instead somehow get and parse the native platform's target triple.

## Development

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=FFFFFF)

This WASM component enables the `serde_support` and `arch_zkasm` features of the original [target-lexicon](https://docs.rs/target-lexicon/latest/target_lexicon/) Cargo package.

Some conventions for porting Rust definitions to WIT:

- Map Rust enum methods to free functions prefixed with the enum name.
- Map Rust enums to variants if the enum contains data.
- Map Rust structs to WIT structs if all of the fields are public.
- Map Rust structs to WIT resources if any of the fields are internal or private.
- If a Rust struct is mapped to a WIT struct then the struct's methods are free functions prefixed with the struct's name.
- The `serde::Serialize` and `serde::Deserialize` traits are replaced with `<type>-serialize-json` and `<type>-deserialize-json` functions. This means that the `serde_json` package becomes a dependency.
- Any `Result<T, ()>` types are replaced with `result<T, bool>` types where the boolean side is always false. WIT doesn't have a void or undefined type so this is the closest approximation.
- Any `Result<T, impl Error>` types are replaced with `result<T, string>` types. Stringify the error message.
- Any internal type that is exposed is instead replaced with a void type. Since WIT doesn't have void types, use an always-false boolean type instead.

Try to keep this project's version tags in step with the target-lexicon project's versions. This means v1.0.0 of this project should correspond with target-lexicon v1.0.0 with the same API surface.

To create a release:

1. Make sure the `Cargo.toml` version field is what you want.
2. Run the "Create a release" workflow manually.
3. Wait for it to run to completion.
4. Check the releases page on GitHub to see the new release.
