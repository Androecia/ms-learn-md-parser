use std::fs;
use std::path::Path;
use std::{collections::HashMap, fmt::Display};

/// ||||||(?:\*\*\*(?<boldAndItalic>[^*]*)\*\*\*)|(?:\*\*(?<bold>[^*]*)\*\*)|(?:\*(?<italic>[^*]*)\*)|(?:~~(?<strikethrough>[^~]*)~~)||||(?<quote>((?<!> .+\n)> (?:.+))|(> (?:.+)(?!.*\n> .+))|(> (?:.+)))|)
// \[!INCLUDE \[.+\]\(.+\)]

// need text block regex
// (?<text_block>^[ ]*?[^-!>].+$)

// (?<comment>^[ ]*?<!-{2,3}(?:[\s\S]*?)-{2,3}>$)|(?<code_block>[ ]*?`{1,4}[a-zA-Z]*?\n(?:[^`]+) *?`{1,4}\n)|(?<metadata>(?:^---\n)(?:^.+:.+\n)+(?:^---$))|(?<md_table>\|(?:.+)\|\n\|(?: ?:?-+:? ?\|)+\n(?:(?:\|(?:.+)\|)+\n)+)|(?<html_table>^<table[\s\w\W]*(:?^<\/table>))|(?<heading>(?:^#{1,6}\s)(?:.*?).$)|(?<code_ext>(?:^[ ]*?:::code.+:::))|(?<multi_line_image_ext>^:::image.+:::\n(?:.+\n)+(?:^[ ]*?:::image-end:::$))|(?<single_line_image_ext>^[ ]*?:::image.+:::$)|(?<list>^[ ]*?(?:(?:\d\.)|(?:[-*])) .+\n(?:^[ ]*?(?:(?:\d\.)|(?:[-*])) .+\n)*)|(?<horizontal_line>^-{3}\n)|(?<image>(?:^[ ]*?!\[.*?\]\(.*?\)))|(?<next_step_action_ext>^[ ]*?> \[!div class="nextstepaction"\]\n(?:^[ ]*?> \[.+\]\(.+\)\n)+)|(?<op_multi_selector_ext>^[ ]*?> \[!div class="op_multi_selector".*?\]\s(?:> - \[.+\]\(.+\)\n)+)|(?<op_single_selector_ext>^> \[!div class="op_single_selector"\]\n(> - \[.+\]\(.+\)\n)+)|(?<checklist>^[ ]*?> \[!div class="checklist"\]\n(?:(?:^[ ]*?> [-*] .+\n)|(^[ ]*?>\n))+)|(?<alert>(?:^[ ]*?>[^ ]!(?:(?:[Nn][Oo][Tt][Ee])|(?:[Tt][Ii][Pp])|(?:[Ii][Mm][Pp][Oo][Rr][Tt][Aa][Nn][Tt])|(?:[Cc][Aa][Uu][Tt][Ii][Oo][Nn])|(?:[Ww][Aa][Rr][Nn][Ii][Nn][Gg]))\]\n(?:(?:[ ]*?>[^ ].+\n)|(?:[ ]*?>\n))+)|(^[ ]*?>[ ]*?\[!(?:(?:[Nn][Oo][Tt][Ee])|(?:[Tt][Ii][Pp])|(?:[Ii][Mm][Pp][Oo][Rr][Tt][Aa][Nn][Tt])|(?:[Cc][Aa][Uu][Tt][Ii][Oo][Nn])|(?:[Ww][Aa][Rr][Nn][Ii][Nn][Gg]))\]\n(?:(?:[ ]*?> .+\n)|(?:[ ]*?>\n))+))|(?<row>^:::row(?:[\s\S]*?)row-end:::$)|(?<column>^:::column(?:[\S\s]*?)column-end:::.$)|(?<multi_line_quote>^[ ]?>[ ]?.+\n(?:(?:[ ]*?>[ ]?.+\n)|(?:[ ]*?>\n))+)|(?<single_line_quote>^[ ]*?>[ ]?.+)|(?<line_break>\n{2})|(?<text_block>.*?)
use deno_core::JsRuntime;

