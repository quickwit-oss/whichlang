pub use crate::weights::{Lang, LANGUAGES};

#[allow(clippy::all)]
mod weights;

const NUM_LANGUAGES: usize = LANGUAGES.len();

#[doc(hidden)]
pub const DIMENSION: usize = 1 << 12;
const BIGRAM_MASK: u32 = (1 << 16) - 1;
const TRIGRAM_MASK: u32 = (1 << 24) - 1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum Feature {
    AsciiNGram(u32),
    Unicode(char),
    UnicodeClass(char),
}

const SEED: u32 = 3_242_157_231u32;

#[inline(always)]
fn murmurhash2(mut k: u32, seed: u32) -> u32 {
    const M: u32 = 0x5bd1_e995;
    let mut h: u32 = seed;
    k = k.wrapping_mul(M);
    k ^= k >> 24;
    k = k.wrapping_mul(M);
    h = h.wrapping_mul(M);
    h ^= k;
    h ^= h >> 13;
    h = h.wrapping_mul(M);
    h ^ (h >> 15)
}

impl Feature {
    pub fn to_hash(&self) -> u32 {
        match *self {
            Feature::AsciiNGram(ngram) => murmurhash2(ngram, SEED),
            Feature::Unicode(chr) => murmurhash2((chr as u32) >> 7, SEED ^ 2),
            Feature::UnicodeClass(chr) => murmurhash2(classify_codepoint(chr), SEED ^ 4),
        }
    }
}

pub fn detect_language(text: &str) -> Lang {
    let mut scores: [f32; NUM_LANGUAGES] = Default::default();
    let mut num_features: u32 = 0;
    emit_tokens(text, |token| {
        num_features += 1;
        let bucket = token.to_hash() % DIMENSION as u32;
        let idx = bucket as usize * NUM_LANGUAGES;
        scores
            .iter_mut()
            .zip(&weights::WEIGHTS[idx..idx + NUM_LANGUAGES])
            .for_each(|(score, weight)| *score += weight);
    });
    if num_features == 0 {
        // By default, we return English
        return Lang::Eng;
    }
    let sqrt_inv = 1.0 / (num_features as f32).sqrt();
    scores
        .iter_mut()
        .zip(weights::INTERCEPTS.iter())
        .for_each(|(score, intercept)| *score = *score * sqrt_inv + intercept);
    let mut lang_id = 0;
    let mut max_score = scores[0];
    for (i, &score) in scores.iter().enumerate().skip(1) {
        if score > max_score {
            max_score = score;
            lang_id = i;
        }
    }
    weights::LANGUAGES[lang_id]
}

#[doc(hidden)]
pub fn emit_tokens(text: &str, mut listener: impl FnMut(Feature)) {
    let mut prev = ' ' as u32;
    let mut num_prev_ascii = 1;
    for chr in text.chars() {
        if !chr.is_ascii() {
            listener(Feature::Unicode(chr));
            listener(Feature::UnicodeClass(chr));
            num_prev_ascii = 0;
        } else {
            let code = chr.to_ascii_lowercase() as u32;
            prev = prev << 8 | code;
            match num_prev_ascii {
                0 => {
                    num_prev_ascii = 1;
                }
                1 => {
                    listener(Feature::AsciiNGram(prev & BIGRAM_MASK));
                    num_prev_ascii = 2;
                }
                2 => {
                    listener(Feature::AsciiNGram(prev & BIGRAM_MASK));
                    listener(Feature::AsciiNGram(prev & TRIGRAM_MASK));
                    num_prev_ascii = 3;
                }
                3 => {
                    listener(Feature::AsciiNGram(prev & BIGRAM_MASK));
                    listener(Feature::AsciiNGram(prev & TRIGRAM_MASK));
                    listener(Feature::AsciiNGram(prev));
                }
                _ => unreachable!(),
            }
            if !chr.is_alphanumeric() {
                prev = ' ' as u32;
            }
        }
    }
}

