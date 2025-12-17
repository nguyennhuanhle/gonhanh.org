//! Comprehensive test suite for English word auto-restore feature.
//!
//! # How Auto-Restore Works
//!
//! When typing English words using Telex input method, certain letters act as
//! Vietnamese modifiers (s, f, r, x, j for tones; w for horn mark). This causes
//! English words to be incorrectly transformed. The auto-restore feature detects
//! invalid Vietnamese patterns and restores the original English text.
//!
//! # Detection Patterns
//!
//! The engine can detect English words using these patterns:
//!
//! 1. **Modifier + Consonant**: "text" (x followed by t), "expect" (x followed by p)
//! 2. **EI vowel pair + modifier**: "their" (ei+r)
//! 3. **AI vowel pair + P initial**: "pair" (P initial + ai + r)
//! 4. **Vowel + modifier + vowel (no initial)**: "use" (u+s+e)
//! 5. **W at start + consonant or later W**: "window", "wow"
//! 6. **Invalid Vietnamese initial (F)**: "fair", "fix"
//!
//! # Limitations
//!
//! Some English words produce structurally valid Vietnamese and CANNOT be
//! auto-detected without a dictionary:
//! - "mix" → "mĩ" (M is valid initial, ĩ is valid)
//! - "box" → "bõ" (B is valid initial, õ is valid)
//!
//! Users should use raw mode (\word) or Esc to restore these manually.

mod common;
use common::telex;

// =============================================================================
// PATTERN 1: MODIFIER FOLLOWED BY CONSONANT
// Example: "text" has x followed by t → clearly English
// =============================================================================

#[test]
fn pattern1_modifier_then_consonant() {
    telex(&[
        // x + consonant
        ("text ", "text "),
        ("next ", "next "),
        ("context ", "context "),
        ("textbook ", "textbook "),
        ("extend ", "extend "),
        ("extent ", "extent "),
        ("extern ", "extern "),
        ("extra ", "extra "),
        ("extract ", "extract "),
        ("extreme ", "extreme "),
        // exp- pattern (x + p)
        ("expect ", "expect "),
        ("export ", "export "),
        ("express ", "express "),
        ("expand ", "expand "),
        ("expense ", "expense "),
        ("expert ", "expert "),
        ("explore ", "explore "),
        ("exploit ", "exploit "),
        ("explode ", "explode "),
        ("explain ", "explain "),
        ("explicit ", "explicit "),
        ("experiment ", "experiment "),
        ("experience ", "experience "),
        // exc- pattern (x + c)
        ("excel ", "excel "),
        ("except ", "except "),
        ("excess ", "excess "),
        ("exchange ", "exchange "),
        ("excite ", "excite "),
        ("exclude ", "exclude "),
        ("excuse ", "excuse "),
        ("execute ", "execute "),
        // s + consonant (Pattern 1 with s)
        ("test ", "test "),
        ("rest ", "rest "),
        ("best ", "best "),
        ("nest ", "nest "),
        ("west ", "west "),
        ("most ", "most "),
        ("post ", "post "),
        ("cost ", "cost "),
        ("lost ", "lost "),
        ("host ", "host "),
        ("fast ", "fast "),
        ("last ", "last "),
        ("past ", "past "),
        ("vast ", "vast "),
        ("cast ", "cast "),
        ("just ", "just "),
        ("must ", "must "),
        ("dust ", "dust "),
        ("rust ", "rust "),
        ("list ", "list "),
        ("mist ", "mist "),
        ("disk ", "disk "),
        ("risk ", "risk "),
        ("task ", "task "),
        ("mask ", "mask "),
        ("desk ", "desk "),
    ]);
}

// =============================================================================
// PATTERN 2: EI VOWEL PAIR + MODIFIER AT END
// Example: "their" has ei before r → English pattern
// =============================================================================

#[test]
fn pattern2_ei_vowel_pair() {
    telex(&[("their ", "their "), ("weird ", "weird ")]);
}

