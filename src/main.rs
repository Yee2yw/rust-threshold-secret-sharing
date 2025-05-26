mod shamir;

fn main() {
    // 创建一个 ShamirSecretSharing 实例
    let tss = shamir::ShamirSecretSharing {
        threshold: 2,
        share_count: 6,
        prime: 41,
    };

    let secret = 1;

    // 生成分享
    let shares = tss.share(secret);
    println!("生成的分享: {:?}", shares);

    // 使用部分分享重建秘密
    let recovered_secret = tss.reconstruct(&[0, 1, 2], &shares[0..3]);
    println!("重建的秘密: {}", recovered_secret);

    assert_eq!(recovered_secret, secret);
}