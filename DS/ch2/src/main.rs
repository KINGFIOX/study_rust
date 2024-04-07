fn main() {
    let s1 = "rust";
    let s2 = "trus";
    let result = anagram_solution4(s1, s2);
    println!("{s1} == {s2} ? {result}");
}

fn anagram_solution4(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for c in s1.chars() {
        let pos = (c as usize) - 97;
        c1[pos] += 1;
    }
    for c in s1.chars() {
        let pos = (c as usize) - 97;
        c2[pos] += 1;
    }
    let mut pos = 0;
    let mut ok = true;
    while pos < 26 && ok {
        if c1[pos] == c2[pos] {
            pos += 1;
        } else {
            ok = false;
        }
    }
    ok
}
