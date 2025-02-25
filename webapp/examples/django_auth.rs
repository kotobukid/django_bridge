use ring::pbkdf2 as ring_pbkdf2;
use std::num::NonZeroU32;
use std::time::Instant;

use base64::{engine::general_purpose, Engine};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;

/// メイン関数：結果を簡潔に表示し、事前定義されたテストケースを確認
fn main() {
    let password_user_posted = "hogehoge";

    // Django PBKDF2ハッシュ（テスト用）
    let hashed_password_pbkdf2 =
        "pbkdf2_sha256$870000$sHkKiWrJE8kN0gXdkpHMw4$wBQp3tnoICz/cQLhydscEYIBT0VvG5OGFQDJ/I67fpE=";

    println!("=== Begin Password Verification Tests ===");
    println!("\nCases of VALID password:");
    // Test 1: ringによる検証
    match try_login_with_ring(hashed_password_pbkdf2, password_user_posted) {
        true => println!("[PASS] PBKDF2 verification (ring): Password matched."),
        false => println!("[FAIL] PBKDF2 verification (ring): Password did not match."),
    }

    // Test 2: pbkdf2クレートによる検証
    match verify_django_password(hashed_password_pbkdf2, password_user_posted) {
        Ok(true) => println!("[PASS] PBKDF2 verification (pbkdf2 crate): Password matched."),
        Ok(false) => println!("[FAIL] PBKDF2 verification (pbkdf2 crate): Password did not match."),
        Err(e) => println!(
            "[ERROR] PBKDF2 verification (pbkdf2 crate): Error occurred: {}",
            e
        ),
    }

    println!("\nCases of INVALID password:");
    // Test 3: ringによる検証(失敗)
    match try_login_with_ring(hashed_password_pbkdf2, "hogehoge1") {
        true => println!("[PASS] PBKDF2 verification (ring): Password matched."),
        false => println!("[FAIL] PBKDF2 verification (ring): Password did not match."),
    }

    // Test 4: pbkdf2クレートによる検証(失敗)
    match verify_django_password(hashed_password_pbkdf2, "hogehoge1") {
        Ok(true) => println!("[PASS] PBKDF2 verification (pbkdf2 crate): Password matched."),
        Ok(false) => println!("[FAIL] PBKDF2 verification (pbkdf2 crate): Password did not match."),
        Err(e) => println!(
            "[ERROR] PBKDF2 verification (pbkdf2 crate): Error occurred: {}",
            e
        ),
    }

    println!("=== End Password Verification Tests ===");
}

/// `ring`クレートを使用したPBKDF2認証
fn try_login_with_ring(hashed_password: &str, password: &str) -> bool {
    // Djangoのハッシュ形式を解析
    let parsed: Vec<&str> = hashed_password.split('$').collect();
    if parsed.len() != 4 {
        println!("[ERROR] Invalid password hash format (ring).");
        return false;
    }

    // イテレーション数やその他データを抽出
    let iterations = match parsed[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("[ERROR] Failed to parse iterations (ring).");
            return false;
        }
    };
    let salt = parsed[2];
    let hash_base64 = parsed[3];

    // Base64でエンコードされたハッシュ値をデコード
    let decoded_hash = match general_purpose::STANDARD.decode(hash_base64) {
        Ok(h) => h,
        Err(_) => {
            println!("[ERROR] Failed to decode hash from base64 (ring).");
            return false;
        }
    };

    // 時間計測開始
    let start_time = Instant::now();

    // 実際の認証
    let result = ring_pbkdf2::verify(
        ring_pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        salt.as_bytes(),
        password.as_bytes(),
        &decoded_hash,
    )
    .is_ok();

    // 時間計測終了
    let elapsed_time = start_time.elapsed();
    println!(
        "PBKDF2 (ring) verification completed in {}ms",
        elapsed_time.as_millis()
    );

    result
}

/// `pbkdf2`クレートを使用したDjango形式PBKDF2認証
fn verify_django_password(
    django_hash: &str,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Djangoハッシュ形式を分解
    let parts: Vec<&str> = django_hash.split('$').collect();
    if parts.len() != 4 {
        return Err("Invalid hash format".into());
    }

    // イテレーション数とソルト、元ハッシュを抽出
    let iterations = parts[1].parse::<u32>()?;
    let salt = parts[2];
    let hash = parts[3];

    // 時間計測開始
    let start_time = Instant::now();

    // pbkdf2クレートでハッシュを計算
    let output = pbkdf2_hmac_array::<Sha256, 32>(password.as_bytes(), salt.as_bytes(), iterations);

    // 計算したハッシュをBase64エンコード
    let calculated_hash = general_purpose::STANDARD.encode(output);

    // 末尾のパディングを除外して比較
    let result = calculated_hash.trim_end_matches('=') == hash.trim_end_matches('=');

    // 時間計測終了
    let elapsed_time = start_time.elapsed();
    println!(
        "PBKDF2 (pbkdf2 crate) verification completed in {}ms",
        elapsed_time.as_millis()
    );

    Ok(result)
}
