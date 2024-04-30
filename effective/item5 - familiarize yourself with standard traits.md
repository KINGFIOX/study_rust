请对 standard tratis 保持熟悉：

- Clone
- Copy
- Default
- PartialEq
- Eq
- PartialOrd
- Ord
- Hash
- Debug
- Display

当然还有其他特征，当前的 Item4 不会讲这些

- Fn FnOnce FnMut
- Error
- Drop
- From TryFrom
- Deref DerefMut
- Iterator
- Send
- Sync

## Clone

Clone 就是拷贝构造.

- 不该 Clone :

    - 如果是 unique_ptr 或者 RAII ， 那么不应该 Clone

    - 有些字段包含 un-Clone-able : 比方说： fields that are mutable reference(`&mut T`)，因为 mut ref 应当是 single at a time ； 如果是 mutex 或者是 mutexguard 那么不应该 Clone

- 应该手动实现 Clone : 缓存（如果有了 cache ，那么不就返回 cache 里面的就好了，不用再拷贝）、深拷贝等

## Copy

Clone 也是 拷贝构造。这是一个 marker trait ，他不用我们实现什么。(not user-defined)

与 Clone 相比，Copy 只是浅拷贝。
然后他将 assign 的语义从 Move 到了 Copy

## Default

```rust
#[derive(Default)]
struct Colour {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}

let c = Colour {
	red: 128,
	// 这个很有趣
	..Default::default()
};
```

## Eq && PartialEq

`Eq <: PartialEq` Eq 至少与 PartialEq 一样有用。
Eq 只是一个 marker ，他比 PartialEq 多了一个 自反性(_reflexivity_)

什么情况下是没有自反性的，比方说 IEEE754 float point，这个很有趣了：
因为 NaN 与 ∞ 确实是没有自反性的，自身与自身不能比较。

因为 user-defined 类型一般是不会像 float point 那样操蛋的，因此：you should implement `Eq` whenever you implement `PartialEq`，特别是在 Hash 中一定要有

Hash 需要 Eq

## Ord && PartialOrd

`Ord <: Eq` && `PartialOrd <: PartialEq`

这个就是 数学意义上的 偏序与全序了。
偏序：并不是所有的东西都是可比的。
（当然，其实也可以修改成自己的定义啦，不一定那么数学）

```rust
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.x.cmp(&other.x))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

fn main() {
    let a = Point { x: 5, y: 2 };
    let b = Point { x: 3, y: 8 };

    // 使用 PartialOrd
    println!("{:?}", a.partial_cmp(&b)); // Some(Greater)

    // 使用 Ord
    println!("{:?}", a.cmp(&b)); // Greater
}
```

## Display && Debug

Debug 是可以自动派生的； 但是 Display 是不能自动派生的（因为 Display 是面向 user 的，是需要我们自己控制的）

推荐都 derived Debug ，除非有涉及到一些 隐私安全的

## overload operators

- **avoid overloading operators for unrelated types** as it often leads to code that is hard to maintain and has unexpected performance properties (e.g. `x + y` silently invokes an expensive O(N) method).

- **implement a coherent set of operator overloads**. For example, if `x + y` has an overload ([`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)), and `-y` ([`Neg`](https://doc.rust-lang.org/std/ops/trait.Neg.html)), then you should also implement `x - y` ([`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)) and make sure it gives the same answer as `x + (-y)`.
