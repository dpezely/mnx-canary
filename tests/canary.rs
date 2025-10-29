//! Proverbial canary in the coal mine.
//!
//! See <https://w3c.github.io/mnx/docs/> for "Raw JSON Schema"
//! link to <https://w3c.github.io/mnx/docs/mnx-schema.json>
//!
//! Structural changes to the schema should manifest as compile errors,
//! and material changes to an example will be found by running tests.
//!
//! These tests must be passing whenever W3C updates their repo.
//! Otherwise, keep with their prior known-working revision's commit.

#[cfg(test)]
mod test {
    extern crate mnx_canary;

    use std::fs::read_to_string;
    use std::path::Path;

    use mnx_canary::mnx::*;

    /// Version number extracted from an MNX file.
    ///
    /// ```bash
    /// jq .mnx.version hello-world.json
    /// ```
    const MNX_VERSION: i64 = 1;

    /// Path to downloaded JSON files' subdirectory within local copy
    /// of W3C's repo.
    ///
    /// Fetch their example JSON examples from:
    /// <https://github.com/w3c/mnx/tree/main/docs/static/examples/json>
    const JSON_PATH: &str = "w3c-mnx/docs/static/examples/json";

    /// Canonical "hello, world" example JSON file name from W3C's repo.
    const HELLO_WORLD_JSON: &str = "hello-world.json";

    /// Constructor.
    fn from_file(file_name: &str) -> MnxDocument {
        let file_path = Path::new(JSON_PATH).join(file_name);
        eprintln!("Processing: {file_path:?}");
        let json = read_to_string(file_path).expect("Read existing JSON file");
        serde_json::from_str::<MnxDocument>(&json)
            .expect("Deserialize JSON to struct derived from W3C schema")
    }

    /// Confirm "Hello, world" example by exercising all observable aspects.
    ///
    /// This example is a snippet, thus incomplete as a score.
    #[test]
    fn test_hello_world_example() {
        let doc = from_file(HELLO_WORLD_JSON);

        // BEWARE: expect lots of variable name reuse after unpacking via
        // .unwrap() or destructuring-bind.

        let MnxDocument(Root { mnx, global, parts, .. }) = doc;

        let Mnx { version: VersionNumber(v), .. } = mnx;
        assert_eq!(MNX_VERSION, v);

        assert_eq!(1, global.measures.len());

        let first = global.measures.first();
        assert!(first.is_some());
        let measure = first.unwrap();
        assert!(measure.barline.is_some());
        assert_eq!(BarlineType::Regular, measure.barline.as_ref().unwrap().type_);

        assert!(measure.time.is_some());
        let Some(Time { count: PositiveInteger(c), unit, .. }) = &measure.time else { panic!() };
        assert_eq!(4, *c);
        // Typify's newtype struct wrappers have `impl std::ops::Deref`,
        // so in many situations `.into()` can be elided:
        assert_eq!(4, **unit);

        let Parts(parts) = parts;
        assert_eq!(1, parts.len());
        let Part { id, kit, measures, name, short_name, staves, transposition, .. } = &parts[0];
        assert!(measures.is_some());
        assert!(id.is_none());
        assert!(kit.is_none());
        assert!(name.is_none());
        assert!(short_name.is_none());
        assert!(staves.is_none());
        assert!(transposition.is_none());

        let Some(PartMeasures(measures)) = measures else { panic!() };
        let PartMeasure { clefs, sequences, .. } = &measures[0];
        assert!(clefs.is_some());

        let Some(PositionedClefList(clefs)) = clefs else { panic!() };
        assert_eq!(1, clefs.len());

        let SequenceList(sequence) = sequences;
        let Sequence { content, id, staff, .. } = &sequence[0];
        assert!(id.is_none());
        assert!(staff.is_none());

        let SequenceContent(sequence) = content;
        assert_eq!(1, sequence.len());
        // TODO requires subtype naming patch for `cargo typify` if
        // rustc complains that these fields are named subtype_0, _1, etc.
        let SequenceContentItem { event, grace, tuplet, space, multi_note_tremolo } = &sequence[0];
        assert!(event.is_some());
        assert!(grace.is_none());
        assert!(tuplet.is_none());
        assert!(space.is_none());
        assert!(multi_note_tremolo.is_none());

        let Some(Event { duration, notes, rest, .. }) = &event else { panic!() };
        assert!(duration.is_some());
        assert!(notes.is_some());
        assert!(rest.is_none());

        let Some(NoteValue { base, dots, id, .. }) = &duration else { panic!() };
        assert_eq!(&NoteValueBase::Whole, base);
        assert!(dots.is_none());
        assert!(id.is_none());

        let Some(Notes(notes)) = notes else { panic!() };
        assert_eq!(1, notes.len());
        let Note { pitch, .. } = &notes[0];
        assert_eq!(Step::C, pitch.step);
        let Octave(octave) = pitch.octave;
        assert_eq!(4, octave);

        let last = global.measures.last();
        assert!(last.is_some());
        let measure = last.unwrap();
        assert!(measure.barline.is_some());
        assert_eq!(BarlineType::Regular, measure.barline.as_ref().unwrap().type_);
    }
}
