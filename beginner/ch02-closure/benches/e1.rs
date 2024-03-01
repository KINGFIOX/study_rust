#![feature(test)]

extern crate rand;
extern crate test;

/// 使用 for 手动进行求和
fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

/// 求和
fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[cfg(test)]
mod bench {
    use test::Bencher;
    use rand::{ Rng, thread_rng };
    use super::*;

    const LEN: usize = 1024 * 1024;

    /// 0 到 cnt-1
    fn rand_array(cnt: u32) -> Vec<f64> {
        let mut rng = thread_rng(); // 获得随机数的生成器
        (0..cnt).map(|_| rng.gen::<f64>()).collect() // 生成 cnt 个随机数
    }

    #[bench]
    fn bench_for(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| { sum_for(&samples) })
        // b 是一个 bencher ，b.iter 接受一个匿名函数，是 testbench 的核心
    }

    #[bench]
    fn bench_iter(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| { sum_iter(&samples) })
    }
}
