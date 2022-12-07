#[derive(Debug, Clone, Copy)]
enum Op {
    Num(f64),
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Stack {
    file: String,
    file_index: usize,
    ops: Vec<Op>,
    error: bool,
}

#[cfg(test)]
#[test]
fn it_works() {
    let mut stack = Stack::new("(1 + 2) * -0.5 - 3 * 4 / 5");

    parse(&mut stack);
    assert!(!stack.error);

    let result = eval(&mut stack.ops).unwrap();
    assert_eq!(result, -3.9);
}

fn main() {
    let mut input = String::new();
 
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    
    let mut stack = Stack::new(input);

    if stack.file.is_empty() {
        eprintln!("-- USER ERROR -- no input");
        return;
    }
    
    parse(&mut stack);

    if stack.error {
        return;
    }

    let result = eval(&mut stack.ops).expect("failed to calculate");
    let text = &stack.file;

    println!("{text} = {result}")
}

fn eval(ops: &mut Vec<Op>) -> Option<f64> {
    let mut numbers = Vec::new();

    for &mut op in ops {
        let value = eval_op(op, &mut numbers)?;

        numbers.push(value);
    }

    numbers.pop()
}

fn eval_op(op: Op, n: &mut Vec<f64>) -> Option<f64> {
    Some(match op {
        Op::Num(num) => num,
        Op::Neg => -n.pop()?,
        Op::Add => n.pop()? + n.pop()?,
        Op::Mul => n.pop()? * n.pop()?,
        Op::Sub => {
            let right = n.pop()?;
            let left = n.pop()?;
            left - right
        }
        Op::Div => {
            let right = n.pop()?;
            let left = n.pop()?;
            left / right
        }
    })
}

fn parse(stack: &mut Stack) {
    parse_add(stack);
}

fn parse_add(stack: &mut Stack) {
    parse_mul(stack);
    loop {
        if stack.match_str("+") {
            parse_mul(stack);
            stack.ops.push(Op::Add);
        } else if stack.match_str("-") {
            parse_mul(stack);
            stack.ops.push(Op::Sub);
        } else {
            break;
        }
    }
}

fn parse_mul(stack: &mut Stack) {
    parse_unary(stack);
    loop {
        if stack.match_str("*") {
            parse_unary(stack);
            stack.ops.push(Op::Mul);
        } else if stack.match_str("/") {
            parse_unary(stack);
            stack.ops.push(Op::Div);
        }
        else if stack.match_str("(") {
            parse(stack);
            if !stack.match_str(")") {
                stack.error(stack.file_index, "expected )");
            }
            stack.ops.push(Op::Mul);
        } else {
            break;
        }
    }
}

fn parse_unary(stack: &mut Stack) {
    let mut negate = false;
    while stack.match_str("-") {
        negate = !negate;
    }
    if negate {
        parse_paren(stack);
        stack.ops.push(Op::Neg);
    } else {
        parse_paren(stack);
    }
}

fn parse_paren(stack: &mut Stack) {
    if stack.match_str("(") {
        parse(stack);
        if !stack.match_str(")") {
            stack.error(stack.file_index, "expected )");
        }
    } else {
        parse_num(stack);
    }
}

fn parse_num(stack: &mut Stack) {
    let error_index = stack.file_index;
    if let Some(num) = stack.read_number() {
        stack.ops.push(Op::Num(num));
    } else {
        stack.error(error_index, "expected number");
    }
}

impl Stack {
    fn new(file_contents: impl Into<String>) -> Self {
        let mut file = file_contents.into();
        file.retain(|c| !c.is_whitespace());

        Self {
            file,
            file_index: 0,
            ops: Vec::new(),
            error: false,
        }
    }

    fn next(&self) -> &str {
        &self.file[self.file_index..]
    }
    
    fn read_number(&mut self) -> Option<f64> {
        let start = self.file_index;
        let mut length = 0;
        let mut decimal = false;
        for c in self.next().chars() {
            if c == '.' {
                if !decimal {
                    decimal = true;
                } else {
                    break;
                }
            } else if !c.is_digit(10) {
                break;
            }
            length += 1;
        }
        self.file_index = start + length;
        self.file[start..start + length].parse().ok()
    }
    
    fn match_str(&mut self, string: &'static str) -> bool {
        if self.next().starts_with(string) {
            self.file_index += string.len();
            true
        } else {
            false
        }
    }
    
    fn error(&mut self, pos: usize, message: &'static str) {
        self.error = true;

        eprintln!("-- USER ERROR --");

        let file = &self.file;
        
        let mut error = String::new();
        for _ in 0..pos {
            error.push(' ');
        }
        error.push_str("└─ ");
        error.push_str(message);
        error.push_str(" here");

        eprintln!("{file}");
        eprintln!("{error}");
    }
}
