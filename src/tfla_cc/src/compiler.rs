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
        let res = format!("se {} {}", &self.name[..], &self.regex[..]);
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

#[derive(Debug, Clone)]
pub struct Assembler {
    name: String,
    arbitrary: Vec<String>,

    line: usize,
}
impl Assembler {
    pub fn new(name: String, arbitrary: Vec<String>) -> Self {
        Assembler { name, arbitrary, line: 0 }
    }

    pub fn from(text: String) -> Self {
        let parts: Vec<String> = text.split(" ").map(|a| a.to_string()).collect();
        let line: usize = parts[0].clone().to_string().parse::<usize>().unwrap();
        let name: String = parts[2].clone();
        let arbitrary: Vec<String> = parts[3..].to_vec();

        Assembler { name: String::from(name), arbitrary, line }
    }

    pub fn mount(&self) -> String {
        let mut res = format!("as {}", &self.name[..]);

        for a in &self.arbitrary {
            let last = a.len() - 1;

            if a.starts_with("<") && a.len() > 2 {
                res = format!("{} as-{}", res, &a[1..last]);
            } else if a.starts_with("[") && a.len() > 2 {
                res = format!("{} se-{}", res, &a[1..last]);
            } else if a.starts_with(":") && a.len() > 2 {
                res = format!("{} sy-{}", res, &a[1..last]);
            } else {
                res = format!("{} li-{}", res, a);
            }
        }

        res
    }

    pub fn add_arbitrary(&mut self, arbitrary: &str) {
        self.arbitrary.push(arbitrary.to_string());
    }

    pub fn have_arbitrary(&self, arbitrary: &str) -> bool {
        self.arbitrary.contains(&arbitrary.to_string())
    }

