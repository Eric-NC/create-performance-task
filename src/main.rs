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
    error: Option<String>,
}

#[test]
fn math_is_not_broken() {
    assert_eq!(Ok(-3.9), calculate("-0.5(1 + 2) - 3 * 4 / 5"))
}

fn main() {
    // Read input from the user.
    // If command-line arguments are present, use those.
    // If not, just take user input.
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = if args.is_empty() {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        input
    } else {
        args.join("")
    };

    // Calculate, and if everything comes out ok, print the result.
    //                if an error occurs, print a relevant message.
    match calculate(arg) {
        Ok(result) => println!("= {result}"),
        Err(message) => eprintln!("{message}"),
    }
}

fn calculate(input: impl Into<String>) -> Result<f64, String> {
    let mut stack = Stack::new(input.into());

    parse(&mut stack);

    // If we're finished parsing the expression, but not at the end of the file, then there must be some unexpected character holding us up.
    if stack.file_index != stack.file.len() {
        stack.error(stack.file_index, "unexpected character");
    }

    // If an error occurred, return early.
    if let Some(err) = stack.error {
        return Err(err);
    }

    let result = eval(&mut stack.ops).expect("internal error");

    Ok(result)
}

// Evaluates the operand stack and gets a single number from it.
fn eval(ops: &mut Vec<Op>) -> Option<f64> {
    let mut numbers = Vec::new();

    for &mut op in ops {
        let value = process_operation(op, &mut numbers)?;

        numbers.push(value);
    }

    // After processing all operations, the stack must be exhausted,
    // with exactly one value on the number stack.
    // Pop it and return.
    numbers.pop()
}

// Reads one operation `op` from the op stack, and executes it on the number stack `n`.
// For example, an Add op pops two numbers, adds them, and pushes the result back to `n`.
fn process_operation(op: Op, n: &mut Vec<f64>) -> Option<f64> {
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

// -- Recursive Descent parsing --
// This entire section of code is an implementation of recursive descent parsing.
// Specifically, I'm parsing operands and operations into tokens.
// The tokens are sorted in reverse polish notation.
// That way, with only one list (Vec<Op>), I can evaluate the calculation in one fell swoop.
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
        } else if stack.match_str("(") {
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
    if let Some(num) = stack.match_number() {
        stack.ops.push(Op::Num(num));
    } else {
        stack.error(error_index, "expected number");
    }
}
// -- End Recursive Descent parsing --

impl Stack {
    fn new(file_contents: impl Into<String>) -> Self {
        // Get the file contents.
        let mut file = file_contents.into();

        // Remove whitespace from file for easier processing.
        file.retain(|c| !c.is_whitespace());

        // If the file is empty, that's an error! Use the error message "no user input".
        let error = file.is_empty().then(|| String::from("no user input"));

        // Construct the calculator's Stack.
        Self {
            file,
            file_index: 0,
            ops: Vec::new(),
            error,
        }
    }

    // The next part of the file. (All parts that have not been processed yet.)
    fn next(&self) -> &str {
        &self.file[self.file_index..]
    }

    // Reads a number (like 0, 5.2, and 2.63948) from `next()`.
    fn match_number(&mut self) -> Option<f64> {
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

    // If the specified string `string` is next up, move past it and return `true`.
    fn match_str(&mut self, string: &'static str) -> bool {
        if self.next().starts_with(string) {
            self.file_index += string.len();
            true
        } else {
            false
        }
    }

    // The user has messed up somewhere.
    fn error(&mut self, pos: usize, message: &'static str) {
        // Only store one error message.
        if self.error.is_some() {
            return;
        }

        // Format the error message.
        let mut error = String::from("ERROR | ");
        error.push_str(&self.file);
        error.push('\n');
        error.push_str("        ");
        for _ in 0..pos {
            error.push(' ');
        }
        error.push_str("└─ ");
        error.push_str(message);
        error.push_str(" here");

        // Set error message here.
        self.error = Some(error);
    }
}
