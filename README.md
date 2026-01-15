# MNX Canary

This exercises [Typify](https://github.com/oxidecomputer/typify)'s
[PR 912](https://github.com/oxidecomputer/typify/pull/912)
using W3C [mnx-schema.json](https://w3c.github.io/mnx/docs/mnx-schema.json)
and [examples](https://github.com/w3c/mnx/tree/main/docs/static/examples/json).

The included [mnx.rs](./src/mnx.rs) was generated from `mnx-schema.json`
using the patched version of Typify via the following command, and
additional comments and Clippy allow attributes added.

```bash
cargo typify w3c-mnx/docs/mnx-schema.json
cp -p w3c-mnx/docs/mnx-schema.rs src/mnx.rs
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

## Parse Failure of MNX tuplets

- `typify` still ignores "unevaluatedProperties" as of early 2026

This is with respect to **musical notation** of
[tuplets](https://w3c.github.io/mnx/docs/mnx-reference/examples/tuplets/),
which indicates that the interior sequence of notes are to be performed
within the duration of the outer regular notes for the specified time
signature.

e.g., Perform three Eighth notes within the span of a single Quarter note.

Expected versus observed:

- Their [example
JSON](https://github.com/w3c/mnx/blob/main/docs/static/examples/json/tuplets.json)
  fails to parse into a `Tuplet` struct when using
[their schema](https://github.com/w3c/mnx/blob/main/docs/mnx-schema.json)
- Walking the chain of structs and enums from [Root](doc/mnx/mnx/struct.Root.html)
  ([via GitHubRAW](https://githubraw.com/dpezely/mnx-canary/dpezely/parse-failure-of-mnx-tuplets/doc/mnx_canary/mnx/struct.Root.html)),
  the logical flow indicates
  [SequenceContentItem](doc/mnx/mnx/struct.SequenceContentItem.html) `::tuple`
  ([via GitHubRAW](https://githubraw.com/dpezely/mnx-canary/dpezely/parse-failure-of-mnx-tuplets/doc/mnx_canary/mnx/struct.SequenceContentItem.html))
  should be populated but remains `None`
- Instead, `SequenceContentItem::event` when unwrapped reveals its inner
  [Event](doc/mnx/mnx/struct.Event.html)
  `::type_` contains `Some(LiteralStringEvent("tuplet"))`
  ([via GitHubRAW](https://githubraw.com/dpezely/mnx-canary/dpezely/parse-failure-of-mnx-tuplets/doc/mnx_canary/mnx/struct.Event.html))

Ultimately, this is a deficiency within `typify` itself due to
[not yet implementing "unevaluatedProperties"](https://github.com/oxidecomputer/typify/issues/579#:~:text=It%27s%20missing%20a%20number%20of%20keywords%20from%20both%20such%20as%20contentEncoding%20and%20dependencies%20from%20Draft%207%20and%20dependentSchemas%20and%20unevaluatedProperties%20from%202019%2D09%2E)
per [draft 2019-09](https://json-schema.org/draft/2019-09/draft-handrews-json-schema-02#unevaluatedProperties),
yet `mnx-schema.json` requires it and
[specifies newer
draft](https://github.com/search?q=repo%3Aw3c/mnx%20%22%24schema%22&type=code).

One possible alternative would have been `mnx-schema.json` to specify
`"required"` for `"event"` and to include `"type"` within there.  That
combination would have disambiguated `event` from `tuplet`.

However, the MNX schema uses `event` as a JSON Object with minimal
attributes that _also behaves as a catch-all_.  That in turn keeps each JSON
instance more streamlined for the most common case _within sheet music_, the
sequence of musical notes providing pitch, octave, etc.  It results in a
more compact JSON object and an overall smaller file than the alternative.

Hence, this is probably why the W3C **Music Notation** Working Group members
opted for `"unevaluatedProperties"`.
