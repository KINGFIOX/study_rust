use crate::expr;
use crate::extensions;
use crate::scanner;

use std::fmt;

#[derive(Default)]
struct Parser {
    tokens: Vec<scanner::Token>,
    current: usize, // 下表，指针
    in_fundec: bool, // in rust, booleans default to false: https://doc.rust-lang.org/std/primitive.bool.html#impl-Default
    extensions: extensions::Extensions,
}

/* ---------- ---------- 错误处理 ---------- ---------- */

pub enum Error {
    UnexpectedToken(scanner::Token),
    TokenMismatch {
        expected: scanner::TokenType,
        found: scanner::Token,
        maybe_on_err_string: Option<String>,
    },
    MaxParamsExceeded {
        kind: FunctionKind,
        line: usize,
        col: i64,
    },
    ReturnNotInFun {
        line: usize,
        col: i64,
    },
    InvalidAssignment {
        line: usize,
        col: i64,
    },
    TooManyArguments {
        line: usize,
        col: i64,
    },
    ExpectedExpression {
        token_type: scanner::TokenType,
        line: usize,
        col: i64,
    },
    InvalidTokenInUnaryOp {
        token_type: scanner::TokenType,
        line: usize,
        col: i64,
    },
    InvalidTokenInBinaryOp {
        token_type: scanner::TokenType,
        line: usize,
        col: i64,
    },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::UnexpectedToken(tok) =>
                write!(f, "Unexpected token {:?} at line={},col={}", tok.ty, tok.line, tok.col),
            Error::TokenMismatch { maybe_on_err_string, expected, found } => {
                write!(
                    f,
                    "Expected token {:?} but found {:?} at line={},col={}",
                    expected,
                    found.ty,
                    found.line,
                    found.col
                )?;
                if let Some(on_err_string) = maybe_on_err_string {
                    write!(f, ": {}", on_err_string)?;
                }
                fmt::Result::Ok(())
            }
            Error::MaxParamsExceeded { kind, line, col } =>
                write!(
                    f,
                    "Cannot have more than 255 parameters in a {:?} declaration. Line={},col={}",
                    kind,
                    line,
                    col
                ),
            Error::ReturnNotInFun { line, col } =>
                write!(
                    f,
                    "return statement not enclosed in a FunDecl at line={},col={}",
                    line,
                    col
                ),
            Error::InvalidAssignment { line, col } => {
                write!(f, "invalid assignment target at line={},col={}", line, col)
            }
            Error::TooManyArguments { line, col } =>
                write!(
                    f,
                    "Cannot have more than 255 arguments to a function call. Line={},col={}",
                    line,
                    col
                ),
            Error::ExpectedExpression { token_type, line, col } =>
                write!(
                    f,
                    "Expected expression, but found token {:?} at line={},col={}",
                    token_type,
                    line,
                    col
                ),
            Error::InvalidTokenInUnaryOp { token_type, line, col } =>
                write!(
                    f,
                    "invalid token in unary op {:?} at line={},col={}",
                    token_type,
                    line,
                    col
                ),
            Error::InvalidTokenInBinaryOp { token_type, line, col } =>
                write!(
                    f,
                    "invalid token in binary op {:?} at line={},col={}",
                    token_type,
                    line,
                    col
                ),
        }
    }
}

#[derive(Debug)]
pub enum FunctionKind {
    Function,
    Method,
    Lambda,
}

pub fn parse(
    extensions: extensions::Extensions,
    tokens: Vec<scanner::Token>
) -> Result<Vec<expr::Stmt>, Error> {
    // 剩下的 usize 和 in_fundec 使用默认构造
    let mut p = Parser { tokens, extensions, ..Default::default() };
    let stmts_or_err = p.parse(); // 得到了许多 statements

    match stmts_or_err {
        Ok(stmts_or_err) => {
            if !p.is_at_end() {
                let tok = &p.tokens[p.current]; // 第几个 token 出现了问题
                Err(Error::UnexpectedToken(tok.clone()))
            } else {
                Ok(stmts_or_err)
            }
        }
        Err(err) => Err(err),
    }
}

