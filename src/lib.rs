//! Core generation logic — pure and testable with a seeded RNG.

use rand::seq::SliceRandom;
use rand::Rng;

/// Compact word list. Entropy per word = log2(WORDS.len()).
pub const WORDS: &[&str] = &[
    "able", "acid", "aged", "also", "area", "army", "away", "baby", "back", "ball", "band", "bank",
    "base", "bath", "bear", "beat", "been", "beer", "bell", "belt", "bend", "bird", "blue", "boat",
    "body", "bone", "book", "boot", "born", "boss", "both", "bowl", "bulk", "burn", "bush", "busy",
    "cake", "call", "calm", "came", "camp", "card", "care", "case", "cash", "cast", "cell", "chip",
    "city", "clay", "clip", "club", "coal", "coat", "code", "cold", "come", "cook", "cool", "cope",
    "copy", "core", "corn", "cost", "crew", "crop", "dark", "data", "date", "dawn", "days", "dead",
    "deal", "dear", "debt", "deep", "deny", "desk", "dial", "diet", "dirt", "dish", "does", "done",
    "door", "dose", "down", "draw", "drop", "drug", "dual", "duke", "dust", "duty", "each", "earn",
    "ease", "east", "easy", "edge", "else", "even", "ever", "evil", "exit", "face", "fact", "fade",
    "fail", "fair", "fall", "farm", "fast", "fate", "fear", "feed", "feel", "feet", "fell", "file",
    "fill", "film", "find", "fine", "fire", "firm", "fish", "five", "flag", "flat", "flow", "food",
    "foot", "ford", "form", "fort", "four", "free", "frog", "fuel", "full", "fund", "gain", "game",
    "gate", "gave", "gear", "gift", "girl", "give", "glad", "goal", "goat", "gold", "gone", "good",
    "gray", "grew", "grid", "grip", "grow", "gulf", "hair", "half", "hall", "hand", "hang", "hard",
    "harm", "hate", "have", "head", "heal", "heap", "hear", "heat", "held", "hell", "helm", "help",
    "herb", "hero", "hide", "high", "hill", "hint", "hire", "hold", "hole", "holy", "home", "hood",
    "hope", "horn", "host", "hour", "huge", "hunt", "hurt", "icon", "idea", "inch", "iron", "item",
    "jade", "jazz", "join", "joke", "jump", "jury", "keen", "keep", "kept", "kick", "kind", "king",
    "kiss", "knee", "knot", "know", "lack", "lake", "lamp", "land", "lane", "last", "late", "lawn",
    "lazy", "lead", "leaf", "lean", "left", "lend", "lens", "less", "life", "lift", "like", "lime",
    "line", "link", "lion", "list", "live", "load", "loan", "lock", "logo", "lone", "long", "look",
    "loop", "lord", "lose", "loss", "lost",
];

pub const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
pub const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const DIGITS: &str = "0123456789";
pub const SYMBOLS: &str = "!@#$%^&*-_=+?";

/// Shannon entropy in bits for `count` independent picks from an `alphabet` of size N.
pub fn entropy_bits(alphabet: usize, count: usize) -> f64 {
    if alphabet <= 1 {
        return 0.0;
    }
    (alphabet as f64).log2() * count as f64
}

/// A diceware-style passphrase: `words` random words joined by `sep`.
/// If `capitalize`, each word's first letter is upper-cased.
pub fn passphrase<R: Rng + ?Sized>(rng: &mut R, words: usize, sep: &str, capitalize: bool) -> String {
    (0..words)
        .map(|_| {
            let w = *WORDS.choose(rng).expect("non-empty word list");
            if capitalize {
                let mut c = w.chars();
                match c.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(sep)
}

/// Build the character pool from the requested classes (always includes lowercase).
pub fn charset(upper: bool, digits: bool, symbols: bool) -> String {
    let mut s = String::from(LOWER);
    if upper {
        s.push_str(UPPER);
    }
    if digits {
        s.push_str(DIGITS);
    }
    if symbols {
        s.push_str(SYMBOLS);
    }
    s
}

/// A random password of `len` characters drawn from `pool`.
pub fn password<R: Rng + ?Sized>(rng: &mut R, len: usize, pool: &str) -> String {
    let chars: Vec<char> = pool.chars().collect();
    (0..len).map(|_| *chars.choose(rng).expect("non-empty pool")).collect()
}

/// Human label for an entropy value.
pub fn strength(bits: f64) -> &'static str {
    match bits as u32 {
        0..=39 => "weak",
        40..=59 => "reasonable",
        60..=79 => "strong",
        _ => "very strong",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn entropy_matches_formula() {
        assert!((entropy_bits(2, 8) - 8.0).abs() < 1e-9);
        assert!((entropy_bits(94, 12) - 12.0 * (94f64).log2()).abs() < 1e-9);
        assert_eq!(entropy_bits(1, 10), 0.0);
    }

    #[test]
    fn passphrase_word_count_and_determinism() {
        let mut a = StdRng::seed_from_u64(42);
        let mut b = StdRng::seed_from_u64(42);
        let pa = passphrase(&mut a, 5, "-", false);
        let pb = passphrase(&mut b, 5, "-", false);
        assert_eq!(pa, pb); // same seed → same output
        assert_eq!(pa.split('-').count(), 5);
        assert!(pa.chars().all(|c| c.is_ascii_lowercase() || c == '-'));
    }

    #[test]
    fn passphrase_capitalizes() {
        let mut r = StdRng::seed_from_u64(7);
        let p = passphrase(&mut r, 3, " ", true);
        for w in p.split(' ') {
            assert!(w.chars().next().unwrap().is_ascii_uppercase());
        }
    }

    #[test]
    fn charset_composition() {
        assert_eq!(charset(false, false, false), LOWER);
        assert_eq!(charset(true, true, true).len(), LOWER.len() + UPPER.len() + DIGITS.len() + SYMBOLS.len());
    }

    #[test]
    fn password_length_and_pool() {
        let mut r = StdRng::seed_from_u64(1);
        let pool = charset(true, true, false);
        let pw = password(&mut r, 20, &pool);
        assert_eq!(pw.chars().count(), 20);
        assert!(pw.chars().all(|c| pool.contains(c)));
    }

    #[test]
    fn strength_bands() {
        assert_eq!(strength(20.0), "weak");
        assert_eq!(strength(50.0), "reasonable");
        assert_eq!(strength(70.0), "strong");
        assert_eq!(strength(128.0), "very strong");
    }
}
