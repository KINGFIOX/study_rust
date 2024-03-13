use crate::builtins;
use crate::bytecode;
use crate::gc;
use crate::value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/* ---------- ---------- 反汇编 ---------- ---------- */

/**
 * 将 bytecode::Op 转换为 String
 */
pub fn disassemble_code(chunk: &bytecode::Chunk) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    for (idx, (op, lineno)) in chunk.code.iter().enumerate() {
        let formatted_op = match op {
            bytecode::Op::Return => "OP_RETURN".to_string(),
            bytecode::Op::Constant(const_idx) =>
                format!("OP_CONSTANT {} (idx={})", chunk.constants[*const_idx], *const_idx),
            bytecode::Op::Nil => "OP_NIL".to_string(),
            bytecode::Op::True => "OP_TRUE".to_string(),
            bytecode::Op::False => "OP_FALSE".to_string(),
            bytecode::Op::Negate => "OP_NEGATE".to_string(),
            bytecode::Op::Add => "OP_ADD".to_string(),
            bytecode::Op::Subtract => "OP_SUBTRACT".to_string(),
            bytecode::Op::Multiply => "OP_MULTIPLY".to_string(),
            bytecode::Op::Divide => "OP_DIVIDE".to_string(),
            bytecode::Op::Not => "OP_NOT".to_string(),
            bytecode::Op::Equal => "OP_NOT".to_string(),
            bytecode::Op::Greater => "OP_GREATER".to_string(),
            bytecode::Op::Less => "OP_LESS".to_string(),
            bytecode::Op::Print => "OP_PRINT".to_string(),
            bytecode::Op::Pop => "OP_POP".to_string(),
            bytecode::Op::DefineGlobal(global_idx) =>
                format!(
                    "OP_DEFINE_GLOBAL {:?} (idx={})",
                    chunk.constants[*global_idx],
                    *global_idx
                ),
            bytecode::Op::GetGlobal(global_idx) =>
                format!("OP_GET_GLOBAL {:?} (idx={})", chunk.constants[*global_idx], *global_idx),
            bytecode::Op::SetGlobal(global_idx) =>
                format!("OP_SET_GLOBAL {:?} (idx={})", chunk.constants[*global_idx], *global_idx),
            bytecode::Op::GetLocal(idx) => format!("OP_GET_LOCAL idx={}", *idx),
            bytecode::Op::SetLocal(idx) => format!("OP_SET_LOCAL idx={}", *idx),
            bytecode::Op::GetUpval(idx) => format!("OP_GET_UPVAL idx={}", *idx),
            bytecode::Op::SetUpval(idx) => format!("OP_SET_UPVAL idx={}", *idx),
            bytecode::Op::JumpIfFalse(loc) => format!("OP_JUMP_IF_FALSE {}", *loc),
            bytecode::Op::Jump(offset) => format!("OP_JUMP {}", *offset),
            bytecode::Op::Loop(offset) => format!("OP_LOOP {}", *offset),
            bytecode::Op::Call(arg_count) => format!("OP_CALL {}", *arg_count),
            bytecode::Op::Closure(idx, _) => format!("OP_CLOSURE {}", chunk.constants[*idx]),
            bytecode::Op::CloseUpvalue => "OP_CLOSE_UPVALUE".to_string(),
            bytecode::Op::Class(idx) => format!("OP_CLASS {}", idx),
            bytecode::Op::SetProperty(idx) => format!("OP_SET_PROPERTY {}", idx),
            bytecode::Op::GetProperty(idx) => format!("OP_GET_PROPERTY {}", idx),
            bytecode::Op::Method(idx) => format!("OP_METHOD {}", idx),
            bytecode::Op::Invoke(method_name, arg_count) => {
                format!("OP_INVOKE {} nargs={}", method_name, arg_count)
            }
            bytecode::Op::Inherit => "OP_INHERIT".to_string(),
            bytecode::Op::GetSuper(idx) => format!("OP_GET_SUPER {}", idx),
            bytecode::Op::SuperInvoke(method_name, arg_count) => {
                format!("OP_SUPER_INOKE {} nargs={}", method_name, arg_count)
            }
            bytecode::Op::BuildList(size) => format!("OP_BUILD_LIST {}", size),
            bytecode::Op::Subscr => "OP_SUBSCR".to_string(),
            bytecode::Op::SetItem => "OP_SETITEM".to_string(),
        };

        lines.push(format!("{0: <04}   {1: <50} line {2: <50}", idx, formatted_op, lineno.value));
    }
    lines
}

/**
 * 对一个 chunk 反汇编
 */
pub fn disassemble_chunk(chunk: &bytecode::Chunk, name: &str) -> String {
    let mut lines: Vec<String> = Vec::new();

    if !name.is_empty() {
        lines.push(format!("============ {} ============", name));
    }

    lines.push("------------ constants -----------".to_string());
    for (idx, constant) in chunk.constants.iter().enumerate() {
        lines.push(format!("{:<4} {}", idx, constant));
    }

    lines.push("\n------------ code -----------------".to_string());

    for code_line in disassemble_code(chunk) {
        lines.push(code_line);
    }

    lines.join("\n")
}

/**
 * 给解释器 + 一个参数（参数是一个 闭包函数的 id）
 * 给一个函数 反汇编
 */
