





// regexp helper functions

//

trait Regexp {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String;
}

enum RegexpFlavor {
    Pcre,
    EcmaScript,
}

pub enum Quantifier {
    /// Zero or one of a
    /// - a?
    Optional,
    /// Zero or more of a
    /// - a\*
    Greedy,
    /// One or more of a
    /// - a+
    OneOrMore,
    /// exactly n of a
    /// - a{n}
    Exactly(usize),
    /// n or more of a
    /// - a{n,}
    AtLeast(usize),
    /// Between n and m of a
    /// - a{n,m}
    Between(usize, usize),

    /// Matches as few characters as possible
    /// - a*?
    Lazy,
}

impl Regexp for Quantifier {
    fn get_regexp(&self, _flavor: &RegexpFlavor) -> String {
        match self {
            Quantifier::Greedy => "*".to_owned(),
            Quantifier::OneOrMore => "+".to_owned(),
            Quantifier::Optional => "?".to_owned(),
            Quantifier::Exactly(n) => n.to_string(),
            Quantifier::AtLeast(n) => format!("{{{},}}", n),
            Quantifier::Between(n, m) => format!("{{{},{}}}", n, m),
            Quantifier::Lazy => "*?".to_owned(),
        }
    }
}

struct RegexpTokens {
    tokens: Vec<Box<dyn Regexp>>,
}

impl Regexp for RegexpTokens {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String {
        self.tokens
            .iter()
            .map(|token| token.get_regexp(flavor))
            .collect::<Vec<String>>()
            .join("")
    }
}

pub enum GroupConstruct {
    /// #### Match Everything Enclosed
    /// #### Description
    /// A non-capturing group allows you to apply quantifiers to part of your regex but does not capture/assign an ID.
    /// #### Syntax
    /// ```js
    ///   /(?:.../g
    /// ```
    /// #### Example
    /// ```js
    ///   /match this (?:match that)/g
    /// ```
    /// match this <mark>match that</mark>
    Match(Box<dyn Regexp>),
    /// Matches everything enclosed in parentheses and captures it
    /// - (...)
    Capture(Box<dyn Regexp>),
    /// Matches everything enclosed in parentheses and captures it by name
    /// - (?<name>...) in ECMA Script
    /// - (?P<name>...) in PCRE
    NamedCapture(Box<dyn Regexp>),
    /// ### Positive Lookahead
    /// #### Description
    /// Asserts that the given subpattern can be matched here, without consuming characters.
    /// #### Syntax
    /// ```js
    ///   /(?=...)/
    /// ```
    /// #### Example
    /// ```js
    ///   /foo(?=bar)/
    /// ```
    /// <mark>foo</mark>bar foobaz
    PositiveLookahead(Box<dyn Regexp>),
    /// ### Negative Lookahead
    /// #### Description
    ///  Starting at the current position in the expression, ensures that the given pattern will not match. Does not consume characters.
    /// #### Syntax
    /// ```js
    ///   /(?!...)/
    /// ```
    /// #### Example
    /// ```js
    ///  /foo(?!bar)/
    /// ```
    /// foobar <mark>foo</mark>baz
    NegativeLookahead(Box<dyn Regexp>),
    /// ### Positive Lookbehind
    /// #### Description
    /// Ensures that the given pattern will match, ending at the current position in the expression. The pattern be of variable width. Does not consume any characters.
    ///
    /// #### Syntax
    /// ```js
    ///   /(?<=...)/
    /// ```
    /// #### Example
    /// ```js
    ///   /(?<=foo)bar/
    /// ```
    /// foo<mark>bar</mark> fuubar
    ///
    PositiveLookbehind(Box<dyn Regexp>),
    /// ### Negative Lookbehind
    /// #### Description
    /// Ensures that the given pattern would not match and end at the current position in the expression. The pattern be of variable width. Does not consume characters.
    /// #### Syntax
    /// ```js
    ///  /(?<!...)/
    /// ```
    /// #### Example
    ///  ```js
    ///    /(<!not )foo/
    /// ```
    /// not foo but <mark>foo<mark>
    NegativeLookbehind(Box<dyn Regexp>),
}

