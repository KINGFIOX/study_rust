use std::ops::{ Add, Sub, Mul, Div };

pub mod scanner;
pub mod parser;
pub mod expr;
pub mod extensions;
pub mod treewalk_interpreter;

pub mod bytecode_interpreter;
pub mod gc;
pub mod value;
pub mod bytecode;
pub mod builtins;

// 表达式trait
trait Expression {
    fn evaluate(&self) -> i32;
}

/* ---------- ---------- 数字表达式 ---------- ---------- */

struct Number {
    value: i32,
}

impl Expression for Number {
    fn evaluate(&self) -> i32 {
        self.value
    }
}

/* ---------- ---------- 加法表达式 ---------- ---------- */

struct AddExpr {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for AddExpr {
    fn evaluate(&self) -> i32 {
        self.left.evaluate() + self.right.evaluate()
    }
}

/* ---------- ---------- 减法表达式 ---------- ---------- */

struct SubExpr {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for SubExpr {
    fn evaluate(&self) -> i32 {
        self.left.evaluate() - self.right.evaluate()
    }
}

/* ---------- ---------- 乘法表达式 ---------- ---------- */

struct MulExpr {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for MulExpr {
    fn evaluate(&self) -> i32 {
        self.left.evaluate() * self.right.evaluate()
    }
}

/* ---------- ---------- 除法表达式 ---------- ---------- */

struct DivExpr {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for DivExpr {
    fn evaluate(&self) -> i32 {
        self.left.evaluate() / self.right.evaluate()
    }
}

/* ---------- ---------- parser ---------- ---------- */

use nom::{
    branch::alt,
    character::complete::{ char, digit1, space0 },
    combinator::{ map, map_res },
    sequence::{ delimited, tuple },
    IResult,
};
use std::str::FromStr;

// 解析数字
fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}

// 解析括号内的表达式
fn parse_parentheses(input: &str) -> IResult<&str, i32> {
    delimited(space0, delimited(char('('), parse_expr, char(')')), space0)(input)
}

// 解析表达式
fn parse_expr(input: &str) -> IResult<&str, i32> {
    let (input, init) = alt((parse_number, parse_parentheses))(input)?;

    // 这里简化处理，仅处理加法和减法
    let (input, _) = space0(input)?;
    let (input, res) = alt((
        map(tuple((char('+'), parse_expr)), move |(_, expr)| init + expr),
        map(tuple((char('-'), parse_expr)), move |(_, expr)| init - expr),
    ))(input)?;

    Ok((input, res))
}

fn main() {
    let test_expr = "3 + (2 - 1)";
    match parse_expr(test_expr) {
        Ok((_, result)) => println!("结果是: {}", result),
        Err(e) => println!("解析错误: {}", e),
    }
}
