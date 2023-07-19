use std::collections::HashMap;

enum Primitives {
    StringLiteral(String),
    Number(i64),
}

// This is the type that represents a parsed line of code.
enum Statement {
    Print(String),
    Assignment(String, Primitives),
    Addition(String, i64),
    GreaterThan(String, String, String),
    LessThan(String, String, String),
    WhileLoop(
}

// A very simple lexer that splits the input line into tokens.
fn lex(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}

// A very simple parser that takes the output of the lexer and turns it into a parsed Statement.
fn parse(tokens: Vec<String>) -> Statement {
    match tokens.as_slice() {
        [func, var] if func == "print" => Statement::Print(var.clone()),

        [var, op, value] if op == "=" => {
            if let Ok(i) = value.parse() {
                Statement::Assignment(var.clone(), Primitives::Number(i))
            } else {
                Statement::Assignment(var.clone(), Primitives::StringLiteral(value.clone()))
            }
        }

        [var, op, value] if op == "+=" => Statement::Addition(var.clone(), value.parse().unwrap()),

        [var, op, value] if op == ">" => Statement::GreaterThan(var.clone(), op.clone(), value.clone()),

        [var, op, value] if op == "<" => Statement::LessThan(var.clone(), op.clone(), value.clone()),

        _ => panic!("Syntax error"),
    }
}

// The evaluator takes a parsed Statement and a mutable reference to the environment (a map from variable names to values),
// and updates the environment according to the statement.
fn eval(statement: Statement, environment: &mut HashMap<String, Primitives>) {
    match statement {
        Statement::Print(var) => {
            if let Some(value) = environment.get(&var) {
                match value {
                    Primitives::Number(n) => println!("{}", n),
                    Primitives::StringLiteral(s) => println!("{}", s),
                }
            }
        }
        
        Statement::Assignment(var, value) => {
            environment.insert(var, value);
        }

        Statement::Addition(var, value) => {
            if let Some(existing) = environment.get_mut(&var) {
                if let Primitives::Number(n) = existing {
                    *n += value;
                } else {
                    panic!("TypeError: Cannot add a number to a string");
                }
            }
        }
    }
}

fn main() {
    let mut environment = HashMap::new();

    let code = "
        x = 10
        print x
        x += 5

        print x
        y = \"hello\"
        print y

        while x > 0 {
            print x
            x -= 1
        }
    ";

    for line in code.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let tokens = lex(line);
        let statement = parse(tokens);
        eval(statement, &mut environment);
    }
}
