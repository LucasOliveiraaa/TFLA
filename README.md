# TFLA
TFLA - Token by Full Line Analysis, is a fast tokenizer and AST generator originaly writen in Javascript, but now, in Rust. The TFLA algorithm works by analysing the source code and then found patterns to create configured tokens or an AST(abstract sintax tree).

You can configure the TFLA algorithm manualy or use the TFLA CC to compile a TFLA Config(TFLAC) file. The manual to use TFLA CC are [here](./src/tfla_cc)

Here is a file map reference:

tfla - Here lives the TFLA Algorithm
├─ src - This folder have all source code to TFLA Algorithm.
│   ├─ tfla_cc - Here lives the TFLA CC, or, TFLA Config "Compiler"
│   │   ├─ src - The source code to TFLA TFLA CC
│   │   │   │   └─ tfla.rs - The TFLA Tokenizer used by TFLA CC
│   │   │   ├─ compiler - Manager all modules to execute in harmony
│   │   │   ├─ main.rs - Manager and configure TFLA CC to execute in harmony
│   │   │   └─ compiler.rs - Have the main implemantation of the TFLA CC
│   │   ├─ Cargo.toml
│   │   ├─ Cargo.lock
│   │   ├─ LICENSE
│   │   └─ README.md
│   ├─ main.rs - Have the implemantation of the TFLA Tokenizer
│   └─ ast.rs - Have the implemantation of the TFLA ASTGen
├─ Cargo.toml
├─ Cargo.lock
├─ LICENSE
└─ README.md
