<h1 align="center">
  spirv_cross
</h1>
<div align="center">
  Safe wrapper around <a href="https://github.com/KhronosGroup/SPIRV-Cross">SPIR-V Cross</a>
</div>
<br />
<div align="center">
  <a href="https://crates.io/crates/spirv_cross"><img src="http://img.shields.io/crates/v/spirv_cross.svg?label=spirv_cross" alt="Crate"></a> <a href="https://travis-ci.org/grovesNL/spirv_cross"><img src="https://travis-ci.org/grovesNL/spirv_cross.svg?branch=master" alt="Travis Build Status" /></a> <a href="https://ci.appveyor.com/project/grovesNL/spirv-cross/branch/master"><img src="https://ci.appveyor.com/api/projects/status/ja22j0ueje51sd76/branch/master?svg=true" alt="Appveyor Build Status" /></a>
</div>

## Example

`spirv_cross` provides a safe wrapper around [SPIRV-Cross](https://github.com/KhronosGroup/SPIRV-Cross) for use with Rust. For example, here is a simple function to parse a SPIR-V module and compile it to HLSL:

```rust
extern crate spirv_cross;
use spirv_cross::spirv;
use spirv_cross::hlsl;

fn generate_hlsl(module: spirv::Module) {
    // Parse a SPIR-V module
    let parsed_module = spirv::Parser::new()
        .parse(&module, &spirv::ParserOptions::default())
        .unwrap();

    // Compile to HLSL
    let hlsl = hlsl::Compiler::new()
        .compile(&parsed_module, &hlsl::CompilerOptions::default())
        .unwrap();

    println!("{}", hlsl);
}
```
