use deno_core::{serde_v8, v8, JsRuntime};
use dprint_plugin_markdown::{configuration, format_text};
use regex::Regex;
use scraper::{Html, Selector};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display, fs, path::Path};
// Internal modules
mod code;
mod metadata;

use code::{Code, Interactive, Language};
use metadata::MsMdMetadata;
// evaluate this syntax before parsing the file
//TODO  \[!INCLUDE \[.+\]\(.+\)]
#[derive(Debug, Clone, PartialEq)]
struct Comment {
    pub content: String,
}

impl Comment {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

use url;

#[derive(Debug, Clone, PartialEq)]
pub enum MsMarkdownToken {
    Metadata(MsMdMetadata),
    Comment(Comment),
    Heading(Heading),
    Code(Code),
    Table(Table),
    LineBreak,
    Image(Image),
    // TODO: Further parsing for inline types
    Columns(Vec<Column>),
    //Image(MsMdImage),
    Alert(Alert),
    BlockQuote(BlockQuote),
    List(List),
    HorizontalLine,
    // impl the anchor into regex
    Anchor(Anchor),
    Inline(Inline),
    TextBlock(TextBlock),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(String),
    Link(Vec<Box<Inline>>, PathType),

    Bold(Vec<Box<Inline>>),
    Italic(Vec<Box<Inline>>),
    Strikethrough(Vec<Box<Inline>>),
    /// Goodbye <sup>This is superscript!</sup>
    Superscript(Vec<Box<Inline>>),
    /// Hello <sub>This is subscript!</sub>
    Subscript(Vec<Box<Inline>>),
    Underline(Vec<Box<Inline>>),
    SmartPunctuation(SmartPunctuation),
    NoLoc(Vec<Box<Inline>>),
    // further parsing
    Xref(String),
}

impl Inline {
    pub fn new_text(text: String) -> Self {
        Inline::Text(text)
    }

    pub fn new_link(items: Vec<Box<Inline>>, path: PathType) -> Self {
        Inline::Link(items, path)
    }

    pub fn new_bold(items: Vec<Box<Inline>>) -> Self {
        Inline::Bold(items)
    }

    pub fn new_italic(items: Vec<Box<Inline>>) -> Self {
        Inline::Italic(items)
    }

    pub fn new_strikethrough(items: Vec<Box<Inline>>) -> Self {
        Inline::Strikethrough(items)
    }

    pub fn new_superscript(items: Vec<Box<Inline>>) -> Self {
        Inline::Superscript(items)
    }

    pub fn new_subscript(items: Vec<Box<Inline>>) -> Self {
        Inline::Subscript(items)
    }

    pub fn new_underline(items: Vec<Box<Inline>>) -> Self {
        Inline::Underline(items)
    }

    pub fn new_smart_punctuation(smart_punctuation: SmartPunctuation) -> Self {
        Inline::SmartPunctuation(smart_punctuation)
    }

    pub fn new_no_loc(items: Vec<Box<Inline>>) -> Self {
        Inline::NoLoc(items)
    }

    pub fn new_xref(id: String) -> Self {
        Inline::Xref(id)
    }