impl Regexp for GroupConstruct {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String {
        match self {
            GroupConstruct::Match(regexp) => format!("(?:{})", regexp.get_regexp(flavor)),
            GroupConstruct::Capture(regexp) => format!("({})", regexp.get_regexp(flavor)),
            GroupConstruct::NamedCapture(regexp) => match flavor {
                RegexpFlavor::Pcre => format!("(?P<{}>)", regexp.get_regexp(flavor)),
                RegexpFlavor::EcmaScript => format!("(?<{}>)", regexp.get_regexp(flavor)),
            },
            GroupConstruct::PositiveLookahead(regexp) => {
                format!("(?={})", regexp.get_regexp(flavor))
            }
            GroupConstruct::NegativeLookahead(regexp) => {
                format!("(?!{})", regexp.get_regexp(flavor))
            }
            GroupConstruct::PositiveLookbehind(regexp) => {
                format!("(?<={})", regexp.get_regexp(flavor))
            }
            GroupConstruct::NegativeLookbehind(regexp) => {
                format!("(?<!{})", regexp.get_regexp(flavor))
            }
        }
    }
}

enum CharacterClass {
    /// A single character of: a, b or c
    /// - [abc]
    Single(Vec<CharacterClassType>),
    /// A character except: a, b or c
    /// - [^abc]
    Except(Vec<CharacterClassType>),
}

impl Regexp for CharacterClass {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String {
        match self {
            CharacterClass::Single(types) => format!(
                "[{}]",
                types
                    .iter()
                    .map(|t| t.get_regexp(flavor))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            CharacterClass::Except(types) => format!(
                "[^{}]",
                types
                    .iter()
                    .map(|t| t.get_regexp(flavor))
                    .collect::<Vec<String>>()
                    .join("")
            ),
        }
    }
}

impl CharacterClass {
    pub fn new_single(types: Vec<CharacterClassType>) -> Self {
        CharacterClass::Single(types)
    }

    pub fn new_except(types: Vec<CharacterClassType>) -> Self {
        CharacterClass::Except(types)
    }
}

pub enum CharacterClassType {
    Single(char),
    Range(char, char),
}

impl Regexp for CharacterClassType {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String {
        match self {
            CharacterClassType::Single(c) => escape_string(&c.to_string()),
            CharacterClassType::Range(c1, c2) => format!("{}-{}", c1, c2),
        }
    }
}

impl CharacterClassType {
    pub fn new_single(c: char) -> Self {
        CharacterClassType::Single(c)
    }

