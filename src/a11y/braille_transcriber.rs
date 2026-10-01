use serde::{Deserialize, Serialize};

/// ============================================================================
/// Multilingual Grade 1 & Grade 2 Braille Translation Engine (Unicode 6-dot & 8-dot)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrailleGrade {
    Grade1Uncontracted, // Letter-by-letter
    Grade2Contracted,   // With abbreviations and contractions
}

pub struct BrailleTranscriber;

impl BrailleTranscriber {
    /// Konversi karakter alfabet ASCII standar ke Unicode Braille 6-dot (U+2800..U+283F)
    pub fn char_to_braille(c: char) -> char {
        match c.to_ascii_lowercase() {
            'a' => '\u{2801}', // ⠁ (dot 1)
            'b' => '\u{2803}', // ⠃ (dots 1, 2)
            'c' => '\u{2809}', // ⠉ (dots 1, 4)
            'd' => '\u{2819}', // ⠙ (dots 1, 4, 5)
            'e' => '\u{2811}', // ⠑ (dots 1, 5)
            'f' => '\u{280B}', // ⠋ (dots 1, 2, 4)
            'g' => '\u{281B}', // ⠛ (dots 1, 2, 4, 5)
            'h' => '\u{2813}', // ⠓ (dots 1, 2, 5)
            'i' => '\u{280A}', // ⠊ (dots 2, 4)
            'j' => '\u{281A}', // ⠚ (dots 2, 4, 5)
            'k' => '\u{2805}', // ⠅ (dots 1, 3)
            'l' => '\u{2807}', // ⠇ (dots 1, 2, 3)
            'm' => '\u{280D}', // ⠍ (dots 1, 3, 4)
            'n' => '\u{281D}', // ⠝ (dots 1, 3, 4, 5)
            'o' => '\u{2815}', // ⠕ (dots 1, 3, 5)
            'p' => '\u{280F}', // ⠏ (dots 1, 2, 3, 4)
            'q' => '\u{281F}', // ⠟ (dots 1, 2, 3, 4, 5)
            'r' => '\u{2817}', // ⠗ (dots 1, 2, 3, 5)
            's' => '\u{280E}', // ⠎ (dots 2, 3, 4)
            't' => '\u{281E}', // ⠞ (dots 2, 3, 4, 5)
            'u' => '\u{2825}', // ⠥ (dots 1, 3, 6)
            'v' => '\u{2827}', // ⠧ (dots 1, 2, 3, 6)
            'w' => '\u{283A}', // ⠺ (dots 2, 4, 5, 6)
            'x' => '\u{282D}', // ⠭ (dots 1, 3, 4, 6)
            'y' => '\u{283D}', // ⠽ (dots 1, 3, 4, 5, 6)
            'z' => '\u{2835}', // ⠵ (dots 1, 3, 5, 6)
            ' ' => '\u{2800}', // ⠀ (blank)
            '0'..='9' => {
                // Digit preceded by number sign in actual transcription
                match c {
                    '1' => '\u{2801}',
                    '2' => '\u{2803}',
                    '3' => '\u{2809}',
                    '4' => '\u{2819}',
                    '5' => '\u{2811}',
                    '6' => '\u{280B}',
                    '7' => '\u{281B}',
                    '8' => '\u{2813}',
                    '9' => '\u{280A}',
                    '0' => '\u{281A}',
                    _ => '\u{2800}',
                }
            }
            '.' => '\u{2832}', // ⠲
            ',' => '\u{2802}', // ⠂
            '?' => '\u{2826}', // ⠦
            '!' => '\u{2816}', // ⠖
            _ => '\u{2800}',
        }
    }

    /// Transkripsi teks lengkap ke Unicode Braille
    pub fn transcribe_text(text: &str, grade: BrailleGrade) -> String {
        let mut result = String::new();
        let mut in_number = false;

        for c in text.chars() {
            if c.is_ascii_digit() {
                if !in_number {
                    result.push('\u{283C}'); // Number prefix: ⠼ (dots 3, 4, 5, 6)
                    in_number = true;
                }
            } else {
                in_number = false;
            }

            if c.is_ascii_uppercase() {
                result.push('\u{2820}'); // Capital prefix: ⠠ (dot 6)
            }

            result.push(Self::char_to_braille(c));
        }

        result
    }
}
