//! "MNX is a new, open standard for representing music notation for
//! interchange and internal use in software applications. It builds
//! on the ideas and success of MusicXML \[...]"
//!
//! This **code has been generated**, thus modifications will be ignored.
//!
//! Official documentation: <https://w3c.github.io/mnx/docs/>
//!
//! Official schema: <https://w3c.github.io/mnx/docs/mnx-schema.json>
//!
//! Run: cargo install cargo-typify && cargo typify mnx-schema.json
#![allow(unused)]
#![allow(clippy::derivable_impls)]
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
#[doc = "`Accent`"]
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
#[doc = "    \"pointing\": {"]
#[doc = "      \"$ref\": \"#/$defs/up-or-down\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Accent {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pointing: ::std::option::Option<UpOrDown>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Accent {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            pointing: Default::default(),
            x: Default::default(),
        }
    }
}
impl Accent {
    pub fn builder() -> builder::Accent {
        Default::default()
    }
}
#[doc = "`AccidentalDisplay`"]
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
#[doc = "    \"show\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"enclosure\": {"]
#[doc = "      \"$ref\": \"#/$defs/accidental-enclosure\""]
#[doc = "    },"]
#[doc = "    \"force\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"show\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccidentalDisplay {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enclosure: ::std::option::Option<AccidentalEnclosure>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub show: bool,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl AccidentalDisplay {
    pub fn builder() -> builder::AccidentalDisplay {
        Default::default()
    }
}
#[doc = "`AccidentalEnclosure`"]
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
#[doc = "    \"symbol\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"$ref\": \"#/$defs/accidental-enclosure-symbol\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AccidentalEnclosure {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub symbol: AccidentalEnclosureSymbol,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl AccidentalEnclosure {
    pub fn builder() -> builder::AccidentalEnclosure {
        Default::default()
    }
}
#[doc = "`AccidentalEnclosureSymbol`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"parentheses\","]
#[doc = "    \"brackets\""]
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
pub enum AccidentalEnclosureSymbol {
    #[serde(rename = "parentheses")]
    Parentheses,
    #[serde(rename = "brackets")]
    Brackets,
}
impl ::std::fmt::Display for AccidentalEnclosureSymbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Parentheses => f.write_str("parentheses"),
            Self::Brackets => f.write_str("brackets"),
        }
    }
}
impl ::std::str::FromStr for AccidentalEnclosureSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "parentheses" => Ok(Self::Parentheses),
            "brackets" => Ok(Self::Brackets),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccidentalEnclosureSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccidentalEnclosureSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccidentalEnclosureSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Alter`"]
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
pub struct Alter(pub i64);
impl ::std::ops::Deref for Alter {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Alter> for i64 {
    fn from(value: Alter) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for Alter {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Alter {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Alter {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Alter {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Alter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Barline`"]
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
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/barline-type\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Barline {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(rename = "type")]
    pub type_: BarlineType,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Barline {
    pub fn builder() -> builder::Barline {
        Default::default()
    }
}
#[doc = "`BarlineType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"regular\","]
#[doc = "    \"dotted\","]
#[doc = "    \"dashed\","]
#[doc = "    \"heavy\","]
#[doc = "    \"double\","]
#[doc = "    \"final\","]
#[doc = "    \"heavyLight\","]
#[doc = "    \"heavyHeavy\","]
#[doc = "    \"tick\","]
#[doc = "    \"short\","]
#[doc = "    \"noBarline\""]
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
pub enum BarlineType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "dotted")]
    Dotted,
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "heavy")]
    Heavy,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "final")]
    Final,
    #[serde(rename = "heavyLight")]
    HeavyLight,
    #[serde(rename = "heavyHeavy")]
    HeavyHeavy,
    #[serde(rename = "tick")]
    Tick,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "noBarline")]
    NoBarline,
}
impl ::std::fmt::Display for BarlineType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Regular => f.write_str("regular"),
            Self::Dotted => f.write_str("dotted"),
            Self::Dashed => f.write_str("dashed"),
            Self::Heavy => f.write_str("heavy"),
            Self::Double => f.write_str("double"),
            Self::Final => f.write_str("final"),
            Self::HeavyLight => f.write_str("heavyLight"),
            Self::HeavyHeavy => f.write_str("heavyHeavy"),
            Self::Tick => f.write_str("tick"),
            Self::Short => f.write_str("short"),
            Self::NoBarline => f.write_str("noBarline"),
        }
    }
}
impl ::std::str::FromStr for BarlineType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "regular" => Ok(Self::Regular),
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "heavy" => Ok(Self::Heavy),
            "double" => Ok(Self::Double),
            "final" => Ok(Self::Final),
            "heavyLight" => Ok(Self::HeavyLight),
            "heavyHeavy" => Ok(Self::HeavyHeavy),
            "tick" => Ok(Self::Tick),
            "short" => Ok(Self::Short),
            "noBarline" => Ok(Self::NoBarline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BarlineType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BarlineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BarlineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Beam`"]
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
#[doc = "    \"events\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"beams\": {"]
#[doc = "      \"$ref\": \"#/$defs/beam-list\""]
#[doc = "    },"]
#[doc = "    \"direction\": {"]
#[doc = "      \"$ref\": \"#/$defs/beam-hook-direction\""]
#[doc = "    },"]
#[doc = "    \"events\": {"]
#[doc = "      \"$ref\": \"#/$defs/id-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Beam {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub beams: ::std::option::Option<BeamList>,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub direction: ::std::option::Option<BeamHookDirection>,
    pub events: IdList,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Beam {
    pub fn builder() -> builder::Beam {
        Default::default()
    }
}
#[doc = "`BeamHookDirection`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"left\","]
#[doc = "    \"right\","]
#[doc = "    \"auto\""]
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
pub enum BeamHookDirection {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for BeamHookDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for BeamHookDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BeamHookDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BeamHookDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BeamHookDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`BeamList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/beam\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct BeamList(pub ::std::vec::Vec<Beam>);
impl ::std::ops::Deref for BeamList {
    type Target = ::std::vec::Vec<Beam>;
    fn deref(&self) -> &::std::vec::Vec<Beam> {
        &self.0
    }
}
impl ::std::convert::From<BeamList> for ::std::vec::Vec<Beam> {
    fn from(value: BeamList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Beam>> for BeamList {
    fn from(value: ::std::vec::Vec<Beam>) -> Self {
        Self(value)
    }
}
#[doc = "`Bpm`"]
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
pub struct Bpm(pub i64);
impl ::std::ops::Deref for Bpm {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Bpm> for i64 {
    fn from(value: Bpm) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for Bpm {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Bpm {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Bpm {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Bpm {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Bpm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`BreathMark`"]
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
#[doc = "    \"symbol\": {"]
#[doc = "      \"$ref\": \"#/$defs/breath-mark-symbol\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BreathMark {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub symbol: ::std::option::Option<BreathMarkSymbol>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for BreathMark {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            symbol: Default::default(),
            x: Default::default(),
        }
    }
}
impl BreathMark {
    pub fn builder() -> builder::BreathMark {
        Default::default()
    }
}
#[doc = "`BreathMarkSymbol`"]
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
pub struct BreathMarkSymbol(pub ::std::string::String);
impl ::std::ops::Deref for BreathMarkSymbol {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BreathMarkSymbol> for ::std::string::String {
    fn from(value: BreathMarkSymbol) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for BreathMarkSymbol {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for BreathMarkSymbol {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for BreathMarkSymbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Clef`"]
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
#[doc = "    \"sign\","]
#[doc = "    \"staffPosition\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"$ref\": \"#/$defs/simple-color\""]
#[doc = "    },"]
#[doc = "    \"glyph\": {"]
#[doc = "      \"$ref\": \"#/$defs/smufl-glyph\""]
#[doc = "    },"]
#[doc = "    \"octave\": {"]
#[doc = "      \"$ref\": \"#/$defs/ottava-amount-or-zero\""]
#[doc = "    },"]
#[doc = "    \"showOctave\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"sign\": {"]
#[doc = "      \"$ref\": \"#/$defs/clef-sign\""]
#[doc = "    },"]
#[doc = "    \"staffPosition\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-position\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Clef {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<SimpleColor>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub glyph: ::std::option::Option<SmuflGlyph>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub octave: ::std::option::Option<OttavaAmountOrZero>,
    #[serde(
        rename = "showOctave",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_octave: ::std::option::Option<bool>,
    pub sign: ClefSign,
    #[serde(rename = "staffPosition")]
    pub staff_position: StaffPosition,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Clef {
    pub fn builder() -> builder::Clef {
        Default::default()
    }
}
#[doc = "`ClefSign`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"C\","]
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
pub enum ClefSign {
    C,
    F,
    G,
}
impl ::std::fmt::Display for ClefSign {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::C => f.write_str("C"),
            Self::F => f.write_str("F"),
            Self::G => f.write_str("G"),
        }
    }
}
impl ::std::str::FromStr for ClefSign {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "C" => Ok(Self::C),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClefSign {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClefSign {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClefSign {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Color`"]
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
pub struct Color(pub ::std::string::String);
impl ::std::ops::Deref for Color {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Color> for ::std::string::String {
    fn from(value: Color) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for Color {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Color {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Dynamic`"]
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
#[doc = "    \"position\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"glyph\": {"]
#[doc = "      \"$ref\": \"#/$defs/smufl-glyph\""]
#[doc = "    },"]
#[doc = "    \"position\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/dynamic-type\""]
#[doc = "    },"]
#[doc = "    \"voice\": {"]
#[doc = "      \"$ref\": \"#/$defs/voice-name\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Dynamic {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub glyph: ::std::option::Option<SmuflGlyph>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub position: RhythmicPosition,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    pub value: DynamicType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Dynamic {
    pub fn builder() -> builder::Dynamic {
        Default::default()
    }
}
#[doc = "`DynamicList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/dynamic\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct DynamicList(pub ::std::vec::Vec<Dynamic>);
impl ::std::ops::Deref for DynamicList {
    type Target = ::std::vec::Vec<Dynamic>;
    fn deref(&self) -> &::std::vec::Vec<Dynamic> {
        &self.0
    }
}
impl ::std::convert::From<DynamicList> for ::std::vec::Vec<Dynamic> {
    fn from(value: DynamicList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Dynamic>> for DynamicList {
    fn from(value: ::std::vec::Vec<Dynamic>) -> Self {
        Self(value)
    }
}
#[doc = "`DynamicType`"]
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
pub struct DynamicType(pub ::std::string::String);
impl ::std::ops::Deref for DynamicType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DynamicType> for ::std::string::String {
    fn from(value: DynamicType) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for DynamicType {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for DynamicType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for DynamicType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Ending`"]
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
#[doc = "    \"duration\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"$ref\": \"#/$defs/color\""]
#[doc = "    },"]
#[doc = "    \"duration\": {"]
#[doc = "      \"$ref\": \"#/$defs/ending-duration\""]
#[doc = "    },"]
#[doc = "    \"numbers\": {"]
#[doc = "      \"$ref\": \"#/$defs/ending-numbers\""]
#[doc = "    },"]
#[doc = "    \"open\": {"]
#[doc = "      \"$ref\": \"#/$defs/ending-open\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Ending {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub duration: EndingDuration,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub numbers: ::std::option::Option<EndingNumbers>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub open: ::std::option::Option<EndingOpen>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Ending {
    pub fn builder() -> builder::Ending {
        Default::default()
    }
}
#[doc = "`EndingDuration`"]
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
pub struct EndingDuration(pub i64);
impl ::std::ops::Deref for EndingDuration {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<EndingDuration> for i64 {
    fn from(value: EndingDuration) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for EndingDuration {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for EndingDuration {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for EndingDuration {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EndingDuration {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for EndingDuration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`EndingNumber`"]
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
pub struct EndingNumber(pub i64);
impl ::std::ops::Deref for EndingNumber {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<EndingNumber> for i64 {
    fn from(value: EndingNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for EndingNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for EndingNumber {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for EndingNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EndingNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for EndingNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`EndingNumbers`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/ending-number\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EndingNumbers(pub ::std::vec::Vec<EndingNumber>);
impl ::std::ops::Deref for EndingNumbers {
    type Target = ::std::vec::Vec<EndingNumber>;
    fn deref(&self) -> &::std::vec::Vec<EndingNumber> {
        &self.0
    }
}
impl ::std::convert::From<EndingNumbers> for ::std::vec::Vec<EndingNumber> {
    fn from(value: EndingNumbers) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<EndingNumber>> for EndingNumbers {
    fn from(value: ::std::vec::Vec<EndingNumber>) -> Self {
        Self(value)
    }
}
#[doc = "`EndingOpen`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"boolean\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EndingOpen(pub bool);
impl ::std::ops::Deref for EndingOpen {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<EndingOpen> for bool {
    fn from(value: EndingOpen) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for EndingOpen {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for EndingOpen {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for EndingOpen {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EndingOpen {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for EndingOpen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
#[doc = "    \"kitNotes\": {"]
#[doc = "      \"$ref\": \"#/$defs/kit-notes\""]
#[doc = "    },"]
#[doc = "    \"lyrics\": {"]
#[doc = "      \"$ref\": \"#/$defs/lyrics\""]
#[doc = "    },"]
#[doc = "    \"markings\": {"]
#[doc = "      \"$ref\": \"#/$defs/event-markings\""]
#[doc = "    },"]
#[doc = "    \"measure\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"$ref\": \"#/$defs/notes\""]
#[doc = "    },"]
#[doc = "    \"orient\": {"]
#[doc = "      \"$ref\": \"#/$defs/orientation\""]
#[doc = "    },"]
#[doc = "    \"rest\": {"]
#[doc = "      \"$ref\": \"#/$defs/rest\""]
#[doc = "    },"]
#[doc = "    \"slurs\": {"]
#[doc = "      \"$ref\": \"#/$defs/slur-list\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"stemDirection\": {"]
#[doc = "      \"$ref\": \"#/$defs/stem-direction\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/literal-string-event\""]
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
    #[serde(
        rename = "kitNotes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub kit_notes: ::std::option::Option<KitNotes>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lyrics: ::std::option::Option<Lyrics>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub markings: ::std::option::Option<EventMarkings>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub measure: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<Notes>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orient: ::std::option::Option<Orientation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rest: ::std::option::Option<Rest>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub slurs: ::std::option::Option<SlurList>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(
        rename = "stemDirection",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stem_direction: ::std::option::Option<StemDirection>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LiteralStringEvent>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Event {
    fn default() -> Self {
        Self {
            c: Default::default(),
            duration: Default::default(),
            id: Default::default(),
            kit_notes: Default::default(),
            lyrics: Default::default(),
            markings: Default::default(),
            measure: Default::default(),
            notes: Default::default(),
            orient: Default::default(),
            rest: Default::default(),
            slurs: Default::default(),
            staff: Default::default(),
            stem_direction: Default::default(),
            type_: Default::default(),
            x: Default::default(),
        }
    }
}
impl Event {
    pub fn builder() -> builder::Event {
        Default::default()
    }
}
#[doc = "`EventLyricLine`"]
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
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"text\": {"]
#[doc = "      \"$ref\": \"#/$defs/string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/event-lyric-line-type\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EventLyricLine {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub text: String,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<EventLyricLineType>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl EventLyricLine {
    pub fn builder() -> builder::EventLyricLine {
        Default::default()
    }
}
#[doc = "`EventLyricLineType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"start\","]
#[doc = "    \"middle\","]
#[doc = "    \"end\","]
#[doc = "    \"whole\""]
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
pub enum EventLyricLineType {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "whole")]
    Whole,
}
impl ::std::fmt::Display for EventLyricLineType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Start => f.write_str("start"),
            Self::Middle => f.write_str("middle"),
            Self::End => f.write_str("end"),
            Self::Whole => f.write_str("whole"),
        }
    }
}
impl ::std::str::FromStr for EventLyricLineType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "start" => Ok(Self::Start),
            "middle" => Ok(Self::Middle),
            "end" => Ok(Self::End),
            "whole" => Ok(Self::Whole),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventLyricLineType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventLyricLineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventLyricLineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EventLyricLines`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \"^.*$\": {"]
#[doc = "      \"$ref\": \"#/$defs/event-lyric-line\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EventLyricLines(pub ::std::collections::HashMap<EventLyricLinesKey, EventLyricLine>);
impl ::std::ops::Deref for EventLyricLines {
    type Target = ::std::collections::HashMap<EventLyricLinesKey, EventLyricLine>;
    fn deref(&self) -> &::std::collections::HashMap<EventLyricLinesKey, EventLyricLine> {
        &self.0
    }
}
impl ::std::convert::From<EventLyricLines>
    for ::std::collections::HashMap<EventLyricLinesKey, EventLyricLine>
{
    fn from(value: EventLyricLines) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<EventLyricLinesKey, EventLyricLine>>
    for EventLyricLines
{
    fn from(value: ::std::collections::HashMap<EventLyricLinesKey, EventLyricLine>) -> Self {
        Self(value)
    }
}
#[doc = "`EventLyricLinesKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^.*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventLyricLinesKey(::std::string::String);
impl ::std::ops::Deref for EventLyricLinesKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventLyricLinesKey> for ::std::string::String {
    fn from(value: EventLyricLinesKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventLyricLinesKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^.*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^.*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventLyricLinesKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventLyricLinesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventLyricLinesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventLyricLinesKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`EventMarkings`"]
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
#[doc = "    \"accent\": {"]
#[doc = "      \"$ref\": \"#/$defs/accent\""]
#[doc = "    },"]
#[doc = "    \"breath\": {"]
#[doc = "      \"$ref\": \"#/$defs/breath-mark\""]
#[doc = "    },"]
#[doc = "    \"softAccent\": {"]
#[doc = "      \"$ref\": \"#/$defs/soft-accent\""]
#[doc = "    },"]
#[doc = "    \"spiccato\": {"]
#[doc = "      \"$ref\": \"#/$defs/spiccato\""]
#[doc = "    },"]
#[doc = "    \"staccatissimo\": {"]
#[doc = "      \"$ref\": \"#/$defs/staccatissimo\""]
#[doc = "    },"]
#[doc = "    \"staccato\": {"]
#[doc = "      \"$ref\": \"#/$defs/staccato\""]
#[doc = "    },"]
#[doc = "    \"stress\": {"]
#[doc = "      \"$ref\": \"#/$defs/stress-marking\""]
#[doc = "    },"]
#[doc = "    \"strongAccent\": {"]
#[doc = "      \"$ref\": \"#/$defs/strong-accent\""]
#[doc = "    },"]
#[doc = "    \"tenuto\": {"]
#[doc = "      \"$ref\": \"#/$defs/tenuto\""]
#[doc = "    },"]
#[doc = "    \"tremolo\": {"]
#[doc = "      \"$ref\": \"#/$defs/tremolo-single\""]
#[doc = "    },"]
#[doc = "    \"unstress\": {"]
#[doc = "      \"$ref\": \"#/$defs/unstress-marking\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EventMarkings {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub accent: ::std::option::Option<Accent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub breath: ::std::option::Option<BreathMark>,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "softAccent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub soft_accent: ::std::option::Option<SoftAccent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub spiccato: ::std::option::Option<Spiccato>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staccatissimo: ::std::option::Option<Staccatissimo>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staccato: ::std::option::Option<Staccato>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stress: ::std::option::Option<StressMarking>,
    #[serde(
        rename = "strongAccent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strong_accent: ::std::option::Option<StrongAccent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tenuto: ::std::option::Option<Tenuto>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tremolo: ::std::option::Option<TremoloSingle>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub unstress: ::std::option::Option<UnstressMarking>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for EventMarkings {
    fn default() -> Self {
        Self {
            accent: Default::default(),
            breath: Default::default(),
            c: Default::default(),
            id: Default::default(),
            soft_accent: Default::default(),
            spiccato: Default::default(),
            staccatissimo: Default::default(),
            staccato: Default::default(),
            stress: Default::default(),
            strong_accent: Default::default(),
            tenuto: Default::default(),
            tremolo: Default::default(),
            unstress: Default::default(),
            x: Default::default(),
        }
    }
}
impl EventMarkings {
    pub fn builder() -> builder::EventMarkings {
        Default::default()
    }
}
#[doc = "`Fifths`"]
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
pub struct Fifths(pub i64);
impl ::std::ops::Deref for Fifths {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<Fifths> for i64 {
    fn from(value: Fifths) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for Fifths {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Fifths {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Fifths {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Fifths {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Fifths {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Fine`"]
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
#[doc = "    \"location\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"$ref\": \"#/$defs/color\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Fine {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub location: RhythmicPosition,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Fine {
    pub fn builder() -> builder::Fine {
        Default::default()
    }
}
#[doc = "`Fraction`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/integer-unsigned\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Fraction(pub ::std::vec::Vec<IntegerUnsigned>);
impl ::std::ops::Deref for Fraction {
    type Target = ::std::vec::Vec<IntegerUnsigned>;
    fn deref(&self) -> &::std::vec::Vec<IntegerUnsigned> {
        &self.0
    }
}
impl ::std::convert::From<Fraction> for ::std::vec::Vec<IntegerUnsigned> {
    fn from(value: Fraction) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<IntegerUnsigned>> for Fraction {
    fn from(value: ::std::vec::Vec<IntegerUnsigned>) -> Self {
        Self(value)
    }
}
#[doc = "`Global`"]
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
#[doc = "    \"measures\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"lyrics\": {"]
#[doc = "      \"$ref\": \"#/$defs/lyrics-global\""]
#[doc = "    },"]
#[doc = "    \"measures\": {"]
#[doc = "      \"$ref\": \"#/$defs/measures-global\""]
#[doc = "    },"]
#[doc = "    \"sounds\": {"]
#[doc = "      \"$ref\": \"#/$defs/sounds-global\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Global {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lyrics: ::std::option::Option<LyricsGlobal>,
    pub measures: MeasuresGlobal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sounds: ::std::option::Option<SoundsGlobal>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Global {
    pub fn builder() -> builder::Global {
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
#[doc = "    \"_x\": {"]
#[doc = "      \"$ref\": \"#/$defs/vendor-extensions\""]
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
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for GlobalAttrs {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            x: Default::default(),
        }
    }
}
impl GlobalAttrs {
    pub fn builder() -> builder::GlobalAttrs {
        Default::default()
    }
}
#[doc = "`Grace`"]
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
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"$ref\": \"#/$defs/color\""]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/$defs/grace-sequence-content\""]
#[doc = "    },"]
#[doc = "    \"graceType\": {"]
#[doc = "      \"$ref\": \"#/$defs/grace-type\""]
#[doc = "    },"]
#[doc = "    \"slash\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/literal-string-grace\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Grace {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub content: GraceSequenceContent,
    #[serde(
        rename = "graceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grace_type: ::std::option::Option<GraceType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub slash: ::std::option::Option<bool>,
    #[serde(rename = "type")]
    pub type_: LiteralStringGrace,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Grace {
    pub fn builder() -> builder::Grace {
        Default::default()
    }
}
#[doc = "`GraceSequenceContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/event\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GraceSequenceContent(pub ::std::vec::Vec<Event>);
impl ::std::ops::Deref for GraceSequenceContent {
    type Target = ::std::vec::Vec<Event>;
    fn deref(&self) -> &::std::vec::Vec<Event> {
        &self.0
    }
}
impl ::std::convert::From<GraceSequenceContent> for ::std::vec::Vec<Event> {
    fn from(value: GraceSequenceContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Event>> for GraceSequenceContent {
    fn from(value: ::std::vec::Vec<Event>) -> Self {
        Self(value)
    }
}
#[doc = "`GraceType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"makeTime\","]
#[doc = "    \"stealFollowing\","]
#[doc = "    \"stealPrevious\""]
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
pub enum GraceType {
    #[serde(rename = "makeTime")]
    MakeTime,
    #[serde(rename = "stealFollowing")]
    StealFollowing,
    #[serde(rename = "stealPrevious")]
    StealPrevious,
}
impl ::std::fmt::Display for GraceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MakeTime => f.write_str("makeTime"),
            Self::StealFollowing => f.write_str("stealFollowing"),
            Self::StealPrevious => f.write_str("stealPrevious"),
        }
    }
}
impl ::std::str::FromStr for GraceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "makeTime" => Ok(Self::MakeTime),
            "stealFollowing" => Ok(Self::StealFollowing),
            "stealPrevious" => Ok(Self::StealPrevious),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GraceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GraceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GraceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "`IdList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/id\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct IdList(pub ::std::vec::Vec<Id>);
impl ::std::ops::Deref for IdList {
    type Target = ::std::vec::Vec<Id>;
    fn deref(&self) -> &::std::vec::Vec<Id> {
        &self.0
    }
}
impl ::std::convert::From<IdList> for ::std::vec::Vec<Id> {
    fn from(value: IdList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Id>> for IdList {
    fn from(value: ::std::vec::Vec<Id>) -> Self {
        Self(value)
    }
}
#[doc = "`IntegerSigned`"]
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
pub struct IntegerSigned(pub i64);
impl ::std::ops::Deref for IntegerSigned {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<IntegerSigned> for i64 {
    fn from(value: IntegerSigned) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for IntegerSigned {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for IntegerSigned {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for IntegerSigned {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for IntegerSigned {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for IntegerSigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`IntegerUnsigned`"]
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
pub struct IntegerUnsigned(pub i64);
impl ::std::ops::Deref for IntegerUnsigned {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<IntegerUnsigned> for i64 {
    fn from(value: IntegerUnsigned) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for IntegerUnsigned {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for IntegerUnsigned {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for IntegerUnsigned {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for IntegerUnsigned {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for IntegerUnsigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Interval`"]
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
#[doc = "    \"halfSteps\","]
#[doc = "    \"staffDistance\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"halfSteps\": {"]
#[doc = "      \"$ref\": \"#/$defs/integer-signed\""]
#[doc = "    },"]
#[doc = "    \"staffDistance\": {"]
#[doc = "      \"$ref\": \"#/$defs/integer-signed\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Interval {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(rename = "halfSteps")]
    pub half_steps: IntegerSigned,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(rename = "staffDistance")]
    pub staff_distance: IntegerSigned,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Interval {
    pub fn builder() -> builder::Interval {
        Default::default()
    }
}
#[doc = "`Jump`"]
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
#[doc = "    \"location\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"location\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/jump-type\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Jump {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub location: RhythmicPosition,
    #[serde(rename = "type")]
    pub type_: JumpType,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Jump {
    pub fn builder() -> builder::Jump {
        Default::default()
    }
}
#[doc = "`JumpType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"dsalfine\","]
#[doc = "    \"segno\""]
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
pub enum JumpType {
    #[serde(rename = "dsalfine")]
    Dsalfine,
    #[serde(rename = "segno")]
    Segno,
}
impl ::std::fmt::Display for JumpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dsalfine => f.write_str("dsalfine"),
            Self::Segno => f.write_str("segno"),
        }
    }
}
impl ::std::str::FromStr for JumpType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "dsalfine" => Ok(Self::Dsalfine),
            "segno" => Ok(Self::Segno),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JumpType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JumpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JumpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Key`"]
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
#[doc = "    \"fifths\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"$ref\": \"#/$defs/color\""]
#[doc = "    },"]
#[doc = "    \"fifths\": {"]
#[doc = "      \"$ref\": \"#/$defs/fifths\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Key {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub fifths: Fifths,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Key {
    pub fn builder() -> builder::Key {
        Default::default()
    }
}
#[doc = "`Kit`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \"^.*$\": {"]
#[doc = "      \"$ref\": \"#/$defs/kit-component\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Kit(pub ::std::collections::HashMap<KitKey, KitComponent>);
impl ::std::ops::Deref for Kit {
    type Target = ::std::collections::HashMap<KitKey, KitComponent>;
    fn deref(&self) -> &::std::collections::HashMap<KitKey, KitComponent> {
        &self.0
    }
}
impl ::std::convert::From<Kit> for ::std::collections::HashMap<KitKey, KitComponent> {
    fn from(value: Kit) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<KitKey, KitComponent>> for Kit {
    fn from(value: ::std::collections::HashMap<KitKey, KitComponent>) -> Self {
        Self(value)
    }
}
#[doc = "`KitComponent`"]
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
#[doc = "    \"staffPosition\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/string\""]
#[doc = "    },"]
#[doc = "    \"sound\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"staffPosition\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-position\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct KitComponent {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sound: ::std::option::Option<Id>,
    #[serde(rename = "staffPosition")]
    pub staff_position: StaffPosition,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl KitComponent {
    pub fn builder() -> builder::KitComponent {
        Default::default()
    }
}
#[doc = "`KitComponentId`"]
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
pub struct KitComponentId(pub ::std::string::String);
impl ::std::ops::Deref for KitComponentId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<KitComponentId> for ::std::string::String {
    fn from(value: KitComponentId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for KitComponentId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for KitComponentId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for KitComponentId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`KitKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^.*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct KitKey(::std::string::String);
impl ::std::ops::Deref for KitKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<KitKey> for ::std::string::String {
    fn from(value: KitKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for KitKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^.*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^.*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for KitKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for KitKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for KitKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for KitKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`KitNote`"]
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
#[doc = "    \"kitComponent\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kitComponent\": {"]
#[doc = "      \"$ref\": \"#/$defs/kit-component-id\""]
#[doc = "    },"]
#[doc = "    \"perform\": {"]
#[doc = "      \"$ref\": \"#/$defs/perform-options\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"ties\": {"]
#[doc = "      \"$ref\": \"#/$defs/tie-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct KitNote {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(rename = "kitComponent")]
    pub kit_component: KitComponentId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perform: ::std::option::Option<PerformOptions>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ties: ::std::option::Option<TieList>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl KitNote {
    pub fn builder() -> builder::KitNote {
        Default::default()
    }
}
#[doc = "`KitNotes`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/kit-note\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct KitNotes(pub ::std::vec::Vec<KitNote>);
impl ::std::ops::Deref for KitNotes {
    type Target = ::std::vec::Vec<KitNote>;
    fn deref(&self) -> &::std::vec::Vec<KitNote> {
        &self.0
    }
}
impl ::std::convert::From<KitNotes> for ::std::vec::Vec<KitNote> {
    fn from(value: KitNotes) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<KitNote>> for KitNotes {
    fn from(value: ::std::vec::Vec<KitNote>) -> Self {
        Self(value)
    }
}
#[doc = "`LanguageCode`"]
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
pub struct LanguageCode(pub ::std::string::String);
impl ::std::ops::Deref for LanguageCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LanguageCode> for ::std::string::String {
    fn from(value: LanguageCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LanguageCode {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LanguageCode {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LayoutChange`"]
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
#[doc = "    \"layout\","]
#[doc = "    \"location\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-rhythmic-position\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LayoutChange {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub layout: Id,
    pub location: MeasureRhythmicPosition,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl LayoutChange {
    pub fn builder() -> builder::LayoutChange {
        Default::default()
    }
}
#[doc = "`LayoutChanges`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/layout-change\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct LayoutChanges(pub ::std::vec::Vec<LayoutChange>);
impl ::std::ops::Deref for LayoutChanges {
    type Target = ::std::vec::Vec<LayoutChange>;
    fn deref(&self) -> &::std::vec::Vec<LayoutChange> {
        &self.0
    }
}
impl ::std::convert::From<LayoutChanges> for ::std::vec::Vec<LayoutChange> {
    fn from(value: LayoutChanges) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<LayoutChange>> for LayoutChanges {
    fn from(value: ::std::vec::Vec<LayoutChange>) -> Self {
        Self(value)
    }
}
#[doc = "`Layouts`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/system-layout\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Layouts(pub ::std::vec::Vec<SystemLayout>);
impl ::std::ops::Deref for Layouts {
    type Target = ::std::vec::Vec<SystemLayout>;
    fn deref(&self) -> &::std::vec::Vec<SystemLayout> {
        &self.0
    }
}
impl ::std::convert::From<Layouts> for ::std::vec::Vec<SystemLayout> {
    fn from(value: Layouts) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<SystemLayout>> for Layouts {
    fn from(value: ::std::vec::Vec<SystemLayout>) -> Self {
        Self(value)
    }
}
#[doc = "`LineType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"dashed\","]
#[doc = "    \"dotted\","]
#[doc = "    \"solid\","]
#[doc = "    \"wavy\""]
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
pub enum LineType {
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "wavy")]
    Wavy,
}
impl ::std::fmt::Display for LineType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dashed => f.write_str("dashed"),
            Self::Dotted => f.write_str("dotted"),
            Self::Solid => f.write_str("solid"),
            Self::Wavy => f.write_str("wavy"),
        }
    }
}
impl ::std::str::FromStr for LineType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "dashed" => Ok(Self::Dashed),
            "dotted" => Ok(Self::Dotted),
            "solid" => Ok(Self::Solid),
            "wavy" => Ok(Self::Wavy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LineType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LineType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`LiteralStringEvent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"event\""]
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
pub struct LiteralStringEvent(pub ::std::string::String);
impl ::std::ops::Deref for LiteralStringEvent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiteralStringEvent> for ::std::string::String {
    fn from(value: LiteralStringEvent) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LiteralStringEvent {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LiteralStringEvent {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LiteralStringEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LiteralStringGrace`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"grace\""]
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
pub struct LiteralStringGrace(pub ::std::string::String);
impl ::std::ops::Deref for LiteralStringGrace {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiteralStringGrace> for ::std::string::String {
    fn from(value: LiteralStringGrace) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LiteralStringGrace {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LiteralStringGrace {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LiteralStringGrace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LiteralStringGroup`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"group\""]
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
pub struct LiteralStringGroup(pub ::std::string::String);
impl ::std::ops::Deref for LiteralStringGroup {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiteralStringGroup> for ::std::string::String {
    fn from(value: LiteralStringGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LiteralStringGroup {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LiteralStringGroup {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LiteralStringGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LiteralStringSpace`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"space\""]
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
pub struct LiteralStringSpace(pub ::std::string::String);
impl ::std::ops::Deref for LiteralStringSpace {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiteralStringSpace> for ::std::string::String {
    fn from(value: LiteralStringSpace) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LiteralStringSpace {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LiteralStringSpace {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LiteralStringSpace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LiteralStringStaff`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"staff\""]
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
pub struct LiteralStringStaff(pub ::std::string::String);
impl ::std::ops::Deref for LiteralStringStaff {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiteralStringStaff> for ::std::string::String {
    fn from(value: LiteralStringStaff) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LiteralStringStaff {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LiteralStringStaff {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LiteralStringStaff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LiteralStringTremolo`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"tremolo\""]
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
pub struct LiteralStringTremolo(pub ::std::string::String);
impl ::std::ops::Deref for LiteralStringTremolo {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiteralStringTremolo> for ::std::string::String {
    fn from(value: LiteralStringTremolo) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LiteralStringTremolo {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LiteralStringTremolo {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LiteralStringTremolo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LiteralStringTuplet`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"const\": \"tuplet\""]
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
pub struct LiteralStringTuplet(pub ::std::string::String);
impl ::std::ops::Deref for LiteralStringTuplet {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LiteralStringTuplet> for ::std::string::String {
    fn from(value: LiteralStringTuplet) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LiteralStringTuplet {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LiteralStringTuplet {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LiteralStringTuplet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LyricLineId`"]
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
pub struct LyricLineId(pub ::std::string::String);
impl ::std::ops::Deref for LyricLineId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LyricLineId> for ::std::string::String {
    fn from(value: LyricLineId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LyricLineId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LyricLineId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LyricLineId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LyricLineIdList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/lyric-line-id\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct LyricLineIdList(pub ::std::vec::Vec<LyricLineId>);
impl ::std::ops::Deref for LyricLineIdList {
    type Target = ::std::vec::Vec<LyricLineId>;
    fn deref(&self) -> &::std::vec::Vec<LyricLineId> {
        &self.0
    }
}
impl ::std::convert::From<LyricLineIdList> for ::std::vec::Vec<LyricLineId> {
    fn from(value: LyricLineIdList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<LyricLineId>> for LyricLineIdList {
    fn from(value: ::std::vec::Vec<LyricLineId>) -> Self {
        Self(value)
    }
}
#[doc = "`LyricLineLabel`"]
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
pub struct LyricLineLabel(pub ::std::string::String);
impl ::std::ops::Deref for LyricLineLabel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LyricLineLabel> for ::std::string::String {
    fn from(value: LyricLineLabel) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for LyricLineLabel {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LyricLineLabel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LyricLineLabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`LyricLineMetadata`"]
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
#[doc = "    \"label\": {"]
#[doc = "      \"$ref\": \"#/$defs/lyric-line-label\""]
#[doc = "    },"]
#[doc = "    \"lang\": {"]
#[doc = "      \"$ref\": \"#/$defs/language-code\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LyricLineMetadata {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<LyricLineLabel>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lang: ::std::option::Option<LanguageCode>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for LyricLineMetadata {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            label: Default::default(),
            lang: Default::default(),
            x: Default::default(),
        }
    }
}
impl LyricLineMetadata {
    pub fn builder() -> builder::LyricLineMetadata {
        Default::default()
    }
}
#[doc = "`LyricLinesMetadata`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \"^.*$\": {"]
#[doc = "      \"$ref\": \"#/$defs/lyric-line-metadata\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct LyricLinesMetadata(
    pub ::std::collections::HashMap<LyricLinesMetadataKey, LyricLineMetadata>,
);
impl ::std::ops::Deref for LyricLinesMetadata {
    type Target = ::std::collections::HashMap<LyricLinesMetadataKey, LyricLineMetadata>;
    fn deref(&self) -> &::std::collections::HashMap<LyricLinesMetadataKey, LyricLineMetadata> {
        &self.0
    }
}
impl ::std::convert::From<LyricLinesMetadata>
    for ::std::collections::HashMap<LyricLinesMetadataKey, LyricLineMetadata>
{
    fn from(value: LyricLinesMetadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<LyricLinesMetadataKey, LyricLineMetadata>>
    for LyricLinesMetadata
{
    fn from(value: ::std::collections::HashMap<LyricLinesMetadataKey, LyricLineMetadata>) -> Self {
        Self(value)
    }
}
#[doc = "`LyricLinesMetadataKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^.*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LyricLinesMetadataKey(::std::string::String);
impl ::std::ops::Deref for LyricLinesMetadataKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LyricLinesMetadataKey> for ::std::string::String {
    fn from(value: LyricLinesMetadataKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for LyricLinesMetadataKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^.*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^.*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LyricLinesMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LyricLinesMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LyricLinesMetadataKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LyricLinesMetadataKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`Lyrics`"]
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
#[doc = "    \"lines\": {"]
#[doc = "      \"$ref\": \"#/$defs/event-lyric-lines\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Lyrics {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lines: ::std::option::Option<EventLyricLines>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Lyrics {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            lines: Default::default(),
            x: Default::default(),
        }
    }
}
impl Lyrics {
    pub fn builder() -> builder::Lyrics {
        Default::default()
    }
}
#[doc = "`LyricsGlobal`"]
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
#[doc = "    \"lineMetadata\": {"]
#[doc = "      \"$ref\": \"#/$defs/lyric-lines-metadata\""]
#[doc = "    },"]
#[doc = "    \"lineOrder\": {"]
#[doc = "      \"$ref\": \"#/$defs/lyric-line-id-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LyricsGlobal {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "lineMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub line_metadata: ::std::option::Option<LyricLinesMetadata>,
    #[serde(
        rename = "lineOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub line_order: ::std::option::Option<LyricLineIdList>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for LyricsGlobal {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            line_metadata: Default::default(),
            line_order: Default::default(),
            x: Default::default(),
        }
    }
}
impl LyricsGlobal {
    pub fn builder() -> builder::LyricsGlobal {
        Default::default()
    }
}
#[doc = "`MeasureCount`"]
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
pub struct MeasureCount(pub i64);
impl ::std::ops::Deref for MeasureCount {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<MeasureCount> for i64 {
    fn from(value: MeasureCount) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for MeasureCount {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for MeasureCount {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for MeasureCount {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for MeasureCount {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for MeasureCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`MeasureGlobal`"]
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
#[doc = "    \"barline\": {"]
#[doc = "      \"$ref\": \"#/$defs/barline\""]
#[doc = "    },"]
#[doc = "    \"ending\": {"]
#[doc = "      \"$ref\": \"#/$defs/ending\""]
#[doc = "    },"]
#[doc = "    \"fine\": {"]
#[doc = "      \"$ref\": \"#/$defs/fine\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-number\""]
#[doc = "    },"]
#[doc = "    \"jump\": {"]
#[doc = "      \"$ref\": \"#/$defs/jump\""]
#[doc = "    },"]
#[doc = "    \"key\": {"]
#[doc = "      \"$ref\": \"#/$defs/key\""]
#[doc = "    },"]
#[doc = "    \"number\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-number\""]
#[doc = "    },"]
#[doc = "    \"repeatEnd\": {"]
#[doc = "      \"$ref\": \"#/$defs/repeat-end\""]
#[doc = "    },"]
#[doc = "    \"repeatStart\": {"]
#[doc = "      \"$ref\": \"#/$defs/repeat-start\""]
#[doc = "    },"]
#[doc = "    \"segno\": {"]
#[doc = "      \"$ref\": \"#/$defs/segno\""]
#[doc = "    },"]
#[doc = "    \"tempos\": {"]
#[doc = "      \"$ref\": \"#/$defs/tempos\""]
#[doc = "    },"]
#[doc = "    \"time\": {"]
#[doc = "      \"$ref\": \"#/$defs/time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MeasureGlobal {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub barline: ::std::option::Option<Barline>,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ending: ::std::option::Option<Ending>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fine: ::std::option::Option<Fine>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<MeasureNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub jump: ::std::option::Option<Jump>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub key: ::std::option::Option<Key>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub number: ::std::option::Option<MeasureNumber>,
    #[serde(
        rename = "repeatEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub repeat_end: ::std::option::Option<RepeatEnd>,
    #[serde(
        rename = "repeatStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub repeat_start: ::std::option::Option<RepeatStart>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub segno: ::std::option::Option<Segno>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tempos: ::std::option::Option<Tempos>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time: ::std::option::Option<Time>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for MeasureGlobal {
    fn default() -> Self {
        Self {
            barline: Default::default(),
            c: Default::default(),
            ending: Default::default(),
            fine: Default::default(),
            id: Default::default(),
            index: Default::default(),
            jump: Default::default(),
            key: Default::default(),
            number: Default::default(),
            repeat_end: Default::default(),
            repeat_start: Default::default(),
            segno: Default::default(),
            tempos: Default::default(),
            time: Default::default(),
            x: Default::default(),
        }
    }
}
impl MeasureGlobal {
    pub fn builder() -> builder::MeasureGlobal {
        Default::default()
    }
}
#[doc = "`MeasureNumber`"]
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
pub struct MeasureNumber(pub i64);
impl ::std::ops::Deref for MeasureNumber {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<MeasureNumber> for i64 {
    fn from(value: MeasureNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for MeasureNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for MeasureNumber {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for MeasureNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for MeasureNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for MeasureNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`MeasureRhythmicPosition`"]
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
#[doc = "    \"measure\","]
#[doc = "    \"position\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"measure\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-number\""]
#[doc = "    },"]
#[doc = "    \"position\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MeasureRhythmicPosition {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub measure: MeasureNumber,
    pub position: RhythmicPosition,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl MeasureRhythmicPosition {
    pub fn builder() -> builder::MeasureRhythmicPosition {
        Default::default()
    }
}
#[doc = "`MeasuresGlobal`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/measure-global\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MeasuresGlobal(pub ::std::vec::Vec<MeasureGlobal>);
impl ::std::ops::Deref for MeasuresGlobal {
    type Target = ::std::vec::Vec<MeasureGlobal>;
    fn deref(&self) -> &::std::vec::Vec<MeasureGlobal> {
        &self.0
    }
}
impl ::std::convert::From<MeasuresGlobal> for ::std::vec::Vec<MeasureGlobal> {
    fn from(value: MeasuresGlobal) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<MeasureGlobal>> for MeasuresGlobal {
    fn from(value: ::std::vec::Vec<MeasureGlobal>) -> Self {
        Self(value)
    }
}
#[doc = "`MidiNumber`"]
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
pub struct MidiNumber(pub i64);
impl ::std::ops::Deref for MidiNumber {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<MidiNumber> for i64 {
    fn from(value: MidiNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for MidiNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for MidiNumber {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for MidiNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for MidiNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for MidiNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Mnx`"]
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
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"support\": {"]
#[doc = "      \"$ref\": \"#/$defs/support\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"$ref\": \"#/$defs/version-number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Mnx {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub support: ::std::option::Option<Support>,
    pub version: VersionNumber,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Mnx {
    pub fn builder() -> builder::Mnx {
        Default::default()
    }
}
#[doc = "An encoding of Common Western Music Notation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://w3c.github.io/mnx/docs/mnx-schema.json\","]
#[doc = "  \"title\": \"MNX document\","]
#[doc = "  \"description\": \"An encoding of Common Western Music Notation.\","]
#[doc = "  \"$ref\": \"#/$defs/root\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MnxDocument(pub Root);
impl ::std::ops::Deref for MnxDocument {
    type Target = Root;
    fn deref(&self) -> &Root {
        &self.0
    }
}
impl ::std::convert::From<MnxDocument> for Root {
    fn from(value: MnxDocument) -> Self {
        value.0
    }
}
impl ::std::convert::From<Root> for MnxDocument {
    fn from(value: Root) -> Self {
        Self(value)
    }
}
#[doc = "`MultiNoteTremolo`"]
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
#[doc = "    \"marks\","]
#[doc = "    \"outer\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/$defs/tremolo-sequence-content\""]
#[doc = "    },"]
#[doc = "    \"individualDuration\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value\""]
#[doc = "    },"]
#[doc = "    \"marks\": {"]
#[doc = "      \"$ref\": \"#/$defs/positive-integer\""]
#[doc = "    },"]
#[doc = "    \"outer\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value-quantity\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/literal-string-tremolo\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MultiNoteTremolo {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub content: TremoloSequenceContent,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "individualDuration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub individual_duration: ::std::option::Option<NoteValue>,
    pub marks: PositiveInteger,
    pub outer: NoteValueQuantity,
    #[serde(rename = "type")]
    pub type_: LiteralStringTremolo,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl MultiNoteTremolo {
    pub fn builder() -> builder::MultiNoteTremolo {
        Default::default()
    }
}
#[doc = "`MultimeasureRest`"]
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
#[doc = "    \"start\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-count\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"$ref\": \"#/$defs/string\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MultimeasureRest {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub duration: MeasureCount,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<String>,
    pub start: MeasureNumber,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl MultimeasureRest {
    pub fn builder() -> builder::MultimeasureRest {
        Default::default()
    }
}
#[doc = "`MultimeasureRests`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/multimeasure-rest\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MultimeasureRests(pub ::std::vec::Vec<MultimeasureRest>);
impl ::std::ops::Deref for MultimeasureRests {
    type Target = ::std::vec::Vec<MultimeasureRest>;
    fn deref(&self) -> &::std::vec::Vec<MultimeasureRest> {
        &self.0
    }
}
impl ::std::convert::From<MultimeasureRests> for ::std::vec::Vec<MultimeasureRest> {
    fn from(value: MultimeasureRests) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<MultimeasureRest>> for MultimeasureRests {
    fn from(value: ::std::vec::Vec<MultimeasureRest>) -> Self {
        Self(value)
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
#[doc = "    \"accidentalDisplay\": {"]
#[doc = "      \"$ref\": \"#/$defs/accidental-display\""]
#[doc = "    },"]
#[doc = "    \"perform\": {"]
#[doc = "      \"$ref\": \"#/$defs/perform-options\""]
#[doc = "    },"]
#[doc = "    \"pitch\": {"]
#[doc = "      \"$ref\": \"#/$defs/pitch\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"ties\": {"]
#[doc = "      \"$ref\": \"#/$defs/tie-list\""]
#[doc = "    },"]
#[doc = "    \"written\": {"]
#[doc = "      \"$ref\": \"#/$defs/written\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Note {
    #[serde(
        rename = "accidentalDisplay",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub accidental_display: ::std::option::Option<AccidentalDisplay>,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perform: ::std::option::Option<PerformOptions>,
    pub pitch: Pitch,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ties: ::std::option::Option<TieList>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub written: ::std::option::Option<Written>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
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
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
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
#[doc = "    \"duplexMaxima\","]
#[doc = "    \"maxima\","]
#[doc = "    \"longa\","]
#[doc = "    \"breve\","]
#[doc = "    \"whole\","]
#[doc = "    \"half\","]
#[doc = "    \"quarter\","]
#[doc = "    \"eighth\","]
#[doc = "    \"16th\","]
#[doc = "    \"32nd\","]
#[doc = "    \"64th\","]
#[doc = "    \"128th\","]
#[doc = "    \"256th\","]
#[doc = "    \"512th\","]
#[doc = "    \"1024th\","]
#[doc = "    \"2048th\","]
#[doc = "    \"4096th\""]
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
    #[serde(rename = "duplexMaxima")]
    DuplexMaxima,
    #[serde(rename = "maxima")]
    Maxima,
    #[serde(rename = "longa")]
    Longa,
    #[serde(rename = "breve")]
    Breve,
    #[serde(rename = "whole")]
    Whole,
    #[serde(rename = "half")]
    Half,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "eighth")]
    Eighth,
    #[serde(rename = "16th")]
    X16th,
    #[serde(rename = "32nd")]
    X32nd,
    #[serde(rename = "64th")]
    X64th,
    #[serde(rename = "128th")]
    X128th,
    #[serde(rename = "256th")]
    X256th,
    #[serde(rename = "512th")]
    X512th,
    #[serde(rename = "1024th")]
    X1024th,
    #[serde(rename = "2048th")]
    X2048th,
    #[serde(rename = "4096th")]
    X4096th,
}
impl ::std::fmt::Display for NoteValueBase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DuplexMaxima => f.write_str("duplexMaxima"),
            Self::Maxima => f.write_str("maxima"),
            Self::Longa => f.write_str("longa"),
            Self::Breve => f.write_str("breve"),
            Self::Whole => f.write_str("whole"),
            Self::Half => f.write_str("half"),
            Self::Quarter => f.write_str("quarter"),
            Self::Eighth => f.write_str("eighth"),
            Self::X16th => f.write_str("16th"),
            Self::X32nd => f.write_str("32nd"),
            Self::X64th => f.write_str("64th"),
            Self::X128th => f.write_str("128th"),
            Self::X256th => f.write_str("256th"),
            Self::X512th => f.write_str("512th"),
            Self::X1024th => f.write_str("1024th"),
            Self::X2048th => f.write_str("2048th"),
            Self::X4096th => f.write_str("4096th"),
        }
    }
}
impl ::std::str::FromStr for NoteValueBase {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "duplexMaxima" => Ok(Self::DuplexMaxima),
            "maxima" => Ok(Self::Maxima),
            "longa" => Ok(Self::Longa),
            "breve" => Ok(Self::Breve),
            "whole" => Ok(Self::Whole),
            "half" => Ok(Self::Half),
            "quarter" => Ok(Self::Quarter),
            "eighth" => Ok(Self::Eighth),
            "16th" => Ok(Self::X16th),
            "32nd" => Ok(Self::X32nd),
            "64th" => Ok(Self::X64th),
            "128th" => Ok(Self::X128th),
            "256th" => Ok(Self::X256th),
            "512th" => Ok(Self::X512th),
            "1024th" => Ok(Self::X1024th),
            "2048th" => Ok(Self::X2048th),
            "4096th" => Ok(Self::X4096th),
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
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
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
#[doc = "`Orientation`"]
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
pub struct Orientation(pub ::std::string::String);
impl ::std::ops::Deref for Orientation {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Orientation> for ::std::string::String {
    fn from(value: Orientation) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for Orientation {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Orientation {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Ottava`"]
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
#[doc = "    \"end\","]
#[doc = "    \"position\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"end\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-rhythmic-position\""]
#[doc = "    },"]
#[doc = "    \"orient\": {"]
#[doc = "      \"$ref\": \"#/$defs/orientation\""]
#[doc = "    },"]
#[doc = "    \"position\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/ottava-amount\""]
#[doc = "    },"]
#[doc = "    \"voice\": {"]
#[doc = "      \"$ref\": \"#/$defs/voice-name\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Ottava {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub end: MeasureRhythmicPosition,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orient: ::std::option::Option<Orientation>,
    pub position: RhythmicPosition,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    pub value: OttavaAmount,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Ottava {
    pub fn builder() -> builder::Ottava {
        Default::default()
    }
}
#[doc = "`OttavaAmount`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"enum\": ["]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    -1,"]
#[doc = "    -2,"]
#[doc = "    3,"]
#[doc = "    -3"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct OttavaAmount(i64);
impl ::std::ops::Deref for OttavaAmount {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<OttavaAmount> for i64 {
    fn from(value: OttavaAmount) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for OttavaAmount {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64, -1_i64, -2_i64, 3_i64, -3_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for OttavaAmount {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`OttavaAmountOrZero`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"enum\": ["]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    -1,"]
#[doc = "    -2,"]
#[doc = "    3,"]
#[doc = "    -3,"]
#[doc = "    0"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct OttavaAmountOrZero(i64);
impl ::std::ops::Deref for OttavaAmountOrZero {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<OttavaAmountOrZero> for i64 {
    fn from(value: OttavaAmountOrZero) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for OttavaAmountOrZero {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64, -1_i64, -2_i64, 3_i64, -3_i64, 0_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for OttavaAmountOrZero {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`OttavaList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/ottava\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct OttavaList(pub ::std::vec::Vec<Ottava>);
impl ::std::ops::Deref for OttavaList {
    type Target = ::std::vec::Vec<Ottava>;
    fn deref(&self) -> &::std::vec::Vec<Ottava> {
        &self.0
    }
}
impl ::std::convert::From<OttavaList> for ::std::vec::Vec<Ottava> {
    fn from(value: OttavaList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Ottava>> for OttavaList {
    fn from(value: ::std::vec::Vec<Ottava>) -> Self {
        Self(value)
    }
}
#[doc = "`Page`"]
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
#[doc = "    \"systems\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"systems\": {"]
#[doc = "      \"$ref\": \"#/$defs/systems\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Page {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<Id>,
    pub systems: Systems,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Page {
    pub fn builder() -> builder::Page {
        Default::default()
    }
}
#[doc = "`Pages`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/page\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Pages(pub ::std::vec::Vec<Page>);
impl ::std::ops::Deref for Pages {
    type Target = ::std::vec::Vec<Page>;
    fn deref(&self) -> &::std::vec::Vec<Page> {
        &self.0
    }
}
impl ::std::convert::From<Pages> for ::std::vec::Vec<Page> {
    fn from(value: Pages) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Page>> for Pages {
    fn from(value: ::std::vec::Vec<Page>) -> Self {
        Self(value)
    }
}
#[doc = "`Part`"]
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
#[doc = "    \"measures\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kit\": {"]
#[doc = "      \"$ref\": \"#/$defs/kit\""]
#[doc = "    },"]
#[doc = "    \"measures\": {"]
#[doc = "      \"$ref\": \"#/$defs/part-measures\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/part-name\""]
#[doc = "    },"]
#[doc = "    \"shortName\": {"]
#[doc = "      \"$ref\": \"#/$defs/part-short-name\""]
#[doc = "    },"]
#[doc = "    \"smuflFont\": {"]
#[doc = "      \"$ref\": \"#/$defs/smufl-font\""]
#[doc = "    },"]
#[doc = "    \"staves\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-count\""]
#[doc = "    },"]
#[doc = "    \"transposition\": {"]
#[doc = "      \"$ref\": \"#/$defs/part-transposition\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Part {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kit: ::std::option::Option<Kit>,
    pub measures: PartMeasures,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<PartName>,
    #[serde(
        rename = "shortName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_name: ::std::option::Option<PartShortName>,
    #[serde(
        rename = "smuflFont",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub smufl_font: ::std::option::Option<SmuflFont>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staves: ::std::option::Option<StaffCount>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transposition: ::std::option::Option<PartTransposition>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Part {
    pub fn builder() -> builder::Part {
        Default::default()
    }
}
#[doc = "`PartMeasure`"]
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
#[doc = "    \"sequences\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"beams\": {"]
#[doc = "      \"$ref\": \"#/$defs/beam-list\""]
#[doc = "    },"]
#[doc = "    \"clefs\": {"]
#[doc = "      \"$ref\": \"#/$defs/positioned-clef-list\""]
#[doc = "    },"]
#[doc = "    \"dynamics\": {"]
#[doc = "      \"$ref\": \"#/$defs/dynamic-list\""]
#[doc = "    },"]
#[doc = "    \"ottavas\": {"]
#[doc = "      \"$ref\": \"#/$defs/ottava-list\""]
#[doc = "    },"]
#[doc = "    \"sequences\": {"]
#[doc = "      \"$ref\": \"#/$defs/sequence-list\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PartMeasure {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub beams: ::std::option::Option<BeamList>,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub clefs: ::std::option::Option<PositionedClefList>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dynamics: ::std::option::Option<DynamicList>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ottavas: ::std::option::Option<OttavaList>,
    pub sequences: SequenceList,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl PartMeasure {
    pub fn builder() -> builder::PartMeasure {
        Default::default()
    }
}
#[doc = "`PartMeasures`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/part-measure\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PartMeasures(pub ::std::vec::Vec<PartMeasure>);
impl ::std::ops::Deref for PartMeasures {
    type Target = ::std::vec::Vec<PartMeasure>;
    fn deref(&self) -> &::std::vec::Vec<PartMeasure> {
        &self.0
    }
}
impl ::std::convert::From<PartMeasures> for ::std::vec::Vec<PartMeasure> {
    fn from(value: PartMeasures) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<PartMeasure>> for PartMeasures {
    fn from(value: ::std::vec::Vec<PartMeasure>) -> Self {
        Self(value)
    }
}
#[doc = "`PartName`"]
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
pub struct PartName(pub ::std::string::String);
impl ::std::ops::Deref for PartName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PartName> for ::std::string::String {
    fn from(value: PartName) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for PartName {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PartName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for PartName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`PartShortName`"]
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
pub struct PartShortName(pub ::std::string::String);
impl ::std::ops::Deref for PartShortName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PartShortName> for ::std::string::String {
    fn from(value: PartShortName) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for PartShortName {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PartShortName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for PartShortName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`PartTransposition`"]
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
#[doc = "    \"interval\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"interval\": {"]
#[doc = "      \"$ref\": \"#/$defs/interval\""]
#[doc = "    },"]
#[doc = "    \"keyFifthsFlipAt\": {"]
#[doc = "      \"$ref\": \"#/$defs/integer-signed\""]
#[doc = "    },"]
#[doc = "    \"prefersWrittenPitches\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PartTransposition {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub interval: Interval,
    #[serde(
        rename = "keyFifthsFlipAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub key_fifths_flip_at: ::std::option::Option<IntegerSigned>,
    #[serde(
        rename = "prefersWrittenPitches",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prefers_written_pitches: ::std::option::Option<bool>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl PartTransposition {
    pub fn builder() -> builder::PartTransposition {
        Default::default()
    }
}
#[doc = "`Parts`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/part\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Parts(pub ::std::vec::Vec<Part>);
impl ::std::ops::Deref for Parts {
    type Target = ::std::vec::Vec<Part>;
    fn deref(&self) -> &::std::vec::Vec<Part> {
        &self.0
    }
}
impl ::std::convert::From<Parts> for ::std::vec::Vec<Part> {
    fn from(value: Parts) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Part>> for Parts {
    fn from(value: ::std::vec::Vec<Part>) -> Self {
        Self(value)
    }
}
#[doc = "`PerformOptions`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PerformOptions(pub GlobalAttrs);
impl ::std::ops::Deref for PerformOptions {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<PerformOptions> for GlobalAttrs {
    fn from(value: PerformOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for PerformOptions {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
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
#[doc = "    \"alter\": {"]
#[doc = "      \"$ref\": \"#/$defs/alter\""]
#[doc = "    },"]
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alter: ::std::option::Option<Alter>,
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
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Pitch {
    pub fn builder() -> builder::Pitch {
        Default::default()
    }
}
#[doc = "`PositionedClef`"]
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
#[doc = "    \"clef\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"clef\": {"]
#[doc = "      \"$ref\": \"#/$defs/clef\""]
#[doc = "    },"]
#[doc = "    \"position\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PositionedClef {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub clef: Clef,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub position: ::std::option::Option<RhythmicPosition>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl PositionedClef {
    pub fn builder() -> builder::PositionedClef {
        Default::default()
    }
}
#[doc = "`PositionedClefList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/positioned-clef\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PositionedClefList(pub ::std::vec::Vec<PositionedClef>);
impl ::std::ops::Deref for PositionedClefList {
    type Target = ::std::vec::Vec<PositionedClef>;
    fn deref(&self) -> &::std::vec::Vec<PositionedClef> {
        &self.0
    }
}
impl ::std::convert::From<PositionedClefList> for ::std::vec::Vec<PositionedClef> {
    fn from(value: PositionedClefList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<PositionedClef>> for PositionedClefList {
    fn from(value: ::std::vec::Vec<PositionedClef>) -> Self {
        Self(value)
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
#[doc = "`RepeatEnd`"]
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
#[doc = "    \"times\": {"]
#[doc = "      \"$ref\": \"#/$defs/repeat-times\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RepeatEnd {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub times: ::std::option::Option<RepeatTimes>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for RepeatEnd {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            times: Default::default(),
            x: Default::default(),
        }
    }
}
impl RepeatEnd {
    pub fn builder() -> builder::RepeatEnd {
        Default::default()
    }
}
#[doc = "`RepeatStart`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RepeatStart(pub GlobalAttrs);
impl ::std::ops::Deref for RepeatStart {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<RepeatStart> for GlobalAttrs {
    fn from(value: RepeatStart) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for RepeatStart {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
    }
}
#[doc = "`RepeatTimes`"]
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
pub struct RepeatTimes(pub i64);
impl ::std::ops::Deref for RepeatTimes {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<RepeatTimes> for i64 {
    fn from(value: RepeatTimes) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for RepeatTimes {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for RepeatTimes {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for RepeatTimes {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for RepeatTimes {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for RepeatTimes {
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
#[doc = "      \"$ref\": \"#/$defs/staff-position\""]
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
    pub staff_position: ::std::option::Option<StaffPosition>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Rest {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            staff_position: Default::default(),
            x: Default::default(),
        }
    }
}
impl Rest {
    pub fn builder() -> builder::Rest {
        Default::default()
    }
}
#[doc = "`RhythmicPosition`"]
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
#[doc = "    \"fraction\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fraction\": {"]
#[doc = "      \"$ref\": \"#/$defs/fraction\""]
#[doc = "    },"]
#[doc = "    \"graceIndex\": {"]
#[doc = "      \"$ref\": \"#/$defs/integer-unsigned\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RhythmicPosition {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub fraction: Fraction,
    #[serde(
        rename = "graceIndex",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grace_index: ::std::option::Option<IntegerUnsigned>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl RhythmicPosition {
    pub fn builder() -> builder::RhythmicPosition {
        Default::default()
    }
}
#[doc = "`Root`"]
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
#[doc = "    \"global\","]
#[doc = "    \"mnx\","]
#[doc = "    \"parts\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"global\": {"]
#[doc = "      \"$ref\": \"#/$defs/global\""]
#[doc = "    },"]
#[doc = "    \"layouts\": {"]
#[doc = "      \"$ref\": \"#/$defs/layouts\""]
#[doc = "    },"]
#[doc = "    \"mnx\": {"]
#[doc = "      \"$ref\": \"#/$defs/mnx\""]
#[doc = "    },"]
#[doc = "    \"parts\": {"]
#[doc = "      \"$ref\": \"#/$defs/parts\""]
#[doc = "    },"]
#[doc = "    \"scores\": {"]
#[doc = "      \"$ref\": \"#/$defs/scores\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Root {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub global: Global,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layouts: ::std::option::Option<Layouts>,
    pub mnx: Mnx,
    pub parts: Parts,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scores: ::std::option::Option<Scores>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Root {
    pub fn builder() -> builder::Root {
        Default::default()
    }
}
#[doc = "`Score`"]
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
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"multimeasureRests\": {"]
#[doc = "      \"$ref\": \"#/$defs/multimeasure-rests\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/score-name\""]
#[doc = "    },"]
#[doc = "    \"pages\": {"]
#[doc = "      \"$ref\": \"#/$defs/pages\""]
#[doc = "    },"]
#[doc = "    \"useWritten\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Score {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<Id>,
    #[serde(
        rename = "multimeasureRests",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub multimeasure_rests: ::std::option::Option<MultimeasureRests>,
    pub name: ScoreName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pages: ::std::option::Option<Pages>,
    #[serde(
        rename = "useWritten",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_written: ::std::option::Option<bool>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Score {
    pub fn builder() -> builder::Score {
        Default::default()
    }
}
#[doc = "`ScoreName`"]
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
pub struct ScoreName(pub ::std::string::String);
impl ::std::ops::Deref for ScoreName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ScoreName> for ::std::string::String {
    fn from(value: ScoreName) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ScoreName {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ScoreName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for ScoreName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Scores`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/score\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Scores(pub ::std::vec::Vec<Score>);
impl ::std::ops::Deref for Scores {
    type Target = ::std::vec::Vec<Score>;
    fn deref(&self) -> &::std::vec::Vec<Score> {
        &self.0
    }
}
impl ::std::convert::From<Scores> for ::std::vec::Vec<Score> {
    fn from(value: Scores) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Score>> for Scores {
    fn from(value: ::std::vec::Vec<Score>) -> Self {
        Self(value)
    }
}
#[doc = "`Segno`"]
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
#[doc = "    \"location\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"$ref\": \"#/$defs/color\""]
#[doc = "    },"]
#[doc = "    \"glyph\": {"]
#[doc = "      \"$ref\": \"#/$defs/smufl-glyph\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Segno {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub glyph: ::std::option::Option<SmuflGlyph>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub location: RhythmicPosition,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Segno {
    pub fn builder() -> builder::Segno {
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
#[doc = "    \"orient\": {"]
#[doc = "      \"$ref\": \"#/$defs/orientation\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"voice\": {"]
#[doc = "      \"$ref\": \"#/$defs/voice-name\""]
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
    pub orient: ::std::option::Option<Orientation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
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
#[doc = "        \"$ref\": \"#/$defs/grace\""]
#[doc = "      },"]
#[doc = "      {"]
#[doc = "        \"$ref\": \"#/$defs/tuplet\""]
#[doc = "      },"]
#[doc = "      {"]
#[doc = "        \"$ref\": \"#/$defs/space\""]
#[doc = "      },"]
#[doc = "      {"]
#[doc = "        \"$ref\": \"#/$defs/multi-note-tremolo\""]
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
#[doc = "      \"$ref\": \"#/$defs/grace\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/tuplet\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/space\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/multi-note-tremolo\""]
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
    pub grace: ::std::option::Option<Grace>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tuplet: ::std::option::Option<Tuplet>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub space: ::std::option::Option<Space>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub multi_note_tremolo: ::std::option::Option<MultiNoteTremolo>,
}
impl ::std::default::Default for SequenceContentItem {
    fn default() -> Self {
        Self {
            event: Default::default(),
            grace: Default::default(),
            tuplet: Default::default(),
            space: Default::default(),
            multi_note_tremolo: Default::default(),
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
impl ::std::convert::From<::std::vec::Vec<Sequence>> for SequenceList {
    fn from(value: ::std::vec::Vec<Sequence>) -> Self {
        Self(value)
    }
}
#[doc = "`SimpleColor`"]
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
pub struct SimpleColor(pub ::std::string::String);
impl ::std::ops::Deref for SimpleColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SimpleColor> for ::std::string::String {
    fn from(value: SimpleColor) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SimpleColor {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SimpleColor {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SimpleColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Slur`"]
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
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"endNote\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"lineType\": {"]
#[doc = "      \"$ref\": \"#/$defs/line-type\""]
#[doc = "    },"]
#[doc = "    \"side\": {"]
#[doc = "      \"$ref\": \"#/$defs/slur-side\""]
#[doc = "    },"]
#[doc = "    \"sideEnd\": {"]
#[doc = "      \"$ref\": \"#/$defs/slur-side\""]
#[doc = "    },"]
#[doc = "    \"startNote\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Slur {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(
        rename = "endNote",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_note: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "lineType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub line_type: ::std::option::Option<LineType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub side: ::std::option::Option<SlurSide>,
    #[serde(
        rename = "sideEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub side_end: ::std::option::Option<SlurSide>,
    #[serde(
        rename = "startNote",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_note: ::std::option::Option<Id>,
    pub target: Id,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Slur {
    pub fn builder() -> builder::Slur {
        Default::default()
    }
}
#[doc = "`SlurList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/slur\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SlurList(pub ::std::vec::Vec<Slur>);
impl ::std::ops::Deref for SlurList {
    type Target = ::std::vec::Vec<Slur>;
    fn deref(&self) -> &::std::vec::Vec<Slur> {
        &self.0
    }
}
impl ::std::convert::From<SlurList> for ::std::vec::Vec<Slur> {
    fn from(value: SlurList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Slur>> for SlurList {
    fn from(value: ::std::vec::Vec<Slur>) -> Self {
        Self(value)
    }
}
#[doc = "`SlurSide`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"up\","]
#[doc = "    \"down\""]
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
pub enum SlurSide {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}
impl ::std::fmt::Display for SlurSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => f.write_str("up"),
            Self::Down => f.write_str("down"),
        }
    }
}
impl ::std::str::FromStr for SlurSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SlurSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SlurSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SlurSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`SlurTieEndLocation`"]
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
pub struct SlurTieEndLocation(pub ::std::string::String);
impl ::std::ops::Deref for SlurTieEndLocation {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SlurTieEndLocation> for ::std::string::String {
    fn from(value: SlurTieEndLocation) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SlurTieEndLocation {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SlurTieEndLocation {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SlurTieEndLocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`SmuflFont`"]
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
pub struct SmuflFont(pub ::std::string::String);
impl ::std::ops::Deref for SmuflFont {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SmuflFont> for ::std::string::String {
    fn from(value: SmuflFont) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SmuflFont {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SmuflFont {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SmuflFont {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`SmuflGlyph`"]
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
pub struct SmuflGlyph(pub ::std::string::String);
impl ::std::ops::Deref for SmuflGlyph {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SmuflGlyph> for ::std::string::String {
    fn from(value: SmuflGlyph) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SmuflGlyph {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SmuflGlyph {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SmuflGlyph {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`SoftAccent`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SoftAccent(pub GlobalAttrs);
impl ::std::ops::Deref for SoftAccent {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<SoftAccent> for GlobalAttrs {
    fn from(value: SoftAccent) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for SoftAccent {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
    }
}
#[doc = "`Sound`"]
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
#[doc = "    \"midiNumber\": {"]
#[doc = "      \"$ref\": \"#/$defs/midi-number\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Sound {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "midiNumber",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub midi_number: ::std::option::Option<MidiNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<String>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Sound {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            midi_number: Default::default(),
            name: Default::default(),
            x: Default::default(),
        }
    }
}
impl Sound {
    pub fn builder() -> builder::Sound {
        Default::default()
    }
}
#[doc = "`SoundsGlobal`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \"^.*$\": {"]
#[doc = "      \"$ref\": \"#/$defs/sound\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SoundsGlobal(pub ::std::collections::HashMap<SoundsGlobalKey, Sound>);
impl ::std::ops::Deref for SoundsGlobal {
    type Target = ::std::collections::HashMap<SoundsGlobalKey, Sound>;
    fn deref(&self) -> &::std::collections::HashMap<SoundsGlobalKey, Sound> {
        &self.0
    }
}
impl ::std::convert::From<SoundsGlobal> for ::std::collections::HashMap<SoundsGlobalKey, Sound> {
    fn from(value: SoundsGlobal) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<SoundsGlobalKey, Sound>> for SoundsGlobal {
    fn from(value: ::std::collections::HashMap<SoundsGlobalKey, Sound>) -> Self {
        Self(value)
    }
}
#[doc = "`SoundsGlobalKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^.*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SoundsGlobalKey(::std::string::String);
impl ::std::ops::Deref for SoundsGlobalKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SoundsGlobalKey> for ::std::string::String {
    fn from(value: SoundsGlobalKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SoundsGlobalKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^.*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^.*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SoundsGlobalKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SoundsGlobalKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SoundsGlobalKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SoundsGlobalKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`Space`"]
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
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"$ref\": \"#/$defs/fraction\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/literal-string-space\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Space {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub duration: Fraction,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(rename = "type")]
    pub type_: LiteralStringSpace,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Space {
    pub fn builder() -> builder::Space {
        Default::default()
    }
}
#[doc = "`Spiccato`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Spiccato(pub GlobalAttrs);
impl ::std::ops::Deref for Spiccato {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<Spiccato> for GlobalAttrs {
    fn from(value: Spiccato) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for Spiccato {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
    }
}
#[doc = "`Staccatissimo`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Staccatissimo(pub GlobalAttrs);
impl ::std::ops::Deref for Staccatissimo {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<Staccatissimo> for GlobalAttrs {
    fn from(value: Staccatissimo) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for Staccatissimo {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
    }
}
#[doc = "`Staccato`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Staccato(pub GlobalAttrs);
impl ::std::ops::Deref for Staccato {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<Staccato> for GlobalAttrs {
    fn from(value: Staccato) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for Staccato {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
    }
}
#[doc = "`Staff`"]
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
#[doc = "    \"sources\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"label\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-label\""]
#[doc = "    },"]
#[doc = "    \"labelref\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-labelref\""]
#[doc = "    },"]
#[doc = "    \"sources\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-sources\""]
#[doc = "    },"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-symbol\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/literal-string-staff\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Staff {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<StaffLabel>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub labelref: ::std::option::Option<StaffLabelref>,
    pub sources: StaffSources,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub symbol: ::std::option::Option<StaffSymbol>,
    #[serde(rename = "type")]
    pub type_: LiteralStringStaff,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Staff {
    pub fn builder() -> builder::Staff {
        Default::default()
    }
}
#[doc = "`StaffCount`"]
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
pub struct StaffCount(pub i64);
impl ::std::ops::Deref for StaffCount {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<StaffCount> for i64 {
    fn from(value: StaffCount) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for StaffCount {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffCount {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for StaffCount {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for StaffCount {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for StaffCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`StaffGroup`"]
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
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"barlineStyle\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-group-barline-style\""]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/$defs/system-layout-content\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-label\""]
#[doc = "    },"]
#[doc = "    \"symbol\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-symbol\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/literal-string-group\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StaffGroup {
    #[serde(
        rename = "barlineStyle",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub barline_style: ::std::option::Option<StaffGroupBarlineStyle>,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub content: SystemLayoutContent,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<StaffLabel>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub symbol: ::std::option::Option<StaffSymbol>,
    #[serde(rename = "type")]
    pub type_: LiteralStringGroup,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl StaffGroup {
    pub fn builder() -> builder::StaffGroup {
        Default::default()
    }
}
#[doc = "`StaffGroupBarlineStyle`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"individual\","]
#[doc = "    \"instrument\","]
#[doc = "    \"unified\","]
#[doc = "    \"mensurstrich\""]
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
pub enum StaffGroupBarlineStyle {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "instrument")]
    Instrument,
    #[serde(rename = "unified")]
    Unified,
    #[serde(rename = "mensurstrich")]
    Mensurstrich,
}
impl ::std::fmt::Display for StaffGroupBarlineStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Individual => f.write_str("individual"),
            Self::Instrument => f.write_str("instrument"),
            Self::Unified => f.write_str("unified"),
            Self::Mensurstrich => f.write_str("mensurstrich"),
        }
    }
}
impl ::std::str::FromStr for StaffGroupBarlineStyle {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "individual" => Ok(Self::Individual),
            "instrument" => Ok(Self::Instrument),
            "unified" => Ok(Self::Unified),
            "mensurstrich" => Ok(Self::Mensurstrich),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StaffGroupBarlineStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StaffGroupBarlineStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StaffGroupBarlineStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StaffLabel`"]
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
pub struct StaffLabel(pub ::std::string::String);
impl ::std::ops::Deref for StaffLabel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StaffLabel> for ::std::string::String {
    fn from(value: StaffLabel) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for StaffLabel {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffLabel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StaffLabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`StaffLabelref`"]
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
pub struct StaffLabelref(pub ::std::string::String);
impl ::std::ops::Deref for StaffLabelref {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StaffLabelref> for ::std::string::String {
    fn from(value: StaffLabelref) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for StaffLabelref {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffLabelref {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StaffLabelref {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`StaffNumber`"]
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
pub struct StaffNumber(pub i64);
impl ::std::ops::Deref for StaffNumber {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<StaffNumber> for i64 {
    fn from(value: StaffNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for StaffNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffNumber {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for StaffNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for StaffNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for StaffNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`StaffPosition`"]
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
pub struct StaffPosition(pub i64);
impl ::std::ops::Deref for StaffPosition {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<StaffPosition> for i64 {
    fn from(value: StaffPosition) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for StaffPosition {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StaffPosition {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for StaffPosition {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for StaffPosition {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for StaffPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`StaffSource`"]
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
#[doc = "    \"part\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"label\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-label\""]
#[doc = "    },"]
#[doc = "    \"labelref\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-labelref\""]
#[doc = "    },"]
#[doc = "    \"part\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"stem\": {"]
#[doc = "      \"$ref\": \"#/$defs/stem-direction\""]
#[doc = "    },"]
#[doc = "    \"voice\": {"]
#[doc = "      \"$ref\": \"#/$defs/voice-name\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StaffSource {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<StaffLabel>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub labelref: ::std::option::Option<StaffLabelref>,
    pub part: Id,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stem: ::std::option::Option<StemDirection>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub voice: ::std::option::Option<VoiceName>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl StaffSource {
    pub fn builder() -> builder::StaffSource {
        Default::default()
    }
}
#[doc = "`StaffSources`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/staff-source\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StaffSources(pub ::std::vec::Vec<StaffSource>);
impl ::std::ops::Deref for StaffSources {
    type Target = ::std::vec::Vec<StaffSource>;
    fn deref(&self) -> &::std::vec::Vec<StaffSource> {
        &self.0
    }
}
impl ::std::convert::From<StaffSources> for ::std::vec::Vec<StaffSource> {
    fn from(value: StaffSources) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<StaffSource>> for StaffSources {
    fn from(value: ::std::vec::Vec<StaffSource>) -> Self {
        Self(value)
    }
}
#[doc = "`StaffSymbol`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"bracket\","]
#[doc = "    \"brace\","]
#[doc = "    \"noSymbol\""]
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
pub enum StaffSymbol {
    #[serde(rename = "bracket")]
    Bracket,
    #[serde(rename = "brace")]
    Brace,
    #[serde(rename = "noSymbol")]
    NoSymbol,
}
impl ::std::fmt::Display for StaffSymbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bracket => f.write_str("bracket"),
            Self::Brace => f.write_str("brace"),
            Self::NoSymbol => f.write_str("noSymbol"),
        }
    }
}
impl ::std::str::FromStr for StaffSymbol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bracket" => Ok(Self::Bracket),
            "brace" => Ok(Self::Brace),
            "noSymbol" => Ok(Self::NoSymbol),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StaffSymbol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StaffSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StaffSymbol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StemDirection`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"up\","]
#[doc = "    \"down\""]
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
pub enum StemDirection {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}
impl ::std::fmt::Display for StemDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => f.write_str("up"),
            Self::Down => f.write_str("down"),
        }
    }
}
impl ::std::str::FromStr for StemDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StemDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StemDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StemDirection {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "`StressMarking`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StressMarking(pub GlobalAttrs);
impl ::std::ops::Deref for StressMarking {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<StressMarking> for GlobalAttrs {
    fn from(value: StressMarking) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for StressMarking {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
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
#[doc = "`StrongAccent`"]
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
#[doc = "    \"pointing\": {"]
#[doc = "      \"$ref\": \"#/$defs/up-or-down\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StrongAccent {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pointing: ::std::option::Option<UpOrDown>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for StrongAccent {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            pointing: Default::default(),
            x: Default::default(),
        }
    }
}
impl StrongAccent {
    pub fn builder() -> builder::StrongAccent {
        Default::default()
    }
}
#[doc = "`Support`"]
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
#[doc = "    \"useAccidentalDisplay\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"useBeams\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Support {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "useAccidentalDisplay",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_accidental_display: ::std::option::Option<bool>,
    #[serde(
        rename = "useBeams",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub use_beams: ::std::option::Option<bool>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Support {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            use_accidental_display: Default::default(),
            use_beams: Default::default(),
            x: Default::default(),
        }
    }
}
impl Support {
    pub fn builder() -> builder::Support {
        Default::default()
    }
}
#[doc = "`System`"]
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
#[doc = "    \"measure\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"layout\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"layoutChanges\": {"]
#[doc = "      \"$ref\": \"#/$defs/layout-changes\""]
#[doc = "    },"]
#[doc = "    \"measure\": {"]
#[doc = "      \"$ref\": \"#/$defs/measure-number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct System {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub layout: ::std::option::Option<Id>,
    #[serde(
        rename = "layoutChanges",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub layout_changes: ::std::option::Option<LayoutChanges>,
    pub measure: MeasureNumber,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl System {
    pub fn builder() -> builder::System {
        Default::default()
    }
}
#[doc = "`SystemLayout`"]
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
#[doc = "      \"$ref\": \"#/$defs/system-layout-content\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SystemLayout {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub content: SystemLayoutContent,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl SystemLayout {
    pub fn builder() -> builder::SystemLayout {
        Default::default()
    }
}
#[doc = "`SystemLayoutContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"anyOf\": ["]
#[doc = "      {"]
#[doc = "        \"$ref\": \"#/$defs/staff-group\""]
#[doc = "      },"]
#[doc = "      {"]
#[doc = "        \"$ref\": \"#/$defs/staff\""]
#[doc = "      }"]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SystemLayoutContent(pub ::std::vec::Vec<SystemLayoutContentItem>);
impl ::std::ops::Deref for SystemLayoutContent {
    type Target = ::std::vec::Vec<SystemLayoutContentItem>;
    fn deref(&self) -> &::std::vec::Vec<SystemLayoutContentItem> {
        &self.0
    }
}
impl ::std::convert::From<SystemLayoutContent> for ::std::vec::Vec<SystemLayoutContentItem> {
    fn from(value: SystemLayoutContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<SystemLayoutContentItem>> for SystemLayoutContent {
    fn from(value: ::std::vec::Vec<SystemLayoutContentItem>) -> Self {
        Self(value)
    }
}
#[doc = "`SystemLayoutContentItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/staff-group\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/staff\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SystemLayoutContentItem {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub staff_group: ::std::option::Option<StaffGroup>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub staff: ::std::option::Option<Staff>,
}
impl ::std::default::Default for SystemLayoutContentItem {
    fn default() -> Self {
        Self {
            staff_group: Default::default(),
            staff: Default::default(),
        }
    }
}
impl SystemLayoutContentItem {
    pub fn builder() -> builder::SystemLayoutContentItem {
        Default::default()
    }
}
#[doc = "`Systems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/system\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Systems(pub ::std::vec::Vec<System>);
impl ::std::ops::Deref for Systems {
    type Target = ::std::vec::Vec<System>;
    fn deref(&self) -> &::std::vec::Vec<System> {
        &self.0
    }
}
impl ::std::convert::From<Systems> for ::std::vec::Vec<System> {
    fn from(value: Systems) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<System>> for Systems {
    fn from(value: ::std::vec::Vec<System>) -> Self {
        Self(value)
    }
}
#[doc = "`Tempo`"]
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
#[doc = "    \"bpm\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bpm\": {"]
#[doc = "      \"$ref\": \"#/$defs/bpm\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"$ref\": \"#/$defs/rhythmic-position\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tempo {
    pub bpm: Bpm,
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<RhythmicPosition>,
    pub value: NoteValue,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Tempo {
    pub fn builder() -> builder::Tempo {
        Default::default()
    }
}
#[doc = "`Tempos`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/tempo\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Tempos(pub ::std::vec::Vec<Tempo>);
impl ::std::ops::Deref for Tempos {
    type Target = ::std::vec::Vec<Tempo>;
    fn deref(&self) -> &::std::vec::Vec<Tempo> {
        &self.0
    }
}
impl ::std::convert::From<Tempos> for ::std::vec::Vec<Tempo> {
    fn from(value: Tempos) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Tempo>> for Tempos {
    fn from(value: ::std::vec::Vec<Tempo>) -> Self {
        Self(value)
    }
}
#[doc = "`Tenuto`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Tenuto(pub GlobalAttrs);
impl ::std::ops::Deref for Tenuto {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<Tenuto> for GlobalAttrs {
    fn from(value: Tenuto) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for Tenuto {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
    }
}
#[doc = "`Tie`"]
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
#[doc = "    \"lv\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"side\": {"]
#[doc = "      \"$ref\": \"#/$defs/slur-side\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"targetType\": {"]
#[doc = "      \"$ref\": \"#/$defs/tie-target-type\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tie {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lv: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub side: ::std::option::Option<SlurSide>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<Id>,
    #[serde(
        rename = "targetType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_type: ::std::option::Option<TieTargetType>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Tie {
    fn default() -> Self {
        Self {
            c: Default::default(),
            id: Default::default(),
            lv: Default::default(),
            side: Default::default(),
            target: Default::default(),
            target_type: Default::default(),
            x: Default::default(),
        }
    }
}
impl Tie {
    pub fn builder() -> builder::Tie {
        Default::default()
    }
}
#[doc = "`TieList`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/tie\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TieList(pub ::std::vec::Vec<Tie>);
impl ::std::ops::Deref for TieList {
    type Target = ::std::vec::Vec<Tie>;
    fn deref(&self) -> &::std::vec::Vec<Tie> {
        &self.0
    }
}
impl ::std::convert::From<TieList> for ::std::vec::Vec<Tie> {
    fn from(value: TieList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Tie>> for TieList {
    fn from(value: ::std::vec::Vec<Tie>) -> Self {
        Self(value)
    }
}
#[doc = "`TieTargetType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"nextNote\","]
#[doc = "    \"crossVoice\","]
#[doc = "    \"arpeggio\","]
#[doc = "    \"crossJump\""]
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
pub enum TieTargetType {
    #[serde(rename = "nextNote")]
    NextNote,
    #[serde(rename = "crossVoice")]
    CrossVoice,
    #[serde(rename = "arpeggio")]
    Arpeggio,
    #[serde(rename = "crossJump")]
    CrossJump,
}
impl ::std::fmt::Display for TieTargetType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NextNote => f.write_str("nextNote"),
            Self::CrossVoice => f.write_str("crossVoice"),
            Self::Arpeggio => f.write_str("arpeggio"),
            Self::CrossJump => f.write_str("crossJump"),
        }
    }
}
impl ::std::str::FromStr for TieTargetType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "nextNote" => Ok(Self::NextNote),
            "crossVoice" => Ok(Self::CrossVoice),
            "arpeggio" => Ok(Self::Arpeggio),
            "crossJump" => Ok(Self::CrossJump),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TieTargetType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TieTargetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TieTargetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Time`"]
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
#[doc = "    \"count\","]
#[doc = "    \"unit\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"count\": {"]
#[doc = "      \"$ref\": \"#/$defs/positive-integer\""]
#[doc = "    },"]
#[doc = "    \"display\": {"]
#[doc = "      \"$ref\": \"#/$defs/time-signature-display\""]
#[doc = "    },"]
#[doc = "    \"unit\": {"]
#[doc = "      \"$ref\": \"#/$defs/time-signature-unit\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Time {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    pub count: PositiveInteger,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub display: ::std::option::Option<TimeSignatureDisplay>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub unit: TimeSignatureUnit,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Time {
    pub fn builder() -> builder::Time {
        Default::default()
    }
}
#[doc = "`TimeSignatureDisplay`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"common\","]
#[doc = "    \"cut\""]
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
pub enum TimeSignatureDisplay {
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "cut")]
    Cut,
}
impl ::std::fmt::Display for TimeSignatureDisplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Common => f.write_str("common"),
            Self::Cut => f.write_str("cut"),
        }
    }
}
impl ::std::str::FromStr for TimeSignatureDisplay {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "common" => Ok(Self::Common),
            "cut" => Ok(Self::Cut),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TimeSignatureDisplay {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TimeSignatureDisplay {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TimeSignatureDisplay {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`TimeSignatureUnit`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"enum\": ["]
#[doc = "    1,"]
#[doc = "    2,"]
#[doc = "    4,"]
#[doc = "    8,"]
#[doc = "    16,"]
#[doc = "    32,"]
#[doc = "    64,"]
#[doc = "    128"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TimeSignatureUnit(i64);
impl ::std::ops::Deref for TimeSignatureUnit {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<TimeSignatureUnit> for i64 {
    fn from(value: TimeSignatureUnit) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i64> for TimeSignatureUnit {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64, 4_i64, 8_i64, 16_i64, 32_i64, 64_i64, 128_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for TimeSignatureUnit {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`TremoloSequenceContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#/$defs/event\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TremoloSequenceContent(pub ::std::vec::Vec<Event>);
impl ::std::ops::Deref for TremoloSequenceContent {
    type Target = ::std::vec::Vec<Event>;
    fn deref(&self) -> &::std::vec::Vec<Event> {
        &self.0
    }
}
impl ::std::convert::From<TremoloSequenceContent> for ::std::vec::Vec<Event> {
    fn from(value: TremoloSequenceContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Event>> for TremoloSequenceContent {
    fn from(value: ::std::vec::Vec<Event>) -> Self {
        Self(value)
    }
}
#[doc = "`TremoloSingle`"]
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
#[doc = "    \"marks\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"marks\": {"]
#[doc = "      \"$ref\": \"#/$defs/positive-integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TremoloSingle {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    pub marks: PositiveInteger,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl TremoloSingle {
    pub fn builder() -> builder::TremoloSingle {
        Default::default()
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
#[doc = "    \"outer\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bracket\": {"]
#[doc = "      \"$ref\": \"#/$defs/yes-no-auto\""]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/$defs/sequence-content\""]
#[doc = "    },"]
#[doc = "    \"inner\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value-quantity\""]
#[doc = "    },"]
#[doc = "    \"orient\": {"]
#[doc = "      \"$ref\": \"#/$defs/orientation\""]
#[doc = "    },"]
#[doc = "    \"outer\": {"]
#[doc = "      \"$ref\": \"#/$defs/note-value-quantity\""]
#[doc = "    },"]
#[doc = "    \"showNumber\": {"]
#[doc = "      \"$ref\": \"#/$defs/tuplet-display-setting\""]
#[doc = "    },"]
#[doc = "    \"showValue\": {"]
#[doc = "      \"$ref\": \"#/$defs/tuplet-display-setting\""]
#[doc = "    },"]
#[doc = "    \"staff\": {"]
#[doc = "      \"$ref\": \"#/$defs/staff-number\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/literal-string-tuplet\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tuplet {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bracket: ::std::option::Option<YesNoAuto>,
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orient: ::std::option::Option<Orientation>,
    pub outer: NoteValueQuantity,
    #[serde(
        rename = "showNumber",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_number: ::std::option::Option<TupletDisplaySetting>,
    #[serde(
        rename = "showValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_value: ::std::option::Option<TupletDisplaySetting>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub staff: ::std::option::Option<StaffNumber>,
    #[serde(rename = "type")]
    pub type_: LiteralStringTuplet,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl Tuplet {
    pub fn builder() -> builder::Tuplet {
        Default::default()
    }
}
#[doc = "`TupletDisplaySetting`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"noNumber\","]
#[doc = "    \"inner\","]
#[doc = "    \"both\""]
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
pub enum TupletDisplaySetting {
    #[serde(rename = "noNumber")]
    NoNumber,
    #[serde(rename = "inner")]
    Inner,
    #[serde(rename = "both")]
    Both,
}
impl ::std::fmt::Display for TupletDisplaySetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NoNumber => f.write_str("noNumber"),
            Self::Inner => f.write_str("inner"),
            Self::Both => f.write_str("both"),
        }
    }
}
impl ::std::str::FromStr for TupletDisplaySetting {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "noNumber" => Ok(Self::NoNumber),
            "inner" => Ok(Self::Inner),
            "both" => Ok(Self::Both),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TupletDisplaySetting {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TupletDisplaySetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TupletDisplaySetting {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`UnstressMarking`"]
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
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct UnstressMarking(pub GlobalAttrs);
impl ::std::ops::Deref for UnstressMarking {
    type Target = GlobalAttrs;
    fn deref(&self) -> &GlobalAttrs {
        &self.0
    }
}
impl ::std::convert::From<UnstressMarking> for GlobalAttrs {
    fn from(value: UnstressMarking) -> Self {
        value.0
    }
}
impl ::std::convert::From<GlobalAttrs> for UnstressMarking {
    fn from(value: GlobalAttrs) -> Self {
        Self(value)
    }
}
#[doc = "`UpOrDown`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"up\","]
#[doc = "    \"down\""]
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
pub enum UpOrDown {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}
impl ::std::fmt::Display for UpOrDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Up => f.write_str("up"),
            Self::Down => f.write_str("down"),
        }
    }
}
impl ::std::str::FromStr for UpOrDown {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UpOrDown {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UpOrDown {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UpOrDown {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`VendorDict`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct VendorDict(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for VendorDict {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<VendorDict>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: VendorDict) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for VendorDict
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`VendorExtensions`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"patternProperties\": {"]
#[doc = "    \"^.*$\": {"]
#[doc = "      \"$ref\": \"#/$defs/vendor-dict\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct VendorExtensions(pub ::std::collections::HashMap<VendorExtensionsKey, VendorDict>);
impl ::std::ops::Deref for VendorExtensions {
    type Target = ::std::collections::HashMap<VendorExtensionsKey, VendorDict>;
    fn deref(&self) -> &::std::collections::HashMap<VendorExtensionsKey, VendorDict> {
        &self.0
    }
}
impl ::std::convert::From<VendorExtensions>
    for ::std::collections::HashMap<VendorExtensionsKey, VendorDict>
{
    fn from(value: VendorExtensions) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<VendorExtensionsKey, VendorDict>>
    for VendorExtensions
{
    fn from(value: ::std::collections::HashMap<VendorExtensionsKey, VendorDict>) -> Self {
        Self(value)
    }
}
#[doc = "`VendorExtensionsKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^.*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VendorExtensionsKey(::std::string::String);
impl ::std::ops::Deref for VendorExtensionsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VendorExtensionsKey> for ::std::string::String {
    fn from(value: VendorExtensionsKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VendorExtensionsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^.*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^.*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VendorExtensionsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VendorExtensionsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VendorExtensionsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VendorExtensionsKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`VersionNumber`"]
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
pub struct VersionNumber(pub i64);
impl ::std::ops::Deref for VersionNumber {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<VersionNumber> for i64 {
    fn from(value: VersionNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for VersionNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for VersionNumber {
    type Err = <i64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for VersionNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for VersionNumber {
    type Error = <i64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for VersionNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`VoiceName`"]
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
pub struct VoiceName(pub ::std::string::String);
impl ::std::ops::Deref for VoiceName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VoiceName> for ::std::string::String {
    fn from(value: VoiceName) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for VoiceName {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for VoiceName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for VoiceName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Written`"]
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
#[doc = "    \"diatonicDelta\": {"]
#[doc = "      \"$ref\": \"#/$defs/integer-signed\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"unevaluatedProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Written {
    #[serde(
        rename = "_c",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub c: ::std::option::Option<String>,
    #[serde(
        rename = "diatonicDelta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diatonic_delta: ::std::option::Option<IntegerSigned>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[serde(
        rename = "_x",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub x: ::std::option::Option<VendorExtensions>,
}
impl ::std::default::Default for Written {
    fn default() -> Self {
        Self {
            c: Default::default(),
            diatonic_delta: Default::default(),
            id: Default::default(),
            x: Default::default(),
        }
    }
}
impl Written {
    pub fn builder() -> builder::Written {
        Default::default()
    }
}
#[doc = "`YesNoAuto`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"yes\","]
#[doc = "    \"no\","]
#[doc = "    \"auto\""]
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
pub enum YesNoAuto {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for YesNoAuto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Yes => f.write_str("yes"),
            Self::No => f.write_str("no"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for YesNoAuto {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "yes" => Ok(Self::Yes),
            "no" => Ok(Self::No),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for YesNoAuto {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for YesNoAuto {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for YesNoAuto {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Accent {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        pointing:
            ::std::result::Result<::std::option::Option<super::UpOrDown>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Accent {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                pointing: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Accent {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn pointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UpOrDown>>,
            T::Error: ::std::fmt::Display,
        {
            self.pointing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pointing: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Accent> for super::Accent {
        type Error = super::error::ConversionError;
        fn try_from(value: Accent) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                pointing: value.pointing?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Accent> for Accent {
        fn from(value: super::Accent) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                pointing: Ok(value.pointing),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AccidentalDisplay {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        enclosure: ::std::result::Result<
            ::std::option::Option<super::AccidentalEnclosure>,
            ::std::string::String,
        >,
        force: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        show: ::std::result::Result<bool, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AccidentalDisplay {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                enclosure: Ok(Default::default()),
                force: Ok(Default::default()),
                id: Ok(Default::default()),
                show: Err("no value supplied for show".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl AccidentalDisplay {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn enclosure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AccidentalEnclosure>>,
            T::Error: ::std::fmt::Display,
        {
            self.enclosure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enclosure: {e}"));
            self
        }
        pub fn force<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.force = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for force: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn show<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.show = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AccidentalDisplay> for super::AccidentalDisplay {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AccidentalDisplay,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                enclosure: value.enclosure?,
                force: value.force?,
                id: value.id?,
                show: value.show?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::AccidentalDisplay> for AccidentalDisplay {
        fn from(value: super::AccidentalDisplay) -> Self {
            Self {
                c: Ok(value.c),
                enclosure: Ok(value.enclosure),
                force: Ok(value.force),
                id: Ok(value.id),
                show: Ok(value.show),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AccidentalEnclosure {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        symbol: ::std::result::Result<super::AccidentalEnclosureSymbol, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AccidentalEnclosure {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                symbol: Err("no value supplied for symbol".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl AccidentalEnclosure {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccidentalEnclosureSymbol>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AccidentalEnclosure> for super::AccidentalEnclosure {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AccidentalEnclosure,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                symbol: value.symbol?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::AccidentalEnclosure> for AccidentalEnclosure {
        fn from(value: super::AccidentalEnclosure) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                symbol: Ok(value.symbol),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Barline {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        type_: ::std::result::Result<super::BarlineType, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Barline {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Barline {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BarlineType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Barline> for super::Barline {
        type Error = super::error::ConversionError;
        fn try_from(value: Barline) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Barline> for Barline {
        fn from(value: super::Barline) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Beam {
        beams: ::std::result::Result<::std::option::Option<super::BeamList>, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        direction: ::std::result::Result<
            ::std::option::Option<super::BeamHookDirection>,
            ::std::string::String,
        >,
        events: ::std::result::Result<super::IdList, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Beam {
        fn default() -> Self {
            Self {
                beams: Ok(Default::default()),
                c: Ok(Default::default()),
                direction: Ok(Default::default()),
                events: Err("no value supplied for events".to_string()),
                id: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Beam {
        pub fn beams<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BeamList>>,
            T::Error: ::std::fmt::Display,
        {
            self.beams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for beams: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BeamHookDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for direction: {e}"));
            self
        }
        pub fn events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IdList>,
            T::Error: ::std::fmt::Display,
        {
            self.events = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for events: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Beam> for super::Beam {
        type Error = super::error::ConversionError;
        fn try_from(value: Beam) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                beams: value.beams?,
                c: value.c?,
                direction: value.direction?,
                events: value.events?,
                id: value.id?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Beam> for Beam {
        fn from(value: super::Beam) -> Self {
            Self {
                beams: Ok(value.beams),
                c: Ok(value.c),
                direction: Ok(value.direction),
                events: Ok(value.events),
                id: Ok(value.id),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BreathMark {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        symbol: ::std::result::Result<
            ::std::option::Option<super::BreathMarkSymbol>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BreathMark {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                symbol: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl BreathMark {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BreathMarkSymbol>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BreathMark> for super::BreathMark {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BreathMark,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                symbol: value.symbol?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::BreathMark> for BreathMark {
        fn from(value: super::BreathMark) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                symbol: Ok(value.symbol),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Clef {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        color:
            ::std::result::Result<::std::option::Option<super::SimpleColor>, ::std::string::String>,
        glyph:
            ::std::result::Result<::std::option::Option<super::SmuflGlyph>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        octave: ::std::result::Result<
            ::std::option::Option<super::OttavaAmountOrZero>,
            ::std::string::String,
        >,
        show_octave: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        sign: ::std::result::Result<super::ClefSign, ::std::string::String>,
        staff_position: ::std::result::Result<super::StaffPosition, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Clef {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                color: Ok(Default::default()),
                glyph: Ok(Default::default()),
                id: Ok(Default::default()),
                octave: Ok(Default::default()),
                show_octave: Ok(Default::default()),
                sign: Err("no value supplied for sign".to_string()),
                staff_position: Err("no value supplied for staff_position".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Clef {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SimpleColor>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn glyph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflGlyph>>,
            T::Error: ::std::fmt::Display,
        {
            self.glyph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for glyph: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn octave<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OttavaAmountOrZero>>,
            T::Error: ::std::fmt::Display,
        {
            self.octave = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for octave: {e}"));
            self
        }
        pub fn show_octave<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.show_octave = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show_octave: {e}"));
            self
        }
        pub fn sign<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ClefSign>,
            T::Error: ::std::fmt::Display,
        {
            self.sign = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sign: {e}"));
            self
        }
        pub fn staff_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaffPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_position: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Clef> for super::Clef {
        type Error = super::error::ConversionError;
        fn try_from(value: Clef) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                color: value.color?,
                glyph: value.glyph?,
                id: value.id?,
                octave: value.octave?,
                show_octave: value.show_octave?,
                sign: value.sign?,
                staff_position: value.staff_position?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Clef> for Clef {
        fn from(value: super::Clef) -> Self {
            Self {
                c: Ok(value.c),
                color: Ok(value.color),
                glyph: Ok(value.glyph),
                id: Ok(value.id),
                octave: Ok(value.octave),
                show_octave: Ok(value.show_octave),
                sign: Ok(value.sign),
                staff_position: Ok(value.staff_position),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Dynamic {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        glyph:
            ::std::result::Result<::std::option::Option<super::SmuflGlyph>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        position: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        value: ::std::result::Result<super::DynamicType, ::std::string::String>,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Dynamic {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                glyph: Ok(Default::default()),
                id: Ok(Default::default()),
                position: Err("no value supplied for position".to_string()),
                staff: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
                voice: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Dynamic {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn glyph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflGlyph>>,
            T::Error: ::std::fmt::Display,
        {
            self.glyph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for glyph: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DynamicType>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Dynamic> for super::Dynamic {
        type Error = super::error::ConversionError;
        fn try_from(value: Dynamic) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                glyph: value.glyph?,
                id: value.id?,
                position: value.position?,
                staff: value.staff?,
                value: value.value?,
                voice: value.voice?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Dynamic> for Dynamic {
        fn from(value: super::Dynamic) -> Self {
            Self {
                c: Ok(value.c),
                glyph: Ok(value.glyph),
                id: Ok(value.id),
                position: Ok(value.position),
                staff: Ok(value.staff),
                value: Ok(value.value),
                voice: Ok(value.voice),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Ending {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        duration: ::std::result::Result<super::EndingDuration, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        numbers: ::std::result::Result<
            ::std::option::Option<super::EndingNumbers>,
            ::std::string::String,
        >,
        open:
            ::std::result::Result<::std::option::Option<super::EndingOpen>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Ending {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                color: Ok(Default::default()),
                duration: Err("no value supplied for duration".to_string()),
                id: Ok(Default::default()),
                numbers: Ok(Default::default()),
                open: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Ending {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EndingDuration>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn numbers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EndingNumbers>>,
            T::Error: ::std::fmt::Display,
        {
            self.numbers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for numbers: {e}"));
            self
        }
        pub fn open<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EndingOpen>>,
            T::Error: ::std::fmt::Display,
        {
            self.open = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for open: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Ending> for super::Ending {
        type Error = super::error::ConversionError;
        fn try_from(value: Ending) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                color: value.color?,
                duration: value.duration?,
                id: value.id?,
                numbers: value.numbers?,
                open: value.open?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Ending> for Ending {
        fn from(value: super::Ending) -> Self {
            Self {
                c: Ok(value.c),
                color: Ok(value.color),
                duration: Ok(value.duration),
                id: Ok(value.id),
                numbers: Ok(value.numbers),
                open: Ok(value.open),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Event {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        duration:
            ::std::result::Result<::std::option::Option<super::NoteValue>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        kit_notes:
            ::std::result::Result<::std::option::Option<super::KitNotes>, ::std::string::String>,
        lyrics: ::std::result::Result<::std::option::Option<super::Lyrics>, ::std::string::String>,
        markings: ::std::result::Result<
            ::std::option::Option<super::EventMarkings>,
            ::std::string::String,
        >,
        measure: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        notes: ::std::result::Result<::std::option::Option<super::Notes>, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        rest: ::std::result::Result<::std::option::Option<super::Rest>, ::std::string::String>,
        slurs: ::std::result::Result<::std::option::Option<super::SlurList>, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        stem_direction: ::std::result::Result<
            ::std::option::Option<super::StemDirection>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::LiteralStringEvent>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Event {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                duration: Ok(Default::default()),
                id: Ok(Default::default()),
                kit_notes: Ok(Default::default()),
                lyrics: Ok(Default::default()),
                markings: Ok(Default::default()),
                measure: Ok(Default::default()),
                notes: Ok(Default::default()),
                orient: Ok(Default::default()),
                rest: Ok(Default::default()),
                slurs: Ok(Default::default()),
                staff: Ok(Default::default()),
                stem_direction: Ok(Default::default()),
                type_: Ok(Default::default()),
                x: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NoteValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn kit_notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::KitNotes>>,
            T::Error: ::std::fmt::Display,
        {
            self.kit_notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kit_notes: {e}"));
            self
        }
        pub fn lyrics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Lyrics>>,
            T::Error: ::std::fmt::Display,
        {
            self.lyrics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lyrics: {e}"));
            self
        }
        pub fn markings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventMarkings>>,
            T::Error: ::std::fmt::Display,
        {
            self.markings = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for markings: {e}"));
            self
        }
        pub fn measure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.measure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measure: {e}"));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Notes>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {e}"));
            self
        }
        pub fn rest<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Rest>>,
            T::Error: ::std::fmt::Display,
        {
            self.rest = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rest: {e}"));
            self
        }
        pub fn slurs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SlurList>>,
            T::Error: ::std::fmt::Display,
        {
            self.slurs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for slurs: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn stem_direction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StemDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.stem_direction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stem_direction: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LiteralStringEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
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
                kit_notes: value.kit_notes?,
                lyrics: value.lyrics?,
                markings: value.markings?,
                measure: value.measure?,
                notes: value.notes?,
                orient: value.orient?,
                rest: value.rest?,
                slurs: value.slurs?,
                staff: value.staff?,
                stem_direction: value.stem_direction?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Event> for Event {
        fn from(value: super::Event) -> Self {
            Self {
                c: Ok(value.c),
                duration: Ok(value.duration),
                id: Ok(value.id),
                kit_notes: Ok(value.kit_notes),
                lyrics: Ok(value.lyrics),
                markings: Ok(value.markings),
                measure: Ok(value.measure),
                notes: Ok(value.notes),
                orient: Ok(value.orient),
                rest: Ok(value.rest),
                slurs: Ok(value.slurs),
                staff: Ok(value.staff),
                stem_direction: Ok(value.stem_direction),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventLyricLine {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        text: ::std::result::Result<super::String, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::EventLyricLineType>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventLyricLine {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl EventLyricLine {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventLyricLineType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventLyricLine> for super::EventLyricLine {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventLyricLine,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                text: value.text?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::EventLyricLine> for EventLyricLine {
        fn from(value: super::EventLyricLine) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                text: Ok(value.text),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventMarkings {
        accent: ::std::result::Result<::std::option::Option<super::Accent>, ::std::string::String>,
        breath:
            ::std::result::Result<::std::option::Option<super::BreathMark>, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        soft_accent:
            ::std::result::Result<::std::option::Option<super::SoftAccent>, ::std::string::String>,
        spiccato:
            ::std::result::Result<::std::option::Option<super::Spiccato>, ::std::string::String>,
        staccatissimo: ::std::result::Result<
            ::std::option::Option<super::Staccatissimo>,
            ::std::string::String,
        >,
        staccato:
            ::std::result::Result<::std::option::Option<super::Staccato>, ::std::string::String>,
        stress: ::std::result::Result<
            ::std::option::Option<super::StressMarking>,
            ::std::string::String,
        >,
        strong_accent: ::std::result::Result<
            ::std::option::Option<super::StrongAccent>,
            ::std::string::String,
        >,
        tenuto: ::std::result::Result<::std::option::Option<super::Tenuto>, ::std::string::String>,
        tremolo: ::std::result::Result<
            ::std::option::Option<super::TremoloSingle>,
            ::std::string::String,
        >,
        unstress: ::std::result::Result<
            ::std::option::Option<super::UnstressMarking>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventMarkings {
        fn default() -> Self {
            Self {
                accent: Ok(Default::default()),
                breath: Ok(Default::default()),
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                soft_accent: Ok(Default::default()),
                spiccato: Ok(Default::default()),
                staccatissimo: Ok(Default::default()),
                staccato: Ok(Default::default()),
                stress: Ok(Default::default()),
                strong_accent: Ok(Default::default()),
                tenuto: Ok(Default::default()),
                tremolo: Ok(Default::default()),
                unstress: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl EventMarkings {
        pub fn accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Accent>>,
            T::Error: ::std::fmt::Display,
        {
            self.accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accent: {e}"));
            self
        }
        pub fn breath<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BreathMark>>,
            T::Error: ::std::fmt::Display,
        {
            self.breath = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for breath: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn soft_accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SoftAccent>>,
            T::Error: ::std::fmt::Display,
        {
            self.soft_accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for soft_accent: {e}"));
            self
        }
        pub fn spiccato<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Spiccato>>,
            T::Error: ::std::fmt::Display,
        {
            self.spiccato = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spiccato: {e}"));
            self
        }
        pub fn staccatissimo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Staccatissimo>>,
            T::Error: ::std::fmt::Display,
        {
            self.staccatissimo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staccatissimo: {e}"));
            self
        }
        pub fn staccato<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Staccato>>,
            T::Error: ::std::fmt::Display,
        {
            self.staccato = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staccato: {e}"));
            self
        }
        pub fn stress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StressMarking>>,
            T::Error: ::std::fmt::Display,
        {
            self.stress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stress: {e}"));
            self
        }
        pub fn strong_accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StrongAccent>>,
            T::Error: ::std::fmt::Display,
        {
            self.strong_accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strong_accent: {e}"));
            self
        }
        pub fn tenuto<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tenuto>>,
            T::Error: ::std::fmt::Display,
        {
            self.tenuto = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenuto: {e}"));
            self
        }
        pub fn tremolo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TremoloSingle>>,
            T::Error: ::std::fmt::Display,
        {
            self.tremolo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tremolo: {e}"));
            self
        }
        pub fn unstress<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UnstressMarking>>,
            T::Error: ::std::fmt::Display,
        {
            self.unstress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unstress: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventMarkings> for super::EventMarkings {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventMarkings,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accent: value.accent?,
                breath: value.breath?,
                c: value.c?,
                id: value.id?,
                soft_accent: value.soft_accent?,
                spiccato: value.spiccato?,
                staccatissimo: value.staccatissimo?,
                staccato: value.staccato?,
                stress: value.stress?,
                strong_accent: value.strong_accent?,
                tenuto: value.tenuto?,
                tremolo: value.tremolo?,
                unstress: value.unstress?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::EventMarkings> for EventMarkings {
        fn from(value: super::EventMarkings) -> Self {
            Self {
                accent: Ok(value.accent),
                breath: Ok(value.breath),
                c: Ok(value.c),
                id: Ok(value.id),
                soft_accent: Ok(value.soft_accent),
                spiccato: Ok(value.spiccato),
                staccatissimo: Ok(value.staccatissimo),
                staccato: Ok(value.staccato),
                stress: Ok(value.stress),
                strong_accent: Ok(value.strong_accent),
                tenuto: Ok(value.tenuto),
                tremolo: Ok(value.tremolo),
                unstress: Ok(value.unstress),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Fine {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        location: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Fine {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                color: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Err("no value supplied for location".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Fine {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Fine> for super::Fine {
        type Error = super::error::ConversionError;
        fn try_from(value: Fine) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                color: value.color?,
                id: value.id?,
                location: value.location?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Fine> for Fine {
        fn from(value: super::Fine) -> Self {
            Self {
                c: Ok(value.c),
                color: Ok(value.color),
                id: Ok(value.id),
                location: Ok(value.location),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Global {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        lyrics: ::std::result::Result<
            ::std::option::Option<super::LyricsGlobal>,
            ::std::string::String,
        >,
        measures: ::std::result::Result<super::MeasuresGlobal, ::std::string::String>,
        sounds: ::std::result::Result<
            ::std::option::Option<super::SoundsGlobal>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Global {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                lyrics: Ok(Default::default()),
                measures: Err("no value supplied for measures".to_string()),
                sounds: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Global {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn lyrics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LyricsGlobal>>,
            T::Error: ::std::fmt::Display,
        {
            self.lyrics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lyrics: {e}"));
            self
        }
        pub fn measures<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasuresGlobal>,
            T::Error: ::std::fmt::Display,
        {
            self.measures = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measures: {e}"));
            self
        }
        pub fn sounds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SoundsGlobal>>,
            T::Error: ::std::fmt::Display,
        {
            self.sounds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sounds: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Global> for super::Global {
        type Error = super::error::ConversionError;
        fn try_from(value: Global) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                lyrics: value.lyrics?,
                measures: value.measures?,
                sounds: value.sounds?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Global> for Global {
        fn from(value: super::Global) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                lyrics: Ok(value.lyrics),
                measures: Ok(value.measures),
                sounds: Ok(value.sounds),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GlobalAttrs {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GlobalAttrs {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                x: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
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
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::GlobalAttrs> for GlobalAttrs {
        fn from(value: super::GlobalAttrs) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Grace {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        content: ::std::result::Result<super::GraceSequenceContent, ::std::string::String>,
        grace_type:
            ::std::result::Result<::std::option::Option<super::GraceType>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        slash: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        type_: ::std::result::Result<super::LiteralStringGrace, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Grace {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                color: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                grace_type: Ok(Default::default()),
                id: Ok(Default::default()),
                slash: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Grace {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GraceSequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn grace_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GraceType>>,
            T::Error: ::std::fmt::Display,
        {
            self.grace_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grace_type: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn slash<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.slash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for slash: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiteralStringGrace>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Grace> for super::Grace {
        type Error = super::error::ConversionError;
        fn try_from(value: Grace) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                color: value.color?,
                content: value.content?,
                grace_type: value.grace_type?,
                id: value.id?,
                slash: value.slash?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Grace> for Grace {
        fn from(value: super::Grace) -> Self {
            Self {
                c: Ok(value.c),
                color: Ok(value.color),
                content: Ok(value.content),
                grace_type: Ok(value.grace_type),
                id: Ok(value.id),
                slash: Ok(value.slash),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Interval {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        half_steps: ::std::result::Result<super::IntegerSigned, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        staff_distance: ::std::result::Result<super::IntegerSigned, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Interval {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                half_steps: Err("no value supplied for half_steps".to_string()),
                id: Ok(Default::default()),
                staff_distance: Err("no value supplied for staff_distance".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Interval {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn half_steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IntegerSigned>,
            T::Error: ::std::fmt::Display,
        {
            self.half_steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for half_steps: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn staff_distance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IntegerSigned>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_distance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_distance: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Interval> for super::Interval {
        type Error = super::error::ConversionError;
        fn try_from(value: Interval) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                half_steps: value.half_steps?,
                id: value.id?,
                staff_distance: value.staff_distance?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Interval> for Interval {
        fn from(value: super::Interval) -> Self {
            Self {
                c: Ok(value.c),
                half_steps: Ok(value.half_steps),
                id: Ok(value.id),
                staff_distance: Ok(value.staff_distance),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Jump {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        location: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        type_: ::std::result::Result<super::JumpType, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Jump {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Err("no value supplied for location".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Jump {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JumpType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Jump> for super::Jump {
        type Error = super::error::ConversionError;
        fn try_from(value: Jump) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                location: value.location?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Jump> for Jump {
        fn from(value: super::Jump) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                location: Ok(value.location),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Key {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        fifths: ::std::result::Result<super::Fifths, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Key {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                color: Ok(Default::default()),
                fifths: Err("no value supplied for fifths".to_string()),
                id: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Key {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn fifths<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Fifths>,
            T::Error: ::std::fmt::Display,
        {
            self.fifths = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fifths: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Key> for super::Key {
        type Error = super::error::ConversionError;
        fn try_from(value: Key) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                color: value.color?,
                fifths: value.fifths?,
                id: value.id?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Key> for Key {
        fn from(value: super::Key) -> Self {
            Self {
                c: Ok(value.c),
                color: Ok(value.color),
                fifths: Ok(value.fifths),
                id: Ok(value.id),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct KitComponent {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        name: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        sound: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        staff_position: ::std::result::Result<super::StaffPosition, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for KitComponent {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                sound: Ok(Default::default()),
                staff_position: Err("no value supplied for staff_position".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl KitComponent {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn sound<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.sound = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sound: {e}"));
            self
        }
        pub fn staff_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaffPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_position: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<KitComponent> for super::KitComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: KitComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                name: value.name?,
                sound: value.sound?,
                staff_position: value.staff_position?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::KitComponent> for KitComponent {
        fn from(value: super::KitComponent) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                name: Ok(value.name),
                sound: Ok(value.sound),
                staff_position: Ok(value.staff_position),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct KitNote {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        kit_component: ::std::result::Result<super::KitComponentId, ::std::string::String>,
        perform: ::std::result::Result<
            ::std::option::Option<super::PerformOptions>,
            ::std::string::String,
        >,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        ties: ::std::result::Result<::std::option::Option<super::TieList>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for KitNote {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                kit_component: Err("no value supplied for kit_component".to_string()),
                perform: Ok(Default::default()),
                staff: Ok(Default::default()),
                ties: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl KitNote {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn kit_component<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::KitComponentId>,
            T::Error: ::std::fmt::Display,
        {
            self.kit_component = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kit_component: {e}"));
            self
        }
        pub fn perform<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PerformOptions>>,
            T::Error: ::std::fmt::Display,
        {
            self.perform = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for perform: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn ties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TieList>>,
            T::Error: ::std::fmt::Display,
        {
            self.ties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ties: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<KitNote> for super::KitNote {
        type Error = super::error::ConversionError;
        fn try_from(value: KitNote) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                kit_component: value.kit_component?,
                perform: value.perform?,
                staff: value.staff?,
                ties: value.ties?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::KitNote> for KitNote {
        fn from(value: super::KitNote) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                kit_component: Ok(value.kit_component),
                perform: Ok(value.perform),
                staff: Ok(value.staff),
                ties: Ok(value.ties),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayoutChange {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        layout: ::std::result::Result<super::Id, ::std::string::String>,
        location: ::std::result::Result<super::MeasureRhythmicPosition, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LayoutChange {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                layout: Err("no value supplied for layout".to_string()),
                location: Err("no value supplied for location".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl LayoutChange {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureRhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LayoutChange> for super::LayoutChange {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LayoutChange,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                layout: value.layout?,
                location: value.location?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::LayoutChange> for LayoutChange {
        fn from(value: super::LayoutChange) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                layout: Ok(value.layout),
                location: Ok(value.location),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LyricLineMetadata {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        label: ::std::result::Result<
            ::std::option::Option<super::LyricLineLabel>,
            ::std::string::String,
        >,
        lang: ::std::result::Result<
            ::std::option::Option<super::LanguageCode>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LyricLineMetadata {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                label: Ok(Default::default()),
                lang: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl LyricLineMetadata {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LyricLineLabel>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn lang<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanguageCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lang: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LyricLineMetadata> for super::LyricLineMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LyricLineMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                label: value.label?,
                lang: value.lang?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::LyricLineMetadata> for LyricLineMetadata {
        fn from(value: super::LyricLineMetadata) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                label: Ok(value.label),
                lang: Ok(value.lang),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Lyrics {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        lines: ::std::result::Result<
            ::std::option::Option<super::EventLyricLines>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Lyrics {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                lines: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Lyrics {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventLyricLines>>,
            T::Error: ::std::fmt::Display,
        {
            self.lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lines: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Lyrics> for super::Lyrics {
        type Error = super::error::ConversionError;
        fn try_from(value: Lyrics) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                lines: value.lines?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Lyrics> for Lyrics {
        fn from(value: super::Lyrics) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                lines: Ok(value.lines),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LyricsGlobal {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        line_metadata: ::std::result::Result<
            ::std::option::Option<super::LyricLinesMetadata>,
            ::std::string::String,
        >,
        line_order: ::std::result::Result<
            ::std::option::Option<super::LyricLineIdList>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LyricsGlobal {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                line_metadata: Ok(Default::default()),
                line_order: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl LyricsGlobal {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn line_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LyricLinesMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.line_metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_metadata: {e}"));
            self
        }
        pub fn line_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LyricLineIdList>>,
            T::Error: ::std::fmt::Display,
        {
            self.line_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_order: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<LyricsGlobal> for super::LyricsGlobal {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LyricsGlobal,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                line_metadata: value.line_metadata?,
                line_order: value.line_order?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::LyricsGlobal> for LyricsGlobal {
        fn from(value: super::LyricsGlobal) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                line_metadata: Ok(value.line_metadata),
                line_order: Ok(value.line_order),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MeasureGlobal {
        barline:
            ::std::result::Result<::std::option::Option<super::Barline>, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        ending: ::std::result::Result<::std::option::Option<super::Ending>, ::std::string::String>,
        fine: ::std::result::Result<::std::option::Option<super::Fine>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        index: ::std::result::Result<
            ::std::option::Option<super::MeasureNumber>,
            ::std::string::String,
        >,
        jump: ::std::result::Result<::std::option::Option<super::Jump>, ::std::string::String>,
        key: ::std::result::Result<::std::option::Option<super::Key>, ::std::string::String>,
        number: ::std::result::Result<
            ::std::option::Option<super::MeasureNumber>,
            ::std::string::String,
        >,
        repeat_end:
            ::std::result::Result<::std::option::Option<super::RepeatEnd>, ::std::string::String>,
        repeat_start:
            ::std::result::Result<::std::option::Option<super::RepeatStart>, ::std::string::String>,
        segno: ::std::result::Result<::std::option::Option<super::Segno>, ::std::string::String>,
        tempos: ::std::result::Result<::std::option::Option<super::Tempos>, ::std::string::String>,
        time: ::std::result::Result<::std::option::Option<super::Time>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MeasureGlobal {
        fn default() -> Self {
            Self {
                barline: Ok(Default::default()),
                c: Ok(Default::default()),
                ending: Ok(Default::default()),
                fine: Ok(Default::default()),
                id: Ok(Default::default()),
                index: Ok(Default::default()),
                jump: Ok(Default::default()),
                key: Ok(Default::default()),
                number: Ok(Default::default()),
                repeat_end: Ok(Default::default()),
                repeat_start: Ok(Default::default()),
                segno: Ok(Default::default()),
                tempos: Ok(Default::default()),
                time: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl MeasureGlobal {
        pub fn barline<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Barline>>,
            T::Error: ::std::fmt::Display,
        {
            self.barline = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for barline: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn ending<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Ending>>,
            T::Error: ::std::fmt::Display,
        {
            self.ending = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ending: {e}"));
            self
        }
        pub fn fine<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Fine>>,
            T::Error: ::std::fmt::Display,
        {
            self.fine = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fine: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MeasureNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for index: {e}"));
            self
        }
        pub fn jump<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Jump>>,
            T::Error: ::std::fmt::Display,
        {
            self.jump = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jump: {e}"));
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Key>>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {e}"));
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MeasureNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {e}"));
            self
        }
        pub fn repeat_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RepeatEnd>>,
            T::Error: ::std::fmt::Display,
        {
            self.repeat_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat_end: {e}"));
            self
        }
        pub fn repeat_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RepeatStart>>,
            T::Error: ::std::fmt::Display,
        {
            self.repeat_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat_start: {e}"));
            self
        }
        pub fn segno<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Segno>>,
            T::Error: ::std::fmt::Display,
        {
            self.segno = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segno: {e}"));
            self
        }
        pub fn tempos<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tempos>>,
            T::Error: ::std::fmt::Display,
        {
            self.tempos = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tempos: {e}"));
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Time>>,
            T::Error: ::std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MeasureGlobal> for super::MeasureGlobal {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MeasureGlobal,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                barline: value.barline?,
                c: value.c?,
                ending: value.ending?,
                fine: value.fine?,
                id: value.id?,
                index: value.index?,
                jump: value.jump?,
                key: value.key?,
                number: value.number?,
                repeat_end: value.repeat_end?,
                repeat_start: value.repeat_start?,
                segno: value.segno?,
                tempos: value.tempos?,
                time: value.time?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::MeasureGlobal> for MeasureGlobal {
        fn from(value: super::MeasureGlobal) -> Self {
            Self {
                barline: Ok(value.barline),
                c: Ok(value.c),
                ending: Ok(value.ending),
                fine: Ok(value.fine),
                id: Ok(value.id),
                index: Ok(value.index),
                jump: Ok(value.jump),
                key: Ok(value.key),
                number: Ok(value.number),
                repeat_end: Ok(value.repeat_end),
                repeat_start: Ok(value.repeat_start),
                segno: Ok(value.segno),
                tempos: Ok(value.tempos),
                time: Ok(value.time),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MeasureRhythmicPosition {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        measure: ::std::result::Result<super::MeasureNumber, ::std::string::String>,
        position: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MeasureRhythmicPosition {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                measure: Err("no value supplied for measure".to_string()),
                position: Err("no value supplied for position".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl MeasureRhythmicPosition {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn measure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.measure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measure: {e}"));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MeasureRhythmicPosition> for super::MeasureRhythmicPosition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MeasureRhythmicPosition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                measure: value.measure?,
                position: value.position?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::MeasureRhythmicPosition> for MeasureRhythmicPosition {
        fn from(value: super::MeasureRhythmicPosition) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                measure: Ok(value.measure),
                position: Ok(value.position),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mnx {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        support:
            ::std::result::Result<::std::option::Option<super::Support>, ::std::string::String>,
        version: ::std::result::Result<super::VersionNumber, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Mnx {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                support: Ok(Default::default()),
                version: Err("no value supplied for version".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Mnx {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn support<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Support>>,
            T::Error: ::std::fmt::Display,
        {
            self.support = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for support: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VersionNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Mnx> for super::Mnx {
        type Error = super::error::ConversionError;
        fn try_from(value: Mnx) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                support: value.support?,
                version: value.version?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Mnx> for Mnx {
        fn from(value: super::Mnx) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                support: Ok(value.support),
                version: Ok(value.version),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MultiNoteTremolo {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        content: ::std::result::Result<super::TremoloSequenceContent, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        individual_duration:
            ::std::result::Result<::std::option::Option<super::NoteValue>, ::std::string::String>,
        marks: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
        outer: ::std::result::Result<super::NoteValueQuantity, ::std::string::String>,
        type_: ::std::result::Result<super::LiteralStringTremolo, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MultiNoteTremolo {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                individual_duration: Ok(Default::default()),
                marks: Err("no value supplied for marks".to_string()),
                outer: Err("no value supplied for outer".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl MultiNoteTremolo {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TremoloSequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn individual_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NoteValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.individual_duration = value.try_into().map_err(|e| {
                format!("error converting supplied value for individual_duration: {e}")
            });
            self
        }
        pub fn marks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.marks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marks: {e}"));
            self
        }
        pub fn outer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueQuantity>,
            T::Error: ::std::fmt::Display,
        {
            self.outer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outer: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiteralStringTremolo>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MultiNoteTremolo> for super::MultiNoteTremolo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MultiNoteTremolo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                content: value.content?,
                id: value.id?,
                individual_duration: value.individual_duration?,
                marks: value.marks?,
                outer: value.outer?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::MultiNoteTremolo> for MultiNoteTremolo {
        fn from(value: super::MultiNoteTremolo) -> Self {
            Self {
                c: Ok(value.c),
                content: Ok(value.content),
                id: Ok(value.id),
                individual_duration: Ok(value.individual_duration),
                marks: Ok(value.marks),
                outer: Ok(value.outer),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MultimeasureRest {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        duration: ::std::result::Result<super::MeasureCount, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        label: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        start: ::std::result::Result<super::MeasureNumber, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MultimeasureRest {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                duration: Err("no value supplied for duration".to_string()),
                id: Ok(Default::default()),
                label: Ok(Default::default()),
                start: Err("no value supplied for start".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl MultimeasureRest {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureCount>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MultimeasureRest> for super::MultimeasureRest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MultimeasureRest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                duration: value.duration?,
                id: value.id?,
                label: value.label?,
                start: value.start?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::MultimeasureRest> for MultimeasureRest {
        fn from(value: super::MultimeasureRest) -> Self {
            Self {
                c: Ok(value.c),
                duration: Ok(value.duration),
                id: Ok(value.id),
                label: Ok(value.label),
                start: Ok(value.start),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Note {
        accidental_display: ::std::result::Result<
            ::std::option::Option<super::AccidentalDisplay>,
            ::std::string::String,
        >,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        perform: ::std::result::Result<
            ::std::option::Option<super::PerformOptions>,
            ::std::string::String,
        >,
        pitch: ::std::result::Result<super::Pitch, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        ties: ::std::result::Result<::std::option::Option<super::TieList>, ::std::string::String>,
        written:
            ::std::result::Result<::std::option::Option<super::Written>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Note {
        fn default() -> Self {
            Self {
                accidental_display: Ok(Default::default()),
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                perform: Ok(Default::default()),
                pitch: Err("no value supplied for pitch".to_string()),
                staff: Ok(Default::default()),
                ties: Ok(Default::default()),
                written: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Note {
        pub fn accidental_display<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AccidentalDisplay>>,
            T::Error: ::std::fmt::Display,
        {
            self.accidental_display = value.try_into().map_err(|e| {
                format!("error converting supplied value for accidental_display: {e}")
            });
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn perform<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PerformOptions>>,
            T::Error: ::std::fmt::Display,
        {
            self.perform = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for perform: {e}"));
            self
        }
        pub fn pitch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Pitch>,
            T::Error: ::std::fmt::Display,
        {
            self.pitch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pitch: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn ties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TieList>>,
            T::Error: ::std::fmt::Display,
        {
            self.ties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ties: {e}"));
            self
        }
        pub fn written<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Written>>,
            T::Error: ::std::fmt::Display,
        {
            self.written = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for written: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Note> for super::Note {
        type Error = super::error::ConversionError;
        fn try_from(value: Note) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accidental_display: value.accidental_display?,
                c: value.c?,
                id: value.id?,
                perform: value.perform?,
                pitch: value.pitch?,
                staff: value.staff?,
                ties: value.ties?,
                written: value.written?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Note> for Note {
        fn from(value: super::Note) -> Self {
            Self {
                accidental_display: Ok(value.accidental_display),
                c: Ok(value.c),
                id: Ok(value.id),
                perform: Ok(value.perform),
                pitch: Ok(value.pitch),
                staff: Ok(value.staff),
                ties: Ok(value.ties),
                written: Ok(value.written),
                x: Ok(value.x),
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
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NoteValue {
        fn default() -> Self {
            Self {
                base: Err("no value supplied for base".to_string()),
                c: Ok(Default::default()),
                dots: Ok(Default::default()),
                id: Ok(Default::default()),
                x: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for base: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn dots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositiveInteger>>,
            T::Error: ::std::fmt::Display,
        {
            self.dots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dots: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
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
                x: value.x?,
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
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NoteValueQuantity {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        duration: ::std::result::Result<super::NoteValue, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        multiple: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NoteValueQuantity {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                duration: Err("no value supplied for duration".to_string()),
                id: Ok(Default::default()),
                multiple: Err("no value supplied for multiple".to_string()),
                x: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValue>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn multiple<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.multiple = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for multiple: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
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
                x: value.x?,
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
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Ottava {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        end: ::std::result::Result<super::MeasureRhythmicPosition, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        position: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        value: ::std::result::Result<super::OttavaAmount, ::std::string::String>,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Ottava {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                end: Err("no value supplied for end".to_string()),
                id: Ok(Default::default()),
                orient: Ok(Default::default()),
                position: Err("no value supplied for position".to_string()),
                staff: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
                voice: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Ottava {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureRhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {e}"));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OttavaAmount>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Ottava> for super::Ottava {
        type Error = super::error::ConversionError;
        fn try_from(value: Ottava) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                end: value.end?,
                id: value.id?,
                orient: value.orient?,
                position: value.position?,
                staff: value.staff?,
                value: value.value?,
                voice: value.voice?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Ottava> for Ottava {
        fn from(value: super::Ottava) -> Self {
            Self {
                c: Ok(value.c),
                end: Ok(value.end),
                id: Ok(value.id),
                orient: Ok(value.orient),
                position: Ok(value.position),
                staff: Ok(value.staff),
                value: Ok(value.value),
                voice: Ok(value.voice),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Page {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        layout: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        systems: ::std::result::Result<super::Systems, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Page {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                layout: Ok(Default::default()),
                systems: Err("no value supplied for systems".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Page {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn systems<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Systems>,
            T::Error: ::std::fmt::Display,
        {
            self.systems = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for systems: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Page> for super::Page {
        type Error = super::error::ConversionError;
        fn try_from(value: Page) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                layout: value.layout?,
                systems: value.systems?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Page> for Page {
        fn from(value: super::Page) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                layout: Ok(value.layout),
                systems: Ok(value.systems),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Part {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        kit: ::std::result::Result<::std::option::Option<super::Kit>, ::std::string::String>,
        measures: ::std::result::Result<super::PartMeasures, ::std::string::String>,
        name: ::std::result::Result<::std::option::Option<super::PartName>, ::std::string::String>,
        short_name: ::std::result::Result<
            ::std::option::Option<super::PartShortName>,
            ::std::string::String,
        >,
        smufl_font:
            ::std::result::Result<::std::option::Option<super::SmuflFont>, ::std::string::String>,
        staves:
            ::std::result::Result<::std::option::Option<super::StaffCount>, ::std::string::String>,
        transposition: ::std::result::Result<
            ::std::option::Option<super::PartTransposition>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Part {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                kit: Ok(Default::default()),
                measures: Err("no value supplied for measures".to_string()),
                name: Ok(Default::default()),
                short_name: Ok(Default::default()),
                smufl_font: Ok(Default::default()),
                staves: Ok(Default::default()),
                transposition: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Part {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn kit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Kit>>,
            T::Error: ::std::fmt::Display,
        {
            self.kit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kit: {e}"));
            self
        }
        pub fn measures<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PartMeasures>,
            T::Error: ::std::fmt::Display,
        {
            self.measures = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measures: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PartName>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PartShortName>>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_name: {e}"));
            self
        }
        pub fn smufl_font<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflFont>>,
            T::Error: ::std::fmt::Display,
        {
            self.smufl_font = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for smufl_font: {e}"));
            self
        }
        pub fn staves<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffCount>>,
            T::Error: ::std::fmt::Display,
        {
            self.staves = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staves: {e}"));
            self
        }
        pub fn transposition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PartTransposition>>,
            T::Error: ::std::fmt::Display,
        {
            self.transposition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transposition: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Part> for super::Part {
        type Error = super::error::ConversionError;
        fn try_from(value: Part) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                kit: value.kit?,
                measures: value.measures?,
                name: value.name?,
                short_name: value.short_name?,
                smufl_font: value.smufl_font?,
                staves: value.staves?,
                transposition: value.transposition?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Part> for Part {
        fn from(value: super::Part) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                kit: Ok(value.kit),
                measures: Ok(value.measures),
                name: Ok(value.name),
                short_name: Ok(value.short_name),
                smufl_font: Ok(value.smufl_font),
                staves: Ok(value.staves),
                transposition: Ok(value.transposition),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartMeasure {
        beams: ::std::result::Result<::std::option::Option<super::BeamList>, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        clefs: ::std::result::Result<
            ::std::option::Option<super::PositionedClefList>,
            ::std::string::String,
        >,
        dynamics:
            ::std::result::Result<::std::option::Option<super::DynamicList>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        ottavas:
            ::std::result::Result<::std::option::Option<super::OttavaList>, ::std::string::String>,
        sequences: ::std::result::Result<super::SequenceList, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartMeasure {
        fn default() -> Self {
            Self {
                beams: Ok(Default::default()),
                c: Ok(Default::default()),
                clefs: Ok(Default::default()),
                dynamics: Ok(Default::default()),
                id: Ok(Default::default()),
                ottavas: Ok(Default::default()),
                sequences: Err("no value supplied for sequences".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl PartMeasure {
        pub fn beams<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BeamList>>,
            T::Error: ::std::fmt::Display,
        {
            self.beams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for beams: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn clefs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PositionedClefList>>,
            T::Error: ::std::fmt::Display,
        {
            self.clefs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clefs: {e}"));
            self
        }
        pub fn dynamics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DynamicList>>,
            T::Error: ::std::fmt::Display,
        {
            self.dynamics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dynamics: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn ottavas<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OttavaList>>,
            T::Error: ::std::fmt::Display,
        {
            self.ottavas = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ottavas: {e}"));
            self
        }
        pub fn sequences<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SequenceList>,
            T::Error: ::std::fmt::Display,
        {
            self.sequences = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sequences: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PartMeasure> for super::PartMeasure {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PartMeasure,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                beams: value.beams?,
                c: value.c?,
                clefs: value.clefs?,
                dynamics: value.dynamics?,
                id: value.id?,
                ottavas: value.ottavas?,
                sequences: value.sequences?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::PartMeasure> for PartMeasure {
        fn from(value: super::PartMeasure) -> Self {
            Self {
                beams: Ok(value.beams),
                c: Ok(value.c),
                clefs: Ok(value.clefs),
                dynamics: Ok(value.dynamics),
                id: Ok(value.id),
                ottavas: Ok(value.ottavas),
                sequences: Ok(value.sequences),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartTransposition {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        interval: ::std::result::Result<super::Interval, ::std::string::String>,
        key_fifths_flip_at: ::std::result::Result<
            ::std::option::Option<super::IntegerSigned>,
            ::std::string::String,
        >,
        prefers_written_pitches:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartTransposition {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                interval: Err("no value supplied for interval".to_string()),
                key_fifths_flip_at: Ok(Default::default()),
                prefers_written_pitches: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl PartTransposition {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Interval>,
            T::Error: ::std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {e}"));
            self
        }
        pub fn key_fifths_flip_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IntegerSigned>>,
            T::Error: ::std::fmt::Display,
        {
            self.key_fifths_flip_at = value.try_into().map_err(|e| {
                format!("error converting supplied value for key_fifths_flip_at: {e}")
            });
            self
        }
        pub fn prefers_written_pitches<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.prefers_written_pitches = value.try_into().map_err(|e| {
                format!("error converting supplied value for prefers_written_pitches: {e}")
            });
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PartTransposition> for super::PartTransposition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PartTransposition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                interval: value.interval?,
                key_fifths_flip_at: value.key_fifths_flip_at?,
                prefers_written_pitches: value.prefers_written_pitches?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::PartTransposition> for PartTransposition {
        fn from(value: super::PartTransposition) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                interval: Ok(value.interval),
                key_fifths_flip_at: Ok(value.key_fifths_flip_at),
                prefers_written_pitches: Ok(value.prefers_written_pitches),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Pitch {
        alter: ::std::result::Result<::std::option::Option<super::Alter>, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        octave: ::std::result::Result<super::Octave, ::std::string::String>,
        step: ::std::result::Result<super::Step, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Pitch {
        fn default() -> Self {
            Self {
                alter: Ok(Default::default()),
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                octave: Err("no value supplied for octave".to_string()),
                step: Err("no value supplied for step".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Pitch {
        pub fn alter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Alter>>,
            T::Error: ::std::fmt::Display,
        {
            self.alter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alter: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn octave<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Octave>,
            T::Error: ::std::fmt::Display,
        {
            self.octave = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for octave: {e}"));
            self
        }
        pub fn step<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Step>,
            T::Error: ::std::fmt::Display,
        {
            self.step = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for step: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Pitch> for super::Pitch {
        type Error = super::error::ConversionError;
        fn try_from(value: Pitch) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alter: value.alter?,
                c: value.c?,
                id: value.id?,
                octave: value.octave?,
                step: value.step?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Pitch> for Pitch {
        fn from(value: super::Pitch) -> Self {
            Self {
                alter: Ok(value.alter),
                c: Ok(value.c),
                id: Ok(value.id),
                octave: Ok(value.octave),
                step: Ok(value.step),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PositionedClef {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        clef: ::std::result::Result<super::Clef, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        position: ::std::result::Result<
            ::std::option::Option<super::RhythmicPosition>,
            ::std::string::String,
        >,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PositionedClef {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                clef: Err("no value supplied for clef".to_string()),
                id: Ok(Default::default()),
                position: Ok(Default::default()),
                staff: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl PositionedClef {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn clef<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Clef>,
            T::Error: ::std::fmt::Display,
        {
            self.clef = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clef: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RhythmicPosition>>,
            T::Error: ::std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PositionedClef> for super::PositionedClef {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PositionedClef,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                clef: value.clef?,
                id: value.id?,
                position: value.position?,
                staff: value.staff?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::PositionedClef> for PositionedClef {
        fn from(value: super::PositionedClef) -> Self {
            Self {
                c: Ok(value.c),
                clef: Ok(value.clef),
                id: Ok(value.id),
                position: Ok(value.position),
                staff: Ok(value.staff),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RepeatEnd {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        times:
            ::std::result::Result<::std::option::Option<super::RepeatTimes>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RepeatEnd {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                times: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl RepeatEnd {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn times<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RepeatTimes>>,
            T::Error: ::std::fmt::Display,
        {
            self.times = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RepeatEnd> for super::RepeatEnd {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RepeatEnd,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                times: value.times?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::RepeatEnd> for RepeatEnd {
        fn from(value: super::RepeatEnd) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                times: Ok(value.times),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Rest {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        staff_position: ::std::result::Result<
            ::std::option::Option<super::StaffPosition>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Rest {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                staff_position: Ok(Default::default()),
                x: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn staff_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffPosition>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_position: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
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
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Rest> for Rest {
        fn from(value: super::Rest) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                staff_position: Ok(value.staff_position),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RhythmicPosition {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        fraction: ::std::result::Result<super::Fraction, ::std::string::String>,
        grace_index: ::std::result::Result<
            ::std::option::Option<super::IntegerUnsigned>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RhythmicPosition {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                fraction: Err("no value supplied for fraction".to_string()),
                grace_index: Ok(Default::default()),
                id: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl RhythmicPosition {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn fraction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Fraction>,
            T::Error: ::std::fmt::Display,
        {
            self.fraction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fraction: {e}"));
            self
        }
        pub fn grace_index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IntegerUnsigned>>,
            T::Error: ::std::fmt::Display,
        {
            self.grace_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grace_index: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RhythmicPosition> for super::RhythmicPosition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RhythmicPosition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                fraction: value.fraction?,
                grace_index: value.grace_index?,
                id: value.id?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::RhythmicPosition> for RhythmicPosition {
        fn from(value: super::RhythmicPosition) -> Self {
            Self {
                c: Ok(value.c),
                fraction: Ok(value.fraction),
                grace_index: Ok(value.grace_index),
                id: Ok(value.id),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Root {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        global: ::std::result::Result<super::Global, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        layouts:
            ::std::result::Result<::std::option::Option<super::Layouts>, ::std::string::String>,
        mnx: ::std::result::Result<super::Mnx, ::std::string::String>,
        parts: ::std::result::Result<super::Parts, ::std::string::String>,
        scores: ::std::result::Result<::std::option::Option<super::Scores>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Root {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                global: Err("no value supplied for global".to_string()),
                id: Ok(Default::default()),
                layouts: Ok(Default::default()),
                mnx: Err("no value supplied for mnx".to_string()),
                parts: Err("no value supplied for parts".to_string()),
                scores: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Root {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn global<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Global>,
            T::Error: ::std::fmt::Display,
        {
            self.global = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for global: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn layouts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Layouts>>,
            T::Error: ::std::fmt::Display,
        {
            self.layouts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layouts: {e}"));
            self
        }
        pub fn mnx<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Mnx>,
            T::Error: ::std::fmt::Display,
        {
            self.mnx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mnx: {e}"));
            self
        }
        pub fn parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Parts>,
            T::Error: ::std::fmt::Display,
        {
            self.parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parts: {e}"));
            self
        }
        pub fn scores<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Scores>>,
            T::Error: ::std::fmt::Display,
        {
            self.scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scores: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Root> for super::Root {
        type Error = super::error::ConversionError;
        fn try_from(value: Root) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                global: value.global?,
                id: value.id?,
                layouts: value.layouts?,
                mnx: value.mnx?,
                parts: value.parts?,
                scores: value.scores?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Root> for Root {
        fn from(value: super::Root) -> Self {
            Self {
                c: Ok(value.c),
                global: Ok(value.global),
                id: Ok(value.id),
                layouts: Ok(value.layouts),
                mnx: Ok(value.mnx),
                parts: Ok(value.parts),
                scores: Ok(value.scores),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Score {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        layout: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        multimeasure_rests: ::std::result::Result<
            ::std::option::Option<super::MultimeasureRests>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::ScoreName, ::std::string::String>,
        pages: ::std::result::Result<::std::option::Option<super::Pages>, ::std::string::String>,
        use_written: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Score {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                layout: Ok(Default::default()),
                multimeasure_rests: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                pages: Ok(Default::default()),
                use_written: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Score {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn multimeasure_rests<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MultimeasureRests>>,
            T::Error: ::std::fmt::Display,
        {
            self.multimeasure_rests = value.try_into().map_err(|e| {
                format!("error converting supplied value for multimeasure_rests: {e}")
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ScoreName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn pages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Pages>>,
            T::Error: ::std::fmt::Display,
        {
            self.pages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pages: {e}"));
            self
        }
        pub fn use_written<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.use_written = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_written: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Score> for super::Score {
        type Error = super::error::ConversionError;
        fn try_from(value: Score) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                layout: value.layout?,
                multimeasure_rests: value.multimeasure_rests?,
                name: value.name?,
                pages: value.pages?,
                use_written: value.use_written?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Score> for Score {
        fn from(value: super::Score) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                layout: Ok(value.layout),
                multimeasure_rests: Ok(value.multimeasure_rests),
                name: Ok(value.name),
                pages: Ok(value.pages),
                use_written: Ok(value.use_written),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Segno {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        glyph:
            ::std::result::Result<::std::option::Option<super::SmuflGlyph>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        location: ::std::result::Result<super::RhythmicPosition, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Segno {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                color: Ok(Default::default()),
                glyph: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Err("no value supplied for location".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Segno {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn glyph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SmuflGlyph>>,
            T::Error: ::std::fmt::Display,
        {
            self.glyph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for glyph: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RhythmicPosition>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Segno> for super::Segno {
        type Error = super::error::ConversionError;
        fn try_from(value: Segno) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                color: value.color?,
                glyph: value.glyph?,
                id: value.id?,
                location: value.location?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Segno> for Segno {
        fn from(value: super::Segno) -> Self {
            Self {
                c: Ok(value.c),
                color: Ok(value.color),
                glyph: Ok(value.glyph),
                id: Ok(value.id),
                location: Ok(value.location),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Sequence {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        content: ::std::result::Result<super::SequenceContent, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Sequence {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                orient: Ok(Default::default()),
                staff: Ok(Default::default()),
                voice: Ok(Default::default()),
                x: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
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
                orient: value.orient?,
                staff: value.staff?,
                voice: value.voice?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Sequence> for Sequence {
        fn from(value: super::Sequence) -> Self {
            Self {
                c: Ok(value.c),
                content: Ok(value.content),
                id: Ok(value.id),
                orient: Ok(value.orient),
                staff: Ok(value.staff),
                voice: Ok(value.voice),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SequenceContentItem {
        event: ::std::result::Result<::std::option::Option<super::Event>, ::std::string::String>,
        grace: ::std::result::Result<::std::option::Option<super::Grace>, ::std::string::String>,
        tuplet: ::std::result::Result<::std::option::Option<super::Tuplet>, ::std::string::String>,
        space: ::std::result::Result<::std::option::Option<super::Space>, ::std::string::String>,
        multi_note_tremolo: ::std::result::Result<
            ::std::option::Option<super::MultiNoteTremolo>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SequenceContentItem {
        fn default() -> Self {
            Self {
                event: Ok(Default::default()),
                grace: Ok(Default::default()),
                tuplet: Ok(Default::default()),
                space: Ok(Default::default()),
                multi_note_tremolo: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
        pub fn grace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Grace>>,
            T::Error: ::std::fmt::Display,
        {
            self.grace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grace: {e}"));
            self
        }
        pub fn tuplet<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Tuplet>>,
            T::Error: ::std::fmt::Display,
        {
            self.tuplet = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tuplet: {e}"));
            self
        }
        pub fn space<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Space>>,
            T::Error: ::std::fmt::Display,
        {
            self.space = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for space: {e}"));
            self
        }
        pub fn multi_note_tremolo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MultiNoteTremolo>>,
            T::Error: ::std::fmt::Display,
        {
            self.multi_note_tremolo = value.try_into().map_err(|e| {
                format!("error converting supplied value for multi_note_tremolo: {e}")
            });
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
                grace: value.grace?,
                tuplet: value.tuplet?,
                space: value.space?,
                multi_note_tremolo: value.multi_note_tremolo?,
            })
        }
    }
    impl ::std::convert::From<super::SequenceContentItem> for SequenceContentItem {
        fn from(value: super::SequenceContentItem) -> Self {
            Self {
                event: Ok(value.event),
                grace: Ok(value.grace),
                tuplet: Ok(value.tuplet),
                space: Ok(value.space),
                multi_note_tremolo: Ok(value.multi_note_tremolo),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Slur {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        end_note: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        line_type:
            ::std::result::Result<::std::option::Option<super::LineType>, ::std::string::String>,
        side: ::std::result::Result<::std::option::Option<super::SlurSide>, ::std::string::String>,
        side_end:
            ::std::result::Result<::std::option::Option<super::SlurSide>, ::std::string::String>,
        start_note: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        target: ::std::result::Result<super::Id, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Slur {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                end_note: Ok(Default::default()),
                id: Ok(Default::default()),
                line_type: Ok(Default::default()),
                side: Ok(Default::default()),
                side_end: Ok(Default::default()),
                start_note: Ok(Default::default()),
                target: Err("no value supplied for target".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Slur {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn end_note<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_note = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_note: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn line_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LineType>>,
            T::Error: ::std::fmt::Display,
        {
            self.line_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_type: {e}"));
            self
        }
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SlurSide>>,
            T::Error: ::std::fmt::Display,
        {
            self.side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {e}"));
            self
        }
        pub fn side_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SlurSide>>,
            T::Error: ::std::fmt::Display,
        {
            self.side_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side_end: {e}"));
            self
        }
        pub fn start_note<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.start_note = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_note: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Slur> for super::Slur {
        type Error = super::error::ConversionError;
        fn try_from(value: Slur) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                end_note: value.end_note?,
                id: value.id?,
                line_type: value.line_type?,
                side: value.side?,
                side_end: value.side_end?,
                start_note: value.start_note?,
                target: value.target?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Slur> for Slur {
        fn from(value: super::Slur) -> Self {
            Self {
                c: Ok(value.c),
                end_note: Ok(value.end_note),
                id: Ok(value.id),
                line_type: Ok(value.line_type),
                side: Ok(value.side),
                side_end: Ok(value.side_end),
                start_note: Ok(value.start_note),
                target: Ok(value.target),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Sound {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        midi_number:
            ::std::result::Result<::std::option::Option<super::MidiNumber>, ::std::string::String>,
        name: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Sound {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                midi_number: Ok(Default::default()),
                name: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Sound {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn midi_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MidiNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.midi_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for midi_number: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Sound> for super::Sound {
        type Error = super::error::ConversionError;
        fn try_from(value: Sound) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                midi_number: value.midi_number?,
                name: value.name?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Sound> for Sound {
        fn from(value: super::Sound) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                midi_number: Ok(value.midi_number),
                name: Ok(value.name),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Space {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        duration: ::std::result::Result<super::Fraction, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        type_: ::std::result::Result<super::LiteralStringSpace, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Space {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                duration: Err("no value supplied for duration".to_string()),
                id: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Space {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Fraction>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiteralStringSpace>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Space> for super::Space {
        type Error = super::error::ConversionError;
        fn try_from(value: Space) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                duration: value.duration?,
                id: value.id?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Space> for Space {
        fn from(value: super::Space) -> Self {
            Self {
                c: Ok(value.c),
                duration: Ok(value.duration),
                id: Ok(value.id),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Staff {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        label:
            ::std::result::Result<::std::option::Option<super::StaffLabel>, ::std::string::String>,
        labelref: ::std::result::Result<
            ::std::option::Option<super::StaffLabelref>,
            ::std::string::String,
        >,
        sources: ::std::result::Result<super::StaffSources, ::std::string::String>,
        symbol:
            ::std::result::Result<::std::option::Option<super::StaffSymbol>, ::std::string::String>,
        type_: ::std::result::Result<super::LiteralStringStaff, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Staff {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                label: Ok(Default::default()),
                labelref: Ok(Default::default()),
                sources: Err("no value supplied for sources".to_string()),
                symbol: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Staff {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffLabel>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn labelref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffLabelref>>,
            T::Error: ::std::fmt::Display,
        {
            self.labelref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for labelref: {e}"));
            self
        }
        pub fn sources<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StaffSources>,
            T::Error: ::std::fmt::Display,
        {
            self.sources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sources: {e}"));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffSymbol>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiteralStringStaff>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Staff> for super::Staff {
        type Error = super::error::ConversionError;
        fn try_from(value: Staff) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                label: value.label?,
                labelref: value.labelref?,
                sources: value.sources?,
                symbol: value.symbol?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Staff> for Staff {
        fn from(value: super::Staff) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                label: Ok(value.label),
                labelref: Ok(value.labelref),
                sources: Ok(value.sources),
                symbol: Ok(value.symbol),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StaffGroup {
        barline_style: ::std::result::Result<
            ::std::option::Option<super::StaffGroupBarlineStyle>,
            ::std::string::String,
        >,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        content: ::std::result::Result<super::SystemLayoutContent, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        label:
            ::std::result::Result<::std::option::Option<super::StaffLabel>, ::std::string::String>,
        symbol:
            ::std::result::Result<::std::option::Option<super::StaffSymbol>, ::std::string::String>,
        type_: ::std::result::Result<super::LiteralStringGroup, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StaffGroup {
        fn default() -> Self {
            Self {
                barline_style: Ok(Default::default()),
                c: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                label: Ok(Default::default()),
                symbol: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl StaffGroup {
        pub fn barline_style<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffGroupBarlineStyle>>,
            T::Error: ::std::fmt::Display,
        {
            self.barline_style = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for barline_style: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SystemLayoutContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffLabel>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn symbol<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffSymbol>>,
            T::Error: ::std::fmt::Display,
        {
            self.symbol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for symbol: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiteralStringGroup>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StaffGroup> for super::StaffGroup {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StaffGroup,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                barline_style: value.barline_style?,
                c: value.c?,
                content: value.content?,
                id: value.id?,
                label: value.label?,
                symbol: value.symbol?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::StaffGroup> for StaffGroup {
        fn from(value: super::StaffGroup) -> Self {
            Self {
                barline_style: Ok(value.barline_style),
                c: Ok(value.c),
                content: Ok(value.content),
                id: Ok(value.id),
                label: Ok(value.label),
                symbol: Ok(value.symbol),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StaffSource {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        label:
            ::std::result::Result<::std::option::Option<super::StaffLabel>, ::std::string::String>,
        labelref: ::std::result::Result<
            ::std::option::Option<super::StaffLabelref>,
            ::std::string::String,
        >,
        part: ::std::result::Result<super::Id, ::std::string::String>,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        stem: ::std::result::Result<
            ::std::option::Option<super::StemDirection>,
            ::std::string::String,
        >,
        voice:
            ::std::result::Result<::std::option::Option<super::VoiceName>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StaffSource {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                label: Ok(Default::default()),
                labelref: Ok(Default::default()),
                part: Err("no value supplied for part".to_string()),
                staff: Ok(Default::default()),
                stem: Ok(Default::default()),
                voice: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl StaffSource {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffLabel>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
        pub fn labelref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffLabelref>>,
            T::Error: ::std::fmt::Display,
        {
            self.labelref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for labelref: {e}"));
            self
        }
        pub fn part<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.part = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for part: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn stem<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StemDirection>>,
            T::Error: ::std::fmt::Display,
        {
            self.stem = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stem: {e}"));
            self
        }
        pub fn voice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VoiceName>>,
            T::Error: ::std::fmt::Display,
        {
            self.voice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StaffSource> for super::StaffSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StaffSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                label: value.label?,
                labelref: value.labelref?,
                part: value.part?,
                staff: value.staff?,
                stem: value.stem?,
                voice: value.voice?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::StaffSource> for StaffSource {
        fn from(value: super::StaffSource) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                label: Ok(value.label),
                labelref: Ok(value.labelref),
                part: Ok(value.part),
                staff: Ok(value.staff),
                stem: Ok(value.stem),
                voice: Ok(value.voice),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StrongAccent {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        pointing:
            ::std::result::Result<::std::option::Option<super::UpOrDown>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StrongAccent {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                pointing: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl StrongAccent {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn pointing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UpOrDown>>,
            T::Error: ::std::fmt::Display,
        {
            self.pointing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pointing: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StrongAccent> for super::StrongAccent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StrongAccent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                pointing: value.pointing?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::StrongAccent> for StrongAccent {
        fn from(value: super::StrongAccent) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                pointing: Ok(value.pointing),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Support {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        use_accidental_display:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        use_beams: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Support {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                use_accidental_display: Ok(Default::default()),
                use_beams: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Support {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn use_accidental_display<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.use_accidental_display = value.try_into().map_err(|e| {
                format!("error converting supplied value for use_accidental_display: {e}")
            });
            self
        }
        pub fn use_beams<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.use_beams = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_beams: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Support> for super::Support {
        type Error = super::error::ConversionError;
        fn try_from(value: Support) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                use_accidental_display: value.use_accidental_display?,
                use_beams: value.use_beams?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Support> for Support {
        fn from(value: super::Support) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                use_accidental_display: Ok(value.use_accidental_display),
                use_beams: Ok(value.use_beams),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct System {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        layout: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        layout_changes: ::std::result::Result<
            ::std::option::Option<super::LayoutChanges>,
            ::std::string::String,
        >,
        measure: ::std::result::Result<super::MeasureNumber, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for System {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                layout: Ok(Default::default()),
                layout_changes: Ok(Default::default()),
                measure: Err("no value supplied for measure".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl System {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {e}"));
            self
        }
        pub fn layout_changes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LayoutChanges>>,
            T::Error: ::std::fmt::Display,
        {
            self.layout_changes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout_changes: {e}"));
            self
        }
        pub fn measure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MeasureNumber>,
            T::Error: ::std::fmt::Display,
        {
            self.measure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measure: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<System> for super::System {
        type Error = super::error::ConversionError;
        fn try_from(value: System) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                layout: value.layout?,
                layout_changes: value.layout_changes?,
                measure: value.measure?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::System> for System {
        fn from(value: super::System) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                layout: Ok(value.layout),
                layout_changes: Ok(value.layout_changes),
                measure: Ok(value.measure),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SystemLayout {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        content: ::std::result::Result<super::SystemLayoutContent, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SystemLayout {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl SystemLayout {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SystemLayoutContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SystemLayout> for super::SystemLayout {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SystemLayout,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                content: value.content?,
                id: value.id?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::SystemLayout> for SystemLayout {
        fn from(value: super::SystemLayout) -> Self {
            Self {
                c: Ok(value.c),
                content: Ok(value.content),
                id: Ok(value.id),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SystemLayoutContentItem {
        staff_group:
            ::std::result::Result<::std::option::Option<super::StaffGroup>, ::std::string::String>,
        staff: ::std::result::Result<::std::option::Option<super::Staff>, ::std::string::String>,
    }
    impl ::std::default::Default for SystemLayoutContentItem {
        fn default() -> Self {
            Self {
                staff_group: Ok(Default::default()),
                staff: Ok(Default::default()),
            }
        }
    }
    impl SystemLayoutContentItem {
        pub fn staff_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffGroup>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff_group = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff_group: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Staff>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SystemLayoutContentItem> for super::SystemLayoutContentItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SystemLayoutContentItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                staff_group: value.staff_group?,
                staff: value.staff?,
            })
        }
    }
    impl ::std::convert::From<super::SystemLayoutContentItem> for SystemLayoutContentItem {
        fn from(value: super::SystemLayoutContentItem) -> Self {
            Self {
                staff_group: Ok(value.staff_group),
                staff: Ok(value.staff),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tempo {
        bpm: ::std::result::Result<super::Bpm, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        location: ::std::result::Result<
            ::std::option::Option<super::RhythmicPosition>,
            ::std::string::String,
        >,
        value: ::std::result::Result<super::NoteValue, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Tempo {
        fn default() -> Self {
            Self {
                bpm: Err("no value supplied for bpm".to_string()),
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                location: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Tempo {
        pub fn bpm<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Bpm>,
            T::Error: ::std::fmt::Display,
        {
            self.bpm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bpm: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RhythmicPosition>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValue>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Tempo> for super::Tempo {
        type Error = super::error::ConversionError;
        fn try_from(value: Tempo) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bpm: value.bpm?,
                c: value.c?,
                id: value.id?,
                location: value.location?,
                value: value.value?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Tempo> for Tempo {
        fn from(value: super::Tempo) -> Self {
            Self {
                bpm: Ok(value.bpm),
                c: Ok(value.c),
                id: Ok(value.id),
                location: Ok(value.location),
                value: Ok(value.value),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tie {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        lv: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        side: ::std::result::Result<::std::option::Option<super::SlurSide>, ::std::string::String>,
        target: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        target_type: ::std::result::Result<
            ::std::option::Option<super::TieTargetType>,
            ::std::string::String,
        >,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Tie {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                lv: Ok(Default::default()),
                side: Ok(Default::default()),
                target: Ok(Default::default()),
                target_type: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Tie {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn lv<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.lv = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lv: {e}"));
            self
        }
        pub fn side<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SlurSide>>,
            T::Error: ::std::fmt::Display,
        {
            self.side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for side: {e}"));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {e}"));
            self
        }
        pub fn target_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TieTargetType>>,
            T::Error: ::std::fmt::Display,
        {
            self.target_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_type: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Tie> for super::Tie {
        type Error = super::error::ConversionError;
        fn try_from(value: Tie) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                lv: value.lv?,
                side: value.side?,
                target: value.target?,
                target_type: value.target_type?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Tie> for Tie {
        fn from(value: super::Tie) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                lv: Ok(value.lv),
                side: Ok(value.side),
                target: Ok(value.target),
                target_type: Ok(value.target_type),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Time {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        count: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
        display: ::std::result::Result<
            ::std::option::Option<super::TimeSignatureDisplay>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        unit: ::std::result::Result<super::TimeSignatureUnit, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Time {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                count: Err("no value supplied for count".to_string()),
                display: Ok(Default::default()),
                id: Ok(Default::default()),
                unit: Err("no value supplied for unit".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Time {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {e}"));
            self
        }
        pub fn display<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TimeSignatureDisplay>>,
            T::Error: ::std::fmt::Display,
        {
            self.display = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for display: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn unit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TimeSignatureUnit>,
            T::Error: ::std::fmt::Display,
        {
            self.unit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unit: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Time> for super::Time {
        type Error = super::error::ConversionError;
        fn try_from(value: Time) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                count: value.count?,
                display: value.display?,
                id: value.id?,
                unit: value.unit?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Time> for Time {
        fn from(value: super::Time) -> Self {
            Self {
                c: Ok(value.c),
                count: Ok(value.count),
                display: Ok(value.display),
                id: Ok(value.id),
                unit: Ok(value.unit),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TremoloSingle {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        marks: ::std::result::Result<super::PositiveInteger, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TremoloSingle {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                id: Ok(Default::default()),
                marks: Err("no value supplied for marks".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl TremoloSingle {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn marks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PositiveInteger>,
            T::Error: ::std::fmt::Display,
        {
            self.marks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for marks: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TremoloSingle> for super::TremoloSingle {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TremoloSingle,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                id: value.id?,
                marks: value.marks?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::TremoloSingle> for TremoloSingle {
        fn from(value: super::TremoloSingle) -> Self {
            Self {
                c: Ok(value.c),
                id: Ok(value.id),
                marks: Ok(value.marks),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Tuplet {
        bracket:
            ::std::result::Result<::std::option::Option<super::YesNoAuto>, ::std::string::String>,
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        content: ::std::result::Result<super::SequenceContent, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        inner: ::std::result::Result<super::NoteValueQuantity, ::std::string::String>,
        orient:
            ::std::result::Result<::std::option::Option<super::Orientation>, ::std::string::String>,
        outer: ::std::result::Result<super::NoteValueQuantity, ::std::string::String>,
        show_number: ::std::result::Result<
            ::std::option::Option<super::TupletDisplaySetting>,
            ::std::string::String,
        >,
        show_value: ::std::result::Result<
            ::std::option::Option<super::TupletDisplaySetting>,
            ::std::string::String,
        >,
        staff:
            ::std::result::Result<::std::option::Option<super::StaffNumber>, ::std::string::String>,
        type_: ::std::result::Result<super::LiteralStringTuplet, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Tuplet {
        fn default() -> Self {
            Self {
                bracket: Ok(Default::default()),
                c: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                id: Ok(Default::default()),
                inner: Err("no value supplied for inner".to_string()),
                orient: Ok(Default::default()),
                outer: Err("no value supplied for outer".to_string()),
                show_number: Ok(Default::default()),
                show_value: Ok(Default::default()),
                staff: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Tuplet {
        pub fn bracket<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::YesNoAuto>>,
            T::Error: ::std::fmt::Display,
        {
            self.bracket = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bracket: {e}"));
            self
        }
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SequenceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn inner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueQuantity>,
            T::Error: ::std::fmt::Display,
        {
            self.inner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inner: {e}"));
            self
        }
        pub fn orient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Orientation>>,
            T::Error: ::std::fmt::Display,
        {
            self.orient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for orient: {e}"));
            self
        }
        pub fn outer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NoteValueQuantity>,
            T::Error: ::std::fmt::Display,
        {
            self.outer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for outer: {e}"));
            self
        }
        pub fn show_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TupletDisplaySetting>>,
            T::Error: ::std::fmt::Display,
        {
            self.show_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show_number: {e}"));
            self
        }
        pub fn show_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TupletDisplaySetting>>,
            T::Error: ::std::fmt::Display,
        {
            self.show_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show_value: {e}"));
            self
        }
        pub fn staff<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StaffNumber>>,
            T::Error: ::std::fmt::Display,
        {
            self.staff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for staff: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LiteralStringTuplet>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Tuplet> for super::Tuplet {
        type Error = super::error::ConversionError;
        fn try_from(value: Tuplet) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bracket: value.bracket?,
                c: value.c?,
                content: value.content?,
                id: value.id?,
                inner: value.inner?,
                orient: value.orient?,
                outer: value.outer?,
                show_number: value.show_number?,
                show_value: value.show_value?,
                staff: value.staff?,
                type_: value.type_?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Tuplet> for Tuplet {
        fn from(value: super::Tuplet) -> Self {
            Self {
                bracket: Ok(value.bracket),
                c: Ok(value.c),
                content: Ok(value.content),
                id: Ok(value.id),
                inner: Ok(value.inner),
                orient: Ok(value.orient),
                outer: Ok(value.outer),
                show_number: Ok(value.show_number),
                show_value: Ok(value.show_value),
                staff: Ok(value.staff),
                type_: Ok(value.type_),
                x: Ok(value.x),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Written {
        c: ::std::result::Result<::std::option::Option<super::String>, ::std::string::String>,
        diatonic_delta: ::std::result::Result<
            ::std::option::Option<super::IntegerSigned>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        x: ::std::result::Result<
            ::std::option::Option<super::VendorExtensions>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Written {
        fn default() -> Self {
            Self {
                c: Ok(Default::default()),
                diatonic_delta: Ok(Default::default()),
                id: Ok(Default::default()),
                x: Ok(Default::default()),
            }
        }
    }
    impl Written {
        pub fn c<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.c = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c: {e}"));
            self
        }
        pub fn diatonic_delta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IntegerSigned>>,
            T::Error: ::std::fmt::Display,
        {
            self.diatonic_delta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diatonic_delta: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VendorExtensions>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Written> for super::Written {
        type Error = super::error::ConversionError;
        fn try_from(value: Written) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                c: value.c?,
                diatonic_delta: value.diatonic_delta?,
                id: value.id?,
                x: value.x?,
            })
        }
    }
    impl ::std::convert::From<super::Written> for Written {
        fn from(value: super::Written) -> Self {
            Self {
                c: Ok(value.c),
                diatonic_delta: Ok(value.diatonic_delta),
                id: Ok(value.id),
                x: Ok(value.x),
            }
        }
    }
}
