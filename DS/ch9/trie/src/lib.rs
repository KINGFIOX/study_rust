#[derive(Default)]
struct Trie {
    root: Node,
}

#[derive(Default)]
struct Node {
    end: bool, // 是否是单词的结尾
    children: [Option<Box<Node>>; 26],
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    // 单词插入
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root; // node 就是 cur 游标 ，非常的优雅
        for c in word.as_bytes() {
            let index = (c - b'a') as usize;
            let next = &mut node.children[index];
            node = next.get_or_insert_with(Box::<Node>::default);
        }
        node.end = true;
    }

    fn search(&self, word: &str) -> bool {
        // 优雅: word_node -> Option<&Node>
        // 1. None 说明没有找到前缀
        // 2. n.end 说明了是单词的结尾
        self.word_node(word).map_or(false, |n| n.end)
    }

    fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some()
    }

    // wps : word prefix string 前缀字符串
    fn word_node(&self, wps: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in wps.as_bytes() {
            let index = (c - b'a') as usize;
            match &node.children[index] {
                None => return None,
                Some(next) => node = next.as_ref(),
            }
        }
        Some(node)
    }
}

mod tests {
    use crate::Trie;

    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.insert("box");
        trie.insert("wangfiox");
        trie.insert("insert");
        trie.insert("apple");
        trie.insert("appeal");

        let res1 = trie.search("apple");
        let res2 = trie.search("apples");
        let res3 = trie.search("ins");
        let res4 = trie.search("ina");
        let res5 = trie.start_with("app");
        let res6 = trie.search("app");

        println!("word: apple, search: {}", res1);
        println!("word: apples, search: {}", res2);
        println!("word: ins, search: {}", res3);
        println!("word: ina, search: {}", res4);
        println!("prefix: app, start_with: {}", res5);
        println!("word: app, search: {}", res6);
    }
}
