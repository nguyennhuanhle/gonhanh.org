# Vietnamese IME Algorithm Tree

## MAIN PROCESSING TREE

```
Key Event
├── IME Disabled OR Modifier Key?
│   ├── YES: Clear Buffer → Pass Through
│   └── NO: Continue
├── Break Key?
│   ├── YES: Clear Buffer → Pass Through
│   └── NO: Continue
├── Delete Key?
│   ├── YES: Remove Last Char → Pass Through
│   └── NO: Process IME Logic
```

## IME COMPOSITION TREE

```
Process Key
├── Try đ Transformation
│   ├── Immediate Mode (dd/d9)?
│   │   ├── YES: Replace → Rebuild
│   │   └── NO: Continue
│   ├── Delayed Mode (VNI 9 + unconverted d)?
│   │   ├── YES: Mark d → Rebuild
│   │   └── NO: Continue
├── Try Tone Modifier
│   ├── Valid Modifier Key?
│   │   ├── YES: Apply to Vowels → Rebuild
│   │   └── NO: Continue
├── Try Tone Mark
│   ├── Valid Mark Key?
│   │   ├── YES: Place Mark → Rebuild
│   │   └── NO: Continue
├── Remove Key?
│   ├── YES: Remove Diacritics → Rebuild
│   └── NO: Add Character
```

## BUFFER MANAGEMENT TREE

```
Buffer State: length ∈ [0,32]
├── Append Character
│   ├── Pre: length < 32
│   ├── Action: data[length] = char, length++
│   └── Post: length increased by 1
├── Remove Last
│   ├── Pre: length > 0
│   ├── Action: length--
│   └── Post: last char removed
├── Clear
│   ├── Action: length = 0
│   └── Post: all chars discarded
├── Access Position
│   ├── Pre: 0 ≤ pos < length
│   ├── Action: return data[pos]
│   └── Post: read-only access
```

## KEY CLASSIFICATION TREE

```
Key Type Classification
├── Modifier Keys (Ctrl, Alt, Cmd)?
│   ├── YES: IME Bypass → Clear Buffer
│   └── NO: Continue
├── Break Keys?
│   ├── Navigation: ← → ↑ ↓
│   ├── Control: Space Tab Return Enter Esc
│   ├── Punctuation: . , / ; ' [ ] \ - = `
│   └── Action: Clear Buffer → Word Boundary
├── Delete/Backspace?
│   ├── Buffer Empty?
│   │   ├── YES: Pass Through
│   │   └── NO: Remove Last → Pass Through
│   └── Destructive Operation
└── Regular Keys → IME Processing
```

## BREAK KEY BEHAVIOR TREE

```
Break Key Triggered
├── Clear Entire Buffer
│   └── length = 0
├── Reset Transformation State
│   └── last_transform = None
├── Pass Key Through Unchanged
│   └── Application Handles Key
└── Word Composition Terminated
```

### Buffer Management Details

#### Buffer Structure

```rust
struct Buffer {
    data: [Char; 32],  // Fixed-size array, max 32 characters
    len: usize,        // Current number of characters (0-32)
}
```

Each `Char` contains:

- `key: u16` - Virtual keycode
- `caps: bool` - Shift/CapsLock state
- `tone: u8` - Diacritic type (0=none, 1=circumflex, 2=horn/breve)
- `mark: u8` - Tone mark (0=none, 1-5=sắc,huyền,hỏi,ngã,nặng)
- `stroke: bool` - Special flag for đ conversion

#### Buffer Operations

- **Push**: Add character to end (`data[len] = char; len += 1`)
- **Pop**: Remove from end (`len -= 1; return data[len]`)
- **Clear**: Reset to empty (`len = 0`)
- **Get**: Access by index (bounds checked)
- **Iterate**: Access all current characters (`data[0..len]`)

#### Memory Management

- **No dynamic allocation**: Fixed 32-character buffer
- **No garbage collection**: Simple array operations
- **Thread-safe**: Protected by Mutex in FFI layer
- **Zero-copy**: Direct array access, no cloning

## Đ TRANSFORMATION TREE

```
đ Transformation Attempt
├── Immediate Mode?
│   ├── Telex: Second 'd' after 'd'
│   │   ├── YES: Remove second 'd' → Replace first with đ
│   │   └── NO: Continue
│   ├── VNI: '9' after 'd'
│   │   ├── YES: Remove '9' → Replace 'd' with đ
│   │   └── NO: Continue
├── Delayed Mode (VNI)?
│   ├── '9' key AND unconverted 'd' exists?
│   │   ├── YES: Find first 'd' → Mark stroke flag → Rebuild
│   │   └── NO: Continue
└── No Transformation → Next Priority
```

## TONE MODIFIER TREE

```
Tone Modifier Processing
├── Modifier Key Detected?
│   ├── Telex: aa/ee/oo (circumflex), aw/ow/uw (horn)
│   ├── VNI: 6 (circumflex), 7 (horn), 8 (breve)
│   └── NO: Skip
├── Scan Buffer for Target Vowels
│   ├── Find unmodified vowels matching modifier
│   ├── Handle compound patterns (uo → ươ)
│   └── Select target vowel(s)
├── Apply Modifier
│   ├── Set tone flag on target(s)
│   ├── Record transformation state
│   └── Rebuild text
└── Revert Check
    ├── Same key pressed again?
    │   ├── YES: Remove modifier → Append key
    │   └── NO: Normal processing
