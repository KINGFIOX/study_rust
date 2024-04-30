
## 迭代的范式

Iterator transformation expressions like this can roughly be broken down into three parts:

- An initial source iterator, from one of Rust's iterator traits.
- A sequence of iterator transforms.
- A ﬁnal consumer method to combine the results of the iteration into a ﬁnal value.

## source

- An initial source iterator, from one of Rust's iterator traits.


### 迭代器的等价性 ref

在 Rust 中，`(&collection).into_iter()` 和 `collection.iter()` 实际上是等价的，它们都创建了一个不可变的迭代器来遍历集合中的元素。这两种形式虽然在语法上略有不同，但在功能上是一样的。

这里是怎么回事：

- `collection.iter()` 直接调用集合的 `iter` 方法，这是最直观的方式来获取一个不可变迭代器。

- `(&collection).into_iter()` 则是先获取集合的引用，然后调用 `IntoIterator` trait 的 `into_iter` 方法。对于集合的引用，`IntoIterator` 实现通常会返回一个与 `.iter()` 相同的迭代器。

### 迭代器的等价性 ref_mut

`(&mut collection).into_iter()` 和 `collection.iter_mut()` 在 Rust 中基本上是等价的，都用来创建一个可变迭代器，允许你在迭代时修改集合中的元素。这两种方法虽然语法上不同，但功能上是相同的。

具体来说：

- `collection.iter_mut()` 直接调用集合的 `iter_mut` 方法，这是直接和明确的方式来获得一个可变迭代器。

- `(&mut collection).into_iter()` 是获取集合的可变引用，然后调用 `IntoIterator` trait 的 `into_iter` 方法。对于可变引用，`IntoIterator` 的实现通常会返回与 `.iter_mut()` 相同的迭代器。

## transform

- A sequence of iterator transforms.

### 常见 api

- take(n)
- skip(n)
- step_by(n)
- cycle()
- rev() 要求实现 DoubleEndedIterator(next_back)
- map(closure)
- cloned()
- copied()
- enumerater()
- zip(it)
- filter(closure)
- take_while() 和 skip_while()
- chain
- flatten

### chain

```rust
fn main() {
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 创建第一个迭代器
    let iter1 = array1.iter();
    // 创建第二个迭代器
    let iter2 = array2.iter();

    // 使用 chain 方法连接两个迭代器
    let combined_iter = iter1.chain(iter2);

    // 遍历组合后的迭代器并打印每个元素
    for number in combined_iter {
        println!("{}", number);
    }
}
```

### flatten

```rust
fn main() {
    let nested_vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let flat_iter = nested_vec.into_iter().flatten();

    for num in flat_iter {
        println!("{}", num);
    }
}
```

## consumer

- A ﬁnal consumer method to combine the results of the iteration into a ﬁnal value.

### 常见 api

- sum()
- product()
- min() , max()
- min_by(f) , max_by(f)
- reduce(f)
- fold(f)
- scan(f)
- find(p)
- position(p)
- nth(n)
- all(p)
- try_for_each
- try_fold(f)
- try_find(f)
- collect()

举一个例子

```rust
fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
where
	Self: Sized,
	F: FnMut(&mut St, Self::Item) -> Option<B>,
```

### collect()

Finally, there are methods that accumulate all of the iterated items into a new collection. The most important of these is collect() , which can be used to build a new instance of any collection type that implements the FromIterator trait.

## from reasult value

不抛出错误，但是得到的是 Result

```rust
let result: Vec<Result<u8, _>> =
	inputs.into_iter().map(|v| <u8>::try_from(v)).collect();
```

抛出错误，但是

```rust
let result: Vec<u8> = inputs
	.into_iter()
	.map(|v| <u8>::try_from(v))
	.collect::<Result<Vec<_>, _>>()?;
```

? 意味着：

- If the iteration encounters an error value, that error value is emitted to the caller and iteration stops.
- If no errors are encountered, the remainder of the code can deal with a sensible collection of values of the right type.

