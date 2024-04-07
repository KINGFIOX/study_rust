macro_rules! what_is {
    (self) => {
        "the keyword `self`"
    };
    ($i:ident) => {
        concat!("the identifier `", stringify!($i), "`")
    };
}

macro_rules! call_with_ident {
    // 这里的意思是: 期待一个 xxx(xxxx) 的形式
    ($c:ident($i:ident)) => {
        $c!($i)
    };
}

fn main() {
    println!("{}", what_is!(self));
    println!("{}", call_with_ident!(what_is(self)));
    // the keyword `self`
    // the keyword `self`
}

// 意思就是: 这里 self 按道理来说是一个 keyword 来着。但是这里确实是被 what_is! 宏处理的。