```

## TONE MARK TREE

```
Tone Mark Processing
├── Mark Key Detected?
│   ├── Telex: s f r x j
│   ├── VNI: 1 2 3 4 5
│   └── NO: Skip
├── Analyze Vowel Sequence
│   ├── Count vowels in buffer
│   ├── Check for final consonant
│   ├── Determine syllable structure
│   └── Calculate mark position
├── Apply Mark
│   ├── Set mark flag on target vowel
│   ├── Record transformation state
│   └── Rebuild text
└── Revert Check
    ├── Same key pressed again?
    │   ├── YES: Remove mark → Append key
    │   └── NO: Normal processing
```

## PHONOLOGY DECISION TREE

```
Tone Mark Position (Vietnamese Rules)
├── Single Vowel?
│   └── Position: 0 (the vowel)
├── Two Vowels + Final Consonant?
│   └── Position: 1 (second vowel)
├── Two Vowels Open Syllable?
│   ├── Medial + Main pattern (oa, oe, uy, qua)?
│   │   └── Position: 1 (main vowel)
│   ├── Main + Glide pattern (ai, ao, au, oi)?
│   │   └── Position: 0 (main vowel)
│   ├── Compound vowel (ươ, uô, iê)?
│   │   └── Position: 1 (second vowel)
│   ├── ư + a pattern?
│   │   └── Position: 0 (ư has diacritic)
│   └── u + a without q?
│       └── Position: 0 (u is main)
└── Three+ Vowels?
    └── Position: Middle vowel
```

## 4. Text Reconstruction Algorithm

### 4.1 rebuild_from Function

**Purpose:** Generate Unicode output for buffer segment starting at specified position.

**Algorithm:**

```rust
fn rebuild_from(from: usize) -> Result {
    let mut output = Vec::with_capacity(buffer.len - from);
    let mut backspace_count = 0;

    for i in from..buffer.len {
        backspace_count += 1;

        if let Some(char) = buffer.get(i) {
            let unicode_char = generate_unicode_char(char);
            output.push(unicode_char);
        }
    }

    if output.is_empty() {
        Result::none()
    } else {
        Result::send(backspace_count, &output)
    }
}
```

### 4.2 Unicode Character Generation

```rust
fn generate_unicode_char(char: &Char) -> char {
    // Special handling for đ conversion
    if char.key == D && char.stroke {
        return if char.caps { 'Đ' } else { 'đ' };
    }

    // Vowel with diacritics
    if let Some(base) = get_base_vowel(char.key, char.tone) {
        let marked = apply_mark_to_vowel(base, char.mark);
        return apply_case(marked, char.caps);
    }

    // Regular character
    let base = key_to_base_char(char.key);
    apply_case(base, char.caps)
}
```

### 4.3 Example Execution

**Input Buffer:** `[t, o, a, n]` with `mark = 1` (sắc) on position 2
**Call:** `rebuild_from(2)`
**Process:**

- `backspace_count = 2` (positions 2, 3)
- `output[0] = 'á'` (a + sắc)
- `output[1] = 'n'` (n unchanged)
  **Result:** `Result::send(2, ['á', 'n'])`

## TRANSFORMATION STATE TREE

```
State Tracking
├── Last Transformation Type
│   ├── Mark: (key, mark_value)
│   ├── Tone: (key, tone_value, target_vowel)
│   └── None: No transformation recorded
├── Revert Trigger
│   ├── Same key pressed again?
│   │   ├── YES: Execute revert
│   │   └── NO: Normal processing
└── State Reset
    ├── After successful transformation
    └── After revert operation
