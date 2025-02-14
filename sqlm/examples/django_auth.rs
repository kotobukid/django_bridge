use base64;
use base64::{engine::general_purpose, Engine as _};
use ring::pbkdf2;
use std::num::NonZeroU32;

fn main() {
    let hashed_password =
        "pbkdf2_sha256$870000$sHkKiWrJE8kN0gXdkpHMw4$wBQp3tnoICz/cQLhydscEYIBT0VvG5OGFQDJ/I67fpE=";
    let password_user_posted = "hogehoge";

    let result = try_login(hashed_password, password_user_posted);
    println!("login result: {}", result);
}

fn try_login(hashed_password: &str, password_user_posted: &str) -> bool {
    // ハッシュ文字列を分割
    let parsed: Vec<&str> = hashed_password.split('$').collect();
    if parsed.len() != 4 {
        return false;
    }

    let algo = parsed[0];
    let iterations = parsed[1].parse::<u32>().unwrap_or(0);
    let salt = parsed[2];
    let hash = parsed[3];

    // アルゴリズムチェック
    if algo != "pbkdf2_sha256" {
        return false;
    }

    let start_time = std::time::Instant::now();

    // ハッシュされたパスワードを取得
    let decoded_hash = match general_purpose::STANDARD.decode(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    // PBKDF2でパスワードを再計算
    let mut derived_hash = vec![0u8; decoded_hash.len()];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        salt.as_bytes(),
        password_user_posted.as_bytes(),
        &mut derived_hash,
    );

    // ハッシュを比較（constant-time comparison）
    let result = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        salt.as_bytes(),
        password_user_posted.as_bytes(),
        &decoded_hash,
    )
    .is_ok();

    let duration = start_time.elapsed();

    println!("pbkdf2_sha256: {}ms", duration.as_millis());

    result
}