// =============================================================================
// PATTERN 3: AI VOWEL PAIR + RARE INITIAL (P)
// P alone (not PH) is rare in native Vietnamese
// =============================================================================

#[test]
fn pattern3_ai_with_p_initial() {
    telex(&[("pair ", "pair ")]);
}

// =============================================================================
// PATTERN 4: VOWEL + MODIFIER + VOWEL (NO INITIAL CONSONANT)
// Example: "use" starts with vowel, has s between u and e
// =============================================================================

#[test]
fn pattern4_vowel_modifier_vowel() {
    telex(&[("use ", "use "), ("user ", "user ")]);
}

// =============================================================================
// PATTERN 5: W AT START + CONSONANT / W + VOWEL + W
// W is not a valid Vietnamese initial consonant
// =============================================================================

#[test]
fn pattern5_w_start_consonant() {
    telex(&[
        // w + consonant
        ("water ", "water "),
        ("winter ", "winter "),
        ("window ", "window "),
        ("wonder ", "wonder "),
        ("worker ", "worker "),
        ("world ", "world "),
        ("worth ", "worth "),
        ("write ", "write "),
        ("wrong ", "wrong "),
        ("wrap ", "wrap "),
        ("wrist ", "wrist "),
        // wh- words
        ("what ", "what "),
        ("when ", "when "),
        ("where ", "where "),
        ("which ", "which "),
        ("while ", "while "),
        ("white ", "white "),
        ("whole ", "whole "),
        ("why ", "why "),
        ("wheat ", "wheat "),
        ("wheel ", "wheel "),
    ]);
}

#[test]
fn pattern5_w_vowel_w() {
    telex(&[("wow ", "wow ")]);
}

// =============================================================================
// PATTERN 6: INVALID VIETNAMESE INITIAL (F)
// F is not a Vietnamese initial (Vietnamese uses PH for /f/ sound)
// =============================================================================

#[test]
fn pattern6_invalid_initial_f() {
    telex(&[
        ("fair ", "fair "),
        ("fall ", "fall "),
        ("false ", "false "),
        ("far ", "far "),
        ("farm ", "farm "),
        ("fast ", "fast "),
        ("fat ", "fat "),
        ("fear ", "fear "),
        ("feed ", "feed "),
        ("feel ", "feel "),
        ("few ", "few "),
        ("file ", "file "),
        ("fill ", "fill "),
        ("film ", "film "),
        ("final ", "final "),
        ("find ", "find "),
        ("fine ", "fine "),
        ("fire ", "fire "),
        ("firm ", "firm "),
        ("first ", "first "),
        ("fish ", "fish "),
        ("fit ", "fit "),
        ("fix ", "fix "),
        ("flag ", "flag "),
        ("flat ", "flat "),
        ("flex ", "flex "),
        ("floor ", "floor "),
        ("flow ", "flow "),
        ("fly ", "fly "),
        ("focus ", "focus "),
        ("fold ", "fold "),
        ("follow ", "follow "),
        ("font ", "font "),
        ("food ", "food "),
        ("foot ", "foot "),
        ("for ", "for "),
        ("force ", "force "),
        ("fork ", "fork "),
        ("form ", "form "),
        ("format ", "format "),
        ("forward ", "forward "),
        ("found ", "found "),
        ("four ", "four "),
        ("frame ", "frame "),
        ("free ", "free "),
        ("fresh ", "fresh "),
        ("from ", "from "),
        ("front ", "front "),
        ("full ", "full "),
        ("fun ", "fun "),
        ("function ", "function "),
        ("future ", "future "),
        // Tech terms with F
        ("facebook ", "facebook "),
        ("firebase ", "firebase "),
        ("firefox ", "firefox "),
        ("flutter ", "flutter "),
        ("framework ", "framework "),
        ("frontend ", "frontend "),
        ("fullstack ", "fullstack "),
    ]);
}

// =============================================================================
// TECH & PROGRAMMING TERMS (WITH DETECTABLE PATTERNS)
// =============================================================================

