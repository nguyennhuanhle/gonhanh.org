//! Auto-correct Engine
//!
//! Provides automatic spelling correction for Vietnamese and English text.
//! Triggered on word boundaries (space, punctuation).
//!
//! ## Features
//! - Vietnamese typo correction (n/l, i/y, tone errors)
//! - English typo correction (programming-focused)
//! - Case-preserving corrections
//! - Toggle on/off via settings
//!
//! ## Design
//! - O(1) lookup using HashMap
//! - Lazy-loaded correction maps
//! - Memory-efficient: ~50KB for full database

use crate::data::corrections::{
    build_all_corrections, build_english_corrections, build_vietnamese_corrections,
    ENGLISH_CORRECTIONS, VIETNAMESE_CORRECTIONS,
};
use std::collections::HashMap;

/// Auto-correct mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AutoCorrectMode {
    /// Disabled (default)
    #[default]
    Off,
    /// Vietnamese only
    Vietnamese,
    /// English only
    English,
    /// Both Vietnamese and English
    All,
}

impl AutoCorrectMode {
    /// Create from u8 value (for FFI)
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Off,
            1 => Self::Vietnamese,
            2 => Self::English,
            3 => Self::All,
            _ => Self::Off,
        }
    }

    /// Convert to u8 value (for FFI)
    pub fn to_u8(self) -> u8 {
        match self {
            Self::Off => 0,
            Self::Vietnamese => 1,
            Self::English => 2,
            Self::All => 3,
        }
    }

    /// Check if auto-correct is enabled
    pub fn is_enabled(self) -> bool {
        self != Self::Off
    }
}

/// Auto-correct result
#[derive(Debug, Clone)]
pub struct AutoCorrectResult {
    /// Original (wrong) word
    pub original: String,
    /// Corrected word
    pub corrected: String,
    /// Number of characters to backspace
    pub backspace_count: usize,
}

/// Auto-correct engine
pub struct AutoCorrect {
    /// Current mode
    mode: AutoCorrectMode,
    /// Vietnamese corrections map (lazy-loaded)
    vi_map: Option<HashMap<&'static str, &'static str>>,
    /// English corrections map (lazy-loaded)
    en_map: Option<HashMap<&'static str, &'static str>>,
    /// Combined corrections map (lazy-loaded)
    all_map: Option<HashMap<&'static str, &'static str>>,
}

impl Default for AutoCorrect {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoCorrect {
    /// Create a new auto-correct engine (disabled by default)
    pub fn new() -> Self {
        Self {
            mode: AutoCorrectMode::Off,
            vi_map: None,
            en_map: None,
            all_map: None,
        }
    }

    /// Set the auto-correct mode
    pub fn set_mode(&mut self, mode: AutoCorrectMode) {
        self.mode = mode;
        // Lazy-load the appropriate map when mode changes
        self.ensure_maps_loaded();
    }

    /// Get current mode
    pub fn mode(&self) -> AutoCorrectMode {
        self.mode
    }

    /// Check if auto-correct is enabled
    pub fn is_enabled(&self) -> bool {
        self.mode.is_enabled()
    }

    /// Ensure the required correction maps are loaded
    fn ensure_maps_loaded(&mut self) {
        match self.mode {
            AutoCorrectMode::Off => {
                // Don't need to load anything
            }
            AutoCorrectMode::Vietnamese => {
                if self.vi_map.is_none() {
                    self.vi_map = Some(build_vietnamese_corrections());
                }
            }
            AutoCorrectMode::English => {
                if self.en_map.is_none() {
                    self.en_map = Some(build_english_corrections());
                }
            }
            AutoCorrectMode::All => {
                if self.all_map.is_none() {
                    self.all_map = Some(build_all_corrections());
                }
            }
        }
    }

