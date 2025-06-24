use crate::gen::cards;
use crate::CardExport;

/// 内部フィルタリング関数：fetch_by_combined_bits のロジック
pub fn filter_by_combined_bits(bit1: i64, bit2: i64, mode: &str) -> Vec<CardExport> {
    cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            match mode {
                "and" => {
                    // AND条件: 指定されたビットが全て立っている
                    (bit1 == 0 || (feature_bits1 & bit1) == bit1)
                        && (bit2 == 0 || (feature_bits2 & bit2) == bit2)
                }
                "or" => {
                    // OR条件: 指定されたビットのいずれかが立っている
                    if bit1 == 0 && bit2 == 0 {
                        true
                    } else {
                        (bit1 > 0 && (feature_bits1 & bit1) != 0)
                            || (bit2 > 0 && (feature_bits2 & bit2) != 0)
                    }
                }
                _ => true,
            }
        })
        .map(|c| CardExport::from(c))
        .collect()
}

/// 内部フィルタリング関数：fetch_by_features_and のロジック
pub fn filter_by_features_and(features: &[i32]) -> Vec<CardExport> {
    cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            // 全てのフィーチャーを満たすかチェック（AND条件）
            for i in (0..features.len()).step_by(2) {
                if i + 1 >= features.len() {
                    break;
                }

                let shift1 = features[i];
                let shift2 = features[i + 1];

                // 両方とも-1の場合はスキップ
                if shift1 < 0 && shift2 < 0 {
                    continue;
                }

                let bit1 = if shift1 >= 0 { 1_i64 << shift1 } else { 0 };
                let bit2 = if shift2 >= 0 { 1_i64 << shift2 } else { 0 };

                let has_feature = if bit1 > 0 && bit2 > 0 {
                    (feature_bits1 & bit1) != 0 && (feature_bits2 & bit2) != 0
                } else if bit1 > 0 {
                    (feature_bits1 & bit1) != 0
                } else if bit2 > 0 {
                    (feature_bits2 & bit2) != 0
                } else {
                    false
                };

                if !has_feature {
                    return false;
                }
            }

            true
        })
        .map(|c| CardExport::from(c))
        .collect()
}

/// 内部フィルタリング関数：fetch_by_f_bits のロジック
pub fn filter_by_f_bits(bit1: i64, bits2: i64) -> Vec<CardExport> {
    cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            // 条件関数の確定
            if bit1 == 0 && bits2 == 0 {
                true
            } else if bits2 == 0 || bits2 == 1 {
                (feature_bits1 & bit1) != 0
            } else if bit1 == 0 || bit1 == 1 {
                (feature_bits2 & bits2) != 0
            } else {
                (feature_bits1 & bit1) == bit1 && (feature_bits2 & bits2) == bits2
            }
        })
        .map(|c| CardExport::from(c))
        .collect()
}

/// 内部フィルタリング関数：fetch_by_f_shifts のロジック  
pub fn filter_by_f_shifts(shift1: isize, shift2: isize) -> Vec<CardExport> {
    let bits1 = 1_i64 << shift1;
    let bits2 = 1_i64 << shift2;

    cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.19;
            let feature_bits2 = c.20;

            // 条件関数の確定
            if bits1 == 0 && bits2 == 0 {
                true
            } else if bits2 == 0 || bits2 == 1 {
                (feature_bits1 & bits1) != 0
            } else if bits1 == 0 || bits1 == 1 {
                (feature_bits2 & bits2) != 0
            } else {
                (feature_bits1 & bits1) == bits1 && (feature_bits2 & bits2) == bits2
            }
        })
        .map(|c| CardExport::from(c))
        .collect()
}

/// 内部フィルタリング関数：fetch_by_burst_bits のロジック
pub fn filter_by_burst_bits(burst_bits: i64, mode: &str) -> Vec<CardExport> {
    cards::CARD_LIST
        .iter()
        .filter(|c| {
            let card_burst_bits = c.22; // burst_bitsフィールドのインデックス（23番目）

            match mode {
                "and" => {
                    // AND条件: 指定されたビットが全て立っている
                    burst_bits == 0 || (card_burst_bits & burst_bits) == burst_bits
                }
                "or" => {
                    // OR条件: 指定されたビットのいずれかが立っている
                    burst_bits == 0 || (card_burst_bits & burst_bits) != 0
                }
                _ => true,
            }
        })
        .map(|c| CardExport::from(c))
        .collect()
}