// This is a hack to make the `#[op]` macro work with
// deno_core examples.
// You can remove this:
use deno_core::*;

use deno_core::v8;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum MsMarkdownGroup {
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

impl Display for MsMarkdownGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsMarkdownGroup::Comment => write!(f, "comment"),
            MsMarkdownGroup::CodeBlock => write!(f, "code_block"),
            MsMarkdownGroup::Metadata => write!(f, "metadata"),
            MsMarkdownGroup::MdTable => write!(f, "md_table"),
            MsMarkdownGroup::HtmlTable => write!(f, "html_table"),
            MsMarkdownGroup::Heading => write!(f, "heading"),
            MsMarkdownGroup::CodeExt => write!(f, "code_ext"),
            MsMarkdownGroup::MultiLineImageExt => write!(f, "multi_line_image_ext"),
            MsMarkdownGroup::SingleLineImageExt => write!(f, "single_line_image_ext"),
            MsMarkdownGroup::List => write!(f, "list"),
            MsMarkdownGroup::HorizontalLine => write!(f, "horizontal_line"),
            MsMarkdownGroup::Image => write!(f, "image"),
            MsMarkdownGroup::NextStepActionExt => write!(f, "next_step_action_ext"),
            MsMarkdownGroup::OpMultiSelectorExt => write!(f, "op_multi_selector_ext"),
            MsMarkdownGroup::OpSingleSelectorExt => write!(f, "op_single_selector_ext"),
            MsMarkdownGroup::Checklist => write!(f, "checklist"),
            MsMarkdownGroup::Alert => write!(f, "alert"),
            MsMarkdownGroup::Row => write!(f, "row"),
            MsMarkdownGroup::Column => write!(f, "column"),
            MsMarkdownGroup::MultiLineQuote => write!(f, "multi_line_quote"),
            MsMarkdownGroup::SingleLineQuote => write!(f, "single_line_quote"),
            MsMarkdownGroup::LineBreak => write!(f, "line_break"),
            MsMarkdownGroup::TextBlock => write!(f, "text_block"),
        }
    }
}

pub struct MsMarkdown {
    pub tokens: Vec<MsMarkdownToken>,
}
#[derive(Debug)]
pub enum MsMarkdownToken {
    Metadata(MsMdMetadata),
    Comment(String),
    Heading {
        level: u8,
        text: String,
    },
    Code(MsMarkdownCode),
    Table(MdTable),
    LineBreak,
    // TODO: Further parsing for inline types
    TextBlock {
        indent: u8,
        content: String,
    },
    Row(Vec<MsMdColumn>),
    //Image(MsMdImage),
    Alert {
        indent: u8,
        content: String,
        alert_type: AlertType,
    },
    Quote {
        indent: u8,
        content: String,
    },
    List(Vec<MdListItem>),
    HorizontalLine,
}

#[derive(Debug)]
pub enum MdTable {
    Unnamed(Vec<Vec<String>>),
    Named(Vec<HashMap<String, String>>),
}

#[derive(Debug)]
pub struct MdQuote {
    pub indent: u8,
    pub text: String,
}
#[derive(Debug)]
pub struct MdTextBlock {
    pub indent: u8,
    pub text: String,
}

#[derive(Debug)]
pub enum MarkdownTableType {
    String,
    Nested(Box<HashMap<String, MarkdownTableType>>),
}

use regex::Regex;
use serde_yaml;

