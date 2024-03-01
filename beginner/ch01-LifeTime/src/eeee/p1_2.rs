use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    let i = String::from("fuck");

    let j = &i;

    print_it(&i);

    print_it(j);
}
