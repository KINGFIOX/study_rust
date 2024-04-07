#[macro_export]
macro_rules! count_exprs {
    () => (0);  // 空匹配， 0
    ($head:expr) => (1);
    // 有点函数式编程的感觉
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

// 宏的导出一定要 macro_export
#[macro_export]
macro_rules! recurrence {
    // $(...), + 代表的是一个重复的模式
    // 只不过我们这里的 ... 也是要匹配一个东西，而不是一个字面量，因此里面也有 $
    // ident 元类型 可以保证不会出现 上下文冲突
    ($seq:ident[$ind:ident]: $sty:ty = $($inits:expr),+ ; ... ; $recur:expr ) => {{
        /*
            What follows here is *literally* the code from before,
            cut and pasted into a new position. No other changes
            have been made.
        */

        use std::ops::Index;

        const MEM_SIZE: usize = count_exprs!($($inits),+);  // 有种 C++ trait 的感觉

        struct Recurrence {
            mem: [$sty; MEM_SIZE],
            pos: usize,
        }

        struct IndexOffset<'a> {
            slice: &'a [$sty; MEM_SIZE],
            offset: usize,
        }

        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = $sty;

            #[inline(always)]
            fn index<'b>(&'b self, index: usize) -> &'b $sty {
                use std::num::Wrapping;

                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(MEM_SIZE);

                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }

        impl Iterator for Recurrence {
            type Item = $sty;

            #[inline]
            fn next(&mut self) -> Option<$sty> {
                if self.pos < MEM_SIZE {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let next_val = {
                        let $ind = self.pos;
                        let $seq = IndexOffset {
                            slice: &self.mem,
                            offset: $ind,
                        };
                        $recur
                    };

                    {
                        use std::mem::swap;

                        let mut swap_tmp = next_val;
                        for i in (0..MEM_SIZE).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }

                    self.pos += 1;
                    Some(next_val)
                }
            }
        }

        // 这里暂时还是 写死的
        Recurrence {
            mem: [$($inits),+],  // 但是我们接下来要知道: 这个长度是多少，会用在 window 窗口大小中
            pos: 0,
        }
    }};
}