```

## REVERT OPERATION TREE

```
Revert Processing
├── Tone Revert
│   ├── Find vowels with tone modifier
│   ├── Remove modifier (reverse order)
│   ├── Rebuild from first change
│   └── Append original key as character
├── Mark Revert
│   ├── Find vowel with tone mark
│   ├── Remove mark (reverse order)
│   ├── Rebuild from first change
│   └── Append original key as character
└── State Cleanup
    └── last_transform = None
```

## Edge Cases and Special Behaviors

### Empty Buffer Operations

- **Backspace on empty buffer**: Pass through (application handles)
- **Break key on empty buffer**: Pass through (no buffer to clear)

### Buffer Overflow Protection

- **Max 32 characters**: Prevents infinite composition
- **Silent truncation**: Excess characters ignored

### Modifier Key Interactions

- **Ctrl/Alt/Cmd + key**: IME disabled, buffer cleared
- **Shift**: Recorded in `caps` flag, affects output case

### Unicode Output Details

- **Vowel composition**: tone + mark applied in correct order
- **Case handling**: Uses Rust's `to_uppercase()` for Unicode awareness
- **đ conversion**: Special handling separate from vowel system

## OUTPUT COMMUNICATION TREE

```
IME Result Structure
├── Action Type
│   ├── None: Pass key through unchanged
│   ├── Send: Delete + Insert operation
│   └── Restore: Invalid state recovery
├── Backspace Count
│   └── N: Characters to delete
├── Character Array
│   └── M: Unicode codepoints to insert
└── Valid Count
    └── Number of valid characters in array
```

## APPLICATION INTERPRETATION TREE

```
Process IME Result
├── Action = None?
│   ├── YES: Send original key to application
│   └── NO: Continue
├── Action = Send?
│   ├── YES: Execute delete + insert
│   │   ├── Send N backspace/delete operations
│   │   └── Insert M Unicode characters
│   └── NO: Continue
└── Action = Restore?
    ├── YES: Restore original text state
    └── NO: Error condition
```

## PROCESSING SEQUENCE TREE

```
Key Event Flow
├── Application → IME: Send key event
├── IME: Process through algorithm tree
├── IME → Application: Return result structure
├── Application: Interpret result
│   ├── Execute text operations
│   └── Update display
└── Application → IME: Release result memory
```

## INPUT METHOD SPECIFICATION TREE

```
Telex Method
├── Tone Marks
│   ├── s → sắc, f → huyền, r → hỏi
│   ├── x → ngã, j → nặng
│   └── Immediate placement
├── Tone Modifiers
│   ├── aa/ee/oo → circumflex (^)
│   ├── aw/ow/uw → horn/breve
│   └── dd → đ (stroke)
├── Remove Key
│   └── z → remove all diacritics
└── Delayed Input
    └── Modifiers work on existing text
```

```
VNI Method
├── Tone Marks
│   ├── 1 → sắc, 2 → huyền, 3 → hỏi
│   ├── 4 → ngã, 5 → nặng
│   └── Immediate placement
├── Tone Modifiers
│   ├── 6 → circumflex on a,e,o
│   ├── 7 → horn on o,u
│   ├── 8 → breve on a
│   └── 9 → đ (stroke)
├── Remove Key
│   └── 0 → remove all diacritics
└── Delayed Input
    └── Modifiers and đ work on existing text
```

## PHONOLOGICAL RULES TREE

```
Vietnamese Syllable Structure
├── Pattern: (C)(G)V(C)
│   ├── C: Consonant (optional)
│   ├── G: Medial glide o/u (optional)
│   ├── V: Main vowel(s) (required)
│   └── C: Final consonant (optional)
└── Tone mark placement based on structure
```

```
Tone Mark Placement Rules
├── Single Vowel Syllable
│   └── Mark on the vowel
├── Two Vowels + Final Consonant
│   └── Mark on second vowel
├── Two Vowels Open Syllable
│   ├── Medial + Main (oa, oe, uy, qua)
│   │   └── Mark on second (main) vowel
│   ├── Main + Glide (ai, ao, au, oi)
│   │   └── Mark on first (main) vowel
│   ├── Compound (ươ, uô, iê)
│   │   └── Mark on second vowel
│   ├── ư + a pattern
│   │   └── Mark on ư (diacritic precedence)
│   └── u + a without q
│       └── Mark on u (main vowel)
└── Three+ Vowels
    └── Mark on middle vowel
```

```
Modifier Precedence Tree
├── Base Vowel Selection
│   └── Choose appropriate base (a, ă, â, etc.)
├── Tone Modifier Application
│   ├── Circumflex (^): Changes base form
│   ├── Horn/Breve: Changes base form
│   └── Applied before tone marks
├── Tone Mark Application
│   └── Applied to modified base vowel
└── Case Application
    └── Final upper/lower case transformation
