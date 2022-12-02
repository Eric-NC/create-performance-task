/*
[dependencies]

*/

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Num(f64),
}

#[derive(Debug)]
struct Stack {
    ops: Vec<Op>,
    file_index: usize,
    file: String,
}

#[cfg(test)]
#[test]
fn it_works() {
    main();
}

fn main() {
    let text = "5 + 10";
    let mut stack = Stack::new(text);

    parse(&mut stack);

    println!("{stack:?}");
}

fn parse(stack: &mut Stack) {
    parse_add(stack);
}

fn parse_add(stack: &mut Stack) {
    let index = stack.ops.len();
    parse_mul(stack);
    stack.skip_whitespace();
    if stack.match_char(|c| c == '+') {
        stack.ops.insert(index, Op::Add);
        parse_mul(stack);
    } else if stack.match_char(|c| c == '-') {
        stack.ops.insert(index, Op::Sub);
        parse_mul(stack);
    }
}

fn parse_mul(stack: &mut Stack) {
    parse_paren(stack);
}

fn parse_paren(stack: &mut Stack) {
    parse_num(stack);
}

fn parse_num(stack: &mut Stack) {
    if let Ok(num) = stack.next_word().parse() {
        stack.advance_word();
        stack.ops.push(Op::Num(num));
    }
}

impl Stack {
    fn new(file: impl Into<String>) -> Self {
        Self {
            ops: Vec::new(),
            file_index: 0,
            file: file.into(),
        }
    }

    fn next(&self) -> &str {
        &self.file[self.file_index..]
    }
    fn next_char(&self) -> Option<char> {
        self.next().chars().next()
    }
    fn next_word(&mut self) -> &str {
        self.skip_whitespace();
        self.next().split_whitespace().next().unwrap_or("")
    }
    fn advance_word(&mut self) {
        self.file_index += self.next_word().len();
    }
    fn match_char(&mut self, predicate: impl FnOnce(char) -> bool) -> bool {
        let mut chars = self.next().char_indices();
        let next = chars.next();
        if let Some((_, c)) = next && predicate(c) {
            if let Some((i, _)) = chars.next() {
                self.file_index = i;
            } else {
                self.file_index = self.file.len();
            }
            true
        } else {
            false
        }
    }
    fn match_str(&mut self, string: &'static str) -> bool {
        if self.next().starts_with(string) {
            self.file_index += string.len();
            true
        } else {
            false
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some((i, c)) = self.next().char_indices().next() {
            if c.is_whitespace() {
                self.file_index += i;
            } else {
                break;
            }
        }
    }
}
