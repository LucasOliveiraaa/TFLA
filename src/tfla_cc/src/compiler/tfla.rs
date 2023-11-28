use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub ty: &'a str,
    pub content: &'a str,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl<'a> Token<'a> {
    pub fn new(ty: &'a str, content: &'a str, line: usize, start: usize, end: usize) -> Self {
        Token {
            ty,
            content,
            line,
            start,
            end,
        }
    }
}

pub struct TFLA<'a> {
    pub searchers: Vec<(&'a str, Regex)>,
    pub assemblers: Vec<(&'a str, Vec<&'a str>)>,
    pub ignore_spaces: bool,

    in_line_comment: bool,
    in_block_comment: bool,

    line: usize,
    row: usize,

    pub tokens: Vec<Token<'a>>,
}

impl<'a> TFLA<'a> {
    pub fn new(
        searchers: Vec<(&'a str, &'a str)>,
        assemblers: Vec<(&'a str, Vec<&'a str>)>,
        ignore_spaces: bool,
    ) -> Self {
        let mut s: Vec<(&'a str, Regex)> = vec![];

        for (a, b) in &searchers {
            let c = Regex::new(b).unwrap();
            s.push((a, c));
        }

        TFLA {
            searchers: s,
            assemblers,
            ignore_spaces,
            in_block_comment: false,
            in_line_comment: false,
            line: 1,
            row: 1,
            tokens: vec![],
        }
    }

    fn found_tokens(&mut self, code: &'a str) -> &'a str {
        for searcher in &self.searchers {
            if let Some(caps) = searcher.1.captures(&code) {
                let cap = caps.get(0).unwrap();
                let end = cap.end();
                let content: &'a str = &code[..end];

                let tk: Token<'a> =
                    Token::new(searcher.0, content, self.line, self.row, self.row + end);

                self.row += end + 1;
                let l_type = searcher.0;

                if l_type == "NEW_LINE" {
                    self.row = 1;
                    self.line += 1;

                    self.in_line_comment = false;
                } else if l_type == "LINE_COMMENT" {
                    self.in_line_comment = true;
                } else if l_type == "BLOCK_COMMENT" {
                    self.in_block_comment = !self.in_block_comment;
                } else if l_type == "BLOCK_COMMENT_OPEN" || l_type == "BLOCK_COMMENT_CLOSE" {
                    self.in_block_comment = l_type == "BLOCK_COMMENT_OPEN";
                }

                if self.in_block_comment || self.in_line_comment {
                    return &code[1..];
                }

                let slice = tk.end - tk.start;
                if searcher.0 == "SPACE" && self.ignore_spaces {
                } else {
                    self.tokens.push(tk);
                }

                return &code[slice..];
            }
        }

        println!(
            "ERROR: Any token match found.\nRest of code to tokenize:\n{}",
            code
        );
        return "";
    }

    pub fn tokenize(&mut self, source: &'a str) {
        let mut code = source;
        let mut code_: &'a str = &mut code;

        while code_ != "" {
            code_ = self.found_tokens(code_);
        }
    }
}
