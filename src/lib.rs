pub use crate::weights::{Lang, LANGUAGES};

mod weights;

const NUM_LANGUAGES: usize = LANGUAGES.len();

#[doc(hidden)]
pub const DIMENSION: usize = 1 << 11;
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
    #[inline(always)]
    pub fn to_hash(&self) -> u32 {
        match self {
            Feature::AsciiNGram(ngram) => murmurhash2(*ngram, SEED),
            Feature::Unicode(chr) => murmurhash2(*chr as u32 / 128, SEED ^ 2),
            Feature::UnicodeClass(chr) => murmurhash2(classify_codepoint(*chr), SEED ^ 4),
        }
    }
}

pub fn detect_language(text: &str) -> Lang {
    let mut scores: [f32; NUM_LANGUAGES] = weights::INTERCEPTS.clone();
    emit_tokens(
        text,
        #[inline(always)]
        |token| {
            let bucket = token.to_hash() % DIMENSION as u32;
            let idx = bucket as usize * NUM_LANGUAGES;
            let per_language_scores = &weights::WEIGHTS[idx..idx + NUM_LANGUAGES];
            for i in 0..NUM_LANGUAGES {
                scores[i] += per_language_scores[i];
            }
        },
    );
    let lang_id = scores
        .iter()
        .enumerate()
        .max_by(|(_, &score_left), (_, &score_right)| score_left.partial_cmp(&score_right).unwrap())
        .map(|(pos, _val)| pos)
        .unwrap();
    weights::LANGUAGES[lang_id]
}

#[doc(hidden)]
pub fn emit_tokens(text: &str, mut listener: impl FnMut(Feature)) {
    let mut prev = 0u32;
    let mut num_previous_ascii_chr = 0;
    for (_pos, chr) in text.char_indices() {
        let code = chr as u32;
        if !chr.is_ascii() {
            listener(Feature::Unicode(chr));
            listener(Feature::UnicodeClass(chr));
            num_previous_ascii_chr = 0;
            continue;
        }
        prev = prev << 8 | code;
        match num_previous_ascii_chr {
            0 => {
                num_previous_ascii_chr = 1;
            }
            1 => {
                listener(Feature::AsciiNGram(prev & BIGRAM_MASK));
                num_previous_ascii_chr = 2;
            }
            2 => {
                listener(Feature::AsciiNGram(prev & BIGRAM_MASK));
                listener(Feature::AsciiNGram(prev & TRIGRAM_MASK));
            }
            3 => {
                listener(Feature::AsciiNGram(prev & BIGRAM_MASK));
                listener(Feature::AsciiNGram(prev & TRIGRAM_MASK));
            }
            _ => {
                unreachable!();
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
    use crate::detect_language;
    use crate::emit_tokens;
    use crate::Feature;
    use crate::Lang;

    fn ascii_ngram_feature(text: &str) -> Feature {
        assert!(text.is_ascii());
        let bytes = text.as_bytes();
        match text.len() {
            2 => {
                let b0 = bytes[0] as u32;
                let b1 = bytes[1] as u32;
                Feature::AsciiNGram(b0 << 8 | b1)
            }
            3 => {
                let b0 = bytes[0] as u32;
                let b1 = bytes[1] as u32;
                let b2 = bytes[2] as u32;
                Feature::AsciiNGram(b0 << 16 | b1 << 8 | b2)
            }
            _ => {
                panic!("only supports, 1, 2, or 3 character strings");
            }
        }
    }

    #[test]
    fn test_emit_tokens() {
        let mut tokens = Vec::new();
        emit_tokens("hello　こん！", |token| tokens.push(token));
        assert_eq!(
            &tokens,
            &[
                ascii_ngram_feature("he"),
                ascii_ngram_feature("el"),
                ascii_ngram_feature("hel"),
                ascii_ngram_feature("ll"),
                ascii_ngram_feature("ell"),
                ascii_ngram_feature("lo"),
                ascii_ngram_feature("llo"),
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
        // assert_eq!(detect_language("Hola feliz contribuyente"), Lang::Spa);
        assert_eq!(
            detect_language("Hola feliz pagador de impuestos!"),
            Lang::Spa
        );
        // Portuguese
        assert_eq!(detect_language("Olá feliz contribuinte"), Lang::Por);
    }
}