fn dis_builtin(interp: &mut Interpreter, args: &[value::Value]) -> Result<value::Value, String> {
    // arity checking is done in the interpreter
    match &args[0] {
        value::Value::Function(closure_handle) => {
            let closure = interp.heap.get_closure(*closure_handle);
            disassemble_chunk(&closure.function.chunk, "");
            Ok(value::Value::Nil)
        }
        _ =>
            Err(
                format!("Invalid call: expected lox function, got {:?}.", value::type_of(&args[0]))
            ),
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Interpreter {
    pub frames: Vec<CallFrame>,
    pub stack: Vec<value::Value>,
    output: Vec<String>, // 可能是用来记载输出结果的
    pub globals: HashMap<String, value::Value>, // 全局变量表
    pub upvalues: Vec<Rc<RefCell<value::Upvalue>>>, // 对闭包的支持
    pub heap: gc::Heap, // 用来管理堆空间
    gray_stack: Vec<gc::HeapId>, // 垃圾回收辅助栈
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        let mut res = Interpreter { // 创建一个 result，其中的东西都是默认构造
            frames: Default::default(),
            stack: Default::default(),
            output: Default::default(),
            globals: Default::default(),
            upvalues: Default::default(),
            heap: Default::default(),
            gray_stack: Default::default(),
        };
        res.stack.reserve(256);
        res.frames.reserve(64);

        /* ---------- 添加一些内置的函数 ---------- */

        res.globals.insert(
            String::from("dis"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 1,
                name: String::from("dis"),
                func: dis_builtin,
            })
        );
        res.globals.insert(
            String::from("clock"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 0,
                name: String::from("clock"),
                func: builtins::clock,
            })
        );
        res.globals.insert(
            String::from("exp"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 1,
                name: String::from("exp"),
                func: builtins::exp,
            })
        );
        res.globals.insert(
            String::from("sqrt"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 1,
                name: String::from("sqrt"),
                func: builtins::sqrt,
            })
        );
        res.globals.insert(
            String::from("len"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 1,
                name: String::from("len"),
                func: builtins::len,
            })
        );
        res.globals.insert(
            String::from("forEach"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 2,
                name: String::from("forEach"),
                func: builtins::for_each,
            })
        );
        res.globals.insert(
            String::from("map"),
            value::Value::NativeFunction(value::NativeFunction {
                arity: 2,
                name: String::from("map"),
                func: builtins::map,
            })
        );

        res
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum InterpreterError {
    Runtime(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::Runtime(err) => write!(f, "Lox runtime error: {}", err),
        }
    }
}

#[derive(Default)]
pub struct CallFrame {
    pub closure: value::Closure,
    pub ip: usize, // 当前执行指令的位置
    pub slots_offset: usize, // 当前调用帧 在 解释器栈的偏移位置
}

impl CallFrame {
    fn next_op(&self) -> (bytecode::Op, bytecode::Lineno) {
        self.closure.function.chunk.code[self.ip].clone()
    }

    fn next_op_and_advance(&mut self) -> (bytecode::Op, bytecode::Lineno) {
        let res = self.next_op();
        self.ip += 1;
        res
    }

    /**
     * 获取指定下标的 字面量
     */
    fn read_constant(&self, idx: usize) -> bytecode::Constant {
        // chunk 存放了两个维度的东西：1. 操作符 2. 字面量池
        self.closure.function.chunk.constants[idx].clone()
    }
}

impl Interpreter {
    /**
     * 解释函数，并运行
     */
    pub fn interpret(&mut self, func: bytecode::Function) -> Result<(), InterpreterError> {
        self.prepare_interpret(func);
        self.run()
    }

    pub fn prepare_interpret(&mut self, func: bytecode::Function) {
        // 把闭包推入栈中
        self.stack.push(
            value::Value::Function(
                self.heap.manage_closure(value::Closure {
                    function: func.clone(),
                    upvalues: Vec::new(),
                })
            )
        );

        // 调用栈中 推入要执行的函数
        self.frames.push(CallFrame {
            closure: value::Closure {
                function: func,
                upvalues: Vec::new(),
            },
            ip: 0,
            slots_offset: 1,
        });
    }

    fn run(&mut self) -> Result<(), InterpreterError> {
        loop {
            if self.is_done() {
                return Ok(());
            }

            self.step()?;
        }
    }

    /**
     * 两种判断是否执行完毕的方法
     * frame 空了
     * ip 越界了
     */
    pub fn is_done(&self) -> bool {
        self.frames.is_empty() || self.frame().ip >= self.frame().closure.function.chunk.code.len()
    }

