#[derive(Clone)]
struct Container<T>(T);

fn clone_Containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();
}
