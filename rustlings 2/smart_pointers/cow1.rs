// cow1.rs
//
// 写时复制. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily 当出现 mut 或者 所有权转移的时候才复制
// required. The type is designed to work with general borrowed data via the
// Borrow trait. 写时复制 与 常规的 borrowed 一起使用（通过 borrow trait）
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

// TODO 这一章有点问题

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn reference_mutation() -> Result<(), &'static str> {
    //     // Clone occurs because `input` needs to be mutated.
    //     let slice = [-1, 0, 1];
    //     let mut input = Cow::from(&slice[..]);
    //     match abs_all(&mut input) {
    //         Cow::Owned(_) => Ok(()),
    //         _ => Err("Expected owned value"),
    //     }
    // }

    // #[test]
    // fn reference_no_mutation() -> Result<(), &'static str> {
    //     // 这里有啥问题
    //     // No clone occurs because `input` doesn't need to be mutated.
    //     let slice = [0, 1, 2];
    //     let mut input = Cow::from(&slice[..]); // 写时复制
    //     match abs_all(&mut input) {
    //         Cow::Owned(o) => {
    //             assert_eq!(o, slice.to_vec());
    //             Ok(())
    //         }
    //         _ => Err("Expected owned value"),
    //     }
    // }

    // #[test]
    // fn owned_no_mutation() -> Result<(), &'static str> {
    //     // We can also pass `slice` without `&` so Cow owns it directly. In this
    //     // case no mutation occurs and thus also no clone, but the result is
    //     // still owned because it was never borrowed or mutated.
    //     let slice = vec![0, 1, 2];
    //     let mut input = Cow::from(slice);
    //     match abs_all(&mut input) {
    //         Cow::Owned(o) => {
    //             assert_eq!(o.to_vec(), input.to_vec());
    //             Ok(())
    //         }
    //         _ => Err("Expected owned value"),
    //     }
    // }

    // #[test]
    // fn owned_mutation() -> Result<(), &'static str> {
    //     // Of course this is also the case if a mutation does occur. In this
    //     // case the call to `to_mut()` in the abs_all() function returns a
    //     // reference to the same data as before.
    //     let slice = vec![-1, 0, 1];
    //     let mut input = Cow::from(slice);
    //     match abs_all(&mut input) {
    //         Cow::Owned(o) => {
    //             assert_eq!(o.to_vec(), input.to_vec());
    //             Ok(())
    //         }
    //         _ => Err("Expected owned value"),
    //     }
    // }
}
