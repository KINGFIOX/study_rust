// in lib.rs

/// Add one to the given value and return the value
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

// cargo doc --open

/** Add two to the given value and return a new value


let arg = 5;
let answer = my_crate::add_two(arg);

assert_eq!(7, answer);

*/

/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doc-comments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

/// Add one to the given value and return a [`Option`] type
pub fn add_three(x: i32) -> Option<i32> {
    Some(x + 3)
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
