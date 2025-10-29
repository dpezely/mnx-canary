# MNX Canary

This exercises [Typify](https://github.com/oxidecomputer/typify)'s
[PR 912](https://github.com/oxidecomputer/typify/pull/912)
using W3C [mnx-schema.json](https://w3c.github.io/mnx/docs/mnx-schema.json)
and [examples](https://github.com/w3c/mnx/tree/main/docs/static/examples/json).

The included [mnx.rs](./src/mnx.rs) was generated from `mnx-schema.json`
using the patched version of Typify via the following command, and
additional comments and Clippy allow attributes added.

```bash
cargo typify w3c-mnx/docs/mnx-schema.json src/mnx.rs
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