fn parse_ms_md_from_file(path: &Path) -> Vec<MsMarkdownToken> {
    let text = fs::read_to_string(path).unwrap();

    const REGEXP: &str = r#"/(?<comment>^[ ]*?<!-{2,3}(?:[\s\S]*?)-{2,3}>$)|(?<code_block>[ ]*?`{1,4}[a-zA-Z]*?\n(?:[^`]+) *?`{1,4}\n)|(?<metadata>(?:^---\n)(?:^.+:.+\n)+(?:^---$))|(?<md_table>\|(?:.+)\|\n\|(?: ?:?-+:? ?\|)+\n(?:(?:\|(?:.+)\|)+\n)+)|(?<html_table>^<table[\s\w\W]*(:?^<\/table>))|(?<heading>(?:^#{1,6}\s)(?:.*?).$)|(?<code_ext>(?:^[ ]*?:::code.+:::))|(?<multi_line_image_ext>^:::image.+:::\n(?:.+\n)+(?:^[ ]*?:::image-end:::$))|(?<single_line_image_ext>^[ ]*?:::image.+:::$)|(?<list>^[ ]*?(?:(?:\d\.)|(?:[+\-*])) .+\n(?:^[ ]*?(?:(?:\d\.)|(?:[-+*])) .+\n)*)|(?<horizontal_line>^-{3}\n)|(?<image>(?:^[ ]*?!\[.*?\]\(.*?\)))|(?<next_step_action_ext>^[ ]*?> \[!div class="nextstepaction"\]\n(?:^[ ]*?> \[.+\]\(.+\)\n)+)|(?<op_multi_selector_ext>^[ ]*?> \[!div class="op_multi_selector".*?\]\s(?:> - \[.+\]\(.+\)\n)+)|(?<op_single_selector_ext>^> \[!div class="op_single_selector"\]\n(> - \[.+\]\(.+\)\n)+)|(?<checklist>^[ ]*?> \[!div class="checklist"\]\n(?:(?:^[ ]*?> [-*] .+\n)|(^[ ]*?>\n))+)|(?<alert>(?:^[ ]*?>[^ ]!(?:(?:[Nn][Oo][Tt][Ee])|(?:[Tt][Ii][Pp])|(?:[Ii][Mm][Pp][Oo][Rr][Tt][Aa][Nn][Tt])|(?:[Cc][Aa][Uu][Tt][Ii][Oo][Nn])|(?:[Ww][Aa][Rr][Nn][Ii][Nn][Gg]))\]\n(?:(?:[ ]*?>[^ ].+\n)|(?:[ ]*?>\n))+)|(^[ ]*?>[ ]*?\[!(?:(?:[Nn][Oo][Tt][Ee])|(?:[Tt][Ii][Pp])|(?:[Ii][Mm][Pp][Oo][Rr][Tt][Aa][Nn][Tt])|(?:[Cc][Aa][Uu][Tt][Ii][Oo][Nn])|(?:[Ww][Aa][Rr][Nn][Ii][Nn][Gg]))\]\n(?:(?:[ ]*?> .+\n)|(?:[ ]*?>\n))+))|(?<row>^:::row(?:[\s\S]*?)row-end:::$)|(?<column>^:::column(?:[\S\s]*?)column-end:::.$)|(?<multi_line_quote>^[ ]*?>[ ]?.+\n(?:(?:[ ]*?>[ ]?.+\n)|(?:[ ]*?>\n))+)|(?<single_line_quote>^[ ]*?>[ ]?.+)|(?<line_break>\n{2})|(?<text_block>^.+$)/gm"#;

    let matches = ecma_regex_match_groups::<MsMarkdownGroup>(&text, REGEXP, None).unwrap();

    let mut tokens = Vec::new();

    for m in matches {
        match m.group {
            MsMarkdownGroup::Metadata => {
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

            MsMarkdownGroup::Comment => {
                // extract the comment its like an html comment with regex
                // <!-- comment -->
                let comment = Regex::new(r"<!--(?P<comment>[\s\S]*?)-->").unwrap();
                let comment = comment
                    .captures(&m.text)
                    .unwrap()
                    .name("comment")
                    .unwrap()
                    .as_str();
                tokens.push(MsMarkdownToken::Comment(comment.to_string()));
            }

            MsMarkdownGroup::Heading => {
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

                tokens.push(MsMarkdownToken::Heading { level, text });
            }

            MsMarkdownGroup::CodeBlock => {
                let code = Regex::new(r"```(?P<lang>.*?)\r?\n(?P<code>[\s\S]*?)```").unwrap();
                let code = code.captures(&m.text).unwrap();

                let lang = match code.name("lang") {
                    Some(lang) => Some(lang.as_str().to_string()),
                    None => None,
                };

                // TODO PARSE IF JSONC JSON or json jsonc etc

                // indent  spaces

                let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                let code = code.name("code").unwrap().as_str().to_string();

                // remove the indentation from the code

                tokens.push(MsMarkdownToken::Code(MsMarkdownCode {
                    indent: indentation as u8,
                    language: lang,
                    data: Some(remove_indentation(&code, indentation as u8)),
                    highlight: None,
                    id: None,
                    interactive: None,
                    range: None,
                    source: None,
                }));
            }
            MsMarkdownGroup::CodeExt => {
                let code = m.text.trim_start_matches(":::code").trim_end_matches(":::");

                let re_lang = Regex::new(r#"language="(?P<data>.*?)""#).unwrap();
                let re_source = Regex::new(r#"source="(?P<data>.*?)""#).unwrap();
                let re_range = Regex::new(r#"range="(?P<data>.*?)""#).unwrap();
                let re_id = Regex::new(r#"id="(?P<data>.*?)""#).unwrap();
                let re_highlight = Regex::new(r#"highlight="(?P<data>.*?)""#).unwrap();
                let re_interactive = Regex::new(r#"interactive="(?P<data>.*?)""#).unwrap();

                //                let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                /// get indent spaces ` `
                let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                let mut out = MsMarkdownCode {
                    indent: indentation as u8,
                    language: None,
                    data: None,
                    source: None,
                    range: None,
                    id: None,
                    highlight: None,
                    interactive: None,
                };
                if let Some(captures) = re_lang.captures(code) {
                    out.language = Some(captures["data"].to_string());
                }

                if let Some(captures) = re_source.captures(code) {
                    out.source = Some(captures["data"].to_string());
                }

                if let Some(captures) = re_range.captures(code) {
                    let range_data: Vec<u32> = captures["data"]
                        .split('-')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();

                    if range_data.len() == 2 {
                        out.range = Some([range_data[0], range_data[1]]);
                    }
                }

                if let Some(captures) = re_id.captures(code) {
                    out.id = Some(captures["data"].to_string());
                }

                if let Some(captures) = re_highlight.captures(code) {
                    out.highlight = Some(captures["data"].to_string());
                }

                if let Some(captures) = re_interactive.captures(code) {
                    out.interactive = Some(captures["data"].to_string());
                }

                let source_file_path = path.parent().unwrap().join(out.get_source().unwrap());

                if let Some(range) = out.range {
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
                    out.data = Some(
                        source_file_lines[range[0] as usize - 1..range[1] as usize - 1]
                            .join("\n")
                            .trim()
                            .to_string(),
                    );
                } else {
                    out.data = match fs::read_to_string(source_file_path) {
                        Ok(content) => Some(content),
                        Err(_) => None,
                    };
                }

                // TODO PARSE IF JSONC JSON or json jsonc etc

                tokens.push(MsMarkdownToken::Code(out));
            }

            MsMarkdownGroup::MdTable => {
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

                    tokens.push(MsMarkdownToken::Table(MdTable::Unnamed(table_data)));
                    continue;
                }
                let mut table_data = Vec::new();

                for line in lines {
                    if line.trim().is_empty() || line.starts_with("|--") || line.starts_with("|:--")
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
                tokens.push(MsMarkdownToken::Table(MdTable::Named(table_data)));
            }

            MsMarkdownGroup::HtmlTable => {
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
                tokens.push(MsMarkdownToken::Table(MdTable::Named(table_data)));
            }

            MsMarkdownGroup::LineBreak => {
                tokens.push(MsMarkdownToken::LineBreak);
            }

            MsMarkdownGroup::TextBlock => {
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

                tokens.push(MsMarkdownToken::TextBlock {
                    indent: indent_level as u8,
                    content: text_block,
                });
            }

            MsMarkdownGroup::Row => {
                let row_regex = Regex::new(r"(?s):::row:::(.*?):::row-end:::").unwrap();
                let column_regex =
                    Regex::new(r#"(?s):::column ?span="(\d*)":::(.*?):::column-end:::"#).unwrap();

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

                        row_data.push(MsMdColumn {
                            span: match span.parse() {
                                Ok(span) => Some(span),
                                Err(_) => None,
                            },
                            content: content.to_string(),
                        });
                    }

                    tokens.push(MsMarkdownToken::Row(row_data));
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
            MsMarkdownGroup::Alert => {
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

                let content = remove_indentation(&m.text, indentation as u8)
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
            MsMarkdownGroup::MultiLineQuote => {
                // get the content of the quote, remove the first line which is the quote type make sure to remove the > from the start of the line and trim the line

                let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                let content = remove_indentation(&m.text, indentation as u8)
                    .lines()
                    .map(|line| line.trim_start_matches(">").trim())
                    .collect::<Vec<&str>>()
                    .join("\n");

                tokens.push(MsMarkdownToken::Quote {
                    content,
                    indent: indentation as u8,
                });
            }
            // quote single line
            MsMarkdownGroup::SingleLineQuote => {
                // get the content of the quote, remove the first line which is the quote type make sure to remove the > from the start of the line and trim the line

                let content = m
                    .text
                    .lines()
                    .map(|line| line.trim_start_matches(">").trim())
                    .collect::<Vec<&str>>()
                    .join("\n");

                let indentation = m.text.find(|c: char| !c.is_whitespace()).unwrap();

                tokens.push(MsMarkdownToken::Quote {
                    content,
                    indent: indentation as u8,
                });
            }
            // List with nests
            MsMarkdownGroup::List => {
                let mut items = vec![];

                for line in m.text.lines() {
                    let indentation = line.find(|c: char| !c.is_whitespace()).unwrap();

                    let list_type = if line.starts_with(|c: char| c.is_digit(10)) {
                        OrderType::Ordered
                    } else {
                        OrderType::Unordered
                    };

                    let list_start_regex = Regex::new(r#"(?m)^(?:(?:\d+\.)|[-*])"#).unwrap();

                    let line = list_start_regex.replace(&line.trim_start(), "");

                    let content = line.trim();

                    items.push(MdListItem {
                        indent: indentation as u8,
                        list_type,
                        content: content.to_string(),
                    });
                }

                tokens.push(MsMarkdownToken::List(items));
            }
            MsMarkdownGroup::HorizontalLine => {
                tokens.push(MsMarkdownToken::HorizontalLine);
            }
            _ => {
                println!("unhandled group {:?}", m.group);
            }
        }
    }
    tokens
}

#[derive(Debug, Clone, PartialEq)]
pub struct MdListItem {
    indent: u8,
    list_type: OrderType,
    content: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OrderType {
    Ordered,
    Unordered,
}
#[derive(Debug)]
pub struct MsMdColumn {
    pub span: Option<u8>,
    pub content: String,
}
#[derive(Debug)]
pub enum AlertType {
    Note,
    Tip,
    Important,
    Caution,
    Warning,
}
#[derive(Debug)]
pub struct MsMdAlert {
    pub indent: u8,
    pub alert_type: AlertType,
    pub content: String,
}

fn remove_indentation(text: &str, indent: u8) -> String {
    let mut out = vec![];

    for line in text.lines() {
        let regex = Regex::new(&format!("^ {{{}}}", indent)).unwrap();

        let line = regex.replace(&line, "");

        out.push(line.to_string());
    }

    out.join("\n")
}

#[test]
fn test_parse_ms_md() {
    let path = Path::new("test.md");

    let tokens = parse_ms_md_from_file(&path);

    // make a an output file with the tokens as pretty

    fs::write("md_test_parse_output.rs", format!("{:#?}", &tokens)).unwrap();

    for i in tokens {
        // println!("{:?}", i);
    }
}

/// Struct representing the metadata for a document.
#[derive(Debug, Deserialize, Serialize)]
pub struct MsMdMetadata {
    /// The author's GitHub account ID.
    author: String,

    /// A summary of the content. 75-300 characters.
    description: Option<String>,

    /// The author's Microsoft alias, without "@microsoft.com".
    /// Identifies the article's owner.
    #[serde(rename = "ms.author")]
    ms_author: String,

    /// A date in the format MM/DD/YYYY. Displayed on the published page
    /// to indicate the last time the article was substantially edited
    /// or guaranteed fresh.
    #[serde(rename = "ms.data")]
    ms_date: Option<String>,

    /// The service identifier. Used for issue triage and reporting.
    /// Generally, use for cloud applications.
    #[serde(rename = "ms.service")]
    ms_service: Option<String>,

    /// The product identifier. Used for issue triage and reporting.
    /// Generally, use for on-premises servers and applications.
    #[serde(rename = "ms.prod")]
    ms_prod: Option<String>,

    /// The type of content for reporting purposes.
    #[serde(rename = "ms.topic")]
    ms_topic: Option<String>,

    /// The page title. This is the page title that's displayed on the browser tab.
    /// It's the most important metadata for SEO.
    title: String,

    /// For writer or team use only. Used for tracking specific docs or sets of content in telemetry tools.
    /// The maximum string value length is 125 characters.
    #[serde(rename = "ms.custom")]
    ms_custom: Option<String>,

    /// The Microsoft alias of a person who reviews the content.
    #[serde(rename = "ms.reviewer")]
    ms_reviewer: Option<String>,

    /// The more granular service to which the content belongs. Only use if `ms.service` is also used.
    /// This attribute is a way to drill down further in reporting for a given `ms.service`.
    #[serde(rename = "ms.subservice")]
    ms_subservice: Option<String>,

    /// The technology to which the content belongs. Only use if `ms.prod` is also used.
    /// This attribute is a way to drill down further in reporting for a given `ms.prod`.
    #[serde(rename = "ms.technology")]
    technology: Option<String>,

    /// Use in your metadata section to prevent the build and publishing process from showing content on search pages.
    /// When you want to use ROBOTS (and yes, it's all capitalized, even though other metadata tags aren't):
    /// - Add ROBOTS: NOINDEX to your metadata section.
    /// - NOINDEX causes the asset to not show up in search results.
    /// - Use NOFOLLOW only when you archive an entire content set.
    robots: Option<Robots>,

    /// A list of words in the article that should never be translated (localized).
    /// Use this metadata to prevent "overlocalization."
    #[serde(rename = "no-loc")]
    no_loc: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Robots {
    #[serde(rename = "NOINDEX")]
    NoIndex,
    #[serde(rename = "NOFOLLOW")]
    NoFollow,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MarkdownHeading {
    pub level: u8,
    pub text: String,
}

enum Language {
    CSharp,
    JavaScript,
    TypeScript,
    JSON,
    JSONC,
    Other(String),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MsMarkdownCode {
    /// This is read from the file referenced in the source attribute.
    pub data: Option<String>,

    /// for ::: this is always available
    pub language: Option<String>,
    /// for ::: this is always available
    pub source: Option<String>,

    pub range: Option<[u32; 2]>,
    pub id: Option<String>,
    pub highlight: Option<String>,
    pub interactive: Option<String>,

    pub indent: u8,
}

impl MsMarkdownCode {
    pub fn get_source(&self) -> Option<&str> {
        self.source.as_deref()
    }
}
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

pub struct MarkdownChecklist {
    text: Vec<String>,
}

pub enum ListType {
    Star,
    Dash,
    Number,
}

pub struct MarkdownList {
    text: Vec<String>,
    list_type: ListType,
}

/// The generic type <T> is the type of the group name which serializes from a string it defaults to String
#[derive(Serialize, Deserialize, Debug)]
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
    runtime: Option<JsRuntime>,
) -> Result<Vec<RegexpGroupMatch<T>>, String>
where
    T: DeserializeOwned,
{
    // Initialize the runtime if it is not provided
    let mut runtime = runtime.unwrap_or_else(|| JsRuntime::new(Default::default()));
    /// The match code that collects the matches
    const CODE:&str = "let matches = [];for (let match of text.matchAll(regex)) if (match.groups) for (let group of Object.keys(match.groups) ) {if (!match.groups[group]) continue; matches.push({group:group, text:match.groups[group]})};matches";
    // Evaluate the code and return the result
    eval::<Vec<RegexpGroupMatch<T>>>(
        &mut runtime,
        &format!(
            "let text=`{}`,regex={};{}",
            text.replace(r"\", r"\\").replace("`", r"\`"),
            regex,
            CODE
        ),
    )
}

use scraper::{Html, Selector};

fn main() {}

use crate::serde::de::DeserializeOwned;
fn eval<T>(context: &mut JsRuntime, code: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let res = context.execute_script("<anon>", code);
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
