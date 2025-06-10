use feature::feature::CardFeature;
use models::card::CreateCard;
use models::r#gen::django_models::RawCardDb;
use std::collections::HashSet;

/// Convert full-width characters to half-width equivalents
/// Handles:
/// - Full-width alphanumeric characters (Ａ-Ｚ, ａ-ｚ, ０-９)
/// - Full-width space (　)
/// - Common full-width symbols
pub fn to_half(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            // Full-width uppercase letters (Ａ-Ｚ) -> (A-Z)
            'Ａ'..='Ｚ' => {
                let offset = ch as u32 - 'Ａ' as u32;
                char::from_u32('A' as u32 + offset).unwrap()
            }
            // Full-width lowercase letters (ａ-ｚ) -> (a-z)
            'ａ'..='ｚ' => {
                let offset = ch as u32 - 'ａ' as u32;
                char::from_u32('a' as u32 + offset).unwrap()
            }
            // Full-width digits (０-９) -> (0-9)
            '０'..='９' => {
                let offset = ch as u32 - '０' as u32;
                char::from_u32('0' as u32 + offset).unwrap()
            }
            // Full-width space
            '　' => ' ',
            // Common full-width symbols
            '－' => '-',  // Full-width hyphen/minus
            '＋' => '+',  // Full-width plus
            '．' => '.',  // Full-width period
            '，' => ',',  // Full-width comma
            '：' => ':',  // Full-width colon
            '；' => ';',  // Full-width semicolon
            '！' => '!',  // Full-width exclamation
            '？' => '?',  // Full-width question mark
            '（' => '(',  // Full-width left parenthesis
            '）' => ')',  // Full-width right parenthesis
            '［' => '[',  // Full-width left bracket
            '］' => ']',  // Full-width right bracket
            '＊' => '*',  // Full-width asterisk
            '＆' => '&',  // Full-width ampersand
            '＝' => '=',  // Full-width equals
            '／' => '/',  // Full-width slash
            '＜' => '<',  // Full-width less than
            '＞' => '>',  // Full-width greater than
            // Keep other characters as-is
            _ => ch,
        })
        .collect()
}

/// Error type for card analysis
#[derive(Debug, Clone)]
pub struct AnalysisError {
    pub message: String,
    pub card_id: i64,
}

impl AnalysisError {
    pub fn new(message: String, card_id: i64) -> Self {
        Self { message, card_id }
    }
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Analysis error for card {}: {}",
            self.card_id, self.message
        )
    }
}

impl std::error::Error for AnalysisError {}

/// Trait for analyzing raw card data and converting to CreateCard
#[async_trait::async_trait]
pub trait RawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError>;

    /// Analyze multiple cards in batch
    async fn analyze_batch(
        &self,
        raw_cards: &[RawCardDb],
    ) -> Vec<Result<CreateCard, AnalysisError>> {
        let mut results = Vec::new();
        for raw_card in raw_cards {
            results.push(self.analyze(raw_card).await);
        }
        results
    }
}


/// Convert a set of features to two 64-bit integers
pub fn features_to_bits(features: &HashSet<CardFeature>) -> (i64, i64) {
    let mut bits1: i64 = 0;
    let mut bits2: i64 = 0;

    for feature in features {
        let (shift1, shift2) = feature.to_bit_shifts();
        if shift1 != 0 {
            bits1 |= shift1;
        }
        if shift2 != 0 {
            bits2 |= shift2;
        }
    }

    (bits1, bits2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_half_uppercase() {
        assert_eq!(to_half("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), 
                   "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    #[test]
    fn test_to_half_lowercase() {
        assert_eq!(to_half("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), 
                   "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn test_to_half_numbers() {
        assert_eq!(to_half("０１２３４５６７８９"), "0123456789");
    }

    #[test]
    fn test_to_half_space() {
        assert_eq!(to_half("ハロー　ワールド"), "ハロー ワールド");
    }

    #[test]
    fn test_to_half_mixed() {
        assert_eq!(to_half("カードＷＸ２４－Ｐ１"), "カードWX24-P1");
        assert_eq!(to_half("レベル３　パワー１２０００"), "レベル3 パワー12000");
        assert_eq!(to_half("ＬＢ：カードを１枚引く"), "LB:カードを1枚引く");
    }

    #[test]
    fn test_to_half_preserves_japanese() {
        let input = "このシグニはアサシンを持つ";
        assert_eq!(to_half(input), input);
    }

    #[test]
    fn test_to_half_symbols() {
        // Test that full-width symbols are converted
        assert_eq!(to_half("【チャーム】《ガードアイコン》：；・"), "【チャーム】《ガードアイコン》:;・");
        assert_eq!(to_half("（パワー＋１０００）"), "(パワー+1000)");
        assert_eq!(to_half("！？＝＜＞"), "!?=<>");
    }

    #[test]
    fn test_to_half_real_card_examples() {
        // Real card name examples
        assert_eq!(to_half("コードアクセル　Ｈｙａｈｈａｈ"), "コードアクセル Hyahhah");
        assert_eq!(to_half("羅植姫　ガーベラ／／Ｍｅｍｏｒｉａｌ"), "羅植姫 ガーベラ//Memorial");
        
        // Real skill text examples
        assert_eq!(to_half("【エナチャージ１】をする"), "【エナチャージ1】をする");
        assert_eq!(to_half("パワーを＋２０００する"), "パワーを+2000する");
    }

    #[test]
    fn test_to_half_empty_string() {
        assert_eq!(to_half(""), "");
    }

    #[test]
    fn test_features_to_bits() {
        let mut features = HashSet::new();
        features.insert(CardFeature::Draw);
        features.insert(CardFeature::Banish);

        let (bits1, bits2) = features_to_bits(&features);

        // Get the actual bit shifts for the features
        let (draw_shift1, draw_shift2) = CardFeature::Draw.to_bit_shifts();
        let (banish_shift1, banish_shift2) = CardFeature::Banish.to_bit_shifts();

        // Check that the bits are set correctly
        assert_eq!(bits1, draw_shift1 | banish_shift1);
        assert_eq!(bits2, draw_shift2 | banish_shift2);
    }

}
