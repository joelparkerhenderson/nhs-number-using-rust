[`nhs-number` specification](index.md) — section 5 of 19. Section numbers (§5.x) are stable and cited from code, tests, and commit messages.

# 5. String forms (parsing and formatting)

### 5.1 Canonical display form [R4, R18]

`Display` always produces **exactly** twelve characters:

```
DDD DDD DDDD
```

- three digits, single space, three digits, single space, four digits;
- no leading, trailing, or doubled spaces;
- no alternative separators (hyphen, period, slash, NBSP);
- output is pure ASCII.

`From<NHSNumber> for String` and the blanket-provided `Into<String>`
delegate to `Display`, so they produce the same twelve-character string.

### 5.2 Accepted input forms [R5, R6]

`FromStr::from_str` accepts **exactly two** shapes:

1. **Ten contiguous digits**, no separators: `"DDDDDDDDDD"` (length 10).
2. **Canonical with single spaces**: `"DDD DDD DDDD"` (length 12, space at
   position 3 and position 7 only).

Both must be **ASCII** digits 0–9 in every digit position. Non-ASCII
digit characters that render as digits (Arabic-Indic `٠..٩`, full-width
`０..９`, Devanagari `०..९`, etc.) are rejected because
`char::to_digit(10)` only accepts ASCII `0..=9`.

### 5.3 Rejected input forms

Everything else returns `Err(ParseError)`. Notable examples — each is
covered by a dedicated test in `src/from_str.rs::tests`:

| Input              | Reason                                                         |
| ------------------ | -------------------------------------------------------------- |
| `""`               | Length 0 — neither accepted length.                            |
| `"12345"`          | Length 5.                                                      |
| `"01234567890"`    | Length 11.                                                     |
| `"0123 4567 8901"` | Length 14.                                                     |
| `"012-345-6789"`   | Length 12 but separators are hyphens.                          |
| `" 012 345 6789"`  | Leading space — position 3 is a digit, not the required space. |
| `"012 345 6789 "`  | Trailing space — length becomes 13.                            |
| `"012  345  6789"` | Doubled spaces shift the second group out of position.         |
| `"012 3456789"`    | One space only — length 11.                                    |
| `"012345 6789"`    | One space only, wrong place — length 11.                       |
| `"abc 123 4567"`   | Non-digit characters.                                          |
| `"012 345 abcd"`   | Non-digit characters in the last group.                        |
| `"012\u{00A0}345\u{00A0}6789"` | NBSP separators, not ASCII space.                  |
| `"012\t345\t6789"` | Tab separators, not ASCII space.                               |
| `"٠١٢٣٤٥٦٧٨٩"`     | Arabic-Indic digits; not ASCII.                                |
| `"-123456789"`     | Sign is not a digit.                                           |
| `"+123456789"`     | Sign is not a digit.                                           |
| `"12345.6789"`     | Decimal point.                                                 |
| `"0123🦀5678"`     | Emoji / non-digit `char`.                                      |

### 5.4 Round-trip property [R8]

For every `n: NHSNumber`:

```
NHSNumber::from_str(&n.to_string()).unwrap() == n
```

The reverse direction also holds: a value produced by `from_str` from a
canonical string renders back to the same canonical string. This is
enforced by `src/lib.rs::tests::properties::round_trip_via_canonical_form`
and `round_trip_via_tight_form`.

### 5.5 Normalisation policy [R7]

The parser performs **no normalisation** of its input. Specifically, it
does not:

- trim leading or trailing whitespace,
- collapse runs of internal whitespace,
- swap alternative separators (hyphens, full-stops, NBSP, tabs) for spaces,
- accept non-ASCII digit characters even when they map to a value 0–9
  under Unicode's `Nd` category,
- accept uppercase or other non-digit characters that look like digits
  (e.g. lowercase `l` for `1`).

Callers whose upstream sources produce variant forms must normalise
before delegating to `FromStr`. This keeps the parser fast and its
contract small; richer ergonomics belong in caller code.
