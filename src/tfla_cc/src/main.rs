use std::env;
use std::fs;

mod compiler;
use compiler::{Searcher, TflaCC};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("ERROR: Any file .tflac was passed!");
    }

    let content = fs::read_to_string(&args[1]).expect("Can't open the file.");

    let mut c_searchers: Vec<(&str, &str)> = vec![];
    c_searchers.push(Searcher::new("comment", r"^\-\-").transform());
    c_searchers.push(Searcher::new("NEW_LINE", r"^(\r)?\n").transform());
    c_searchers.push(Searcher::new("SPACE", r"^\s").transform());
    c_searchers.push(Searcher::new("searcher", r"^\[(\w|_)+\]").transform());
    c_searchers.push(Searcher::new("assembler", r"^<(\w|_)+>").transform());
    c_searchers.push(Searcher::new("symbol", r"^\:(\w|_)+\:").transform());
    c_searchers.push(Searcher::new("colon", r"^\:").transform());
    c_searchers.push(Searcher::new("colon", r"^\:r").transform());
    c_searchers.push(Searcher::new("pipe", r"^\|").transform());
    c_searchers.push(Searcher::new("entity", r"^[^\s]*").transform());

    let cc: TflaCC = TflaCC::new(&content[..], c_searchers);

    cc.analyze();
}