#[test]
fn tech_terms_restore() {
    telex(&[
        // exp- pattern
        ("Express ", "Express "),
        // ext- pattern
        ("extension ", "extension "),
        // F initial
        ("Firebase ", "Firebase "),
        ("Flutter ", "Flutter "),
        // W initial
        ("webpack ", "webpack "),
        ("WebSocket ", "WebSocket "),
        // -est pattern
        ("localhost ", "localhost "),
        ("request ", "request "),
        // -ost pattern
        ("post ", "post "),
        ("host ", "host "),
    ]);
}

// =============================================================================
// PUNCTUATION TRIGGERS RESTORE
// =============================================================================

#[test]
fn punctuation_triggers_restore() {
    // Only certain punctuation triggers auto-restore (comma, period)
    telex(&[("text, ", "text, "), ("expect. ", "expect. ")]);
}

// =============================================================================
// VIETNAMESE WORDS THAT SHOULD NOT RESTORE
// =============================================================================

#[test]
fn vietnamese_single_syllable_preserved() {
    telex(&[
        // Single syllable with tones
        ("mas ", "má "), // má (mother)
        ("maf ", "mà "), // mà (but)
        ("mar ", "mả "), // mả (grave)
        ("max ", "mã "), // mã (horse - Sino-Viet)
        ("maj ", "mạ "), // mạ (rice seedling)
        ("bas ", "bá "), // bá (aunt)
        ("baf ", "bà "), // bà (grandmother)
        ("cas ", "cá "), // cá (fish)
        ("caf ", "cà "), // cà (eggplant)
        ("las ", "lá "), // lá (leaf)
        ("laf ", "là "), // là (is)
        ("tas ", "tá "), // tá (dozen)
        ("taf ", "tà "), // tà (side)
    ]);
}

#[test]
fn vietnamese_multi_syllable_preserved() {
    telex(&[
        ("gox ", "gõ "),       // gõ (to type/knock)
        ("tooi ", "tôi "),     // tôi (I)
        ("Vieetj ", "Việt "),  // Việt
        ("thoaij ", "thoại "), // thoại (speech)
        ("giuwax ", "giữa "),  // giữa (middle)
        ("dduowcj ", "được "), // được (can/get)
        ("muwowjt ", "mượt "), // mượt (smooth)
    ]);
}

#[test]
fn vietnamese_ai_pattern_preserved() {
    // AI pattern with common Vietnamese initials should NOT restore
    telex(&[
        ("mais ", "mái "),     // mái (roof)
        ("cais ", "cái "),     // cái (classifier)
        ("xaif ", "xài "),     // xài (to use - Southern)
        ("taif ", "tài "),     // tài (talent)
        ("gais ", "gái "),     // gái (girl)
        ("hoaij ", "hoại "),   // hoại (decay)
        ("ngoaij ", "ngoại "), // ngoại (outside)
    ]);
}

#[test]
fn vietnamese_complex_words_preserved() {
    telex(&[
        // Words with horn marks (ư, ơ)
        ("nuwowcs ", "nước "),     // nước (water)
        ("dduowngf ", "đường "),   // đường (road)
        ("truwowcs ", "trước "),   // trước (before)
        ("giuwowngf ", "giường "), // giường (bed)
        // Words with circumflex (â, ê, ô)
        ("caaps ", "cấp "), // cấp (level)
        ("taanf ", "tần "), // tần (frequency)
        ("laauj ", "lậu "), // lậu (illegal)
        ("leex ", "lễ "),   // lễ (ceremony)
    ]);
}

// =============================================================================
// AIR PATTERN - SPECIAL CASE
// "air" → "ải" is valid Vietnamese (border/pass), should NOT restore
// =============================================================================

#[test]
fn air_stays_vietnamese() {
    // "air" typed becomes "ải" - valid Vietnamese word
    // Should NOT restore because "ải" (border/pass) is a real word
    telex(&[("air ", "ải ")]);
}

