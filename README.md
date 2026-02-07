# MNX Canary

This exercises [Typify](https://github.com/oxidecomputer/typify)'s
[PR 912](https://github.com/oxidecomputer/typify/pull/912)
using W3C [mnx-schema.json](https://w3c.github.io/mnx/docs/mnx-schema.json)
and [examples](https://github.com/w3c/mnx/tree/main/docs/static/examples/json).

The included [mnx.rs](./src/mnx.rs) was generated from `mnx-schema.json`
using the patched version of Typify via the following command, and
additional comments and Clippy allow attributes added.

## With modified mnx-schema.json

This branch exercises a modification to `mnx-schema.json`
where `"sequence-content"` -> `"items"` specifies `"oneOf"`
(rather than `"anyOf"`).

Additional test added to `tests/canary.rs` exercises destructuring of the
following hierarchy:

1. Root
2. Parts
3. PartMeasure
4. SequenceList
7. Sequence
8. SequenceContent
9. SequenceContentItem which as an Enum contains variants:
    + Event
    + Grace
    + Tuplet
    + Space
    + MultiNoteTremolo

Confirm experimental change to schema:

```bash
diff w3c-mnx/docs/mnx-schema.json mnx-schema_*.json
```

Yields:

```diff
1526c1526
<         "anyOf": [
---
>         "oneOf": [
```

See comment marked `XXX` in `tests/canary.rs` file.

```bash
# Normally:
# cargo typify w3c-mnx/docs/mnx-schema.json --output src/mnx.rs
# Use this instead for branch, `dpezely/sequence-content-item_via_oneOf`
cargo typify mnx-schema_*.json --output src/mnx.rs
```

The W3C MNX schema and examples are an ongoing work-in-progress and likely to
have regular updates, so be sure to fetch the git submodule pinned to a
specific commit known to work with tests here.

```bash
git submodule update --init --recursive
```

Then, run tests:

```bash
cargo test
```

Optionally, generate docs to see API generated from the modified schema:

```bash
cargo doc
```
