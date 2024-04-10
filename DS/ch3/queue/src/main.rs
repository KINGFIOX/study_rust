struct Queue<T, const N: usize> {
    data: [Option<T>; N],
    front: usize,
    rear: usize,
    size: usize,
}

impl<T, const N: usize> Queue<T, N>
where
    T: Copy,
{
    fn new(cap: usize) -> Self {
        Queue {
            data: [None; N],
            front: 0,
            rear: 0,
            size: 0,
        }
    }

    fn enqueue(&mut self, item: T) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear] = Some(item);
        self.rear = (self.rear + 1) % N;
        self.size += 1;
        true
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // 将 self.data[self.front] 的值取出来，并将 填充默认值
        let result = std::mem::take(&mut self.data[self.front]);
        self.front = (self.front + 1) % N;
        self.size -= 1;
        result
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == N
    }
}

fn main() {
    println!("Hello, world!");
}
