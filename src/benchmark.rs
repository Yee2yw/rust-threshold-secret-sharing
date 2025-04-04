use rand::distributions::{Distribution, Uniform};
use rand::rngs::OsRng;
use std::time::Instant;

fn evaluate_polynomial(coefficients: &[i64], x: i64, prime: i64) -> i64 {
    coefficients.iter()
        .enumerate()
        .map(|(i, &coeff)| coeff * x.pow(i as u32))
        .sum::<i64>() % prime
}

fn benchmark_polynomial(prime: i64, x_values: &[i64], coefficients: &[i64], iterations: usize) {
    for &x in x_values {
        let mut total_time = 0;
        for _ in 0..5 {
            let start = Instant::now();
            for _ in 0..iterations {
                evaluate_polynomial(coefficients, x, prime);
            }
            let duration = start.elapsed();
            total_time += duration.as_millis();
            println!("x = {}, iteration time: {} ms", x, duration.as_millis());
        }
        let average_time = total_time as f64 / 5.0;
        println!("x = {}, average time: {} ms", x, average_time);
    }
}

fn main() {
    let prime = 256; // GF(2^8)
    let coefficients = [255, 254]; // 多项式系数
    let x_values = [5, 10]; // x 的值
    let iterations = 10_000_000; // 每次计算 1000w 组

    benchmark_polynomial(prime, &x_values, &coefficients, iterations);
}