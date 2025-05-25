#[macro_use]
extern crate bencher;
extern crate num_bigint;

use bencher::Bencher;
use std::time::Instant;
use num_bigint::{BigUint, ToBigUint};

/// 测试不同有限域的多项式计算性能
fn bench_polynomial_gf(bench: &mut Bencher, bits: u32) {
    // 定义有限域 GF(2^n) 的模数
    let modulus = (BigUint::from(1u32) << bits) - BigUint::from(1u32); // 2^n - 1

    // 定义多项式系数 a 和 b
    let a = modulus.clone() - BigUint::from(1u32); // 2^n - 2
    let b = modulus.clone() - BigUint::from(2u32); // 2^n - 3

    // 定义 x 的值
    let x_values = [5u32.to_biguint().unwrap(), 10u32.to_biguint().unwrap()];

    // 定义计算次数
    let num_calculations = 10_000_000;

    println!("----------------------------------------");
    println!("测试有限域 GF(2^{}) 上的多项式计算:", bits);
    println!("域大小: 2^{} - 1", bits);
    println!("多项式: y = ax + b，其中 a = 2^{} - 2, b = 2^{} - 3", bits, bits);
    println!("计算次数: {}", num_calculations);

    // 记录 5 组运行时间
    let mut times = Vec::new();
    for i in 1..=5 {
        let start = Instant::now();

        // 使用一个累加器防止编译器优化
        let mut dummy = BigUint::from(0u32);

        for _ in 0..num_calculations {
            for x in &x_values {
                // 计算 y = (a * x + b) % modulus
                let ax = &a * x;
                let y = (ax + &b) % &modulus;

                // 确保结果被保留
                dummy = (dummy + y) % &modulus;
                std::hint::black_box(&dummy);
            }
        }

        // 防止优化
        std::hint::black_box(&dummy);

        let duration = start.elapsed();
        let seconds = duration.as_secs_f64();
        times.push(seconds);

        println!("运行 {}: {:.6} 秒", i, seconds);
    }

    // 计算平均运行时间
    let avg_time = times.iter().sum::<f64>() / times.len() as f64;
    println!("平均运行时间: {:.9} 秒", avg_time);
    println!("----------------------------------------");

    // 使用 Bencher 进行基准测试
    bench.iter(|| {
        let mut result = BigUint::from(0u32);
        for _ in 0..num_calculations {
            for x in &x_values {
                let y = (&a * x + &b) % &modulus;
                result = (result + y) % &modulus;
                std::hint::black_box(&result);
            }
        }
        std::hint::black_box(result);
    });
}

// 为所有有限域大小创建基准测试函数
fn bench_polynomial_gf2_8(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 8);
}

fn bench_polynomial_gf2_16(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 16);
}

fn bench_polynomial_gf2_32(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 32);
}

fn bench_polynomial_gf2_64(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 64);
}

fn bench_polynomial_gf2_128(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 128);
}

fn bench_polynomial_gf2_256(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 256);
}

fn bench_polynomial_gf2_512(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 512);
}

fn bench_polynomial_gf2_1024(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 1024);
}

fn bench_polynomial_gf2_2048(bench: &mut Bencher) {
    bench_polynomial_gf(bench, 2048);
}

// 定义基准测试组
benchmark_group!(
    polynomial_gf_benchmarks,
    bench_polynomial_gf2_8,
    bench_polynomial_gf2_16,
    bench_polynomial_gf2_32,
    bench_polynomial_gf2_64,
    bench_polynomial_gf2_128,
    bench_polynomial_gf2_256,
    bench_polynomial_gf2_512,
    bench_polynomial_gf2_1024,
    bench_polynomial_gf2_2048
);

benchmark_main!(polynomial_gf_benchmarks);
//why there is no change on github after commit??