    pub fn new_range(c1: char, c2: char) -> Self {
        CharacterClassType::Range(c1, c2)
    }
}

fn escape_string(string: &str) -> String {
    string
        .chars()
        .map(|c| match c {
            '\\' => "\\\\".to_owned(),
            '.' => "\\.".to_owned(),
            '*' => "\\*".to_owned(),
            '+' => "\\+".to_owned(),
            '?' => "\\?".to_owned(),
            '^' => "\\^".to_owned(),
            '$' => "\\$".to_owned(),
            '|' => "\\|".to_owned(),
            '(' => "\\(".to_owned(),
            ')' => "\\)".to_owned(),
            '[' => "\\[".to_owned(),
            ']' => "\\]".to_owned(),
            '{' => "\\{".to_owned(),
            '}' => "\\}".to_owned(),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

pub enum MetaSequence {
    /// ### Any Single Character
    /// #### Description
    /// Matches any single character except line terminators.
    /// #### Syntax
    /// ```js
    ///  /./
    /// ```
    AnySingleChar,
    Or(Box<dyn Regexp>, Box<dyn Regexp>),
    Whitespace,
    NonWhitespace,
    Digit,
    NonDigit,
    /// Matches any letter, digit or underscore. Equivalent to [A-Za-z0-9_].
    WordCharacter,
    /// Matches any character that is not a letter, digit or underscore. Equivalent to [^A-Za-z0-9_].
    NonWordCharacter,
    VerticalWhitespace,
    SubPatternNumber(u32),
    //TODO typings for this
    //https://www.fileformat.info/info/unicode/category/index.htm
    //http://www.regular-expressions.info/unicode.html#category
    UnicodeProperty(String),
    NonUnicodeProperty(String),
    MatchSubPattern(String),
    // check if this is valid 0-f panic if not
    HexCharacter4(char, char, char, char),
    HexCharacter2(char, char),
    // if its  only 2 digits add a 0 to the front of the string panic if its not a valid hex character
    OctalCharacter3(u8),
    // typings for this
    ///  ASCII characters typically associated with Control+A through Control+Z: \x01 through \x1A.
    ControlCharacter(char),
    BackspaceCharacter,
    Literal(char),
}

enum ControlCharacter {}

fn valid_hex_character(c: &char) -> bool {
    match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => true,
        _ => false,
    }
}

fn valid_octal_character(c: &char) -> bool {
    match c {
        '0'..='7' => true,
        _ => false,
    }
}

impl Regexp for MetaSequence {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String {
        match self {
            MetaSequence::AnySingleChar => ".".to_owned(),
            MetaSequence::Or(r1, r2) => {
                format!("(?:{}|{})", r1.get_regexp(flavor), r2.get_regexp(flavor))
            }
            MetaSequence::Whitespace => "\\s".to_owned(),
            MetaSequence::NonWhitespace => "\\S".to_owned(),
            MetaSequence::Digit => "\\d".to_owned(),
            MetaSequence::NonDigit => "\\D".to_owned(),
            MetaSequence::WordCharacter => "\\w".to_owned(),
            MetaSequence::NonWordCharacter => "\\W".to_owned(),
            MetaSequence::VerticalWhitespace => "\\v".to_owned(),
            MetaSequence::SubPatternNumber(n) => format!("\\{}", n),
            MetaSequence::UnicodeProperty(s) => format!("\\p{{{}}}", s),
            MetaSequence::NonUnicodeProperty(s) => format!("\\P{{{}}}", s),
            MetaSequence::MatchSubPattern(s) => format!("\\k<{}>", s),
            MetaSequence::HexCharacter4(c1, c2, c3, c4) => {
                // check if its a valid hex character

                if !valid_hex_character(c1)
                    || !valid_hex_character(c2)
                    || !valid_hex_character(c3)
                    || !valid_hex_character(c4)
                {
                    panic!("Invalid hex: {}", format!("\\u{}{}{}{}", c1, c2, c3, c4));
                }

                format!("\\u{}{}{}{}", c1, c2, c3, c4)
            }
            MetaSequence::HexCharacter2(c1, c2) => {
                // check if its a valid hex character
                if !valid_hex_character(c1) || !valid_hex_character(c2) {
                    panic!("Invalid hex: {}", format!("\\x{}{}", c1, c2));
                }

                format!("\\x{}{}", c1, c2)
            }
            MetaSequence::OctalCharacter3(c) => format!("\\{:03}", c),
            MetaSequence::ControlCharacter(c) => format!("\\c{}", c),
            MetaSequence::BackspaceCharacter => "\\b".to_owned(),
            MetaSequence::Literal(c) => escape_string(&c.to_string()),
        }
    }
}

enum GeneralTokens {
    Newline,
    CarriageReturn,
    Tab,
    Null,
}

enum Flags {
    Global,
    IgnoreCase,
    Multiline,
    Sticky,
    Unicode,
    DotAll,
}

impl Regexp for GeneralTokens {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String {
        match self {
            GeneralTokens::Newline => "\\n".to_owned(),
            GeneralTokens::CarriageReturn => "\\r".to_owned(),
            GeneralTokens::Tab => "\\t".to_owned(),
            GeneralTokens::Null => "\\0".to_owned(),
        }
    }
}

pub enum Anchor {
    Start,
    End,
    /// Matches, without consuming any characters, immediately between a character matched by and a character not matched by (in either order). It cannot be used to separate non words from words.
    WordBoundary,
    NonWordBoundary,
}

impl Regexp for Anchor {
    fn get_regexp(&self, flavor: &RegexpFlavor) -> String {
        match self {
            Anchor::Start => "^".to_owned(),
            Anchor::End => "$".to_owned(),
            Anchor::WordBoundary => "\\b".to_owned(),
            Anchor::NonWordBoundary => "\\B".to_owned(),
        }
    }
}