// =============================================================================
// WORDS THAT CANNOT BE AUTO-DETECTED (DOCUMENTATION)
// These produce structurally valid Vietnamese
// =============================================================================

#[test]
#[ignore] // These CANNOT be auto-detected without dictionary
fn words_that_stay_transformed() {
    // These produce valid Vietnamese structures
    // Users should use raw mode (\word) or Esc to restore
    telex(&[
        ("mix ", "mix "), // → "mĩ" (valid Vietnamese)
        ("box ", "box "), // → "bõ" (valid Vietnamese)
        ("six ", "six "), // → "sĩ" (valid Vietnamese)
    ]);
}

// =============================================================================
// PATTERN 7: VOWEL + MODIFIER + VOWEL (WITH INITIAL CONSONANT)
// Example: "core" = c + o + r + e → "cỏe" invalid → restore
// =============================================================================

#[test]
fn pattern7_vowel_modifier_vowel_with_initial() {
    telex(&[
        ("core ", "core "),
        ("more ", "more "),
        ("care ", "care "),
        ("rare ", "rare "),
        ("are ", "are "),
        ("ore ", "ore "),
        ("bore ", "bore "),
        ("fore ", "fore "), // F initial also triggers Pattern 6
        ("sore ", "sore "),
        ("wore ", "wore "), // W initial also triggers Pattern 5
        ("store ", "store "),
        ("score ", "score "),
    ]);
}

#[test]
fn vietnamese_ua_uo_preserved() {
    // Vietnamese ưa/ươ patterns should NOT restore
    // u + modifier + a → ưa family (cửa, mua, bưa)
    // u + modifier + o → ươ family (được, bước)
    telex(&[
        ("cura ", "của "),      // của (of) - common Vietnamese
        ("muar ", "mủa "),      // mủa (not common but valid structure)
        ("dduwowcj ", "được "), // được (can/get)
    ]);
}

// =============================================================================
// PATTERN 8: W AS FINAL (NOT MODIFIER)
// Example: "raw" = r + a + w → W can't modify A, stays as W final
// =============================================================================

#[test]
fn pattern8_w_as_final() {
    telex(&[
        ("raw ", "raw "),
        ("law ", "law "),
        ("saw ", "saw "),
        ("jaw ", "jaw "),
        ("draw ", "draw "),
        ("straw ", "straw "),
    ]);
}

// =============================================================================
// VIETNAMESE WORDS WITH TH INITIAL - MUST NOT RESTORE
// "th" is one of the most common Vietnamese initials
// =============================================================================

#[test]
fn vietnamese_th_initial_preserved() {
    telex(&[
        ("thees ", "thế "),   // thế (so/thus) - common Vietnamese
        ("these ", "thế "),   // same as above, different typing order
        ("ther ", "thẻ "),    // thẻ (card)
        ("thes ", "thé "),    // thé (not common but valid)
        ("thef ", "thè "),    // thè (stick out tongue)
        ("thej ", "thẹ "),    // thẹ (shy)
        ("thax ", "thã "),    // valid structure
        ("thar ", "thả "),    // thả (release)
        ("thas ", "thá "),    // valid structure
        ("thaf ", "thà "),    // thà (rather)
        ("thaj ", "thạ "),    // valid structure
        ("thor ", "thỏ "),    // thỏ (rabbit)
        ("thos ", "thó "),    // valid structure
        ("thof ", "thò "),    // thò (peek out)
        ("thoj ", "thọ "),    // thọ (longevity)
        ("thux ", "thũ "),    // valid structure
        ("thur ", "thủ "),    // thủ (leader/chief)
        ("thus ", "thú "),    // thú (animal/fun)
        ("thuf ", "thù "),    // thù (hatred)
        ("thuj ", "thụ "),    // thụ (passive)
    ]);
}

// =============================================================================
// MORE VIETNAMESE COMMON WORDS - MUST NOT RESTORE
// =============================================================================

