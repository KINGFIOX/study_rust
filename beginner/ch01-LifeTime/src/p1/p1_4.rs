/* 让代码工作 */ use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    // i 是有所有权的数据，并没有包含任何引用，因此它是 'static
    let i = 5;
    print_it(i);

    // 但是 &i 是一个引用，生命周期受限于作用域，因此它不是 'static
    print_it(&i);

    print_it1(&i);

    // 但是下面的代码可以正常运行 !
    print_it2(&i);
}
