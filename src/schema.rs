#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`Event`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"$ref\": \"#/$defs/notes\""]
#[doc = "    },"]
#[doc = "    \"rest\": {"]
#[doc = "      \"$ref\": \"#/$defs/rest\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Event {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: ::std::option::Option<NoteValue>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<Notes>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rest: ::std::option::Option<Rest>,
}
impl ::std::convert::From<&Event> for Event {
    fn from(value: &Event) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Event {
    fn default() -> Self {
        Self {
            c: Default::default(),
            duration: Default::default(),
            id: Default::default(),
            notes: Default::default(),
            rest: Default::default(),
        }
    }
}
impl Event {
    pub fn builder() -> builder::Event {
        Default::default()
    }
}
#[doc = "`GlobalAttrs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_c\": {"]
#[doc = "      \"$ref\": \"#/$defs/string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GlobalAttrs {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
}
impl ::std::convert::From<&GlobalAttrs> for GlobalAttrs {
    fn from(value: &GlobalAttrs) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for GlobalAttrs {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
        }
    }
}
impl GlobalAttrs {
    pub fn builder() -> builder::GlobalAttrs {
        Default::default()
    }
}
#[doc = "`Id`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct Id(pub ::std::string::String);
impl ::std::ops::Deref for Id {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Id> for ::std::string::String {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Id> for Id {
    fn from(value: &Id) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Id {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Id {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Id {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Note`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"pitch\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pitch\": {"]
#[doc = "      \"$ref\": \"#/$defs/pitch\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Note {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub pitch: Pitch,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<Staff>,
}
impl ::std::convert::From<&Note> for Note {
    fn from(value: &Note) -> Self {
        value.clone()
    }
}
impl Note {
    pub fn builder() -> builder::Note {
        Default::default()
    }
}
#[doc = "`NoteValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"base\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"base\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value-base\""]
#[doc = "    },"]
#[doc = "    \"dots\": {"]
#[doc = "      \"$ref\": \"#/$defs/positive-integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NoteValue {
    pub base: NoteValueBase,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dots: ::std::option::Option<PositiveInteger>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
}
impl ::std::convert::From<&NoteValue> for NoteValue {
    fn from(value: &NoteValue) -> Self {
        value.clone()
    }
}
impl NoteValue {
    pub fn builder() -> builder::NoteValue {
        Default::default()
    }
}
#[doc = "`NoteValueBase`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"whole\","]
#[doc = "    \"half\","]
#[doc = "    \"quarter\","]
#[doc = "    \"eighth\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum NoteValueBase {
    #[serde(rename = "whole")]
    Whole,
    #[serde(rename = "half")]
    Half,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "eighth")]
    Eighth,
}
impl ::std::convert::From<&Self> for NoteValueBase {
    fn from(value: &NoteValueBase) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for NoteValueBase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Whole => f.write_str("whole"),
            Self::Half => f.write_str("half"),
            Self::Quarter => f.write_str("quarter"),
            Self::Eighth => f.write_str("eighth"),
        }
    }
}
impl ::std::str::FromStr for NoteValueBase {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "whole" => Ok(Self::Whole),
            "half" => Ok(Self::Half),
            "quarter" => Ok(Self::Quarter),
            "eighth" => Ok(Self::Eighth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NoteValueBase {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NoteValueBase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NoteValueBase {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`NoteValueQuantity`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"duration\","]
#[doc = "    \"multiple\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value\""]
#[doc = "    },"]
#[doc = "    \"multiple\": {"]
#[doc = "      \"$ref\": \"#/$defs/positive-integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NoteValueQuantity {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub duration: NoteValue,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub multiple: PositiveInteger,
}
impl ::std::convert::From<&NoteValueQuantity> for NoteValueQuantity {
    fn from(value: &NoteValueQuantity) -> Self {
        value.clone()
    }
}
impl NoteValueQuantity {
    pub fn builder() -> builder::NoteValueQuantity {
        Default::default()
    }
}
#[doc = "`Notes`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/note\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Notes(pub ::std::vec::Vec<Note>);
impl ::std::ops::Deref for Notes {
    type Target = ::std::vec::Vec<Note>;
    fn deref(&self) -> &::std::vec::Vec<Note> {
        &self.0
    }
}
impl ::std::convert::From<Notes> for ::std::vec::Vec<Note> {
    fn from(value: Notes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Notes> for Notes {
    fn from(value: &Notes) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Note>> for Notes {
    fn from(value: ::std::vec::Vec<Note>) -> Self {
        Self(value)
    }
}
#[doc = "`Octave`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Octave(pub i64);
impl ::std::ops::Deref for Octave {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Octave> for i64 {
    fn from(value: Octave) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Octave> for Octave {
    fn from(value: &Octave) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for Octave {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Octave {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Octave {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Octave {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Octave {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Octave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Pitch`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"octave\","]
#[doc = "    \"step\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"octave\": {"]
#[doc = "      \"$ref\": \"#/$defs/octave\""]
#[doc = "    },"]
#[doc = "    \"step\": {"]
#[doc = "      \"$ref\": \"#/$defs/step\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Pitch {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub octave: Octave,
    pub step: Step,
}
impl ::std::convert::From<&Pitch> for Pitch {
    fn from(value: &Pitch) -> Self {
        value.clone()
    }
}
impl Pitch {
    pub fn builder() -> builder::Pitch {
        Default::default()
    }
}
#[doc = "`PositiveInteger`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PositiveInteger(pub i64);
impl ::std::ops::Deref for PositiveInteger {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<PositiveInteger> for i64 {
    fn from(value: PositiveInteger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PositiveInteger> for PositiveInteger {
    fn from(value: &PositiveInteger) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for PositiveInteger {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PositiveInteger {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PositiveInteger {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for PositiveInteger {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PositiveInteger {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for PositiveInteger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Rest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"staffPosition\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Rest {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "staffPosition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub staff_position: ::std::option::Option<Staff>,
}
impl ::std::convert::From<&Rest> for Rest {
    fn from(value: &Rest) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Rest {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            staff_position: Default::default(),
        }
    }
}
impl Rest {
    pub fn builder() -> builder::Rest {
        Default::default()
    }
}
#[doc = "`Sequence`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"content\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/$defs/sequence-content\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Sequence {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub content: SequenceContent,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<Staff>,
}
impl ::std::convert::From<&Sequence> for Sequence {
    fn from(value: &Sequence) -> Self {
        value.clone()
    }
}
impl Sequence {
    pub fn builder() -> builder::Sequence {
        Default::default()
    }
}
#[doc = "`SequenceContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"anyOf\": ["]
#[doc = "      {"]
#[doc = "        \"$ref\": \"#/$defs/event\""]
#[doc = "      },"]
#[doc = "      {"]
#[doc = "        \"$ref\": \"#/$defs/tuplet\""]
#[doc = "      }"]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SequenceContent(pub ::std::vec::Vec<SequenceContentItem>);
impl ::std::ops::Deref for SequenceContent {
    type Target = ::std::vec::Vec<SequenceContentItem>;
    fn deref(&self) -> &::std::vec::Vec<SequenceContentItem> {
        &self.0
    }
}
impl ::std::convert::From<SequenceContent> for ::std::vec::Vec<SequenceContentItem> {
    fn from(value: SequenceContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SequenceContent> for SequenceContent {
    fn from(value: &SequenceContent) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<SequenceContentItem>> for SequenceContent {
    fn from(value: ::std::vec::Vec<SequenceContentItem>) -> Self {
        Self(value)
    }
}
#[doc = "`SequenceContentItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/event\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/tuplet\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SequenceContentItem {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub event: ::std::option::Option<Event>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tuplet: ::std::option::Option<Tuplet>,
}
impl ::std::convert::From<&SequenceContentItem> for SequenceContentItem {
    fn from(value: &SequenceContentItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SequenceContentItem {
    fn default() -> Self {
        Self {
            event: Default::default(),
            tuplet: Default::default(),
        }
    }
}
impl SequenceContentItem {
    pub fn builder() -> builder::SequenceContentItem {
        Default::default()
    }
}
#[doc = "`SequenceList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/sequence\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SequenceList(pub ::std::vec::Vec<Sequence>);
impl ::std::ops::Deref for SequenceList {
    type Target = ::std::vec::Vec<Sequence>;
    fn deref(&self) -> &::std::vec::Vec<Sequence> {
        &self.0
    }
}
impl ::std::convert::From<SequenceList> for ::std::vec::Vec<Sequence> {
    fn from(value: SequenceList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SequenceList> for SequenceList {
    fn from(value: &SequenceList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<Sequence>> for SequenceList {
    fn from(value: ::std::vec::Vec<Sequence>) -> Self {
        Self(value)
    }
}
#[doc = "`Staff`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Staff(pub i64);
impl ::std::ops::Deref for Staff {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Staff> for i64 {
    fn from(value: Staff) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Staff> for Staff {
    fn from(value: &Staff) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for Staff {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Staff {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Staff {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Staff {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Staff {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Staff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Step`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"A\","]
#[doc = "    \"B\","]
#[doc = "    \"C\","]
#[doc = "    \"D\","]
#[doc = "    \"E\","]
#[doc = "    \"F\","]
#[doc = "    \"G\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Step {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
impl ::std::convert::From<&Self> for Step {
    fn from(value: &Step) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Step {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => f.write_str("A"),
            Self::B => f.write_str("B"),
            Self::C => f.write_str("C"),
            Self::D => f.write_str("D"),
            Self::E => f.write_str("E"),
            Self::F => f.write_str("F"),
            Self::G => f.write_str("G"),
        }
    }
}
impl ::std::str::FromStr for Step {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Step {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Step {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Step {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`String`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct String(pub ::std::string::String);
impl ::std::ops::Deref for String {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<String> for ::std::string::String {
    fn from(value: String) -> Self {
        value.0
    }
}
impl ::std::convert::From<&String> for String {
    fn from(value: &String) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for String {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for String {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for String {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "Generated .rs code should be free of `subtype_0` field names"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://w3c.github.io/mnx/docs/mnx-schema.json\","]
#[doc = "  \"title\": \"test\","]
#[doc = "  \"description\": \"Generated .rs code should be free of `subtype_0` field names\","]
#[doc = "  \"$ref\": \"#/$defs/sequence\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Test(pub Sequence);
impl ::std::ops::Deref for Test {
    type Target = Sequence;
    fn deref(&self) -> &Sequence {
        &self.0
    }
}
impl ::std::convert::From<Test> for Sequence {
    fn from(value: Test) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Test> for Test {
    fn from(value: &Test) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Sequence> for Test {
    fn from(value: Sequence) -> Self {
        Self(value)
    }
}
#[doc = "`Tuplet`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/global-attrs\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"inner\","]
#[doc = "    \"outer\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/$defs/sequence-content\""]
#[doc = "    },"]
#[doc = "    \"inner\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value-quantity\""]
#[doc = "    },"]
#[doc = "    \"outer\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value-quantity\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tuplet {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub content: SequenceContent,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub inner: NoteValueQuantity,
    pub outer: NoteValueQuantity,
}
impl ::std::convert::From<&Tuplet> for Tuplet {
    fn from(value: &Tuplet) -> Self {
        value.clone()
    }
}
impl Tuplet {
    pub fn builder() -> builder::Tuplet {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Event {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        duration:
            ::std::result::Result<::std::option::Option<super::NoteValue>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        notes: ::std::result::Result<::std::option::Option<super::Notes>, ::std::string::String>,
        rest: ::std::result::Result<::std::option::Option<super::Rest>, ::std::string::String>,
    }
    impl ::std::default::Default for Event {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                duration: Ok(Default::default()),
                id: Ok(Default::default()),
                notes: Ok(Default::default()),
                rest: Ok(Default::default()),
            }
        }
    }
    impl Event {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NoteValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Notes>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {}", e));
            self
        }
        pub fn rest<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Rest>>,
            T::Error: ::std::fmt::Display,
        {
            self.rest = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rest: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Event> for super::Event {
        type Error = super::error::ConversionError;
        fn try_from(value: Event) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                duration: value.duration?,
                id: value.id?,
                notes: value.notes?,
                rest: value.rest?,
            })
        }
    }
    impl ::std::convert::From<super::Event> for Event {
        fn from(value: super::Event) -> Self {
            Self {
                c: Ok(value.c),
                duration: Ok(value.duration),
                id: Ok(value.id),
                notes: Ok(value.notes),
                rest: Ok(value.rest),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GlobalAttrs {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
    }
    impl ::std::default::Default for GlobalAttrs {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl GlobalAttrs {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<GlobalAttrs> for super::GlobalAttrs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GlobalAttrs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::GlobalAttrs> for GlobalAttrs {
        fn from(value: super::GlobalAttrs) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Note {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        pitch: ::std::result::Result<super::Pitch, ::std::string::String>,
        staff: ::std::result::Result<::std::option::Option<super::Staff>, ::std::string::String>,
    }
    impl ::std::default::Default for Note {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                pitch: Err("no value supplied for pitch".to_string()),
                staff: Ok(Default::default()),
            }
        }
    }
    impl Note {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn pitch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Pitch>,
            T::Error: ::std::fmt::Display,
        {
            self.pitch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pitch: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Staff>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Note> for super::Note {
        type Error = super::error::ConversionError;
        fn try_from(value: Note) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                pitch: value.pitch?,
                staff: value.staff?,
            })
        }
    }
    impl ::std::convert::From<super::Note> for Note {
        fn from(value: super::Note) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                pitch: Ok(value.pitch),
                staff: Ok(value.staff),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NoteValue {
        base: ::std::result::Result<super::NoteValueBase, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        dots: ::std::result::Result<
            ::std::option::Option<super::PositiveInteger>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
    }
    impl ::std::default::Default for NoteValue {
        fn default() -> Self {
            Self {
                base: Err("no value supplied for base".to_string()),
                c: Ok(Default::default()),
                dots: Ok(Default::default()),
                id: Ok(Default::default()),
            }
        }
    }
    impl NoteValue {
        pub fn base<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueBase>,
            T::Error: ::std::fmt::Display,
        {
            self.base = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for base: {}", e));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn dots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveInteger>>,
            T::Error: ::std::fmt::Display,
        {
            self.dots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dots: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NoteValue> for super::NoteValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NoteValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                base: value.base?,
                c: value.c?,
                dots: value.dots?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::NoteValue> for NoteValue {
        fn from(value: super::NoteValue) -> Self {
            Self {
                base: Ok(value.base),
                c: Ok(value.c),
                dots: Ok(value.dots),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NoteValueQuantity {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        duration: ::std::result::Result<super::NoteValue, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        multiple: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
    }
    impl ::std::default::Default for NoteValueQuantity {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                duration: Err("no value supplied for duration".to_string()),
                id: Ok(Default::default()),
                multiple: Err("no value supplied for multiple".to_string()),
            }
        }
    }
    impl NoteValueQuantity {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValue>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn multiple<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.multiple = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for multiple: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NoteValueQuantity> for super::NoteValueQuantity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NoteValueQuantity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                duration: value.duration?,
                id: value.id?,
                multiple: value.multiple?,
            })
        }
    }
    impl ::std::convert::From<super::NoteValueQuantity> for NoteValueQuantity {
        fn from(value: super::NoteValueQuantity) -> Self {
            Self {
                c: Ok(value.c),
                duration: Ok(value.duration),
                id: Ok(value.id),
                multiple: Ok(value.multiple),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Pitch {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        octave: ::std::result::Result<super::Octave, ::std::string::String>,
        step: ::std::result::Result<super::Step, ::std::string::String>,
    }
    impl ::std::default::Default for Pitch {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                octave: Err("no value supplied for octave".to_string()),
                step: Err("no value supplied for step".to_string()),
            }
        }
    }
    impl Pitch {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn octave<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Octave>,
            T::Error: ::std::fmt::Display,
        {
            self.octave = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for octave: {}", e));
            self
        }
        pub fn step<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Step>,
            T::Error: ::std::fmt::Display,
        {
            self.step = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Pitch> for super::Pitch {
        type Error = super::error::ConversionError;
        fn try_from(value: Pitch) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                octave: value.octave?,
                step: value.step?,
            })
        }
    }
    impl ::std::convert::From<super::Pitch> for Pitch {
        fn from(value: super::Pitch) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                octave: Ok(value.octave),
                step: Ok(value.step),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Rest {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        staff_position:
            ::std::result::Result<::std::option::Option<super::Staff>, ::std::string::String>,
    }
    impl ::std::default::Default for Rest {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                staff_position: Ok(Default::default()),
            }
        }
    }
    impl Rest {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn staff_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Staff>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_position: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Rest> for super::Rest {
        type Error = super::error::ConversionError;
        fn try_from(value: Rest) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                staff_position: value.staff_position?,
            })
        }
    }
    impl ::std::convert::From<super::Rest> for Rest {
        fn from(value: super::Rest) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                staff_position: Ok(value.staff_position),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Sequence {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        content: ::std::result::Result<super::SequenceContent, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        staff: ::std::result::Result<::std::option::Option<super::Staff>, ::std::string::String>,
    }
    impl ::std::default::Default for Sequence {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                staff: Ok(Default::default()),
            }
        }
    }
    impl Sequence {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Staff>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Sequence> for super::Sequence {
        type Error = super::error::ConversionError;
        fn try_from(value: Sequence) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                content: value.content?,
                id: value.id?,
                staff: value.staff?,
            })
        }
    }
    impl ::std::convert::From<super::Sequence> for Sequence {
        fn from(value: super::Sequence) -> Self {
            Self {
                c: Ok(value.c),
                content: Ok(value.content),
                id: Ok(value.id),
                staff: Ok(value.staff),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SequenceContentItem {
        event: ::std::result::Result<::std::option::Option<super::Event>, ::std::string::String>,
        tuplet: ::std::result::Result<::std::option::Option<super::Tuplet>, ::std::string::String>,
    }
    impl ::std::default::Default for SequenceContentItem {
        fn default() -> Self {
            Self {
                event: Ok(Default::default()),
                tuplet: Ok(Default::default()),
            }
        }
    }
    impl SequenceContentItem {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Event>>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {}", e));
            self
        }
        pub fn tuplet<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tuplet>>,
            T::Error: ::std::fmt::Display,
        {
            self.tuplet = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tuplet: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SequenceContentItem> for super::SequenceContentItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SequenceContentItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                tuplet: value.tuplet?,
            })
        }
    }
    impl ::std::convert::From<super::SequenceContentItem> for SequenceContentItem {
        fn from(value: super::SequenceContentItem) -> Self {
            Self {
                event: Ok(value.event),
                tuplet: Ok(value.tuplet),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tuplet {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        content: ::std::result::Result<super::SequenceContent, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        inner: ::std::result::Result<super::NoteValueQuantity, ::std::string::String>,
        outer: ::std::result::Result<super::NoteValueQuantity, ::std::string::String>,
    }
    impl ::std::default::Default for Tuplet {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                inner: Err("no value supplied for inner".to_string()),
                outer: Err("no value supplied for outer".to_string()),
            }
        }
    }
    impl Tuplet {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {}", e));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn inner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueQuantity>,
            T::Error: ::std::fmt::Display,
        {
            self.inner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inner: {}", e));
            self
        }
        pub fn outer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueQuantity>,
            T::Error: ::std::fmt::Display,
        {
            self.outer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outer: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Tuplet> for super::Tuplet {
        type Error = super::error::ConversionError;
        fn try_from(value: Tuplet) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                content: value.content?,
                id: value.id?,
                inner: value.inner?,
                outer: value.outer?,
            })
        }
    }
    impl ::std::convert::From<super::Tuplet> for Tuplet {
        fn from(value: super::Tuplet) -> Self {
            Self {
                c: Ok(value.c),
                content: Ok(value.content),
                id: Ok(value.id),
                inner: Ok(value.inner),
                outer: Ok(value.outer),
            }
        }
    }
}