    /// Try to correct a word
    ///
    /// Returns Some(AutoCorrectResult) if correction found, None otherwise.
    ///
    /// # Arguments
    /// * `word` - The word to check for corrections
    pub fn try_correct(&self, word: &str) -> Option<AutoCorrectResult> {
        if !self.is_enabled() || word.is_empty() {
            return None;
        }

        // Normalize to lowercase for lookup
        let word_lower = word.to_lowercase();

        // Lookup in appropriate map
        let correction = match self.mode {
            AutoCorrectMode::Off => None,
            AutoCorrectMode::Vietnamese => {
                self.vi_map.as_ref()?.get(word_lower.as_str()).copied()
            }
            AutoCorrectMode::English => {
                self.en_map.as_ref()?.get(word_lower.as_str()).copied()
            }
            AutoCorrectMode::All => {
                self.all_map.as_ref()?.get(word_lower.as_str()).copied()
            }
        };

        correction.map(|corrected| {
            // Preserve original case
            let corrected_with_case = apply_case(word, corrected);
            AutoCorrectResult {
                original: word.to_string(),
                corrected: corrected_with_case,
                backspace_count: word.chars().count(),
            }
        })
    }

    /// Get total number of corrections available
    pub fn corrections_count(&self) -> usize {
        match self.mode {
            AutoCorrectMode::Off => 0,
            AutoCorrectMode::Vietnamese => VIETNAMESE_CORRECTIONS.len(),
            AutoCorrectMode::English => ENGLISH_CORRECTIONS.len(),
            AutoCorrectMode::All => VIETNAMESE_CORRECTIONS.len() + ENGLISH_CORRECTIONS.len(),
        }
    }
}

/// Apply the case pattern from original word to corrected word
///
/// Handles:
/// - All uppercase: "TEH" -> "THE"
/// - First letter uppercase: "Teh" -> "The"
/// - All lowercase: "teh" -> "the"
fn apply_case(original: &str, corrected: &str) -> String {
    if original.is_empty() || corrected.is_empty() {
        return corrected.to_string();
    }

    let original_chars: Vec<char> = original.chars().collect();

    // Check if all uppercase
    if original_chars.iter().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
        return corrected.to_uppercase();
    }

    // Check if first letter is uppercase
    if original_chars.first().map(|c| c.is_uppercase()).unwrap_or(false) {
        let mut chars = corrected.chars();
        match chars.next() {
            Some(first) => {
                let mut result: String = first.to_uppercase().collect();
                result.extend(chars);
                return result;
            }
            None => return corrected.to_string(),
        }
    }

    // Default: lowercase
    corrected.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_correct_disabled_by_default() {
        let ac = AutoCorrect::new();
        assert!(!ac.is_enabled());
        assert_eq!(ac.mode(), AutoCorrectMode::Off);
    }

    #[test]
    fn test_auto_correct_mode_conversion() {
        assert_eq!(AutoCorrectMode::from_u8(0), AutoCorrectMode::Off);
        assert_eq!(AutoCorrectMode::from_u8(1), AutoCorrectMode::Vietnamese);
        assert_eq!(AutoCorrectMode::from_u8(2), AutoCorrectMode::English);
        assert_eq!(AutoCorrectMode::from_u8(3), AutoCorrectMode::All);
        assert_eq!(AutoCorrectMode::from_u8(255), AutoCorrectMode::Off); // Invalid

        assert_eq!(AutoCorrectMode::Off.to_u8(), 0);
        assert_eq!(AutoCorrectMode::Vietnamese.to_u8(), 1);
        assert_eq!(AutoCorrectMode::English.to_u8(), 2);
        assert_eq!(AutoCorrectMode::All.to_u8(), 3);
    }

    #[test]
    fn test_try_correct_disabled() {
        let ac = AutoCorrect::new();
        assert!(ac.try_correct("teh").is_none());
        assert!(ac.try_correct("nà").is_none());
    }

