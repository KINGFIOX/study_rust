use std::collections::HashMap;

#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;
            self.data.pop()
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn size(&self) -> usize {
        self.top
    }
}

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn par_checker3(par: &str) -> bool {
    let mut char_list: Vec<char> = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        }

        if c == ')' || c == ']' || c == '}' {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}

fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut rem_stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str.push(digits[rem]);
    }
    base_str
}

fn infix_to_postfix(infix: &str) -> Option<String> {
    // 如果括号不配对
    if !par_checker3(infix) {
        return None;
    }

    // 运算符优先级
    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();

    for token in infix.split_whitespace() {
        // 0 到 9 和 A 到 Z 入栈 ， 这里的 ABCD 是 未知数
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if token == "(" {
            op_stack.push(token);
        } else if token == ")" {
            let mut top_token = op_stack.pop().unwrap();
            while top_token != "(" {
                // 一直找到 最左边的 (
                postfix.push(top_token);
                top_token = op_stack.pop().unwrap();
            }
        } else {
            while !op_stack.is_empty() && (prec[op_stack.peek().unwrap()] >= prec[token]) {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }

    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }

    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str.push_str(c);
        postfix_str.push(' ');
    }

    Some(postfix_str)
}

fn postfix_eval(postfix: &str) -> Option<i32> {
    if postfix.len() < 5 {
        return None;
    }

    let mut op_stack = Stack::new();

    for token in postfix.split_whitespace() {
        if "0" <= token && token <= "9" {
            op_stack.push(token.parse::<i32>().unwrap());
        } else {
            let b = op_stack.pop().unwrap();
            let a = op_stack.pop().unwrap();
            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => 0,
            };
            op_stack.push(result);
        }
    }
    Some(op_stack.pop().unwrap())
}

fn main() {
    let infix = "( A + B ) * C - ( D - E ) * ( F + G )";

    let postfix = infix_to_postfix(infix).unwrap();
    println!("{postfix}");

    let postfix = " 1 2 + 3 *";
    let res = postfix_eval(postfix).unwrap();
    println!("{res}");

    println!("Hello, world!");
}
