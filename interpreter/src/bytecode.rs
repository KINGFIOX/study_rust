/// 定义了一系列的结构和枚举类型

use serde::{ Deserialize, Serialize };

use std::f64;
use std::fmt;

/* ---------- ---------- 记录行号（调试信息） ---------- ---------- */

#[derive(Default, Copy, Clone, Debug)]
pub struct Lineno {
    pub value: usize,
}

#[allow(non_snake_case)]
pub fn Lineno(value: usize) -> Lineno {
    Lineno { value }
}

/**
 * Upvalue 或者是 局部变量
 */

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum UpvalueLoc {
    Upvalue(/*upvalue idx*/ usize),
    Local(/*stack idx*/ usize),
}

/* ---------- ---------- 操作符个数 ---------- ---------- */

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Op {
    Return,
    Constant(usize),
    Closure(usize, Vec<UpvalueLoc>),
    Nil,
    True,
    False,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Equal,
    Greater,
    Less,
    Print,
    Pop,
    DefineGlobal(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    GetLocal(usize),
    SetLocal(usize),
    GetUpval(usize),
    SetUpval(usize),
    JumpIfFalse(usize),
    Jump(usize),
    Loop(usize),
    Call(u8),
    CloseUpvalue,
    Class(usize),
    SetProperty(usize),
    GetProperty(usize),
    Method(usize),
    // 调用
    Invoke(/*method_name*/ String, /*arg count*/ u8),
    Inherit,
    GetSuper(usize),
    SuperInvoke(/*method_name*/ String, /*arg count*/ u8),
    BuildList(usize),
    Subscr,
    SetItem,
}

/* ---------- ---------- 函数、闭包 ---------- ---------- */

/**
 * 函数，函数包含：参数个数、Chunk（代码块）、函数名
 */
#[derive(Default, Clone, Debug)]
pub struct Function {
    pub arity: u8,
    pub chunk: Chunk,
    pub name: String,
}

/**
 * 闭包除了函数有的性质以外，还有环境的一些内容
 */
#[derive(Debug, Clone, Default)]
pub struct Closure {
    pub function: Function,
    pub upvalues: Vec<UpvalueLoc>,
}

/* ---------- ---------- 常量 ---------- ---------- */

/**
 * 这些是常量
 */
#[derive(Debug, Clone)]
pub enum Constant {
    Number(f64),
    String(String),
    Function(Closure),
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Number(n) => write!(f, "{}", n),
            Constant::String(s) => write!(f, "\"{}\"", s),
            Constant::Function(
                Closure { function: Function { arity: _, chunk: _, name }, upvalues: _ },
            ) => write!(f, "<fn {}>", name),
        }
    }
}

/* ---------- ---------- 代码块 ---------- ---------- */

/**
 * Chunk 用于表示一个代码块，保存了操作符 和 常量池
 */
#[derive(Debug, Default, Clone)]
pub struct Chunk {
    pub code: Vec<(Op, Lineno)>,
    pub constants: Vec<Constant>, // 字面量池
}

impl Chunk {
    /**
     * 添加字面量 数字
     */
    pub fn add_constant_number(&mut self, c: f64) -> usize {
        if let Some(id) = self.find_number(c) { id } else { self.add_constant(Constant::Number(c)) }
    }

    /**
     * 添加字面量 String
     */
    pub fn add_constant_string(&mut self, s: String) -> usize {
        if let Some(id) = self.find_string(&s) {
            id
        } else {
            self.add_constant(Constant::String(s))
        }
    }

    /**
     * 添加字面量 Constant
     */
    pub fn add_constant(&mut self, val: Constant) -> usize {
        let const_idx = self.constants.len();
        self.constants.push(val);
        const_idx
    }

    /**
     * 第一个匹配的 索引
     */
    fn find_string(&self, s: &str) -> Option<usize> {
        self.constants.iter().position(|c| {
            if let Constant::String(s2) = c { s == s2 } else { false }
        })
    }

    /**
     * 第一个匹配的 索引
     */
    fn find_number(&self, num: f64) -> Option<usize> {
        self.constants.iter().position(|c| {
            if let Constant::Number(num2) = c { (num - num2).abs() < f64::EPSILON } else { false }
        })
    }
}
