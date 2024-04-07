macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}

macro_rules! match_tokens {
    ($a:tt + $b:tt) => {
        "got an addition"
    };
    (($i:ident)) => {
        "got an identifier"
    };
    ($($other:tt)*) => {
        "got something else"
    };
}

fn main() {
    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5)
    );
    // got an identifier
    // got an addition
    // got something else

    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5)
    );
    // got something else
    // got something else
    // got something else
    // 这是因为 ast 树已经确定了，这里都是 expr，
    // 然后到了 match_tokens 中 ，只剩下 tt 可以匹配了
}

/* ---------- */