    pub fn arbitrary(&self) -> Vec<String> {
        self.arbitrary.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn contain(assemblers: &Vec<Assembler>, value: &str) -> bool {
        for a in assemblers {
            if a.name() == value {
                return true
            }
        }

        false
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
        let res = format!("sy {} {}", &self.name[..], &self.arbitrary[..]);

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
        TflaCC {
            code,
            searchers,
        }
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
        as_: &mut Assembler,
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
        line: usize,
    ) -> String {
        let res;
        if active == "searcher" {
            res = format!("\n{} {}", line, se.mount()).to_string();
        } else if active == "assembler" {
            res = format!("\n{} {}", line, as_.mount()).to_string();
        } else if active == "symbol" {
            res = format!("\n{} {}", line, sy.mount()).to_string();
        } else {
            return String::new();
        }

        *active = "".to_string();
        res
    }

    fn parse(&self) -> String {
        let mut res = String::from(r"0 sy nwl (\r)?\n
0 sy eof \z
0 sy eol $
0 sy tab \t 
0 sy noh \0
0 sy num \d+");

        let tokens = self.tokenize();

        let mut token_type: String = String::from("");
        let mut tk_num: i16 = 0;
        let mut in_comment: bool = false;
        let mut def_line: usize = 0;

        let mut searcher: Searcher = Searcher::new("", "");
        let mut assembler: Assembler = Assembler::new("".to_string(), vec![]);
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
                        res += &self.mount_this(
                            &mut token_type,
                            &mut searcher,
                            &mut assembler,
                            &mut symbol,
                            def_line,
                        );
                        let last = content.len() - 1;
                        token_type = "searcher".to_string();
                        searcher = Searcher::new(&content[1..last], "");
                        def_line = *line;
                    }
                    "assembler" => {
                        res += &self.mount_this(
                            &mut token_type,
                            &mut searcher,
                            &mut assembler,
                            &mut symbol,
                            def_line,
                        );
                        let last = content.len() - 1;
                        token_type = "assembler".to_string();
                        assembler = Assembler::new(content[1..last].to_string(), vec![]);
                        def_line = *line;
                    }
                    "symbol" => {
                        res += &self.mount_this(
                            &mut token_type,
                            &mut searcher,
                            &mut assembler,
                            &mut symbol,
                            def_line,
                        );
                        let last = content.len() - 1;
                        token_type = "symbol".to_string();
                        symbol = Symbol::new(&content[1..last], "");
                        def_line = *line;
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
        res += &self.mount_this(
            &mut token_type,
            &mut searcher,
            &mut assembler,
            &mut symbol,
            def_line,
        );


        res.trim().to_string()
    }

    fn get_assembler(&self, name: &str, assemblers: &Vec<Assembler>) -> Vec<Assembler> {
        let mut res: Vec<Assembler> = vec![];

        for a in assemblers {
            if a.name() == name {
                res.push(a.clone());
            }
        }

        res
    }

    fn found_circular_exp(&self, name: String, assembler: &Assembler, assemblers: &Vec<Assembler>) {
        if assembler.have_arbitrary(&format!("as-{}", name)[..]) {
            if assembler.name() == name {
                println!("\nImpossible Case! The Assembler <{}> have a expansion to itself! |{}|", name, assembler.line);
                exit(0);
            }else {
                println!("\nImpossible Case! The Assembler <{}> is part of an circular expansion! |{}|", name, assembler.line);
                exit(0);
            }
        }

        let arbitraries = assembler.arbitrary();

        for arbitrary in arbitraries {
            if arbitrary.starts_with("as-") {
                let definitions = self.get_assembler(&arbitrary[3..], assemblers);

                for def in definitions {
                    self.found_circular_exp(name.clone(), &def, assemblers);
                }
            }
        }
    }

    fn contain(&self, assemblers: &Vec<(String, usize)>, name: &str) -> bool {
        for (a, _) in assemblers {
            if a == name {
                return true
            }
        }

        false
    }

    pub fn digest(&self) -> String {
        let code = self.parse();

        let mut lines: Vec<String> = code.split("\n").map(|a| a.to_string()).collect();
        for line in &mut lines {
            let parts: Vec<&str> = line.split(" ").collect();

            *line = parts[1..].join(" ").to_string();
        }

        lines.join("\n")
    }

    pub fn analyse(&self) -> String {
        let code = self.parse();

        let mut symbols: Vec<(String, usize)> = vec![];
        let mut searchers: Vec<(String, usize)> = vec![];
        let mut assemblers: Vec<Assembler> = vec![];

        let mut lines: Vec<String> = code.split("\n").map(|a| a.to_string()).collect();

        for line in &mut lines {
            let parts: Vec<String> = line.split(" ").map(|a| a.to_string()).collect();
            let num_line = parts[0].parse::<usize>().unwrap();
            let prefix = &parts[1];
            let name = &parts[2];

            if prefix == "sy" {
                symbols.push((name.clone(), num_line));
            } else if prefix == "se" {
                searchers.push((name.clone(), num_line));
            } else if prefix == "as" {
                assemblers.push(Assembler::from(line.clone()));
            }

            *line = parts[1..].join(" ");
        }

        for assembler in &assemblers.clone() {
            let name = assembler.name();

            let arbitraries = assembler.arbitrary();

            for arbitrary in &arbitraries {
                let prefix = &arbitrary[..3];
                let value = &arbitrary[3..];

                if prefix == "sy-" && !self.contain(&symbols, &value) {
                    println!("\nERROR: The Assembler <{}> have an expansion to Symbol :{}:, but it's don't exists. |{}|",
                        name, 
                        value, assembler.line);
                    exit(0); 
                }else if prefix == "se-" && !self.contain(&searchers, &value) {
                    println!("\nERROR: The Assembler <{}> have an expansion to Searcher [{}], but it's don't exists. |{}|",
                        name, 
                        value, assembler.line);
                    exit(0); 
                }else if prefix == "as-" {
                    if !Assembler::contain(&assemblers, &value) {
                        println!("\nERROR: The Assembler <{}> have an expansion to Assembler <{}>, but it's don't exists. |{}|",
                            name, 
                            value, assembler.line);

                        exit(0);
                    }

                    self.found_circular_exp(name.clone(), &assembler, &assemblers);
                }
            }
        }

        lines.join("\n")
    }
}
