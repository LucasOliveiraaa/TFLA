# TFLA
TFLA - Token by Full Line Analysis, is a fast tokenizer and AST generator originaly writen in Javascript, but now, in Rust. The TFLA algorithm works by analysing the source code and then found patterns to create configured tokens or an AST(abstract sintax tree).

You can configure the TFLA algorithm manualy or use the TFLA CC to compile a TFLA Config(TFLAC) file. The manual to use TFLA CC are [here](./src/tfla_cc)<br>

Here is a file map reference:<br><br>

tfla - Here lives the TFLA Algorithm<br>
├─ src - This folder have all source code to TFLA Algorithm.<br>
│   ├─ tfla_cc - Here lives the TFLA CC, or, TFLA Config "Compiler"<br>
│   │   ├─ src - The source code to TFLA TFLA CC<br>
│   │   │   │   └─ tfla.rs - The TFLA Tokenizer used by TFLA CC<br>
│   │   │   ├─ compiler - Manager all modules to execute in harmony<br>
│   │   │   ├─ main.rs - Manager and configure TFLA CC to execute in harmony<br>
│   │   │   └─ compiler.rs - Have the main implemantation of the TFLA CC<br>
│   │   ├─ Cargo.toml<br>
│   │   ├─ Cargo.lock<br>
│   │   ├─ LICENSE<br>
│   │   └─ README.md<br>
│   ├─ main.rs - Have the implemantation of the TFLA Tokenizer<br>
│   └─ ast.rs - Have the implemantation of the TFLA ASTGen<br>
├─ Cargo.toml<br>
├─ Cargo.lock<br>
├─ LICENSE<br>
└─ README.md<br>
