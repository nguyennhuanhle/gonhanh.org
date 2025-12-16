# Auto-Correct Feature

Gõ Nhanh includes an optional auto-correct feature that automatically fixes common spelling mistakes as you type. This feature is **disabled by default** and can be enabled in the settings.

## Overview

The auto-correct system works by detecting common typos and automatically replacing them with the correct spelling when you press the space bar. It supports:

- **Vietnamese corrections**: Common n/l confusion, i/y normalization, abbreviations
- **English corrections**: Programming typos, letter swaps, common misspellings

## How It Works

1. As you type, the engine builds up a buffer of keystrokes
2. When you press space, it checks the buffer against a dictionary of known corrections
3. If a match is found, it automatically replaces the word with the correct spelling
4. The correction preserves your original case (lowercase, UPPERCASE, or Title Case)

## Correction Modes

| Mode | Description |
|------|-------------|
| **Off** (Default) | No auto-correction |
| **Vietnamese** | Only Vietnamese corrections |
| **English** | Only English corrections |
| **All** | Both Vietnamese and English corrections |

## Enabling Auto-Correct

### macOS
1. Click the Gõ Nhanh icon in the menu bar
2. Select "Cài đặt" (Settings)
3. Find "Tự sửa lỗi chính tả" (Auto-correct)
4. Select your preferred mode from the dropdown

### Windows / Linux
Auto-correct settings will be available in the settings panel (coming soon).

## Vietnamese Corrections

### N/L Consonant Confusion (Southern Dialect)
Common in Southern Vietnamese dialects where n/l sounds are often confused:

| Typo | Correction |
|------|------------|
| nà | là |
| nàm | làm |
| nên | lên |
| nời | lời |
| nại | lại |
| lăm | năm |
| lày | này |
| lói | nói |

### I/Y Normalization
Following modern Vietnamese orthography standards:

| Typo | Correction |
|------|------------|
| lí | lý |
| kí | ký |
| quí | quý |
| tỉ | tỷ |

### Common Abbreviations
Popular Vietnamese text shortcuts:

| Abbreviation | Expansion |
|--------------|-----------|
| ko | không |
| dc | được |
| vs | với |
| cx | cũng |
| j | gì |
| z | vậy |
| ntn | như thế nào |

### Tone/Diacritic Errors
Common mistakes with Vietnamese diacritics:

| Typo | Correction |
|------|------------|
| dể | dễ |
| củng | cũng |
| giử | giữ |
| mổi | mỗi |

## English Corrections

### Programming Typos
Common mistakes when coding:

| Typo | Correction |
|------|------------|
| fucntion | function |
| retunr | return |
| cosnt | const |
| calss | class |
| improt | import |
| exprot | export |
| pritn | print |
| lenght | length |

### Letter Swaps
Common transposition errors:

| Typo | Correction |
|------|------------|
| teh | the |
| taht | that |
| wiht | with |
| waht | what |
| adn | and |
| hte | the |
| yuo | you |

### Double Letter Errors
Missing or extra repeated letters:

| Typo | Correction |
|------|------------|
| adress | address |
| occured | occurred |
| recieve | receive |
| untill | until |
| sucessful | successful |

## Technical Details

### Implementation
- **Location**: `core/src/engine/autocorrect.rs`
- **Data**: `core/src/data/corrections.rs`
- **Lookup**: O(1) HashMap-based lookup
- **Memory**: ~50KB for full correction database
- **Lazy Loading**: Maps are loaded on-demand when mode is enabled

### FFI Functions
```rust
// Set auto-correct mode (0=Off, 1=Vietnamese, 2=English, 3=All)
ime_autocorrect_mode(mode: u8)

// Get current mode
ime_autocorrect_get_mode() -> u8

// Check if enabled
ime_autocorrect_enabled() -> bool
```

### Case Preservation
The auto-correct system preserves the case of your original input:

- `teh` → `the` (lowercase)
- `Teh` → `The` (title case)
- `TEH` → `THE` (uppercase)

## Limitations

1. **Telex/VNI Transform Interaction**: Auto-correct checks the raw keystrokes in the buffer, not the transformed Vietnamese output. This means corrections for Vietnamese words with diacritics (like "nà" → "là") require the word to be in the correction database as it appears after typing.

2. **Context-Free**: The auto-correct is purely pattern-based and doesn't understand context. Words that have multiple valid forms (like "form" vs "from") may be incorrectly corrected in some contexts.

3. **No Learning**: The correction database is static and doesn't learn from user behavior.

## Adding Custom Corrections

Custom corrections can be added via the shortcut system, which takes priority over auto-correct. Add shortcuts in Settings → "Từ viết tắt" for your personal corrections.

## Disabling Auto-Correct

If auto-correct interferes with your typing:
1. Go to Settings
2. Set "Tự sửa lỗi chính tả" to "Tắt" (Off)

Or use shortcuts for specific words you don't want corrected.
