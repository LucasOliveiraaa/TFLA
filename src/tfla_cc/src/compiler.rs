mod tfla;
use tfla::{Token, TFLA};

use std::process::exit;

pub struct Searcher<'a> {
    name: &'a str,
    regex: &'a str,
}

impl<'a> Searcher<'a> {
    pub fn new(name: &'a str, regex: &'a str) -> Self {
        Searcher { name, regex }
    }

    pub fn mount(&self) -> String {
        let res = format!("se: {} r({})", &self.name[..], &self.regex[..]);
        res
    }

    pub fn transform(&self) -> (&'a str, &'a str) {
        let name = &self.name[..];
        let regex = &self.regex[..];

        (name, regex)
    }

    pub fn add_arbitrary(&mut self, regex: &'a str) {
        self.regex = regex;
    }
}

pub struct Assembler<'a> {
    name: &'a str,
    arbitrary: Vec<&'a str>,
}
impl<'a> Assembler<'a> {
    pub fn new(name: &'a str, arbitrary: Vec<&'a str>) -> Self {
        Assembler { name, arbitrary }
    }

    pub fn mount(&self) -> String {
        let mut res = format!("as: {}", &self.name[..]);

        for a in &self.arbitrary {
            let last = a.len() - 1;

            if a.starts_with("<") && a.len() > 2 {
                if &a[1..last] == self.name {
                    println!(
                        "\n>> Impossible Case! The assembler <{}> have a reference to itself!\n",
                        self.name
                    );

                    exit(1);
                }

                res = format!("{} as({})", res, &a[1..last]);
            } else if a.starts_with("[") && a.len() > 2 {
                res = format!("{} se({})", res, &a[1..last]);
            } else if a.starts_with(":") && a.len() > 2 {
                res = format!("{} sy({})", res, &a[1..last]);
            } else {
                res = format!("{} l({})", res, a);
            }
        }

        res
    }

    pub fn add_arbitrary(&mut self, arbitrary: &'a str) {
        self.arbitrary.push(arbitrary);
    }
}

pub struct Symbol<'a> {
    name: &'a str,
    arbitrary: &'a str,
}
impl<'a> Symbol<'a> {
    pub fn new(name: &'a str, arbitrary: &'a str) -> Self {
        Symbol { name, arbitrary }
    }

    pub fn mount(&self) -> String {
        let res = format!("sy: {} r({})", &self.name[..], &self.arbitrary[..]);

        res
    }

    pub fn add_arbitrary(&mut self, arbitrary: &'a str) {
        self.arbitrary = arbitrary;
    }
}

pub struct TflaCC<'a> {
    code: &'a str,
    searchers: Vec<(&'a str, &'a str)>,
}

impl<'a> TflaCC<'a> {
    pub fn new(code: &'a str, searchers: Vec<(&'a str, &'a str)>) -> Self {
        TflaCC { code, searchers }
    }

    pub fn tokenize(&self) -> Vec<Token<'a>> {
        let mut tokens: Vec<Token> = vec![];

        let mut cc: TFLA = TFLA::new(self.searchers.clone(), vec![], true);

        cc.tokenize(self.code);

        for t in &cc.tokens {
            tokens.push(*t);
        }

        tokens
    }

    fn add_to(
        &self,
        active: &String,
        value: &'a str,
        se: &mut Searcher<'a>,
        as_: &mut Assembler<'a>,
        sy: &mut Symbol<'a>,
    ) {
        match &active[..] {
            "searcher" => se.add_arbitrary(value),
            "assembler" => as_.add_arbitrary(value),
            "symbol" => sy.add_arbitrary(value),
            &_ => (),
        }
    }

    fn mount_this(
        &self,
        active: &mut String,
        se: &mut Searcher,
        as_: &mut Assembler,
        sy: &mut Symbol,
    ) {
        if active == "searcher" {
            println!("{}", se.mount())
        } else if active == "assembler" {
            println!("{}", as_.mount())
        } else if active == "symbol" {
            println!("{}", sy.mount())
        } else {
            ()
        }

        *active = "".to_string();
    }

    pub fn analyze(&self) {
        let tokens = self.tokenize();

        let mut token_type: String = String::from("");
        let mut tk_num: i16 = 0;
        let mut in_comment: bool = false;

        let mut searcher: Searcher = Searcher::new("", "");
        let mut assembler: Assembler = Assembler::new("", vec![]);
        let mut symbol: Symbol = Symbol::new("", "");

        for Token {
            ref content,
            ref ty,
            ref line,
            ref start,
            ..
        } in &tokens
        {
            tk_num += 1;
            if ty == &"NEW_LINE" {
                tk_num = 0;
                in_comment = false;
            } else if tk_num == 1 && !in_comment {
                match *ty {
                    "comment" => {
                        in_comment = true;
                        continue;
                    }
                    "searcher" => {
                        self.mount_this(
                            &mut token_type,
                            &mut searcher,
                            &mut assembler,
                            &mut symbol,
                        );
                        let last = content.len() - 1;
                        token_type = "searcher".to_string();
                        searcher = Searcher::new(&content[1..last], "");
                    }
                    "assembler" => {
                        self.mount_this(
                            &mut token_type,
                            &mut searcher,
                            &mut assembler,
                            &mut symbol,
                        );
                        let last = content.len() - 1;
                        token_type = "assembler".to_string();
                        assembler = Assembler::new(&content[1..last], vec![]);
                    }
                    "symbol" => {
                        self.mount_this(
                            &mut token_type,
                            &mut searcher,
                            &mut assembler,
                            &mut symbol,
                        );
                        let last = content.len() - 1;
                        token_type = "symbol".to_string();
                        symbol = Symbol::new(&content[1..last], "");
                    }
                    "colon" | "pipe" => {
                        if token_type != "assembler" {
                            println!("\nWARN: The token \":\" was typing an AB, this operation was ocourring in a {}, but only Assemblers support this action. (IGNORED DURING ANALYSIS) |{} row {}|\n", token_type, line, start);
                            continue;
                        }
                        println!("{}", assembler.mount());
                    }
                    &_ => {
                        println!(
                            "\nERROR: Token \"{}\"({}) don't match to any AB Type. |{} row {}|\n",
                            content, ty, line, start
                        );
                        exit(1);
                    }
                }
            } else if tk_num > 2 && !in_comment {
                self.add_to(
                    &token_type,
                    &content,
                    &mut searcher,
                    &mut assembler,
                    &mut symbol,
                );
            }
        }
        self.mount_this(&mut token_type, &mut searcher, &mut assembler, &mut symbol);
    }
}
