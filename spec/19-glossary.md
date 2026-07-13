[`nhs-number` specification](index.md) — section 19 of 19. Section numbers (§19.x) are stable and cited from code, tests, and commit messages.

# 19. Glossary

| Term                | Meaning                                                                |
| ------------------- | ---------------------------------------------------------------------- |
| NHS Number          | The ten-digit identifier defined in §2.1.                              |
| Check digit         | The tenth digit, computed per §6.                                      |
| Canonical form      | `"DDD DDD DDDD"` — three groups separated by single spaces (§5.1).     |
| Tight form          | `"DDDDDDDDDD"` — ten contiguous digits, no separators (§5.2).          |
| Testable range      | `999 000 0000` – `999 999 9999`, never issued to real patients (§7.3). |
| Issued range        | Any of the ranges in §7.1.                                             |
| Issuable range      | An issued range net of the §7.2 reservations — what `is_issuable_range` tests (§7.5). |
| Validates           | `check_digit() == calculate_check_digit()`.                            |
| Strict spec         | The NHS-published check-digit algorithm in §6.1.                       |
| Sentinel `10`       | The value `calculate_check_digit` returns when `sum % 11 == 1` (§6.1.1). |
| Rule `R<n>`         | A numbered behavioural rule (§2.3).                                    |
| Task `T<n>`         | A numbered backlog item (§17).                                         |
| Canonical fixture   | An `NHSNumber` value listed in §13.3.                                  |