    pub fn get_text(&self) -> Option<&String> {
        match self {
            Inline::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn get_link(&self) -> Option<(&Vec<Box<Inline>>, &PathType)> {
        match self {
            Inline::Link(items, path) => Some((items, path)),
            _ => None,
        }
    }

    pub fn get_bold(&self) -> Option<&Vec<Box<Inline>>> {
        match self {
            Inline::Bold(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_italic(&self) -> Option<&Vec<Box<Inline>>> {
        match self {
            Inline::Italic(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_strikethrough(&self) -> Option<&Vec<Box<Inline>>> {
        match self {
            Inline::Strikethrough(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_superscript(&self) -> Option<&Vec<Box<Inline>>> {
        match self {
            Inline::Superscript(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_subscript(&self) -> Option<&Vec<Box<Inline>>> {
        match self {
            Inline::Subscript(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_underline(&self) -> Option<&Vec<Box<Inline>>> {
        match self {
            Inline::Underline(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_smart_punctuation(&self) -> Option<&SmartPunctuation> {
        match self {
            Inline::SmartPunctuation(smart_punctuation) => Some(smart_punctuation),
            _ => None,
        }
    }

    pub fn get_no_loc(&self) -> Option<&Vec<Box<Inline>>> {
        match self {
            Inline::NoLoc(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_xref(&self) -> Option<&String> {
        match self {
            Inline::Xref(id) => Some(id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TextBlock {
    content: String,
    indent: u8,
}
// TODO: REMOVE This after Inline is implemented
impl TextBlock {
    fn new(text: String, indent: u8) -> Self {
        Self {
            content: text,
            indent,
        }
    }

    fn get_content(&self) -> &String {
        &self.content
    }

    fn get_content_mut(&mut self) -> &mut String {
        &mut self.content
    }
}

impl Indent for TextBlock {
    fn get_indent(&self) -> u8 {
        self.indent
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Xref {
    pub id: String,
    pub display: Option<String>,
}

struct OpSingleSelector {
    items: Vec<OpSingleSelectorItem>,
}

struct OpSingleSelectorItem {}

enum OpSelector {
    Single(OpSingleSelector),
    Multi(),
}

trait Indent {
    fn get_indent(&self) -> u8;
}

trait Content {
    fn get_content(&self) -> &MsMarkdown;
}

#[derive(Debug, Clone, PartialEq)]
enum Image {
    Complex {
        alt_text: String,
        source: PathType,
        /// loc-scope
        loc_scope: Option<String>,
        description: Option<MsMarkdown>,
        /// true by default
        border: bool,
    },
    Content {
        alt_text: String,
        source: PathType,
        /// loc-scope
        loc_scope: Option<String>,
        /// true by default
        border: bool,
    },
    Icon {
        source: PathType,
        /// not true by default
        border: bool,
    },
    // This is a normal markdown image
    Simple {
        alt_text: String,
        source: PathType,
    },
}

impl Image {
    fn new_complex(
        alt_text: String,
        source: PathType,
        loc_scope: Option<String>,
        description: Option<MsMarkdown>,
        border: bool,
    ) -> Self {
        Image::Complex {
            alt_text,
            source,
            loc_scope,
            description,
            border,
        }
    }

    fn new_content(
        alt_text: String,
        source: PathType,
        loc_scope: Option<String>,
        border: bool,
    ) -> Self {
        Image::Content {
            alt_text,
            source,
            loc_scope,
            border,
        }
    }

    fn new_icon(source: PathType, border: bool) -> Self {
        Image::Icon { source, border }
    }

    fn new_simple(alt_text: String, source: PathType) -> Self {
        Image::Simple { alt_text, source }
    }

    fn get_alt_text(&self) -> &String {
        match self {
            Image::Complex { alt_text, .. } => alt_text,
            Image::Content { alt_text, .. } => alt_text,
            Image::Icon { .. } => &String::from(""),
            Image::Simple { alt_text, .. } => alt_text,
        }
    }

    fn get_source(&self) -> &PathType {
        match self {
            Image::Complex { source, .. } => source,
            Image::Content { source, .. } => source,
            Image::Icon { source, .. } => source,
            Image::Simple { source, .. } => source,
        }
    }

    fn get_loc_scope(&self) -> &Option<String> {
        match self {
            Image::Complex { loc_scope, .. } => loc_scope,
            Image::Content { loc_scope, .. } => loc_scope,
            Image::Icon { .. } => &None,
            Image::Simple { .. } => &None,
        }
    }

    fn get_description(&self) -> &Option<MsMarkdown> {
        match self {
            Image::Complex { description, .. } => description,
            Image::Content { .. } => &None,
            Image::Icon { .. } => &None,
            Image::Simple { .. } => &None,
        }
    }

    fn get_border(&self) -> bool {
        match self {
            Image::Complex { border, .. } => *border,
            Image::Content { border, .. } => *border,
            Image::Icon { border, .. } => *border,
            Image::Simple { .. } => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathType {
    Local(String),
    Remote(url::Url),
}

pub struct Columns {
    columns: Vec<Column>,
}

pub struct Checklist {
    pub items: Vec<ChecklistItem>,
}

pub struct ChecklistItem {
    pub content: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
enum SmartPunctuation {
    /// “
    DoubleLeftQuote,
    /// ”
    DoubleRightQuote,
    /// ‘
    SingleLeftQuote,
    /// ’
    SingleRightQuote,
}

#[derive(Debug, Clone, PartialEq)]
struct List {
    indent: u8,
    items: Vec<ListItem>,
    list_type: ListType,
}

#[derive(Debug, Clone, PartialEq)]
enum ListType {
    Ordered,
    Unordered,
}

#[derive(Debug, Clone, PartialEq)]
struct Anchor {
    pub name: Option<String>,
    pub id: Option<String>,
}

impl Anchor {
    pub fn new(name: Option<String>, id: Option<String>) -> Self {
        Self { name, id }
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn get_id(&self) -> &Option<String> {
        &self.id
    }
}

/// ## <a id="anchortext" />Header text
#[derive(Debug, Clone, PartialEq)]
struct Heading {
    pub level: u8,
    pub name: String,
    pub anchor: Option<Anchor>,
}

impl Heading {
    pub fn new(level: u8, name: String, anchor: Option<Anchor>) -> Self {
        Self {
            level,
            name,
            anchor,
        }
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_anchor(&self) -> &Option<Anchor> {
        &self.anchor
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BlockQuote {
    pub indent: u8,
    pub content: MsMarkdown,
}

impl BlockQuote {
    pub fn new(indent: u8, content: MsMarkdown) -> Self {
        Self { indent, content }
    }
}

impl Indent for BlockQuote {
    fn get_indent(&self) -> u8 {
        self.indent
    }
}

impl Content for BlockQuote {
    fn get_content(&self) -> &MsMarkdown {
        &self.content
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertType {
    Note,
    Tip,
    Important,
    Caution,
    Warning,
}

impl ToString for AlertType {
    fn to_string(&self) -> String {
        match self {
            AlertType::Note => "NOTE".to_string(),
            AlertType::Tip => "TIP".to_string(),
            AlertType::Important => "IMPORTANT".to_string(),
            AlertType::Caution => "CAUTION".to_string(),
            AlertType::Warning => "WARNING".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Alert {
    pub indent: u8,
    pub content: MsMarkdown,
    pub alert_type: AlertType,
}

impl Alert {
    pub fn new(indent: u8, content: MsMarkdown, alert_type: AlertType) -> Self {
        Self {
            content,
            alert_type,
            indent,
        }
    }

    pub fn get_type(&self) -> &AlertType {
        &self.alert_type
    }
}

impl Indent for Alert {
    fn get_indent(&self) -> u8 {
        self.indent
    }
}

impl Content for Alert {
    fn get_content(&self) -> &MsMarkdown {
        &self.content
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub indent: u8,
    pub rows: Vec<HashMap<String, String>>,
}

impl Table {
    pub fn new(indent: u8, rows: Vec<HashMap<String, String>>) -> Self {
        Self { indent, rows }
    }

    pub fn get_indent(&self) -> u8 {
        self.indent
    }

    pub fn get_rows(&self) -> &Vec<HashMap<String, String>> {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsMarkdown {
    pub tokens: Vec<MsMarkdownToken>,
}

impl MsMarkdown {
    pub fn new(tokens: Vec<MsMarkdownToken>) -> Self {
        Self { tokens }
    }

    pub fn get_metadata(&self) -> Option<&MsMdMetadata> {
        // return first token if it is metadata

        if let Some(MsMarkdownToken::Metadata(metadata)) = self.tokens.get(0) {
            return Some(metadata);
        }

        None
    }
}

use html2md::parse_html;

impl From<&Path> for MsMarkdown {
    fn from(path: &Path) -> Self {
        let text = fs::read_to_string(path).unwrap();

        let configuration = configuration::ConfigurationBuilder::new().build();

        let text = parse_html(&text);

        let text = format_text(&text, &configuration, |_, _, _| Ok(None))
            .unwrap()
            .unwrap();

        fs::write("after_fmt_test.md", &text).unwrap();

        const REGEXP: &str = r#"/(?<comment>^[ ]*?<!-{2,3}(?:[\s\S]*?)-{2,3}>$)|(?<code_block>[ ]*?`{1,4}[a-zA-Z]*?\n(?:[^`]+) *?`{1,4}\n)|(?<metadata>(?:^---\n)(?:\s|(^.+:.+\n))+(?:^---$))|(?<md_table>\|(?:.+)\|\s+\|(?: ?:?-+:? ?\|)+\s+(?:(?:\|(?:.+)\|)+\s*)+)|(?<html_table>^<table[\s\w\W]*(:?^<\/table>))|(?<heading>(?:^#{1,6}\s)(?:.*?).$)|(?<code_ext>(?:^[ ]*?:::code.+:::))|(?<multi_line_image_ext>^:::image.+:::\n(?:.+\n)+(?:^[ ]*?:::image-end:::$))|(?<single_line_image_ext>^[ ]*?:::image.+:::$)|(?<list>^[ ]*?(?:(?:\d\.)|(?:[+\-*])) .+\n(?:^[ ]*?(?:(?:\d\.)|(?:[-+*])) .+\n)*)|(?<horizontal_line>^-{3}\n)|(?<image>(?:^[ ]*?!\[.*?\]\(.*?\)))|(?<next_step_action_ext>^[ ]*?> \[!div class="nextstepaction"\]\n(?:^[ ]*?> \[.+\]\(.+\)\n)+)|(?<op_multi_selector_ext>^[ ]*?> \[!div class="op_multi_selector".*?\]\s(?:> - \[.+\]\(.+\)\n)+)|(?<op_single_selector_ext>^> \[!div class="op_single_selector"\]\n(> - \[.+\]\(.+\)\n)+)|(?<checklist>^[ ]*?> \[!div class="checklist"\]\n(?:(?:^[ ]*?> [-*] .+\n)|(^[ ]*?>\n))+)|(?<alert>(?:^[ ]*?>[^ ]!(?:(?:[Nn][Oo][Tt][Ee])|(?:[Tt][Ii][Pp])|(?:[Ii][Mm][Pp][Oo][Rr][Tt][Aa][Nn][Tt])|(?:[Cc][Aa][Uu][Tt][Ii][Oo][Nn])|(?:[Ww][Aa][Rr][Nn][Ii][Nn][Gg]))\]\n(?:(?:[ ]*?>[^ ].+\n)|(?:[ ]*?>\n))+)|(^[ ]*?>[ ]*?\[!(?:(?:[Nn][Oo][Tt][Ee])|(?:[Tt][Ii][Pp])|(?:[Ii][Mm][Pp][Oo][Rr][Tt][Aa][Nn][Tt])|(?:[Cc][Aa][Uu][Tt][Ii][Oo][Nn])|(?:[Ww][Aa][Rr][Nn][Ii][Nn][Gg]))\]\n(?:(?:[ ]*?> .+\n)|(?:[ ]*?>\n))+))|(?<row>^:::row(?:[\s\S]*?)row-end:::$)|(?<column>^:::column(?:[\S\s]*?)column-end:::.$)|(?<multi_line_quote>^[ ]*?>[ ]?.+\n(?:(?:[ ]*?>[ ]?.+\n)|(?:[ ]*?>\n))+)|(?<single_line_quote>^[ ]*?>[ ]?.+)|(?<line_break>\n{2})|(?<text_block>^.+$)/gm"#;

        let matches = ecma_regex_match_groups::<MsMdRegexGroup>(&text, REGEXP, None).unwrap();

        let mut tokens = Vec::new();

        for m in matches {
            match m.group {
                MsMdRegexGroup::Metadata => {
                    // remove the --- from the beginning and end of the metadata

                    let m = RegexpGroupMatch {
                        group: m.group,
                        text: m
                            .text
                            .trim_start_matches("---")
                            .trim_end_matches("---")
                            .trim()
                            .to_string(),
                    };
                    let metadata = serde_yaml::from_str(&m.text).unwrap();
                    tokens.push(MsMarkdownToken::Metadata(metadata));
                }
                MsMdRegexGroup::Comment => {
                    // extract the comment its like an html comment with regex
                    // <!-- comment -->
                    let comment = Regex::new(r"<!--(?P<comment>[\s\S]*?)-->").unwrap();
                    let comment = comment
                        .captures(&m.text)
                        .unwrap()
                        .name("comment")
                        .unwrap()
                        .as_str();
                    tokens.push(MsMarkdownToken::Comment(Comment::new(comment.to_string())));
                }
                MsMdRegexGroup::Heading => {
                    // extract the heading
                    let heading = Regex::new(r"^(?P<level>#{1,6})\s+(?P<text>.*)").unwrap();
                    let heading = heading.captures(&m.text).unwrap();
                    let level = heading.name("level").unwrap().as_str().len() as u8;
                    let text = heading
                        .name("text")
                        .unwrap()
                        .as_str()
                        .trim_end_matches(|c: char| c == '#' || c == ' ')
                        .to_string();

                    tokens.push(MsMarkdownToken::Heading(Heading {
                        level,
                        name: text,
                        // TODO
                        anchor: None,
                    }));
                }

                MsMdRegexGroup::CodeBlock => {
                    let code = Regex::new(r"```(?P<lang>.*?)\r?\n(?P<code>[\s\S]*?)```").unwrap();
                    let code = code.captures(&m.text).unwrap();
                    let lang = Language::from(code.name("lang").unwrap().as_str().to_string());

                    // indent  spaces

                    let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                    let code = code.name("code").unwrap().as_str().to_string();

                    // remove the indentation from the code

                    let code_block: Code = Code::new_block(
                        remove_space_indentation(&code, indentation as u8),
                        lang,
                        indentation as u8,
                    );

                    tokens.push(MsMarkdownToken::Code(code_block));
                }
                MsMdRegexGroup::CodeExt => {
                    let code = m.text.trim_start_matches(":::code").trim_end_matches(":::");

                    let re_lang = Regex::new(r#"language="(?P<data>.*?)""#).unwrap();
                    let re_source = Regex::new(r#"source="(?P<data>.*?)""#).unwrap();
                    let re_range = Regex::new(r#"range="(?P<data>.*?)""#).unwrap();
                    let re_id = Regex::new(r#"id="(?P<data>.*?)""#).unwrap();
                    let re_highlight = Regex::new(r#"highlight="(?P<data>.*?)""#).unwrap();
                    let re_interactive = Regex::new(r#"interactive="(?P<data>.*?)""#).unwrap();

                    //                let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                    // get indent spaces ` `
                    let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                    let mut out = Code::new_reference(
                        "".to_string(),
                        re_source
                            .captures(code)
                            .unwrap()
                            .name("data")
                            .unwrap()
                            .as_str()
                            .to_string(),
                        Language::from(
                            re_lang
                                .captures(code)
                                .unwrap()
                                .name("data")
                                .unwrap()
                                .as_str()
                                .to_string(),
                        ),
                        indentation as u8,
                    );

                    if let Some(captures) = re_range.captures(code) {
                        let range_data: Vec<u32> = captures["data"]
                            .split('-')
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect();

                        if range_data.len() == 2 {
                            out.set_range(Some([range_data[0], range_data[1]]))
                        }
                    }

                    if let Some(captures) = re_id.captures(code) {
                        out.set_id(Some(captures["data"].to_string()));
                    }

                    if let Some(captures) = re_highlight.captures(code) {
                        out.set_highlight(Some(captures["data"].to_string()));
                    }

                    if let Some(captures) = re_interactive.captures(code) {
                        out.set_interactive(Some(Interactive::from(captures["data"].to_string())));
                    }

                    let source_file_path = path.parent().unwrap().join(out.get_source().unwrap());

                    if let Some(range) = out.get_range() {
                        let source_file_content: Option<String> =
                            match fs::read_to_string(source_file_path) {
                                Ok(content) => Some(content),
                                Err(_) => None,
                            };

                        if source_file_content.is_none() {
                            tokens.push(MsMarkdownToken::Code(out));
                            continue;
                        }

                        let source_file_content = source_file_content.unwrap();

                        let source_file_lines: Vec<&str> = source_file_content.lines().collect();
                        out.set_content(
                            source_file_lines[range[0] as usize - 1..range[1] as usize - 1]
                                .join("\n")
                                .trim()
                                .to_string(),
                        );
                    } else {
                        out.set_content(match fs::read_to_string(source_file_path) {
                            Ok(content) => content,
                            Err(_) => "".to_string(),
                        });
                    }

                    tokens.push(MsMarkdownToken::Code(out));
                }

                MsMdRegexGroup::MdTable => {
                    let table_str = m
                        .text
                        .replace("> |", "|")
                        .replace("| ", "|")
                        .replace(" |", "|");
                    let mut lines = table_str.lines().collect::<Vec<_>>();
                    let headers_line = lines.remove(0);
                    let headers = headers_line.split("|").collect::<Vec<_>>();

                    // Check if all headers are empty

                    let is_headers_empty = headers.iter().all(|x| x.trim().is_empty());

                    if is_headers_empty {
                        let mut table_data: Vec<Vec<String>> = Vec::new();

                        for line in lines {
                            if line.trim().is_empty()
                                || line.starts_with("|--")
                                || line.starts_with("|:--")
                            {
                                continue;
                            }

                            let row_data = line.split("|").collect::<Vec<_>>();
                            let mut row_map = Vec::new();

                            for cell in row_data.iter() {
                                // check if both are empty if so continue
                                if cell.trim().is_empty() {
                                    continue;
                                }

                                row_map.push(cell.trim().to_string());
                            }

                            table_data.push(row_map);
                        }

                        panic!("empty headers");

                        //   tokens.push(MsMarkdownToken::Table(Table::new(0,  table_data)));
                        continue;
                    }
                    let mut table_data = Vec::new();

                    for line in lines {
                        if line.trim().is_empty()
                            || line.starts_with("|--")
                            || line.starts_with("|:--")
                        {
                            continue;
                        }

                        let row_data = line.split("|").collect::<Vec<_>>();
                        let mut row_map = HashMap::new();

                        for (header, cell) in headers.iter().zip(row_data.iter()) {
                            // check if both are empty if so continue
                            if header.trim().is_empty() && cell.trim().is_empty() {
                                continue;
                            }

                            row_map.insert(header.trim().to_string(), cell.trim().to_string());
                        }

                        table_data.push(row_map);
                    }
                    // TODO: Indents?
                    tokens.push(MsMarkdownToken::Table(Table::new(0, table_data)));
                }

                MsMdRegexGroup::HtmlTable => {
                    let fragment = Html::parse_fragment(&m.text);
                    let tr_selector = Selector::parse("tr").unwrap();
                    let th_selector = Selector::parse("th").unwrap();
                    let td_selector = Selector::parse("td").unwrap();

                    let mut table_data = Vec::new();
                    let mut headers = Vec::new();
                    let mut is_header = true;

                    for tr in fragment.select(&tr_selector) {
                        if is_header {
                            for th in tr.select(&th_selector) {
                                headers.push(th.text().collect::<String>().trim().to_string());
                            }
                            is_header = false;
                        } else {
                            let row_data = tr
                                .select(&td_selector)
                                .map(|td| td.text().collect::<String>().trim().to_string())
                                .collect::<Vec<_>>();
                            let mut row_map = HashMap::new();

                            for (header, cell) in headers.iter().zip(row_data.iter()) {
                                row_map.insert(header.clone(), cell.clone());
                            }
                            table_data.push(row_map);
                        }
                    }
                    // TODO: Unnamed table
                    tokens.push(MsMarkdownToken::Table(Table::new(0, table_data)));
                }

                MsMdRegexGroup::LineBreak => {
                    tokens.push(MsMarkdownToken::LineBreak);
                }

                MsMdRegexGroup::TextBlock => {
                    let indent_level = m.text.chars().take_while(|c| c.is_whitespace()).count();

                    let text_block = m.text.trim().to_string();

                    if text_block.is_empty() {
                        continue;
                    }
                    // get indent level

                    let text_block = text_block.trim().to_string();

                    let text_block = text_block
                        .lines()
                        .map(|line| line.trim())
                        .collect::<Vec<_>>()
                        .join("\n");

                    tokens.push(MsMarkdownToken::TextBlock(TextBlock::new(
                        text_block,
                        indent_level as u8,
                    )));
                }

                MsMdRegexGroup::Row => {
                    let row_regex = Regex::new(r"(?s):::row:::(.*?):::row-end:::").unwrap();
                    let column_regex =
                        Regex::new(r#"(?s):::column ?span="(\d*)":::(.*?):::column-end:::"#)
                            .unwrap();

                    for row_captures in row_regex.captures_iter(&m.text) {
                        let row_content = row_captures.get(1).unwrap().as_str();
                        let mut row_data = Vec::new();

                        for column_captures in column_regex.captures_iter(row_content) {
                            let span = column_captures.get(1).unwrap().as_str();
                            let content = column_captures.get(2).unwrap().as_str();

                            // split content by new line and trim then join back together

                            let content = content
                                .lines()
                                .map(|line| line.trim())
                                .collect::<Vec<&str>>()
                                .join("\n");

                            row_data.push(Column {
                                span: match span.parse() {
                                    Ok(span) => Some(span),
                                    Err(_) => None,
                                }, //TODO INLINE PARSING
                                content: content.to_string(),
                            });
                        }

                        tokens.push(MsMarkdownToken::Columns(row_data));
                    }
                }
                /*
                            MsMarkdownGroup::SingleLineImageExt => {
                                // an  or group with each type="([^"]*)" source="([^"]*)" alt-text="([^"]*)" in (?s):::image :::

                                // type regex
                                let type_regex = Regex::new(r#"(?s)type="([^"]*)""#).unwrap();
                                // source regex
                                let source_regex = Regex::new(r#"(?s)source="([^"]*)""#).unwrap();
                                // alt-text regex
                                let alt_text_regex = Regex::new(r#"(?s)alt-text="([^"]*)""#).unwrap();

                                // border

                                let border_regex = Regex::new(r#"(?s)border="([^"]*)""#).unwrap();

                                /*pub struct MsMdImage {
                                    pub alt_text: String,
                                    pub source: String,
                                    pub image_type: Option<ImageType>,
                                }

                                pub enum ImageType {
                                    Complex,
                                    Icon,
                                    Content,
                                } */

                                // get type exists then make enum variant else None

                                let image_type = match type_regex.captures(&m.text) {
                                    Some(captures) => match captures.get(1).unwrap().as_str() {
                                        "complex" => Some(ImageType::Complex),
                                        "icon" => Some(ImageType::Icon),
                                        "content" => Some(ImageType::Content),
                                        _ => None,
                                    },
                                    None => None,
                                };

                                let out = MsMdImage {
                                    image_type,
                                    source: source_regex
                                        .captures(&m.text)
                                        .unwrap()
                                        .get(1)
                                        .unwrap()
                                        .as_str()
                                        .to_string(),
                                    alt_text: alt_text_regex
                                        .captures(&m.text)
                                        .unwrap()
                                        .get(1)
                                        .unwrap()
                                        .as_str()
                                        .to_string(),
                                    // border may not exist  if it does not it is false
                                    border: match border_regex.captures(&m.text) {
                                        Some(captures) => match captures.get(1).unwrap().as_str() {
                                            "true" => true,
                                            "false" => false,
                                            _ => false,
                                        },
                                        None => false,
                                    },

                                    description: None,
                                };
                            }

                            MsMarkdownGroup::MultiLineImageExt => {}
                */
                MsMdRegexGroup::Alert => {
                    let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                    let alert_type_regex = Regex::new(r#"(?s)\[!([a-zA-Z]+)\]"#).unwrap();

                    let alert_type = match alert_type_regex.captures(&m.text) {
                        Some(captures) => {
                            match captures.get(1).unwrap().as_str().to_lowercase().as_str() {
                                "note" => AlertType::Note,
                                "tip" => AlertType::Tip,
                                "important" => AlertType::Important,
                                "caution" => AlertType::Caution,
                                "warning" => AlertType::Warning,
                                _ => panic!("unhandled alert type"),
                            }
                        }
                        None => panic!("no alert type"),
                    };

                    // get the content of the alert, remove the first line which is the alert type make sure to remove the > from the start of the line and trim the line

                    let content = remove_space_indentation(&m.text, indentation as u8)
                        .lines()
                        .skip(1)
                        .map(|line| line.trim().trim_start_matches(">").trim())
                        .collect::<Vec<&str>>()
                        .join("\n");

                    tokens.push(MsMarkdownToken::Alert {
                        alert_type,
                        content,
                        indent: indentation as u8,
                    });
                }
                // quote multiline
                MsMdRegexGroup::MultiLineQuote => {
                    // get the content of the quote, remove the first line which is the quote type make sure to remove the > from the start of the line and trim the line

                    let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                    let content = remove_space_indentation(&m.text, indentation as u8)
                        .lines()
                        .map(|line| line.trim_start_matches(">").trim())
                        .collect::<Vec<&str>>()
                        .join("\n");

                    tokens.push(MsMarkdownToken::BlockQuote {
                        content,
                        indent: indentation as u8,
                    });
                }
                // quote single line
                MsMdRegexGroup::SingleLineQuote => {
                    // get the content of the quote, remove the first line which is the quote type make sure to remove the > from the start of the line and trim the line

                    let content = m
                        .text
                        .lines()
                        .map(|line| line.trim_start_matches(">").trim())
                        .collect::<Vec<&str>>()
                        .join("\n");

                    let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                    tokens.push(MsMarkdownToken::BlockQuote {
                        content,
                        indent: indentation as u8,
                    });
                }
                // List with nests
                MsMdRegexGroup::List => {
                    let mut items = vec![];

                    for line in m.text.lines() {
                        let indentation = line.find(|c: char| !c.is_whitespace()).unwrap();

                        let list_type = if line.starts_with(|c: char| c.is_digit(10)) {
                            ListOrderType::Ordered
                        } else {
                            ListOrderType::Unordered
                        };

                        let list_start_regex = Regex::new(r#"(?m)^(?:(?:\d+\.)|[-*])"#).unwrap();

                        let line = list_start_regex.replace(&line.trim_start(), "");

                        let content = line.trim();

                        items.push(ListItem {
                            indent: indentation as u8,
                            content: content.to_string(),
                        });
                    }

                    tokens.push(MsMarkdownToken::List(items));
                }
                MsMdRegexGroup::HorizontalLine => {
                    tokens.push(MsMarkdownToken::HorizontalLine);
                }
                _ => {
                    println!("unhandled group {:?}", m.group);
                }
            }
        }
        MsMarkdown { tokens }
    }
}

#[test]
fn md_from_path() {
    let path = Path::new("test.md");

    let md = MsMarkdown::from(path);

    // make a an output file with the tokens as pretty

    fs::write("md_test_parse_output.rs", format!("{:#?}", &md)).unwrap();
}

pub fn remove_space_indentation(text: &str, indent: u8) -> String {
    let mut out = vec![];

    for line in text.lines() {
        let regex = Regex::new(&format!("^ {{{}}}", indent)).unwrap();

        let line = regex.replace(&line, "");

        out.push(line.to_string());
    }

    out.join("\n")
}

#[derive(Debug, Deserialize, Serialize)]
enum MsMdRegexGroup {
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "code_block")]
    CodeBlock,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "md_table")]
    MdTable,
    #[serde(rename = "html_table")]
    HtmlTable,
    #[serde(rename = "heading")]
    Heading,
    #[serde(rename = "code_ext")]
    CodeExt,
    #[serde(rename = "multi_line_image_ext")]
    MultiLineImageExt,
    #[serde(rename = "single_line_image_ext")]
    SingleLineImageExt,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "horizontal_line")]
    HorizontalLine,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "next_step_action_ext")]
    NextStepActionExt,
    #[serde(rename = "op_multi_selector_ext")]
    OpMultiSelectorExt,
    #[serde(rename = "op_single_selector_ext")]
    OpSingleSelectorExt,
    #[serde(rename = "checklist")]
    Checklist,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "row")]
    Row,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "multi_line_quote")]
    MultiLineQuote,
    #[serde(rename = "single_line_quote")]
    SingleLineQuote,
    #[serde(rename = "line_break")]
    LineBreak,
    #[serde(rename = "text_block")]
    TextBlock,
}

impl Display for MsMdRegexGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsMdRegexGroup::Comment => write!(f, "comment"),
            MsMdRegexGroup::CodeBlock => write!(f, "code_block"),
            MsMdRegexGroup::Metadata => write!(f, "metadata"),
            MsMdRegexGroup::MdTable => write!(f, "md_table"),
            MsMdRegexGroup::HtmlTable => write!(f, "html_table"),
            MsMdRegexGroup::Heading => write!(f, "heading"),
            MsMdRegexGroup::CodeExt => write!(f, "code_ext"),
            MsMdRegexGroup::MultiLineImageExt => write!(f, "multi_line_image_ext"),
            MsMdRegexGroup::SingleLineImageExt => write!(f, "single_line_image_ext"),
            MsMdRegexGroup::List => write!(f, "list"),
            MsMdRegexGroup::HorizontalLine => write!(f, "horizontal_line"),
            MsMdRegexGroup::Image => write!(f, "image"),
            MsMdRegexGroup::NextStepActionExt => write!(f, "next_step_action_ext"),
            MsMdRegexGroup::OpMultiSelectorExt => write!(f, "op_multi_selector_ext"),
            MsMdRegexGroup::OpSingleSelectorExt => write!(f, "op_single_selector_ext"),
            MsMdRegexGroup::Checklist => write!(f, "checklist"),
            MsMdRegexGroup::Alert => write!(f, "alert"),
            MsMdRegexGroup::Row => write!(f, "row"),
            MsMdRegexGroup::Column => write!(f, "column"),
            MsMdRegexGroup::MultiLineQuote => write!(f, "multi_line_quote"),
            MsMdRegexGroup::SingleLineQuote => write!(f, "single_line_quote"),
            MsMdRegexGroup::LineBreak => write!(f, "line_break"),
            MsMdRegexGroup::TextBlock => write!(f, "text_block"),
        }
    }
}

// TODO: Use this with serde when implemented
/*
#[derive(Debug, Clone, PartialEq, )]
enum MsMdTokenType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "link")]
    Table,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "quote")]
    Quote,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "horizontal_line")]
    HorizontalLine,
    #[serde(rename = "table")]
    Comment,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "heading")]
    Heading,
    #[serde(rename = "line_break")]
    LineBreak,
}
*/

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub indent: u8,
    pub content: MsMarkdown,
}

impl ListItem {
    pub fn new(indent: u8, content: MsMarkdown) -> Self {
        Self { indent, content }
    }
}

impl Indent for ListItem {
    fn get_indent(&self) -> u8 {
        self.indent
    }
}

impl Content for ListItem {
    fn get_content(&self) -> &MsMarkdown {
        &self.content
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub span: Option<u8>,
    pub content: MsMarkdown, // TODO change to MsMarkdown
}

impl Column {
    pub fn new(span: Option<u8>, content: MsMarkdown) -> Self {
        Self { span, content }
    }

    pub fn get_span(&self) -> u8 {
        match self.span {
            Some(span) => span,
            None => 1,
        }
    }
}

impl Content for Column {
    fn get_content(&self) -> &MsMarkdown {
        &self.content
    }
}

// TODO: Image type
/*
#[derive(Debug)]
pub struct MsMdImage {
    pub alt_text: String,
    pub source: String,
    pub image_type: Option<ImageType>,
    pub border: bool,
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum ImageType {
    Complex,
    Icon,
    Content,
}
*/

/// The generic type <T> is the type of the group name which serializes from a string it defaults to String
#[derive(Debug, Serialize, Deserialize)]
struct RegexpGroupMatch<T = String> {
    group: T,
    text: String,
}

impl Display for RegexpGroupMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Match {{ group: {:?}, text: {:?} }}",
            self.group, self.text
        )
    }
}

/// #### Description
/// Matches all named groups in a regex
/// #### Example
/// ```js
/// /test/gm
/// ```
fn ecma_regex_match_groups<T>(
    text: &str,
    regex: &str,
    js_runtime: Option<JsRuntime>,
) -> Result<Vec<RegexpGroupMatch<T>>, String>
where
    T: DeserializeOwned,
{
    // Initialize the runtime if it is not provided
    let mut runtime = js_runtime.unwrap_or_else(|| JsRuntime::new(Default::default()));
    /// The match code that collects the matches
    const CODE:&str = "let matches = [];for (let match of text.matchAll(regex)) if (match.groups) for (let group of Object.keys(match.groups) ) {if (!match.groups[group]) continue; matches.push({group:group, text:match.groups[group]})};matches";
    // Evaluate the code and return the result
    eval_js::<Vec<RegexpGroupMatch<T>>>(
        &mut runtime,
        &format!(
            "let text=`{}`,regex={};{}",
            text.replace(r"\", r"\\").replace("`", r"\`"),
            regex,
            CODE
        ),
    )
}

use deno_core::ModuleCode;

// From the examples in the deno_core repo. Slightly modified
fn eval_js<T>(context: &mut JsRuntime, code: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let res = context.execute_script("Regex Group", ModuleCode::from(code.to_string()));
    match res {
        Ok(global) => {
            let scope = &mut context.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value = serde_v8::from_v8::<T>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
            }
        }
        Err(err) => Err(format!("Evaling error: {err:?}")),
    }
}
