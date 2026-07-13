[`nhs-number` specification](index.md) — section 17 of 19. Section numbers (§17.x) are stable and cited from code, tests, and commit messages.

# 17. Open tasks (backlog)

Each task is a small, specific unit of work. A task moves into the roadmap
(§16) when it is scheduled; it leaves this section when it ships (the
change should leave the spec in a state where the task is no longer
needed). Completed tasks are deleted, not archived in place — the commit
history is the changelog.

Tasks are labelled `T<number>` so they can be referenced from commits and
issues. **IDs are never reused** even after a task ships, so future
references stay unambiguous.

The backlog is currently **empty**: T1 (`Hash`), T2 (`serde_string`
wrapper), T3 (`is_issuable_range`), T5 (`BTreeMap` example), T7
(validating `Deserialize`), and T8 (`ParseError` `Display`/`Error`) all
shipped in 2.0.0; T4 and T6 closed earlier. See the commit history for
each. The next task ID is **T9**.
