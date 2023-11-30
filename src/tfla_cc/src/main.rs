use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::process::exit;

fn compress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut compressed_data = Vec::new();

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    compressed_data.extend_from_slice(&encoder.finish()?);

    Ok(compressed_data)
}

mod compiler;
use compiler::{Searcher, TflaCC};

fn show_help() {
    println!(
        "
TFLA CC - TFLA Config \"Compiler\"

tfla-cc <command> [arguments]

General Commands:
    --help | -h 
        Arguments: [command]
        Show this message or show the help of some command.
    --digest | -d
        Arguments: <path> [-a] [-w|-W] [output]
        Compile a TFLAC source and show the result.
    --analyse | -a 
        Arguments: <path>
        Just analyse the source code to syntax and impossible cases.
    --compile | -c 
        Arguments: <path>
        Compile the passed source code with type .tflac to an .exaust.tfla file with same name.
        aka tfla-cc -d <path> -a -W"
    );
}

fn write_file(file: String, content: String) -> std::io::Result<()> {
    let mut f = File::create(file.clone())?;

    let byt = content.as_bytes();

    f.write_all(&compress(byt)?)?;

    println!("Exaust in {}", file);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();

        exit(0);
    }

    let co = &args[1];

    if co == "-h" || co == "--help" {
        show_help();

        exit(0);
    }

    let mut analyse = false;
    let mut write_same = false;
    let mut write_in = false;
    let mut output = String::from("");
    let mut input = String::from("");

    #[allow(unused_assignments)]
    for arg in &args[2..] {
        if co == "-d" || co == "--digest" {
            if arg == "-a" {
                analyse = true;
            } else if arg == "-w" {
                write_in = true;
            } else if arg == "-W" {
                write_same = true;
            } else {
                if input == "".to_string() {
                    input = arg.to_string();
                } else {
                    output = arg.to_string();
                }
            }
        } else {
            input = arg.to_string();
        }
    }
    #[warn(unused_assignments)]
    if input == "" {
        println!("Pass an input file.");
        exit(0);
    }

    let content = fs::read_to_string(input.clone()).expect("Can't open the input file");

    let mut c_searchers: Vec<(&str, &str)> = vec![];
    c_searchers.push(Searcher::new("comment", r"^\-\-").transform());
    c_searchers.push(Searcher::new("NEW_LINE", r"^(\r)?\n").transform());
    c_searchers.push(Searcher::new("SPACE", r"^\s").transform());
    c_searchers.push(Searcher::new("searcher", r"^\[(\w|_)+\]").transform());
    c_searchers.push(Searcher::new("assembler", r"^<(\w|_)+>").transform());
    c_searchers.push(Searcher::new("symbol", r"^\:(\w|_)+\:").transform());
    c_searchers.push(Searcher::new("colon", r"^(:|:r|::=)").transform());
    c_searchers.push(Searcher::new("pipe", r"^\|").transform());
    c_searchers.push(Searcher::new("entity", r"^[^\s]*").transform());

    let cc: TflaCC = TflaCC::new(&content[..], c_searchers);

    if co == "-d" || co == "--digest" {
        let res: String;

        if analyse {
            res = cc.analyse();
        } else {
            res = cc.digest();
        }

        if write_in {
            if output == "" {
                println!("Pass an output file.");
                exit(0);
            } else {
                let _ = write_file(output, res);
            }
        } else if write_same {
            let mut parts: Vec<String> = input.split(".").map(|a| a.to_string()).collect();

            let last = parts.len() - 1;
            parts[last] = "exaust.tfla".to_string();

            let _ = write_file(parts.join("."), res);
        } else {
            println!("{}", res);
        }
    }

    if co == "-a" || co == "--analyse" {
        cc.analyse();

        println!("\nEnd with 0 Error and 0 Warns");
    }

    if co == "-c" || co == "--compile" {
        let res = cc.analyse();

        let mut parts: Vec<String> = input.split(".").map(|a| a.to_string()).collect();

        let last = parts.len() - 1;
        parts[last] = "exaust.tfla".to_string();

        let _ = write_file(parts.join("."), res);
    }
}