    pub fn step(&mut self) -> Result<(), InterpreterError> {
        let op = self.next_op_and_advance();

        // 每执行一步，都会判断是是否需要执行 垃圾回收
        if self.heap.should_collect() {
            self.collect_garbage();
        }

        match op {
            (bytecode::Op::Return, _) => {
                // 这个结果在 return 步骤之前就已经计算好并放在栈顶了
                let result = self.pop_stack();

                // slots_offset..self.stack.len() 就是 当前 函数 所有的局部变量
                for idx in self.frame().slots_offset..self.stack.len() {
                    self.close_upvalues(idx); // 当外层的函数返回的时候，确实要关闭上值
                }

                // 如果 frame 只有一个元素，说明程序即将结束
                if self.frames.len() <= 1 {
                    self.frames.pop();
                    return Ok(());
                }

                // 计算有多少个局部变量（弹多少次）
                let num_to_pop =
                    self.stack.len() -
                    self.frame().slots_offset +
                    usize::from(self.frame().closure.function.arity);
                self.frames.pop(); // 将当前函数 pop 出 frames

                // 我弹
                self.pop_stack_n_times(num_to_pop);

                // 将结果压栈，这里是先将结果给拿了出来，因为结果在栈顶，要操作栈顶下面的元素
                self.stack.push(result);
            }
            // 创建一个闭包
            (bytecode::Op::Closure(idx, upvals), _) => {
                let constant = self.read_constant(idx); // 期望得到的是一个 Value::Function
                if let value::Value::Function(closure_handle) = constant {
                    // 判断常量是不是一个函数，如果是函数，得到他的句柄

                    let closure = self.get_closure(closure_handle).clone();
                    let upvalues = upvals
                        .iter()
                        .map(|upval| {
                            // 对于每个元素
                            match upval {
                                bytecode::UpvalueLoc::Upvalue(idx) => {
                                    // 因为调用者肯定是 当前的 frames.top
                                    // 继承上值
                                    self.frame().closure.upvalues[*idx].clone()
                                }
                                bytecode::UpvalueLoc::Local(idx) => {
                                    if let Some(upval) = self.find_open_uval(*idx) {
                                        // 如果实际上并并不是一个 local 变量，而是一个 上值，但是伪装成了 local
                                        upval
                                    } else {
                                        // 如果真的只是在 outer 的 一个 local 变量，但是到了 inner 就是一个 上值 了
                                        let index = self.frame().slots_offset + *idx - 1; // 计算出他的 index
                                        let upval = Rc::new(
                                            RefCell::new(value::Upvalue::Open(index))
                                        );
                                        self.upvalues.push(upval.clone());
                                        upval
                                    }
                                }
                            }
                        })
                        .collect();

                    self.stack.push(
                        value::Value::Function(
                            self.heap.manage_closure(value::Closure {
                                function: closure.function,
                                upvalues,
                            })
                        )
                    );
                } else {
                    // 发生错误，直接 panic
                    panic!(
                        "When interpreting bytecode::Op::Closure, expected function, found {:?}",
                        value::type_of(&constant)
                    );
                }
            }

            // 这个有点叶子节点的感觉
            (bytecode::Op::Constant(idx), _) => {
                let constant = self.read_constant(idx);
                self.stack.push(constant);
            }
            (bytecode::Op::Nil, _) => {
                self.stack.push(value::Value::Nil);
            }
            (bytecode::Op::True, _) => {
                self.stack.push(value::Value::Bool(true));
            }
            (bytecode::Op::False, _) => {
                self.stack.push(value::Value::Bool(false));
            }
            (bytecode::Op::Negate, lineno) => {
                let top_stack = self.peek(); // 看一眼栈顶
                let maybe_number = Interpreter::extract_number(top_stack);

                match maybe_number {
                    Some(to_negate) => {
                        self.pop_stack();
                        self.stack.push(value::Value::Number(-to_negate));
                    }
                    None => {
                        // 如果不是数字，但是前面有一个 negate 那么就有问题
                        return Err(
                            InterpreterError::Runtime(
                                format!(
                                    "invalid operand to unary op negate. Expected number, found {:?} at line {}",
                                    value::type_of(top_stack),
                                    lineno.value
                                )
                            )
                        );
                    }
                }
            }
            (bytecode::Op::Add, lineno) => {
                let val1 = self.peek_by(0).clone();
                let val2 = self.peek_by(1).clone();

                match (&val1, &val2) {
                    (value::Value::Number(_), value::Value::Number(_)) => {
                        self.numeric_binop(Binop::Add, lineno)?;
                    }
                    (value::Value::String(s1), value::Value::String(s2)) => {
                        // 弹出 val1 和 val2
                        self.pop_stack();
                        self.pop_stack();
                        self.stack.push(
                            // 拼接字符串
                            value::Value::String(
                                self.heap.manage_str(
                                    // 注意一下这里的顺序，比较有意思
                                    format!("{}{}", self.get_str(*s2), self.get_str(*s1))
                                )
                            )
                        );
                    }
                    (value::Value::List(id1), value::Value::List(id2)) => {
                        self.pop_stack();
                        self.pop_stack();
                        let mut res = self.get_list_elements(*id2).clone();
                        res.extend(self.get_list_elements(*id1).clone());
                        self.stack.push(value::Value::List(self.heap.manage_list(res)));
                    }
                    _ => {
                        return Err(
                            InterpreterError::Runtime(
                                format!(
                                    "invalid operands of type {:?} and {:?} in add expression: \
                                 both operands must be number or string (line={})",
                                    value::type_of(&val1),
                                    value::type_of(&val2),
                                    lineno.value
                                )
                            )
                        );
                    }
                }
            }
            (bytecode::Op::Subtract, lineno) =>
                match self.numeric_binop(Binop::Sub, lineno) {
                    Ok(()) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            (bytecode::Op::Multiply, lineno) =>
                match self.numeric_binop(Binop::Mul, lineno) {
                    Ok(()) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            (bytecode::Op::Divide, lineno) =>
                match self.numeric_binop(Binop::Div, lineno) {
                    Ok(()) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            (bytecode::Op::Not, lineno) => {
                let top_stack = self.peek();
                let maybe_bool = Interpreter::extract_bool(top_stack); // 看一下栈顶是不是 bool 类型

                match maybe_bool {
                    Some(b) => {
                        self.pop_stack();
                        self.stack.push(value::Value::Bool(!b));
                    }
                    None => {
                        return Err(
                            InterpreterError::Runtime(
                                format!(
                                    "invalid operand in not expression. Expected boolean, found {:?} at line {}",
                                    value::type_of(top_stack),
                                    lineno.value
                                )
                            )
                        );
                    }
                }
            }
            (bytecode::Op::Equal, _) => {
                let val1 = self.pop_stack();
                let val2 = self.pop_stack();
                self.stack.push(value::Value::Bool(self.values_equal(&val1, &val2)));
            }
            (bytecode::Op::Greater, lineno) => {
                let val1 = self.peek_by(0).clone();
                let val2 = self.peek_by(1).clone();

                match (&val1, &val2) {
                    (value::Value::Number(n1), value::Value::Number(n2)) => {
                        self.pop_stack();
                        self.pop_stack();

                        self.stack.push(value::Value::Bool(n2 > n1));
                    }
                    _ => {
                        return Err(
                            InterpreterError::Runtime(
                                format!(
                                    "invalid operands in Greater expression. Expected numbers, found {:?} and {:?} at line {}",
                                    value::type_of(&val1),
                                    value::type_of(&val2),
                                    lineno.value
                                )
                            )
                        );
                    }
                }
            }
            (bytecode::Op::Less, lineno) => {
                let val1 = self.peek_by(0).clone();
                let val2 = self.peek_by(1).clone();

                match (&val1, &val2) {
                    (value::Value::Number(n1), value::Value::Number(n2)) => {
                        self.pop_stack();
                        self.pop_stack();
                        self.stack.push(value::Value::Bool(n2 < n1));
                    }
                    _ => {
                        return Err(
                            InterpreterError::Runtime(
                                format!(
                                    "invalid operands in Less expression. Expected numbers, found {:?} and {:?} at line {}",
                                    value::type_of(&val1),
                                    value::type_of(&val2),
                                    lineno.value
                                )
                            )
                        );
                    }
                }
            }
            (bytecode::Op::Print, _) => {
                let to_print = self.peek().clone();
                self.print_val(&to_print);
            }
            (bytecode::Op::Pop, _) => {
                self.pop_stack();
            }
            // 定义一个全局变量
            (bytecode::Op::DefineGlobal(idx), _) => {
                // 读取 idx 处的常量值，并检查这个是不是字符串，如果是字符串，那么这个字符串就是 IDENTIFIER
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    let val = self.pop_stack();
                    // 读取出栈顶的元素作为 val，并放到 hash 表中
                    self.globals.insert(self.get_str(name_id).clone(), val);
                } else {
                    panic!(
                        "expected string when defining global, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            // 目的是 从全局作用域中，得到一个全局变量的值
            (bytecode::Op::GetGlobal(idx), lineno) => {
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    match self.globals.get(self.get_str(name_id)) {
                        Some(val) => {
                            self.stack.push(val.clone());
                        }
                        None => {
                            return Err(
                                InterpreterError::Runtime(
                                    format!(
                                        "Undefined variable '{}' at line {}.",
                                        self.get_str(name_id),
                                        lineno.value
                                    )
                                )
                            );
                        }
                    }
                } else {
                    panic!(
                        "expected string when defining global, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            // 设置全局变量的值
            (bytecode::Op::SetGlobal(idx), lineno) => {
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    let name_str = self.get_str(name_id).clone();
                    let val = self.peek().clone();
                    if
                        // hash.entry 返回一个 enum { Occupid | vacant（空的） }
                        let std::collections::hash_map::Entry::Occupied(mut e) = self.globals.entry(
                            name_str.clone() // 标识符复制一份
                        )
                    {
                        e.insert(val); // 得到 entry，插入 val
                    } else {
                        return Err(
                            // 否则 如果是 vacant ，那么就是有错误产生
                            InterpreterError::Runtime(
                                format!(
                                    "Use of undefined variable {} in setitem expression at line {}.",
                                    name_str,
                                    lineno.value
                                )
                            )
                        );
                    }
                } else {
                    panic!(
                        "expected string when setting global, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }

            /* ---------- get 是放到 stack 顶 ---------- */

            /* ---------- set 从栈顶获取 val，并将 val 赋值给 idx 对应的变量 ---------- */

            /*
             * 根据局部变量的 idx 得到全局 stack 中的 val ，并将 val 加入 stack 中
             */
            (bytecode::Op::GetLocal(idx), _) => {
                let slots_offset = self.frame().slots_offset;
                let val = self.stack[slots_offset + idx - 1].clone();
                self.stack.push(val);
            }
            (bytecode::Op::SetLocal(idx), _) => {
                let val = self.peek();
                let slots_offset = self.frame().slots_offset;
                self.stack[slots_offset + idx - 1] = val.clone();
            }
            /*
             * 将上值 放到 栈顶
             */
            (bytecode::Op::GetUpval(idx), _) => {
                // 获取栈顶的 frame，得到 frame 的 closure，并获得 上值
                let upvalue = self.frame().closure.upvalues[idx].clone();
                let val = match &*upvalue.borrow() {
                    value::Upvalue::Closed(value) => value.clone(),
                    value::Upvalue::Open(stack_index) => self.stack[*stack_index].clone(),
                };
                self.stack.push(val);
            }
            (bytecode::Op::SetUpval(idx), _) => {
                let new_value = self.peek().clone();
                let upvalue = self.frame().closure.upvalues[idx].clone();
                match &mut *upvalue.borrow_mut() {
                    value::Upvalue::Closed(value) => {
                        *value = new_value;
                    }
                    // outer 函数还没有返回
                    value::Upvalue::Open(stack_index) => {
                        self.stack[*stack_index] = new_value;
                    }
                };
            }
            (bytecode::Op::JumpIfFalse(offset), _) => {
                if self.is_falsey(self.peek()) {
                    self.frame_mut().ip += offset; // 将 ip 指向 false 分支
                }
            }
            (bytecode::Op::Jump(offset), _) => {
                self.frame_mut().ip += offset;
            }
            (bytecode::Op::Loop(offset), _) => {
                self.frame_mut().ip -= offset; // 跳回到 loop 的开头
            }
            (bytecode::Op::Call(arg_count), _) => {
                self.call_value(self.peek_by(arg_count.into()).clone(), arg_count)?;
            }
            // 关闭 上值（outer 返回）
            (bytecode::Op::CloseUpvalue, _) => {
                let idx = self.stack.len() - 1;
                self.close_upvalues(idx);
                self.stack.pop();
            }
            (bytecode::Op::Class(idx), _) => {
                if let value::Value::String(name_id) = self.read_constant(idx) {
                    // 获取到 类名
                    let name = self.get_str(name_id).clone();
                    self.stack.push(
                        value::Value::Class(
                            self.heap.manage_class(value::Class {
                                name,
                                methods: HashMap::new(), // 没有继承，那么 方法就是空白的
                            })
                        )
                    );
                } else {
                    panic!(
                        "expected string when defining class, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            (bytecode::Op::SetProperty(idx), _) => {
                if let value::Value::String(attr_id) = self.read_constant(idx) {
                    let val = self.pop_stack();
                    let instance = self.pop_stack();
                    // 给对象设置 属性
                    self.setattr(instance, val.clone(), attr_id)?;
                    self.stack.push(val);
                } else {
                    panic!(
                        "expected string when setting property, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            (bytecode::Op::GetProperty(idx), _) => {
                if let value::Value::String(attr_id) = self.read_constant(idx) {
                    let maybe_instance = self.peek().clone();

                    let (class_id, instance_id) = match maybe_instance {
                        value::Value::Instance(instance_id) => {
                            let instance = self.heap.get_instance(instance_id).clone();
                            (instance.class_id, instance_id)
                        }
                        _ => panic!(),
                    };

                    let class = self.heap.get_class(class_id).clone();
                    // 能到这里 maybe_intance 一定是有一个 instance 的
                    if let Some(attr) = self.getattr(maybe_instance.clone(), attr_id)? {
                        self.pop_stack();
                        self.stack.push(attr);

                        // 如果绑定失败
                    } else if !self.bind_method(instance_id, class, attr_id)? {
                        return Err(
                            InterpreterError::Runtime(
                                format!(
                                    "value {} has no attribute {}.",
                                    self.format_val(&maybe_instance),
                                    self.get_str(attr_id)
                                )
                            )
                        );
                    }
                } else {
                    panic!(
                        "expected string when setting property, found {:?}",
                        value::type_of(&self.read_constant(idx))
                    );
                }
            }
            // 这段代码是在创建一个成员方法
            (bytecode::Op::Method(idx), _) => {
                if let value::Value::String(method_name_id) = self.read_constant(idx) {
                    let method_name = self.heap.get_str(method_name_id).clone();
                    let maybe_method = self.peek_by(0).clone();
                    let maybe_method_id = gc::Heap::extract_id(&maybe_method).unwrap();
                    let maybe_class = self.peek_by(1).clone();
                    match maybe_class {
                        value::Value::Class(class_id) => {
                            let class = self.heap.get_class_mut(class_id);
                            class.methods.insert(method_name, maybe_method_id);
                            self.pop_stack();
                        }
                        _ => {
                            panic!(
                                "should only define methods on a class! tried on {:?}",
                                self.format_val(&maybe_class)
                            );
                        }
                    }
                } else {
                    panic!("expected string when defining a method.");
                }
            }
            // invoke 调用成员函数：方法名 + 参数个数
            (bytecode::Op::Invoke(method_name, arg_count), _) => {
                self.invoke(&method_name, arg_count)?;
            }
            // 继承
            (bytecode::Op::Inherit, lineno) => {
                {
                    let (superclass_id, subclass_id) = match (self.peek_by(1), self.peek()) {
                        // subclass 在栈顶，superclass 是栈顶第二个元素
                        (value::Value::Class(superclass_id), value::Value::Class(subclass_id)) => {
                            (*superclass_id, *subclass_id)
                        }
                        (not_a_class, value::Value::Class(_)) => {
                            return Err(
                                InterpreterError::Runtime(
                                    format!(
                                        "Superclass must be a class, found {:?} at lineno={:?}",
                                        value::type_of(not_a_class),
                                        lineno
                                    )
                                )
                            );
                        }
                        _ => panic!("expected classes when interpreting Inherit!"),
                    };

                    let superclass_methods = self.get_class(superclass_id).methods.clone();
                    let subclass = self.get_class_mut(subclass_id); // 通过 id 获取到 子类的 可变借用

                    subclass.methods.extend(superclass_methods);
                }
                self.pop_stack(); //subclass
            }
            (bytecode::Op::GetSuper(idx), _) => {
                let method_id = if let value::Value::String(method_id) = self.read_constant(idx) {
                    method_id
                } else {
                    panic!();
                };

                // 得到父类的 id
                let maybe_superclass = self.pop_stack();
                let superclass = match maybe_superclass {
                    value::Value::Class(class_id) => self.get_class(class_id).clone(),
                    _ => panic!(),
                };

                // 如果 instance 的 class 对应，那么就可以了
                let maybe_instance = self.peek();
                let instance_id = match maybe_instance {
                    value::Value::Instance(instance_id) => *instance_id,
                    _ => panic!(),
                };

                // 如果绑定失败
                if !self.bind_method(instance_id, superclass, method_id)? {
                    return Err(
                        InterpreterError::Runtime(
                            format!(
                                "superclass {} has no attribute {}.",
                                self.format_val(&maybe_superclass),
                                self.get_str(method_id)
                            )
                        )
                    );
                }
            }

            // 这个有点动态多态的意思
            (bytecode::Op::SuperInvoke(method_name, arg_count), _) => {
                let maybe_superclass = self.pop_stack();
                let superclass_id = match maybe_superclass {
                    value::Value::Class(class_id) => class_id,
                    _ => panic!("{}", self.format_val(&maybe_superclass)),
                };
                self.invoke_from_class(superclass_id, &method_name, arg_count)?;
            }

            // 创建 list
            (bytecode::Op::BuildList(size), _) => {
                let mut list_elements = Vec::new();
                for _ in 0..size {
                    list_elements.push(self.pop_stack());
                }
                list_elements.reverse();
                self.stack.push(value::Value::List(self.heap.manage_list(list_elements)));
            }

            // 访问下标
            (bytecode::Op::Subscr, lineno) => {
                let subscript = self.pop_stack();
                let value_to_subscript = self.pop_stack();
                let res = self.subscript(value_to_subscript, subscript, lineno)?;
                self.stack.push(res);
            }
            (bytecode::Op::SetItem, lineno) => {
                let rhs = self.pop_stack(); // 右操作数
                let subscript = self.pop_stack();
                let lhs = self.pop_stack();
                self.setitem(lhs, subscript, rhs.clone(), lineno)?;
                self.stack.push(rhs);
            }
        }
        Ok(())
    }

    /**
     * 获取数字
     */
    fn extract_number(val: &value::Value) -> Option<f64> {
        match val {
            value::Value::Number(f) => Some(*f),
            _ => None,
        }
    }

    /**
     * 看一眼栈顶
     */
    fn peek(&self) -> &value::Value {
        self.peek_by(0)
    }

    /**
     * 看倒数第 0 个元素
     */
    fn peek_by(&self, n: usize) -> &value::Value {
        &self.stack[self.stack.len() - n - 1]
    }

    /**
     * 关闭上值
     */
    fn close_upvalues(&mut self, index: usize) {
        let value = &self.stack[index];
        for upval in &self.upvalues {
            if upval.borrow().is_open_with_index(index) {
                upval.replace(value::Upvalue::Closed(value.clone()));
            }
        }

        self.upvalues.retain(|u| u.borrow().is_open());
    }

    /**
     * 退栈，退的只是 Value 的栈
     */
    pub fn pop_stack(&mut self) -> value::Value {
        match self.stack.pop() {
            Some(val) => val,
            None => panic!("attempted to pop empty stack!"),
        }
    }

    /**
     * 这里 manage_xxx 实际上是分配内存，返回 id
     * idx --> constant --> value::Value
     */
    fn read_constant(&mut self, idx: usize) -> value::Value {
        let constant = self.frame().read_constant(idx);
        match constant {
            bytecode::Constant::Number(num) => value::Value::Number(num),
            bytecode::Constant::String(s) => value::Value::String(self.heap.manage_str(s)),
            bytecode::Constant::Function(f) => {
                value::Value::Function(
                    self.heap.manage_closure(value::Closure {
                        function: f.function,
                        upvalues: Vec::new(), // 初始的时候没有上值
                    })
                )
            }
        }
    }

    /**
     * 找到了 index 对应的 upval
     */
    fn find_open_uval(&self, index: usize) -> Option<Rc<RefCell<value::Upvalue>>> {
        for upval in self.upvalues.iter().rev() {
            // 从末尾遍历 .iter().rev()
            // 就是这里的 upvalues 与 index 并不是一个对应的关系
            if upval.borrow().is_open_with_index(index) {
                return Some(upval.clone());
            }
        }

        None
    }

    /*
     *
     */
    pub fn format_backtrace(&self) -> String {
        let lines: Vec<_> = self.frames
            .iter()
            .map(|frame| {
                let frame_name = &frame.closure.function.name;
                let (_, lineno) = frame.closure.function.chunk.code[frame.ip];
                if frame_name.is_empty() {
                    format!("[line {}] in script", lineno.value)
                } else {
                    format!("[line {}] in {}()", lineno.value, frame_name)
                }
            })
            .collect();
        format!("Backtrace (most recent call last):\n\n{}", lines.join("\n"))
    }

    pub fn format_upval(&self, val: &value::Upvalue) -> String {
        match val {
            value::Upvalue::Open(idx) => format!("Open({})", idx),
            value::Upvalue::Closed(val) => format!("Closed({})", self.format_val(val)),
        }
    }

    pub fn format_val(&self, val: &value::Value) -> String {
        match val {
            value::Value::Number(num) => num.to_string(),
            value::Value::Bool(b) => b.to_string(),
            value::Value::String(str_handle) => self.get_str(*str_handle).clone(),
            value::Value::Function(closure_handle) => {
                format!("<fn '{}'>", self.get_closure(*closure_handle).function.name)
            }
            value::Value::Class(class_handle) => {
                format!("<class '{}'>", self.get_class(*class_handle).name)
            }
            value::Value::Instance(instance_handle) => {
                let instance = self.get_instance(*instance_handle);
                let class_name = &self.get_class(instance.class_id).name;
                format!("<{} instance>", class_name)
            }
            value::Value::NativeFunction(func) => format!("<native fn {}>", func.name),
            value::Value::BoundMethod(bound_method_id) => {
                let bound_method = self.get_bound_method(*bound_method_id);
                let instance = self.get_instance(bound_method.instance_id);
                let class_name = &self.get_class(instance.class_id).name;
                format!("<bound method of {} instance>", class_name)
            }
            value::Value::Nil => "nil".to_string(),
            value::Value::List(list_id) => {
                let elements = self.get_list_elements(*list_id);
                format!(
                    "[{}]",
                    elements
                        .iter()
                        .map(|element| self.format_val(element))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }

    /*
     * lhs[subscript] = rhs
     */
    fn setitem(
        &mut self,
        lhs: value::Value,
        subscript: value::Value,
        rhs: value::Value,
        lineno: bytecode::Lineno
    ) -> Result<(), InterpreterError> {
        if let value::Value::List(id) = lhs {
            if let value::Value::Number(index_float) = subscript {
                let elements = self.get_list_elements_mut(id);
                match Interpreter::subscript_to_inbound_index(elements.len(), index_float, lineno) {
                    Ok(index_int) => {
                        // 其实主要也就是这一句话
                        elements[index_int] = rhs;
                        Ok(())
                    }
                    Err(err) => Err(InterpreterError::Runtime(err)),
                }
            } else {
                Err(
                    InterpreterError::Runtime(
                        format!(
                            "Invalid subscript of type {:?} in subscript expression",
                            value::type_of(&lhs)
                        )
                    )
                )
            }
        } else {
            Err(
                InterpreterError::Runtime(
                    format!(
                        "Invalid value of type {:?} in subscript expression",
                        value::type_of(&subscript)
                    )
                )
            )
        }
    }

    /*
     * 访问下标 value[subscript]
     */
    fn subscript(
        &mut self,
        value: value::Value,
        subscript: value::Value,
        lineno: bytecode::Lineno
    ) -> Result<value::Value, InterpreterError> {
        if let value::Value::List(id) = value {
            if let value::Value::Number(index_float) = subscript {
                // 因为 lox 中的数字只有 float，就连 下标（字面量）也是 float
                let elements = self.get_list_elements(id);
                match Interpreter::subscript_to_inbound_index(elements.len(), index_float, lineno) {
                    Ok(index_int) => Ok(elements[index_int].clone()),
                    Err(err) => Err(InterpreterError::Runtime(err)),
                }
            } else {
                Err(
                    InterpreterError::Runtime(
                        format!(
                            "Invalid subscript of type {:?} in subscript expression",
                            value::type_of(&value)
                        )
                    )
                )
            }
        } else {
            Err(
                InterpreterError::Runtime(
                    format!(
                        "Invalid value of type {:?} in subscript expression",
                        value::type_of(&value)
                    )
                )
            )
        }
    }

    /*
     * 接受一个 下标（可以是整数，也可以是负数），转化为 真正的下表
     */
    fn subscript_to_inbound_index(
        list_len: usize,
        index_float: f64,
        lineno: bytecode::Lineno
    ) -> Result<usize, String> {
        let index_int = index_float as i64; // 先转化为 i64
        if 0 <= index_int && index_int < (list_len as i64) {
            return Ok(index_int as usize);
        }
        if index_int < 0 && -index_int <= (list_len as i64) {
            return Ok(((list_len as i64) + index_int) as usize);
        }
        Err(format!("List subscript index out of range at {}", lineno.value))
    }

    /*
     * 调用成员函数
     */
    fn invoke(&mut self, method_name: &str, arg_count: u8) -> Result<(), InterpreterError> {
        // 看前几个元素
        let receiver_id = match self.peek_by(arg_count.into()) {
            value::Value::Instance(id) => *id, // 得到实例的 id
            _ => {
                return Err(InterpreterError::Runtime("Only instances have methods.".to_string()));
            }
        };

        if
            let Some(field) = self
                .get_instance(receiver_id)
                .fields.get(&String::from(method_name)) // 得到的是 value::Value::<可调用对象>
                .cloned()
        {
            return self.call_value(field, arg_count);
        }

        let class_id = self.get_instance(receiver_id).class_id;
        self.invoke_from_class(class_id, method_name, arg_count)
    }

    /*
     * 调用类中的函数
     */
    fn invoke_from_class(
        &mut self,
        class_id: gc::HeapId,
        method_name: &str,
        arg_count: u8
    ) -> Result<(), InterpreterError> {
        let method_id = match self.get_class(class_id).methods.get(&String::from(method_name)) {
            Some(method_id) => *method_id,
            None => {
                return Err(
                    InterpreterError::Runtime(format!("Undefined property {}.", method_name))
                );
            }
        };

        self.call_value(value::Value::Function(method_id), arg_count)
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        let frames_len = self.frames.len();
        &mut self.frames[frames_len - 1]
    }

    /**
     * 返回 栈顶的 Option<&CallFrame>
     */
    pub fn maybe_frame(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    /**
     * 返回 栈顶的 frame
     */
    pub fn frame(&self) -> &CallFrame {
        self.maybe_frame().unwrap()
    }

    /*
     * 调用
     */
    pub fn call_value(
        &mut self,
        val_to_call: value::Value,
        arg_count: u8 // 参数的个数
    ) -> Result<(), InterpreterError> {
        match val_to_call {
            value::Value::Function(func) => {
                self.prepare_call(func, arg_count)?;
                Ok(())
            }
            // 也就只有 内置的函数 会真正的调用
            value::Value::NativeFunction(native_func) => {
                self.call_native_func(native_func, arg_count)?;
                Ok(())
            }
            value::Value::Class(class_id) => {
                let new_instance = value::Value::Instance(
                    // 创建了一个空白对象
                    self.heap.manage_instance(value::Instance {
                        class_id,
                        fields: HashMap::new(),
                    })
                );

                let arg_count_usize: usize = arg_count.into();
                let stack_len = self.stack.len();
                self.stack[stack_len - 1 - arg_count_usize] = new_instance; // 将对象压入 栈中

                {
                    let maybe_method_id = self
                        .get_class(class_id)
                        .methods.get(&"init".to_string())
                        .copied(); // 得到构造函数

                    if let Some(method_id) = maybe_method_id {
                        return self.prepare_call(method_id, arg_count);
                    }
                }

                if arg_count > 0 {
                    return Err(
                        InterpreterError::Runtime(
                            format!("Call to class ctor expected 0 arguments, got {}.", arg_count)
                        )
                    );
                }

                self.create_instance(class_id);
                Ok(())
            }
            // BoundMethod 是一个包含 this 对象的方法
            value::Value::BoundMethod(method_id) => {
                self.call_bound_method(method_id, arg_count)?;
                Ok(())
            }
            _ =>
                Err(
                    InterpreterError::Runtime(
                        format!(
                            "attempted to call non-callable value of type {:?}.",
                            value::type_of(&val_to_call)
                        )
                    )
                ),
        }
    }

    /*
     * 真正的调用了
     */
    fn call_native_func(
        &mut self,
        native_func: value::NativeFunction,
        arg_count: u8
    ) -> Result<(), InterpreterError> {
        // 如果参数不相等
        if arg_count != native_func.arity {
            return Err(
                InterpreterError::Runtime(
                    format!(
                        "Native function {} expected {} arguments but found {}.",
                        native_func.name,
                        native_func.arity,
                        arg_count
                    )
                )
            );
        }

        let mut args = Vec::new();
        // args = stack 栈顶的几个参数
        for _ in 0..arg_count {
            args.push(self.pop_stack()); // pop args
        }
        args.reverse(); // 栈顶相反
        let args = args; // 设置为不变
        self.pop_stack(); // native function value

        // 调用函数
        let res = (native_func.func)(self, &args);

        match res {
            Ok(result) => {
                self.stack.push(result);
                Ok(())
            }
            Err(err) =>
                Err(
                    InterpreterError::Runtime(
                        format!("When calling {}: {}.", native_func.name, err)
                    )
                ),
        }
    }

    /*
     * 很奇怪，没有真正的调用
     */
    fn call_bound_method(
        &mut self,
        method_id: gc::HeapId,
        arg_count: u8
    ) -> Result<(), InterpreterError> {
        let bound_method = self.get_bound_method(method_id).clone();
        let closure_id = bound_method.closure_id;
        let arg_count_usize: usize = arg_count.into(); // 很有意思，有 usize::from 也有 <usize>.into
        let stack_len = self.stack.len();
        // 将对象压入 stack 中
        self.stack[stack_len - arg_count_usize - 1] = value::Value::Instance(
            bound_method.instance_id
        );
        self.prepare_call(closure_id, arg_count)
    }

    /*
     * Set up a few call frame so that on the next interpreter step we'll start executing code inside the function.
     * 就是在调用函数之前，要准备他的 frame，以及准备他 栈上的数据
     */
    fn prepare_call(
        &mut self,
        closure_handle: gc::HeapId,
        arg_count: u8
    ) -> Result<(), InterpreterError> {
        let closure = self.get_closure(closure_handle).clone();
        let func = &closure.function;

        // 如果 参数个数不相匹配，那么就返回 Err
        if arg_count != func.arity {
            return Err(
                InterpreterError::Runtime(
                    format!("Expected {} arguments but found {}.", func.arity, arg_count)
                )
            );
        }

        self.frames.push(CallFrame::default()); // 默认构造一个 frame
        let mut frame = self.frames.last_mut().unwrap();
        frame.closure = closure;
        frame.slots_offset = self.stack.len() - usize::from(arg_count); // 给 frame 设置 stack[len - arg_count, len] 的位置
        Ok(())
    }

    fn create_instance(&mut self, class_id: gc::HeapId) {
        self.pop_stack(); // class object
        let instance_id = self.heap.manage_instance(value::Instance {
            class_id,
            fields: HashMap::new(),
        });
        self.stack.push(value::Value::Instance(instance_id));
    }

    fn pop_stack_n_times(&mut self, num_to_pop: usize) {
        for _ in 0..num_to_pop {
            self.pop_stack();
        }
    }

    /*
     * 判断是不是 false
     */
    fn is_falsey(&self, val: &value::Value) -> bool {
        match val {
            value::Value::Nil => true,
            value::Value::Bool(b) => !*b,
            value::Value::Number(f) => *f == 0.0,
            value::Value::Function(_) => false,
            value::Value::NativeFunction(_) => false,
            value::Value::Class(_) => false,
            value::Value::Instance(_) => false,
            value::Value::BoundMethod(_) => false,
            value::Value::String(id) => self.get_str(*id).is_empty(),
            value::Value::List(id) => self.get_list_elements(*id).is_empty(),
        }
    }

    /**
     * 打印 Value
     */
    fn print_val(&mut self, val: &value::Value) {
        let output = self.format_val(val);
        println!("{}", output);
        self.output.push(output);
    }

    /**
     * 判断是不是相等
     */
    fn values_equal(&self, val1: &value::Value, val2: &value::Value) -> bool {
        match (val1, val2) {
            (value::Value::Number(n1), value::Value::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
            (value::Value::Bool(b1), value::Value::Bool(b2)) => b1 == b2,
            (value::Value::String(s1), value::Value::String(s2)) => {
                self.get_str(*s1) == self.get_str(*s2)
            }
            (value::Value::Nil, value::Value::Nil) => true,
            (_, _) => false,
        }
    }

    /**
     * 数字的二元操作，但是我对 优先级有点困惑
     */
    fn numeric_binop(
        &mut self,
        binop: Binop,
        lineno: bytecode::Lineno
    ) -> Result<(), InterpreterError> {
        let val1 = self.peek_by(0).clone();
        let val2 = self.peek_by(1).clone();

        match (&val1, &val2) {
            (value::Value::Number(n1), value::Value::Number(n2)) => {
                self.pop_stack();
                self.pop_stack();
                self.stack.push(
                    value::Value::Number(
                        Interpreter::apply_numeric_binop(
                            *n2,
                            *n1,
                            binop // note the order!
                        )
                    )
                );
                Ok(())
            }
            _ =>
                Err(
                    InterpreterError::Runtime(
                        format!(
                            "Expected numbers in {:?} expression. Found {:?} and {:?} (line={})",
                            binop,
                            value::type_of(&val1),
                            value::type_of(&val2),
                            lineno.value
                        )
                    )
                ),
        }
    }

    fn apply_numeric_binop(left: f64, right: f64, binop: Binop) -> f64 {
        match binop {
            Binop::Add => left + right,
            Binop::Sub => left - right,
            Binop::Mul => left * right,
            Binop::Div => left / right,
        }
    }

    /**
     * instance.attr = val
     */
    fn setattr(
        &mut self,
        maybe_instance: value::Value,
        val: value::Value,
        attr_id: gc::HeapId
    ) -> Result<(), InterpreterError> {
        let attr_name = self.get_str(attr_id).clone();
        match maybe_instance {
            value::Value::Instance(instance_id) => {
                let instance = self.heap.get_instance_mut(instance_id);
                instance.fields.insert(attr_name, val);
                Ok(())
            }
            _ =>
                Err(
                    InterpreterError::Runtime(
                        format!(
                            "can't set attribute on value of type {:?}. Need class instance. val = {:?}",
                            value::type_of(&maybe_instance),
                            self.format_val(&maybe_instance)
                        )
                    )
                ),
        }
    }

    fn getattr(
        &self,
        maybe_instance: value::Value,
        attr_id: gc::HeapId
    ) -> Result<Option<value::Value>, InterpreterError> {
        let attr_name = self.get_str(attr_id).clone();
        match maybe_instance {
            value::Value::Instance(instance_id) => {
                let instance = self.heap.get_instance(instance_id);
                match instance.fields.get(&attr_name) {
                    Some(val) => Ok(Some(val.clone())),
                    None => Ok(None),
                }
            }
            _ =>
                Err(
                    InterpreterError::Runtime(
                        format!(
                            "can't get attribute {}  on value of type {:?}. Need class instance.",
                            attr_name,
                            value::type_of(&maybe_instance)
                        )
                    )
                ),
        }
    }

    /*
     * 将闭包与 对象绑定
     */
    fn bind_method(
        &mut self,
        instance_id: gc::HeapId,
        class: value::Class,
        attr_id: gc::HeapId
    ) -> Result<bool, InterpreterError> {
        let attr_name = self.get_str(attr_id).clone();
        if let Some(closure_id) = class.methods.get(&attr_name) {
            self.pop_stack();
            self.stack.push(
                value::Value::BoundMethod(
                    self.heap.manage_bound_method(value::BoundMethod {
                        instance_id,
                        closure_id: *closure_id,
                    })
                )
            );
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn next_line(&self) -> usize {
        self.next_op().1.value
    }

    pub fn next_op(&self) -> (bytecode::Op, bytecode::Lineno) {
        self.frame().next_op()
    }

    /*
     * 执行 frame 上的 内容
     */
    fn next_op_and_advance(&mut self) -> (bytecode::Op, bytecode::Lineno) {
        self.frame_mut().next_op_and_advance()
    }

    fn extract_bool(val: &value::Value) -> Option<bool> {
        match val {
            value::Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /**
     * 传入一个 str_handle （str 的 句柄）返回 String 类型
     */
    fn get_str(&self, str_handle: gc::HeapId) -> &String {
        self.heap.get_str(str_handle)
    }

    /**
     * 通过 句柄获取 Closure
     */
    fn get_closure(&self, closure_handle: gc::HeapId) -> &value::Closure {
        self.heap.get_closure(closure_handle)
    }

    fn get_class(&self, class_handle: gc::HeapId) -> &value::Class {
        self.heap.get_class(class_handle)
    }

    fn get_class_mut(&mut self, class_handle: gc::HeapId) -> &mut value::Class {
        self.heap.get_class_mut(class_handle)
    }

    fn get_bound_method(&self, method_handle: gc::HeapId) -> &value::BoundMethod {
        self.heap.get_bound_method(method_handle)
    }

    fn get_list_elements(&self, list_handle: gc::HeapId) -> &Vec<value::Value> {
        self.heap.get_list_elements(list_handle)
    }

    fn get_list_elements_mut(&mut self, list_handle: gc::HeapId) -> &mut Vec<value::Value> {
        self.heap.get_list_elements_mut(list_handle)
    }

    fn get_instance(&self, instance_handle: gc::HeapId) -> &value::Instance {
        self.heap.get_instance(instance_handle)
    }

    /*
     * 垃圾回收（但是这样的实现，感觉还是太操蛋了）
     * 你说这种递归，他真的能比得过 AST 的版本吗？
     */
    fn collect_garbage(&mut self) {
        self.heap.unmark(); // 将 heap 所有的元素 mark=false
        self.mark_roots(); // 重新标记
        self.trace_references();

        self.heap.sweep();
    }

    // 递归的标记，所有可以到达的对象
    fn trace_references(&mut self) {
        loop {
            let maybe_val = self.gray_stack.pop();
            match maybe_val {
                Some(val) => self.blacken_object(val),
                None => {
                    break;
                }
            }
        }
    }

    fn blacken_object(&mut self, val: gc::HeapId) {
        let children_to_walk = self.heap.children(val);
        for child_val in children_to_walk {
            if !self.heap.is_marked(child_val) {
                self.heap.mark(child_val);
                self.blacken_object(child_val);
            }
        }
    }

    fn mark_roots(&mut self) {
        let stack_vals_to_mark: Vec<gc::HeapId> = self.stack
            .iter()
            .filter_map(gc::Heap::extract_id)
            .collect();

        let frame_closure_children: Vec<gc::HeapId> = self.frames
            .iter()
            .flat_map(|frame| self.heap.closure_children(&frame.closure))
            .collect();

        let globals_to_mark: Vec<gc::HeapId> = self.globals
            .values()
            .flat_map(gc::Heap::extract_id)
            .collect();

        for val in stack_vals_to_mark
            .iter()
            .chain(frame_closure_children.iter())
            .chain(globals_to_mark.iter()) {
            // chain 起来，然后 mark=true
            self.mark_value(*val);
        }
    }

    fn mark_value(&mut self, handle: gc::HeapId) {
        let is_marked = self.heap.is_marked(handle);
        if !is_marked {
            self.heap.mark(handle);
        }
        self.gray_stack.push(handle)
    }
}