const JP_PUNCT_START: u32 = 0x3000;
const JP_PUNCT_END: u32 = 0x303f;
const JP_HIRAGANA_START: u32 = 0x3040;
const JP_HIRAGANA_END: u32 = 0x309f;
const JP_KATAKANA_START: u32 = 0x30a0;
const JP_KATAKANA_END: u32 = 0x30ff;
const CJK_KANJI_START: u32 = 0x4e00;
const CJK_KANJI_END: u32 = 0x9faf;
const JP_HALFWIDTH_KATAKANA_START: u32 = 0xff61;
const JP_HALFWIDTH_KATAKANA_END: u32 = 0xff90;

fn classify_codepoint(chr: char) -> u32 {
    [
        160,
        161,
        171,
        172,
        173,
        174,
        187,
        192,
        196,
        199,
        200,
        201,
        202,
        205,
        214,
        220,
        223,
        224,
        225,
        226,
        227,
        228,
        231,
        232,
        233,
        234,
        235,
        236,
        237,
        238,
        239,
        242,
        243,
        244,
        245,
        246,
        249,
        250,
        251,
        252,
        333,
        339,
        JP_PUNCT_START,
        JP_PUNCT_END,
        JP_HIRAGANA_START,
        JP_HIRAGANA_END,
        JP_KATAKANA_START,
        JP_KATAKANA_END,
        CJK_KANJI_START,
        CJK_KANJI_END,
        JP_HALFWIDTH_KATAKANA_START,
        JP_HALFWIDTH_KATAKANA_END,
    ]
    .binary_search(&(chr as u32))
    .unwrap_or_else(|pos| pos) as u32
}

#[cfg(test)]
mod tests {
    use crate::{detect_language, emit_tokens, Feature, Lang};

    fn ascii_ngram_feature(text: &str) -> Feature {
        let mut bytes = [0; 4];
        bytes[4 - text.len()..].copy_from_slice(text.as_bytes());
        Feature::AsciiNGram(u32::from_be_bytes(bytes))
    }

    #[test]
    fn test_emit_tokens() {
        let mut tokens = Vec::new();
        emit_tokens("hello　こん！", |token| tokens.push(token));
        assert_eq!(
            tokens,
            vec![
                ascii_ngram_feature(" h"),
                ascii_ngram_feature("he"),
                ascii_ngram_feature(" he"),
                ascii_ngram_feature("el"),
                ascii_ngram_feature("hel"),
                ascii_ngram_feature(" hel"),
                ascii_ngram_feature("ll"),
                ascii_ngram_feature("ell"),
                ascii_ngram_feature("hell"),
                ascii_ngram_feature("lo"),
                ascii_ngram_feature("llo"),
                ascii_ngram_feature("ello"),
                Feature::Unicode('　'),
                Feature::UnicodeClass('　'),
                Feature::Unicode('こ'),
                Feature::UnicodeClass('こ'),
                Feature::Unicode('ん'),
                Feature::UnicodeClass('ん'),
                Feature::Unicode('！'),
                Feature::UnicodeClass('！'),
            ]
        );
    }

    #[test]
    fn test_empty_str() {
        assert_eq!(detect_language(""), Lang::Eng);
    }

    #[test]
    fn test_detect_language() {
        // English
        assert_eq!(detect_language("Hello, happy tax payer"), Lang::Eng);
        // French
        assert_eq!(detect_language("Bonjour joyeux contribuable"), Lang::Fra);
        // German
        assert_eq!(detect_language("Hallo glücklicher Steuerzahler"), Lang::Deu);
        // Japanese
        assert_eq!(detect_language("こんにちは幸せな税金納め"), Lang::Jpn);
        // Mandarin chinese
        assert_eq!(detect_language("你好幸福的纳税人"), Lang::Cmn);
        // Turkish
        assert_eq!(detect_language("Merhaba, mutlu vergi mükellefi"), Lang::Tur);
        // Dutch
        assert_eq!(detect_language("Hallo, blije belastingbetaler"), Lang::Nld);
        // Korean
        assert_eq!(detect_language("안녕하세요 행복한 납세자입니다"), Lang::Kor);
        // Italian
        assert_eq!(detect_language("Ciao, felice contribuente!"), Lang::Ita);
        // Spanish
        assert_eq!(detect_language("Hola feliz contribuyente"), Lang::Spa);
        assert_eq!(detect_language("¡Hola!"), Lang::Spa);
        // Portuguese
        assert_eq!(detect_language("Olá feliz contribuinte"), Lang::Por);
    }
}