impl Parser {
    /**
     * program → declaration* EOF
     */
    pub fn parse(&mut self) -> Result<Vec<expr::Stmt>, Error> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        Ok(statements)
    }

    /**
     * declaration → classDecl
     *             | funDecl
     *             | varDecl
     *             | statement ;
     */
    fn declaration(&mut self) -> Result<expr::Stmt, Error> {
        if self.matches(scanner::TokenType::Var) {
            return self.var_decl();
        }

        if self.matches(scanner::TokenType::Fun) {
            return Ok(expr::Stmt::FunDecl(self.fun_decl(FunctionKind::Function)?));
        }

        if self.matches(scanner::TokenType::Class) {
            return self.class_decl();
        }

        self.statement()
    }

    /**
     * varDecl → "var" IDENTIFIER ( "=" expression )? ";" ;
     */
    fn var_decl(&mut self) -> Result<expr::Stmt, Error> {
        let name_token = self
            .consume(scanner::TokenType::Identifier, "Expected variable name")?
            .clone();

        let maybe_initializer = if self.matches(scanner::TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(scanner::TokenType::Semicolon, "Expected ; after variable declaration")?;

        Ok(
            expr::Stmt::VarDecl(
                expr::Symbol {
                    name: String::from_utf8(name_token.lexeme).unwrap(),
                    line: name_token.line,
                    col: name_token.col,
                },
                maybe_initializer
            )
        )
    }

    /**
     * statement → exprStmt
     *           | forStmt
     *           | ifStmt
     *           | printStmt
     *           | returnStmt
     *           | whileStmt
     *           | block ;
     */
    fn statement(&mut self) -> Result<expr::Stmt, Error> {
        // 当前 词素 是 print
        if self.matches(scanner::TokenType::Print) {
            return self.print_statement();
        }

        if self.matches(scanner::TokenType::While) {
            return self.while_statement();
        }

        if self.matches(scanner::TokenType::LeftBrace) {
            return Ok(expr::Stmt::Block(self.block()?));
        }

        if self.matches(scanner::TokenType::For) {
            return self.for_statement();
        }

        if self.matches(scanner::TokenType::If) {
            return self.if_statement();
        }

        if self.matches(scanner::TokenType::Return) {
            return self.return_statement();
        }

        // expression statements
        self.expression_statement()
    }

    /* ---------- ---------- print ---------- ---------- */

    /**
     * printStmt → "print" expression ";"
     */
    fn print_statement(&mut self) -> Result<expr::Stmt, Error> {
        let expr = self.expression()?;
        self.consume(scanner::TokenType::Semicolon, "Expected ; after value")?;
        Ok(expr::Stmt::Print(expr))
    }

    /**
     * exprStmt  → expression ";" ;
     */
    fn expression_statement(&mut self) -> Result<expr::Stmt, Error> {
        let expr = self.expression()?;
        self.consume(scanner::TokenType::Semicolon, "Expected ; after value")?;
        Ok(expr::Stmt::Expr(expr))
    }

    /* ---------- ---------- expression ---------- ---------- */

    /**
     * expression → assignment ;
     */
    fn expression(&mut self) -> Result<expr::Expr, Error> {
        self.assignment()
    }

    /**
     * assignment → ( call "." )? IDENTIFIER "=" assignment
     *            | logic_or;
     * 其实这里有点意思，equality < logic_and < logic_or < assignment
     */
    fn assignment(&mut self) -> Result<expr::Expr, Error> {
        let expr = self.or()?; // 是不是逻辑或，任何 expr 都可以看成是 expre | 0

        if self.matches(scanner::TokenType::Equal) {
            let equals = self.previous().clone(); // 等号左边
            let new_value = self.assignment()?; // 等号右边

            if let expr::Expr::Variable(sym) = &expr {
                return Ok(expr::Expr::Assign(sym.clone(), Box::new(new_value)));
            }

            // 访问对象的 字段
            if let expr::Expr::Get(e, attr) = expr {
                return Ok(expr::Expr::Set(e, attr, Box::new(new_value)));
            }

            // 下表访问
            if let expr::Expr::Subscript { value, slice, source_location } = expr {
                return Ok(expr::Expr::SetItem {
                    lhs: value,
                    slice,
                    rhs: Box::new(new_value),
                    source_location,
                });
            }

            // 最后也只能是一个错误了
            return Err(Error::InvalidAssignment {
                line: equals.line,
                col: equals.col,
            });
        }

        Ok(expr)
    }

    /**
     * logic_or → logic_and ( "or" logic_and )* ;
     */
    fn or(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.and()?;

        while self.matches(scanner::TokenType::Or) {
            let right = self.and()?;
            expr = expr::Expr::Logical(Box::new(expr), expr::LogicalOp::Or, Box::new(right));
        }

        Ok(expr)
    }

    /**
     * logic_and → equality ( "and" equality )* ;
     */
    fn and(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.equality()?;

        while self.matches(scanner::TokenType::And) {
            let right = self.equality()?;
            expr = expr::Expr::Logical(Box::new(expr), expr::LogicalOp::And, Box::new(right));
        }

        Ok(expr)
    }

    /* ---------- ---------- while ---------- ---------- */

    /**
     * whileStmt → "while" "(" expression ")" statement ;
     */
    fn while_statement(&mut self) -> Result<expr::Stmt, Error> {
        self.consume(scanner::TokenType::LeftParen, "Expected ( after while")?;
        let cond = self.expression()?;
        self.consume(scanner::TokenType::RightParen, "Expected ) after while condition")?;
        let body = Box::new(self.statement()?);
        Ok(expr::Stmt::While(cond, body))
    }

    /* ---------- ---------- block ---------- ---------- */

    /**
     * block → "{" declaration* "}" ;
     */
    fn block(&mut self) -> Result<Vec<expr::Stmt>, Error> {
        let mut stmts = Vec::new();

        while !self.check(scanner::TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }

        self.consume(scanner::TokenType::RightBrace, "Expected } after block.")?;

        Ok(stmts)
    }

    /* ---------- ---------- for ---------- ---------- */

    /**
     * forStmt   → "for" "(" ( varDecl | exprStmt | ";" )
     *                expression? ";"
     *                expression? ")" statement ;
     * 我们的 for 转换为 while 处理
     */
    fn for_statement(&mut self) -> Result<expr::Stmt, Error> {
        self.consume(scanner::TokenType::LeftParen, "Expected ( after for.")?;

        let mut maybe_initializer: Option<expr::Stmt> = None;
        if self.matches(scanner::TokenType::Semicolon) {
            // nothing to do
        } else if self.matches(scanner::TokenType::Var) {
            maybe_initializer = Some(self.var_decl()?);
        } else {
            maybe_initializer = Some(self.expression_statement()?);
        }
        let maybe_initializer = maybe_initializer;

        let mut maybe_condition: Option<expr::Expr> = None;
        if !self.check(scanner::TokenType::Semicolon) {
            maybe_condition = Some(self.expression()?);
        }
        let maybe_condition = maybe_condition;

        self.consume(scanner::TokenType::Semicolon, "Expected ; after loop condition")?;

        let maybe_increment = if !self.check(scanner::TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(scanner::TokenType::RightParen, "Expected ) after for clauses")?;

        let mut body = self.statement()?;

        if let Some(increment) = maybe_increment {
            body = expr::Stmt::Block(vec![body, expr::Stmt::Expr(increment)]);
            // 我们这里的处理是将：增量放到了 body 最后面
        }

        let condition = match maybe_condition {
            Some(cond) => cond,
            None => expr::Expr::Literal(expr::Literal::True),
        };
        body = expr::Stmt::While(condition, Box::new(body)); // 我们是将 for 转换为 while 处理

        if let Some(initializer) = maybe_initializer {
            body = expr::Stmt::Block(vec![initializer, body]);
        }
        let body = body;

        Ok(body)
    }

    /* ---------- ---------- if ---------- ---------- */

    /**
     * ifStmt → "if" "(" expression ")" statement ( "else" statement )? ;
     */
    fn if_statement(&mut self) -> Result<expr::Stmt, Error> {
        self.consume(scanner::TokenType::LeftParen, "Expected ( after if.")?;
        let cond = self.expression()?;
        self.consume(scanner::TokenType::RightParen, "Expected ) after if condition.")?;
        let then_branch = Box::new(self.statement()?);
        let maybe_else_branch = if self.matches(scanner::TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(expr::Stmt::If(cond, then_branch, maybe_else_branch))
    }

    /* ---------- ---------- return ---------- ---------- */

    /**
     * returnStmt → "return" expression? ";" ;
     */
    fn return_statement(&mut self) -> Result<expr::Stmt, Error> {
        let prev_tok = self.previous().clone(); // 获取 return 的词素

        if !self.in_fundec {
            return Err(Error::ReturnNotInFun {
                line: prev_tok.line,
                col: prev_tok.col,
            });
        }

        let maybe_retval = if !self.matches(scanner::TokenType::Semicolon) {
            Some(self.expression()?) // 直接就是 return; 说明无返回值
        } else {
            None
        };

        if maybe_retval.is_some() {
            self.consume(scanner::TokenType::Semicolon, "Expected ; after return value")?;
        }

        Ok(
            expr::Stmt::Return(
                expr::SourceLocation {
                    line: prev_tok.line,
                    col: prev_tok.col,
                },
                maybe_retval
            )
        )
    }

    /* ---------- ---------- 加减乘除、比较 ---------- ---------- */

    /**
     * equality → comparison ( ( "!=" | "==" ) comparison )* ;
     */
    fn equality(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.comparison()?;

        while
            self.match_one_of(vec![scanner::TokenType::BangEqual, scanner::TokenType::EqualEqual])
        {
            let operator_token = self.previous().clone();
            let right = Box::new(self.comparison()?);

            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(expr)
    }

    /**
     * comparison → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
     */
    fn comparison(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.addition()?;

        while
            self.match_one_of(
                vec![
                    scanner::TokenType::Greater,
                    scanner::TokenType::GreaterEqual,
                    scanner::TokenType::Less,
                    scanner::TokenType::LessEqual
                ]
            )
        {
            let operator_token = self.previous().clone();
            let right = Box::new(self.addition()?);
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(expr)
    }

    /**
     * addition → multiplication ( ( "-" | "+" ) multiplication )* ;
     */
    fn addition(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.multiplication()?;

        while self.match_one_of(vec![scanner::TokenType::Minus, scanner::TokenType::Plus]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.multiplication()?);
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(expr)
    }

    /**
     * multiplication → unary ( ( "/" | "*" ) unary )* ;
     */
    fn multiplication(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.unary()?; // 左操作数

        while self.match_one_of(vec![scanner::TokenType::Slash, scanner::TokenType::Star]) {
            let operator_token = self.previous().clone(); // 操作符
            let right = Box::new(self.unary()?); // 右操作数
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(expr)
    }

    /**
     * 将 词素 转化成语义
     */
    fn op_token_to_binop(tok: &scanner::Token) -> Result<expr::BinaryOp, Error> {
        match tok.ty {
            // ==
            scanner::TokenType::EqualEqual =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::EqualEqual,
                    line: tok.line,
                    col: tok.col,
                }),
            // !=
            scanner::TokenType::BangEqual =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::NotEqual,
                    line: tok.line,
                    col: tok.col,
                }),
            // <
            scanner::TokenType::Less =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::Less,
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::LessEqual =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::LessEqual,
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::Greater =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::Greater,
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::GreaterEqual =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::GreaterEqual,
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::Plus =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::Plus,
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::Minus =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::Minus,
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::Star =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::Star,
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::Slash =>
                Ok(expr::BinaryOp {
                    ty: expr::BinaryOpTy::Slash,
                    line: tok.line,
                    col: tok.col,
                }),
            _ =>
                Err(Error::InvalidTokenInBinaryOp {
                    token_type: tok.ty,
                    line: tok.line,
                    col: tok.col,
                }),
        }
    }

    /**
     * unary → ( "!" | "-" ) unary | call ;
     */
    fn unary(&mut self) -> Result<expr::Expr, Error> {
        if self.match_one_of(vec![scanner::TokenType::Bang, scanner::TokenType::Minus]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.unary()?); // 递归了，这里一条链的递归，相当于是循环

            // 将 词素 转换为 一元操作符
            let unary_op_maybe = Parser::op_token_to_unary_op(&operator_token);

            return match unary_op_maybe {
                Ok(unary_op) => Ok(expr::Expr::Unary(unary_op, right)), // 将 一元操作符 + 元素 合并
                Err(err) => Err(err),
            };
        }
        self.call() // 调用函数
    }

    /**
     * 将 词素 转换为 一元操作符
     */
    fn op_token_to_unary_op(tok: &scanner::Token) -> Result<expr::UnaryOp, Error> {
        match tok.ty {
            scanner::TokenType::Minus =>
                Ok(expr::UnaryOp {
                    ty: expr::UnaryOpTy::Minus, // 取负数
                    line: tok.line,
                    col: tok.col,
                }),
            scanner::TokenType::Bang =>
                Ok(expr::UnaryOp {
                    ty: expr::UnaryOpTy::Bang, // 取反
                    line: tok.line,
                    col: tok.col,
                }),
            _ => // 剩下的，进入 error 处理
                Err(Error::InvalidTokenInUnaryOp {
                    token_type: tok.ty,
                    line: tok.line,
                    col: tok.col,
                }),
        }
    }

    /* ---------- ---------- function declare ---------- ---------- */

    /**
     * funDecl → "fun" function ;
     * function → IDENTIFIER "(" parameters? ")" block ; （这个 function 没有 fun 关键字）
     * function 的 kind 有三种：成员方法、函数、lambda
     */
    fn fun_decl(&mut self, kind: FunctionKind) -> Result<expr::FunDecl, Error> {
        // 这个就是
        let name_tok = self
            .consume(scanner::TokenType::Identifier, format!("Expected {:?} name", kind).as_ref())?
            .clone();

        let fun_symbol = expr::Symbol {
            name: String::from_utf8(name_tok.lexeme).unwrap(),
            line: name_tok.line,
            col: name_tok.col,
        };

        let (parameters, body) = self.params_and_body(kind)?;

        Ok(expr::FunDecl {
            name: fun_symbol,
            params: parameters,
            body,
        })
    }

    /**
     * parameters  → IDENTIFIER ( "," IDENTIFIER )* ;
     * @return (parameters, body)
     */
    fn params_and_body(
        &mut self,
        kind: FunctionKind
    ) -> Result<(Vec<expr::Symbol>, Vec<expr::Stmt>), Error> {
        self.consume(
            scanner::TokenType::LeftParen,
            format!("Expected ( after {:?} name", kind).as_ref()
        )?;

        let mut parameters = Vec::new();

        if !self.check(scanner::TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    // 函数的参数太多了
                    let peek_tok = self.peek();
                    return Err(Error::MaxParamsExceeded {
                        kind,
                        line: peek_tok.line,
                        col: peek_tok.col,
                    });
                }

                let tok = self
                    .consume(scanner::TokenType::Identifier, "Expected parameter name")?
                    .clone();

                parameters.push(expr::Symbol {
                    name: String::from_utf8(tok.lexeme).unwrap(),
                    line: tok.line,
                    col: tok.col,
                });

                if !self.matches(scanner::TokenType::Comma) {
                    break;
                } // 如果最后面不是逗号，那么就退出
            }
        } // 否则就是没有参数
        let parameters = parameters;

        self.consume(scanner::TokenType::RightParen, "Expected ) after parameter list")?;
        self.consume(scanner::TokenType::LeftBrace, "Expected { before function body")?;
        let saved_is_in_fundec = self.in_fundec; // 我们的 lox 是可以闭包定义的
        self.in_fundec = true;
        let body = self.block()?;
        self.in_fundec = saved_is_in_fundec;

        Ok((parameters, body))
    }

    /* ---------- ---------- function call ---------- ---------- */

    /**
     * call → primary ( "(" arguments? ")" | "." IDENTIFIER | "[" expression "]" )* ;
     */
    fn call(&mut self) -> Result<expr::Expr, Error> {
        let mut expr = self.primary()?; // 1. 获取函数 2. 获取成员 3. 下标访问 list 的元素

        loop {
            if self.matches(scanner::TokenType::LeftParen) {
                // 直接就开始解析参数
                expr = self.finish_call(expr)?; // expr = (被调函数, 实参列表)
            } else if self.matches(scanner::TokenType::Dot) {
                // name_tok 是成员方法的名字
                let name_tok = self
                    .consume(scanner::TokenType::Identifier, "Expected property name after '.'.")?
                    .clone();
                expr = expr::Expr::Get(Box::new(expr), expr::Symbol {
                    name: String::from_utf8(name_tok.lexeme).unwrap(),
                    line: name_tok.line,
                    col: name_tok.col,
                }); // expr 现在是 Get(对象, 成员函数) 了
            } else if self.extensions.lists && self.matches(scanner::TokenType::LeftBracket) {
                // 如果是 [ ，并且开启了 lists 拓展的话
                let slice_expr = self.expression()?;
                let token = self.consume(
                    scanner::TokenType::RightBracket,
                    "Expected ] after subscript"
                )?;
                expr = expr::Expr::Subscript {
                    value: Box::new(expr),
                    slice: Box::new(slice_expr),
                    source_location: expr::SourceLocation {
                        line: token.line,
                        col: token.col,
                    },
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /**
     * arguments → expression ( "," expression )* ;
     */
    fn finish_call(&mut self, callee: expr::Expr) -> Result<expr::Expr, Error> {
        // 设置实参列表
        let mut arguments = Vec::new();
        if !self.check(scanner::TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    let peek_tok = self.peek();
                    return Err(Error::TooManyArguments {
                        line: peek_tok.line,
                        col: peek_tok.col,
                    });
                }
                arguments.push(self.expression()?);
                if !self.matches(scanner::TokenType::Comma) {
                    break;
                }
            }
        }

        let token = self.consume(scanner::TokenType::RightParen, "Expected ) after arguments.")?;

        Ok(
            expr::Expr::Call(
                Box::new(callee),
                expr::SourceLocation {
                    line: token.line,
                    col: token.col,
                },
                arguments
            )
        )
    }

    /**
     * primary → "true" | "false" | "nil" | "this"
     *         | NUMBER | STRING | IDENTIFIER | "(" expression ")"
     *         | "super" "." IDENTIFIER
     *         | "[" arguments? "]" ;
     */
    fn primary(&mut self) -> Result<expr::Expr, Error> {
        // 都要 previous ，因为 matches 成功以后，我们会 advance
        if self.matches(scanner::TokenType::False) {
            return Ok(expr::Expr::Literal(expr::Literal::False));
        }
        if self.matches(scanner::TokenType::True) {
            return Ok(expr::Expr::Literal(expr::Literal::True));
        }
        if self.matches(scanner::TokenType::Nil) {
            return Ok(expr::Expr::Literal(expr::Literal::Nil));
        }
        // 访问 父类的 元素
        // super
        if self.matches(scanner::TokenType::Super) {
            let super_tok = self.previous().clone();
            self.consume(scanner::TokenType::Dot, "Expected '.' after 'super'.")?;
            let method_tok = self.consume(
                scanner::TokenType::Identifier,
                "Expected superclass method name."
            )?;
            return Ok(
                expr::Expr::Super(
                    expr::SourceLocation {
                        line: super_tok.line,
                        col: super_tok.col,
                    },
                    expr::Symbol {
                        name: String::from_utf8(method_tok.lexeme.clone()).unwrap(),
                        line: method_tok.line,
                        col: method_tok.col,
                    }
                )
            );
        }
        if self.matches(scanner::TokenType::Number) {
            match &self.previous().literal {
                Some(scanner::Literal::Number(n)) => {
                    return Ok(expr::Expr::Literal(expr::Literal::Number(*n)));
                }
                Some(l) =>
                    panic!("internal error in parser: when parsing number, found literal {:?}", l),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }
        if self.matches(scanner::TokenType::String) {
            match &self.previous().literal {
                Some(scanner::Literal::Str(s)) => {
                    return Ok(expr::Expr::Literal(expr::Literal::String(s.clone())));
                }
                Some(l) =>
                    panic!("internal error in parser: when parsing string, found literal {:?}", l),
                None => panic!("internal error in parser: when parsing string, found no literal"),
            }
        }
        if self.matches(scanner::TokenType::This) {
            let prev = self.previous();
            return Ok(
                expr::Expr::This(expr::SourceLocation {
                    line: prev.line,
                    col: prev.col,
                })
            );
        }
        if self.matches(scanner::TokenType::Identifier) {
            match &self.previous().literal {
                Some(scanner::Literal::Identifier(s)) => {
                    return Ok(
                        expr::Expr::Variable(expr::Symbol {
                            name: s.clone(),
                            line: self.previous().line,
                            col: self.previous().col,
                        })
                    );
                }
                Some(l) =>
                    panic!(
                        "internal error in parser: when parsing identifier, found literal {:?}",
                        l
                    ),
                None => {
                    panic!("internal error in parser: when parsing identifier, found no literal");
                }
            }
        }
        if self.matches(scanner::TokenType::LeftParen) {
            let expr = Box::new(self.expression()?);
            self.consume(scanner::TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(expr::Expr::Grouping(expr));
        }

        /* 支持 [1, 2, 3, 5] 之类的 */
        if self.extensions.lists && self.matches(scanner::TokenType::LeftBracket) {
            let mut list_elements = Vec::new();

            if !self.check(scanner::TokenType::RightBracket) {
                loop {
                    list_elements.push(self.expression()?);
                    if !self.matches(scanner::TokenType::Comma) {
                        break;
                    }
                }
            }

            self.consume(scanner::TokenType::RightBracket, "Expected ].")?;

            return Ok(expr::Expr::List(list_elements));
        }

        /* 支持 (xxx, xxxx, xxxxx) { y; yy; yyy; } */
        if self.extensions.lambdas && self.matches(scanner::TokenType::Lambda) {
            let (params, body) = self.params_and_body(FunctionKind::Lambda)?;
            return Ok(expr::Expr::Lambda(expr::LambdaDecl { params, body }));
        }

        Err(Error::ExpectedExpression {
            token_type: self.peek().ty,
            line: self.peek().line,
            col: self.peek().col,
        })
    }

    /* ---------- ---------- class declare ---------- ---------- */

    /**
     * classDecl → "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}" ;
     */
    fn class_decl(&mut self) -> Result<expr::Stmt, Error> {
        let name_tok = self.consume(scanner::TokenType::Identifier, "Expected class name")?.clone();

        let class_symbol = expr::Symbol {
            name: String::from_utf8(name_tok.lexeme).unwrap(),
            line: name_tok.line,
            col: name_tok.col,
        };

        /* 我们这里只允许 单继承 */
        let superclass_maybe = if self.matches(scanner::TokenType::Less) {
            let superclass_tok = self.consume(
                scanner::TokenType::Identifier,
                "Expected class name."
            )?;
            Some(expr::Symbol {
                name: String::from_utf8(superclass_tok.lexeme.clone()).unwrap(),
                line: superclass_tok.line,
                col: superclass_tok.col,
            })
        } else {
            None
        };

        self.consume(scanner::TokenType::LeftBrace, "Expected { after class name")?;

        let mut methods = Vec::new();
        while !self.check(scanner::TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.fun_decl(FunctionKind::Method)?);
        }
        let methods = methods; // 锁定为 不可变

        self.consume(scanner::TokenType::RightBrace, "Expected } after class body")?;

        Ok(
            expr::Stmt::ClassDecl(expr::ClassDecl {
                name: class_symbol,
                superclass: superclass_maybe,
                methods,
            })
        )
    }

    /* ---------- ---------- 下面就是 辅助函数了 ---------- ---------- */

    /**
     * 消费一个 词素
     */
    fn consume(
        &mut self,
        tok: scanner::TokenType,
        on_err_str: &str
    ) -> Result<&scanner::Token, Error> {
        if self.check(tok) {
            Ok(self.advance())
        } else {
            Err(Error::TokenMismatch {
                expected: tok,
                found: self.peek().clone(),
                maybe_on_err_string: Some(on_err_str.into()),
            })
        }
    }

    fn match_one_of(&mut self, types: Vec<scanner::TokenType>) -> bool {
        for ty in types.iter() {
            if self.matches(*ty) {
                return true;
            }
        }
        false
    }

    fn matches(&mut self, ty: scanner::TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, ty: scanner::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ty == ty
    }

    fn advance(&mut self) -> &scanner::Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /**
     * 看一下当前字符是不是到达了 EOF
     * 一般来说，所有的行都读取完成以后，self.peek().tokenType 就会是 EOF 的
     */
    fn is_at_end(&self) -> bool {
        self.peek().ty == scanner::TokenType::Eof
    }

    fn peek(&self) -> &scanner::Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &scanner::Token {
        &self.tokens[self.current - 1]
    }
}
