// Copyright (c) 2016 rust-threshold-secret-sharing developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate bencher;
extern crate threshold_secret_sharing as tss;

mod shamir_vs_packed {

    use bencher::Bencher;
    use tss::shamir::*;

    pub fn bench_100_shamir(b: &mut Bencher) {
        let ref tss = ShamirSecretSharing {
            threshold: 155 / 3,
            // share: 728 / 3,
            share_count: 728,
            prime: 746497,
        };

        let all_secrets: Vec<i64> = vec![5 ; 100 ];
        b.iter(|| {
            let _shares: Vec<Vec<i64>> = all_secrets.iter()
                .map(|&secret| tss.share(secret))
                .collect();
        });
    }

    pub fn bench_100_packed(b: &mut Bencher) {
        use tss::packed::*;
        let ref pss = PSS_155_728_100;
        let all_secrets: Vec<i64> = vec![5 ; 100];
        b.iter(|| {
            let _shares = pss.share(&all_secrets);
        })
    }

}

benchmark_group!(shamir_vs_packed,
                 shamir_vs_packed::bench_100_shamir,
                 shamir_vs_packed::bench_100_packed);


mod packed {

    use bencher::Bencher;
    use tss::packed::*;

    pub fn bench_large_secret_count(b: &mut Bencher) {
        let ref pss = PSS_155_728_100;
        let all_secrets = vec![5 ; pss.secret_count * 100];
        b.iter(|| {
            let _shares: Vec<Vec<i64>> = all_secrets.chunks(pss.secret_count)
                .map(|secrets| pss.share(&secrets))
                .collect();
        });
    }

    pub fn bench_large_share_count(b: &mut Bencher) {
        let ref pss = PSS_155_19682_100;
        let secrets = vec![5 ; pss.secret_count];
        b.iter(|| {
            let _shares = pss.share(&secrets);
        });
    }

    pub fn bench_large_reconstruct(b: &mut Bencher) {
        let ref pss = PSS_155_19682_100;
        let secrets = vec![5 ; pss.secret_count];
        let all_shares = pss.share(&secrets);

        // reconstruct using minimum number of shares required
        let indices: Vec<usize> = (0..pss.reconstruct_limit()).collect();
        let shares = &all_shares[0..pss.reconstruct_limit()];

        b.iter(|| {
            let _recovered_secrets = pss.reconstruct(&indices, &shares);
        });
    }

/*     pub fn bench_polynomial_gf2_8(bench: &mut Bencher) {
        // 定义有限域 GF(2^8) 的范围
        const GF_2_8_MAX: u64 = 255;
    
        // 定义多项式系数 a 和 b
        let a: u64 = GF_2_8_MAX; // 255
        let b: u64 = GF_2_8_MAX - 1; // 254
    
        // 定义 x 的值
        let x_values = vec![5u64, 10u64]; // x = 5 和 x = 10
    
        // 定义每次计算的组数
        let num_calculations = 10_000_000;
    
        // 记录 5 组运行时间
        let mut times = Vec::new();
    
        for _ in 0..5 {
            let start = std::time::Instant::now();
    
            // 进行多项式计算
            for _ in 0..num_calculations {
                for &x in &x_values {
                    let _y = (a * x + b) % GF_2_8_MAX; // 模运算确保结果在 GF(2^8) 范围内
                }
            }
    
            let duration = start.elapsed();
            times.push(duration.as_secs_f64());
        }
    
        // 计算平均时间
        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
    
        // 输出结果
        println!("GF(2^8) Polynomial Calculation Average Time: {:.6} seconds", avg_time);
    
        // 使用 bencher 进行基准测试
        bench.iter(|| {
            for _ in 0..num_calculations {
                for &x in &x_values {
                    let _y = (a * x + b) % GF_2_8_MAX;
                }
            }
        });
    } */

    /* pub fn bench_polynomial_gf2_8(bench: &mut Bencher) {
        // 定义有限域 GF(2^8) 的范围
        const GF_2_8_MAX: u64 = 255;
    
        // 定义多项式系数 a 和 b
        let a: u64 = GF_2_8_MAX; // 255
        let b: u64 = GF_2_8_MAX - 1; // 254
    
        // 定义 x 的值
        let x_values = vec![5u64, 10u64]; // x = 5 和 x = 10
    
        // 定义每次计算的组数 - 增加计算量
        let num_calculations = 10_000_000;
    
        // 在基准测试外部进行计时并输出结果
        // 这样只会输出一次
        {
            // 记录 5 组运行时间
            let mut times = Vec::new();
    
            for i in 0..5 {
                let start = std::time::Instant::now();
    
                // 进行多项式计算
                for _ in 0..num_calculations {
                    for &x in &x_values {
                        let _y = (a * x + b) % GF_2_8_MAX;
                    }
                }
    
                let duration = start.elapsed();
                let seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
                times.push(seconds);
                
                println!("GF(2^8) Run {}: {:.6} seconds", i+1, seconds);
            }
    
            // 计算平均时间
            let avg_time = times.iter().sum::<f64>() / times.len() as f64;
    
            // 输出结果，使用更多小数位
            println!("----------------------------------------");
            println!("GF(2^8) Polynomial Calculation Average Time: {:.9} seconds", avg_time);
            println!("----------------------------------------");
        }
    
        // 为了避免在基准测试中重复输出，使用较小的计算量
        bench.iter(|| {
            for _ in 0..10_000_000 {
                for &x in &x_values {
                    let _y = (a * x + b) % GF_2_8_MAX;
                }
            }
        });
    }
 */
    pub fn bench_polynomial_gf2_8(bench: &mut Bencher) {
    const GF_2_8_MAX: u64 = 255;
    let a = GF_2_8_MAX - 1; // 254
    let b = GF_2_8_MAX - 2; // 253
    let x_values = [5u64, 10u64];
    let num_calculations = 10_000_000;

    // 使用 black_box 强制保留计算
    let mut dummy = 0;
    {
        let mut times = Vec::new();
        for _ in 0..5 {
            let start = std::time::Instant::now();
            for _ in 0..num_calculations {
                for &x in &x_values {
                    let y = (a * x + b) % GF_2_8_MAX;
                    std::hint::black_box(y); // 阻止优化
                    dummy += y as u64; // 确保结果被使用（可选）
                }
            }
            let duration = start.elapsed();
            times.push(duration.as_secs_f64());
            println!("Run time: {:.6} seconds", duration.as_secs_f64());
        }
        let avg_time = times.iter().sum::<f64>() / times.len() as f64;
        println!("Average: {:.9} seconds", avg_time);
    }
    
    // 避免优化消除 dummy
    std::hint::black_box(dummy);

    // Bencher 测试部分
    bench.iter(|| {
        for _ in 0..num_calculations {
            for &x in &x_values {
                let y = (a * x + b) % GF_2_8_MAX;
                std::hint::black_box(y);
            }
        }
    });
    }
}

/* benchmark_group!(packed,
    packed::bench_large_secret_count,
    packed::bench_large_share_count,
    packed::bench_large_reconstruct); */
    
benchmark_group!(packed,
                 packed::bench_large_secret_count,
                 packed::bench_large_share_count,
                 packed::bench_large_reconstruct,
                 packed::bench_polynomial_gf2_8); // 添加新函数

benchmark_main!(shamir_vs_packed, packed);

//初次测试代码GF2^8