    #[test]
    fn test_try_correct_vietnamese() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::Vietnamese);

        // Should correct Vietnamese typos
        let result = ac.try_correct("nà");
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.original, "nà");
        assert_eq!(r.corrected, "là");
        assert_eq!(r.backspace_count, 2);

        // Should correct i/y
        let result = ac.try_correct("lí");
        assert!(result.is_some());
        assert_eq!(result.unwrap().corrected, "lý");

        // Should NOT correct English
        let result = ac.try_correct("teh");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_correct_abbreviations() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::Vietnamese);

        // Should correct Vietnamese abbreviations
        let result = ac.try_correct("ko");
        assert!(result.is_some(), "ko should be corrected to không");
        let r = result.unwrap();
        assert_eq!(r.original, "ko");
        assert_eq!(r.corrected, "không");
        assert_eq!(r.backspace_count, 2);

        // Test another common abbreviation
        let result = ac.try_correct("dc");
        assert!(result.is_some(), "dc should be corrected to được");
        assert_eq!(result.unwrap().corrected, "được");
    }

    #[test]
    fn test_try_correct_english() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::English);

        // Should correct English typos
        let result = ac.try_correct("teh");
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.original, "teh");
        assert_eq!(r.corrected, "the");
        assert_eq!(r.backspace_count, 3);

        // Should correct programming typos
        let result = ac.try_correct("fucntion");
        assert!(result.is_some());
        assert_eq!(result.unwrap().corrected, "function");

        // Should NOT correct Vietnamese
        let result = ac.try_correct("nà");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_correct_all() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::All);

        // Should correct both Vietnamese and English
        let result = ac.try_correct("nà");
        assert!(result.is_some());
        assert_eq!(result.unwrap().corrected, "là");

        let result = ac.try_correct("teh");
        assert!(result.is_some());
        assert_eq!(result.unwrap().corrected, "the");
    }

    #[test]
    fn test_case_preservation_uppercase() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::English);

        // All uppercase
        let result = ac.try_correct("TEH");
        assert!(result.is_some());
        assert_eq!(result.unwrap().corrected, "THE");
    }

    #[test]
    fn test_case_preservation_titlecase() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::English);

        // First letter uppercase
        let result = ac.try_correct("Teh");
        assert!(result.is_some());
        assert_eq!(result.unwrap().corrected, "The");
    }

    #[test]
    fn test_case_preservation_lowercase() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::English);

        // All lowercase
        let result = ac.try_correct("teh");
        assert!(result.is_some());
        assert_eq!(result.unwrap().corrected, "the");
    }

    #[test]
    fn test_apply_case_function() {
        // All uppercase
        assert_eq!(apply_case("TEH", "the"), "THE");

        // First letter uppercase
        assert_eq!(apply_case("Teh", "the"), "The");

        // All lowercase
        assert_eq!(apply_case("teh", "the"), "the");

        // Mixed case starting with uppercase (treats as title case)
        assert_eq!(apply_case("TeH", "the"), "The");

        // Empty strings
        assert_eq!(apply_case("", "the"), "the");
        assert_eq!(apply_case("TEH", ""), "");
    }

    #[test]
    fn test_corrections_count() {
        let mut ac = AutoCorrect::new();

        assert_eq!(ac.corrections_count(), 0);

        ac.set_mode(AutoCorrectMode::Vietnamese);
        assert!(ac.corrections_count() >= 50);

        ac.set_mode(AutoCorrectMode::English);
        assert!(ac.corrections_count() >= 100);

        ac.set_mode(AutoCorrectMode::All);
        assert!(ac.corrections_count() >= 150);
    }

    #[test]
    fn test_empty_word() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::All);

        let result = ac.try_correct("");
        assert!(result.is_none());
    }

    #[test]
    fn test_unknown_word() {
        let mut ac = AutoCorrect::new();
        ac.set_mode(AutoCorrectMode::All);

        let result = ac.try_correct("asdfghjkl");
        assert!(result.is_none());
    }
}