#[test]
fn vietnamese_common_words_preserved() {
    telex(&[
        // Common words with tone modifiers
        ("cos ", "có "),      // có (have)
        ("cof ", "cò "),      // cò (stork)
        ("cor ", "cỏ "),      // cỏ (grass)
        ("cox ", "cõ "),      // valid structure
        ("coj ", "cọ "),      // cọ (palm tree)
        ("mos ", "mó "),      // mó (touch)
        ("mof ", "mò "),      // mò (grope)
        ("mor ", "mỏ "),      // mỏ (beak)
        ("mox ", "mõ "),      // mõ (wooden bell)
        ("moj ", "mọ "),      // mọ (all - dialect)
        ("tos ", "tó "),      // valid structure
        ("tof ", "tò "),      // tò (curious)
        ("tor ", "tỏ "),      // tỏ (garlic)
        ("tox ", "tõ "),      // valid structure
        ("toj ", "tọ "),      // valid structure
        ("nos ", "nó "),      // nó (it/he/she)
        ("nof ", "nò "),      // valid structure
        ("nor ", "nỏ "),      // valid structure
        ("nox ", "nõ "),      // valid structure
        ("noj ", "nọ "),      // nọ (that - dialect)
        // Words with đ
        ("ddos ", "đó "),     // đó (there)
        ("ddof ", "đò "),     // đò (boat)
        ("ddor ", "đỏ "),     // đỏ (red)
        ("ddox ", "đõ "),     // valid structure
        ("ddoj ", "đọ "),     // đọ (compare)
        // Words ending in consonants
        ("cons ", "cón "),    // valid structure
        ("conf ", "còn "),    // còn (still/remain)
        ("conr ", "cỏn "),    // valid structure
        ("conx ", "cõn "),    // valid structure
        ("conj ", "cọn "),    // valid structure
    ]);
}

// =============================================================================
// EXPANDED ENGLISH RESTORE TESTS - COMMON PROGRAMMING TERMS
// =============================================================================

#[test]
fn tech_english_words_restore() {
    telex(&[
        // Database terms
        ("store ", "store "),
        ("score ", "score "),
        // Web terms
        ("core ", "core "),
        ("more ", "more "),
        // Programming - consonant + o/a + modifier + e patterns
        ("code ", "code "),
        ("node ", "node "),
        ("mode ", "mode "),
        ("base ", "base "),
        ("case ", "case "),
        ("name ", "name "),
        ("make ", "make "),
        ("take ", "take "),
        ("fake ", "fake "),
        ("safe ", "safe "),
        ("save ", "save "),
        ("have ", "have "),
        ("wave ", "wave "),
        ("move ", "move "),
        ("love ", "love "),
        ("live ", "live "),
        ("give ", "give "),
        ("five ", "five "),
        ("drive ", "drive "),
        // W initial words → always restore
        ("where ", "where "),
        ("were ", "were "),
        // Different vowel patterns
        ("share ", "share "),
        ("spare ", "spare "),
        ("stare ", "stare "),
        ("aware ", "aware "),
        ("before ", "before "),
        ("explore ", "explore "),
        ("ignore ", "ignore "),
        ("restore ", "restore "),
    ]);
}

// =============================================================================
// SAME-VOWEL PATTERN: Vowel + modifier + SAME vowel
// These produce valid Vietnamese-like structures and should NOT restore
// Users should use ESC or raw mode for these specific English words
// =============================================================================

#[test]
fn same_vowel_pattern_vietnamese() {
    // Same vowel before/after modifier → Vietnamese pattern
    // e + modifier + e, a + modifier + a, o + modifier + o
    telex(&[
        ("these ", "thế "),   // th + e + s + e → thế (Vietnamese word)
        ("there ", "thể "),   // th + e + r + e → thể (Vietnamese word)
        ("here ", "hể "),     // h + e + r + e → hể (valid structure)
        ("mere ", "mể "),     // m + e + r + e → mể (valid structure)
    ]);
}
