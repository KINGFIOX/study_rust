#[derive(Debug, Clone)]
/**
 * 这个文件中，定义了：表达式
 */
pub enum Expr {
    Literal(Literal), // literal 表示 叶子节点，字面量，不需要通过计算得到
    This(SourceLocation), // 面向对象语言中的 this
    Unary(UnaryOp, Box<Expr>), // 一元操作符表达式
    Binary(Box<Expr>, BinaryOp, Box<Expr>), //  这是一个递归结构
    Call(Box<Expr>, SourceLocation, Vec<Expr>),
    Get(Box<Expr>, Symbol), // 字段
    Grouping(Box<Expr>),
    Variable(Symbol),
    Assign(Symbol, Box<Expr>), // 赋值操作，包含变量名 + 内容
    Logical(Box<Expr>, LogicalOp, Box<Expr>), // 可能是 与 或 之类的
    Set(Box<Expr>, Symbol, Box<Expr>), // Expr.symbol = expr
    Super(SourceLocation, Symbol), // super 指的是父对象
    List(Vec<Expr>),
    Subscript { // 下标访问
        value: Box<Expr>,
        slice: Box<Expr>,
        source_location: SourceLocation,
    },
    // 这个是对集合中的某个元素赋值
    SetItem {
        lhs: Box<Expr>, // 整个集合
        slice: Box<Expr>,
        rhs: Box<Expr>,
        source_location: SourceLocation,
    },
    Lambda(LambdaDecl),
}

/**
 * SourceLocation 行列
 */
#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    Or,
    And,
}

/**
 * 变量名之类的
 */
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Symbol {
    pub name: String,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct LambdaDecl {
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: Symbol,
    pub superclass: Option<Symbol>,
    pub methods: Vec<FunDecl>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    FunDecl(FunDecl),
    ClassDecl(ClassDecl),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    VarDecl(Symbol, Option<Expr>),
    Block(Vec<Stmt>),
    Return(SourceLocation, Option<Expr>),
    While(Expr, Box<Stmt>),
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOpTy { // 医院操作符
    Minus,
    Bang,
}

#[derive(Debug, Copy, Clone)]
pub struct UnaryOp {
    pub ty: UnaryOpTy,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOpTy {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Copy, Clone)]
pub struct BinaryOp {
    pub ty: BinaryOpTy,
    pub line: usize,
    pub col: i64,
}

/**
 * 这几种字面量
 */
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}