```

```
Compound Vowel Rules
├── ươ Pattern
│   ├── u + o with horn modifier
│   └── Horn applied to both vowels
├── uô Pattern
│   ├── u + o with circumflex
│   └── Circumflex on o, horn on u
└── iê Pattern
    ├── i + e with circumflex
    └── Circumflex applied to e
```

## ERROR HANDLING TREE

```
Buffer Overflow Protection
├── Maximum Capacity: 32 characters
├── Excess Input?
│   ├── YES: Silently ignore
│   └── NO: Process normally
└── Prevents unbounded growth
```

```
Invalid Input Handling
├── Unknown Keycode?
│   ├── YES: Pass through unchanged
│   └── NO: Process through IME logic
├── Invalid Sequence?
│   ├── YES: Handle gracefully, maintain state
│   └── NO: Normal processing
└── State Consistency Preserved
```

```
Concurrency Protection
├── Mutual Exclusion
│   └── Single-threaded access guarantee
├── State Invariants
│   └── Preserved across operations
└── Thread Safety
    └── No race conditions possible
```

## Input Method Differences

### Telex

- Marks: s(1), f(2), r(3), x(4), j(5)
- Tones: aa(â), ee(ê), oo(ô), aw(ă), ow(ơ), uw(ư), dd(đ)
- Remove: z

### VNI

- Marks: 1(1), 2(2), 3(3), 4(4), 5(5)
- Tones: 6(â), 7(ơ), 8(ă), 9(đ)
- Remove: 0
- Delayed đ: Find 'd' anywhere (dung9 → đung)

## COMPUTATIONAL COMPLEXITY TREE

```
Time Complexity Analysis
├── Key Processing: O(1)
│   └── Constant time decision tree
├── Buffer Operations: O(1)
│   └── Fixed-size array access
├── Vowel Search: O(n), n ≤ 32
│   └── Linear scan of buffer
├── Text Generation: O(m), m ≤ 32
│   └── Character processing loop
└── Phonology Analysis: O(k), k ≤ 32
    └── Syllable structure evaluation
```

```
Space Complexity
├── Fixed Buffer: 256 bytes maximum
│   ├── 32 characters × 8 bytes each
│   └── No dynamic allocation
├── Memory Usage: Bounded and predictable
│   └── No heap allocation during typing
└── Memory Safety: Guaranteed bounds checking
```

```
Execution Time Bounds
├── Simple Operations: < 1 μs
│   └── Buffer insertion/deletion
├── Complex Transformations: < 10 μs
│   └── Multi-vowel processing + rebuild
└── All Operations: Bounded by buffer size
    └── Deterministic worst-case performance
```

```
Concurrency Model
├── Interface Level: Mutual exclusion
├── Core Logic: Single-threaded
└── Instance Isolation: Independent operation
```

## ARCHITECTURE PRINCIPLES TREE

```
Memory Management
├── Zero-Copy Access
│   └── Direct buffer manipulation
├── Fixed-Size Allocations
│   └── No dynamic memory during operation
└── Bounds-Checked Operations
    └── Prevent out-of-bounds access
```

```
Safety Properties
├── No Undefined Behavior
│   └── Deterministic operation
├── Memory Safety Guarantees
│   └── Bounds checking + type safety
└── Type System Enforcement
    └── Compile-time correctness
```

```
Platform Independence
├── Abstract Keycode Mapping
│   └── OS-independent key representation
├── Unicode-Based Output
│   └── Universal character encoding
└── Cross-Platform Compatibility
    └── Same logic across all platforms
```

## KEY MAPPING REFERENCE TREE

```
Telex Method Mappings
├── Tone Marks
│   ├── s → sắc, f → huyền, r → hỏi
│   ├── x → ngã, j → nặng
│   └── Immediate application
├── Tone Modifiers
│   ├── aa/ee/oo → circumflex (^)
│   ├── aw/ow/uw → horn/breve
│   └── dd → đ (stroke)
├── Remove
│   └── z → clear all diacritics
└── Delayed Mode
    └── Modifiers work on existing text
```

```
VNI Method Mappings
├── Tone Marks
│   ├── 1 → sắc, 2 → huyền, 3 → hỏi
│   ├── 4 → ngã, 5 → nặng
│   └── Immediate application
├── Tone Modifiers
│   ├── 6 → circumflex on a,e,o
│   ├── 7 → horn on o,u
│   ├── 8 → breve on a
│   └── 9 → đ (stroke)
├── Remove
│   └── 0 → clear all diacritics
└── Delayed Mode
    └── Modifiers and đ work on existing text
```
