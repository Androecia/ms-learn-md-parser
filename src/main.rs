use lazy_static::lazy_static;
use regex::Regex;

use std::fs;

struct MarkdownHeading {
    pub level: u8,
    pub text: String,
}

enum Language {
    CSharp,
    JavaScript,
    TypeScript,
    JSON,
    JSONC,
}
struct MarkdownCodeBlock {
    pub language: Option<String>,
    pub code: String,
}

impl MarkdownCodeBlock {
    fn new(language: Option<String>, text: String) -> Self {
        Self {
            language,
            code: text,
        }
    }
}

struct MarkdownCodeReference {
    pub language: String,
    pub source: String,
    pub range: Option<[u32; 2]>,
    pub id: Option<String>,
    pub highlight: Option<String>,
    pub interactive: Option<String>,
}

struct MsDocMarkdownImage {
    alt_text: String,
    source: String,
    image_type: ImageType,
}

enum ImageType {
    Complex,
    Icon,
    Content,
}

struct MarkdownChecklist {
    text: Vec<String>,
}

enum ListType {
    Star,
    Dash,
    Number,
}

struct MarkdownList {
    text: Vec<String>,
}

enum MsDocMarkdown {
    Comment(String),
    Heading(MarkdownHeading),
    //Code(MarkdownCode),
    Image(MsDocMarkdownImage),
}

//TODO frontmatter

fn main() {
    // each group is a match type and they in the end are all put into a vector joined by | and then compiled into a regex

    let comment = r##"(?:^<!-{2,3}(?P<comment>[\s\S]*?)-{2,3}>.$)"##;

    let heading = r##"(?P<heading>(?:^#{1,6}\s)(?:.*?)\s$)"##;

    let code_block = r##"(?P<code_block>(?:^(?:[\s]*?[^!"#$%&'()*+,\-.>/:;<=?@\[\]^_{}~]```)(?:[\S\s]*?)([^>]```).$)|(?:^(?:[\s]*?[^!"#$%&'()*+,\-./:;<=?@\[\]^_{}~]> ```)(?:[\S\s]*?)(?:> ```)))"##;

    //?:[\s]*?[^!"#$%&'()*+,\-./:;<=>?@\[\]^_{}~]

    //:::code language="json" source="../../VanillaBehaviorPack/entities/creeper.json":::

    let code_ref = r##"(?P<code_ref>(?:^:::code[\S\s]*?:::.$))"##;

    let image = r##"(?P<image>(?:(?:^:::image(?:.*?):::.$)|(^:::image(?:.*?):::.$[\s\S]*?^:::image-end:::.$)))"##;

    let md_image = r##"(?P<md_image>(?:^!\[.*?\]\(.*?\)))"##;

    // > [!div class="nextstepaction"] - regexp based on this
    let div = r##"(?:^> \[!div (?P<div>.*?)\].$)"##;

    // > - [(Xamarin Android | Javascript)](how-to-write-links.md)
    let selector = r##"(?:^[\s]*?> - (?P<selector>\[.*?\]\(.*?\)).$)"##;

    let checklist = r##"(?:^[\s]*?> [*-] (?P<checklist>.*))"##;
    // when sifting through with this type make sure it is a table again if not send it down the line to other if its a text block
    let table = r##"(?P<table>^[\s]*[>]?[^!"#$%&'()*+,\-./:;<=?@\[\]^_`{}~](?:(?:\r?\n){2}|^[^!"#$%&'()*+,\-./:;<=?@\[\]^_`{}~]{4})(?:[^\r\n]*\|[^\r\n]*(?:\r?\n)?)+(?:(?:\r?\n){2}|$))"##;

    let alert = r##"(?:^[ ]*?^>\s+\[!(?P<alert>NOTE|TIP|IMPORTANT|CAUTION|WARNING)\].*?.$)"##;

    let list = r##"(?P<list>^[ ]*?(?:(?:\d+?\. )|(?:[-*] ))(?:.*?)$)"##;

    let link = r##"(?P<link>^\[.+?\]\(?:.+?\)\n.$)"##;

    let row = r##"(?P<row>^:::row(?:[\s\S]*?)row-end:::.$)"##;

    let column = r##"(?P<column>^:::column(?:[\S\s]*?)column-end:::.$)"##;

    let extension = r##"(?P<extension>^[ ]?[\[!.*?].$)"##;

    let quote = r##"(?P<quote>^[ ]*?> (?:.*?).$)"##;

    let other = r##"(?P<other>^.*?.$)"##;

    /*
    > [Scoreboard Operations Tutorial](ScoreboardOperationsTutorial.md)
    > [Complete the Monument](CommandsHowToMakeACTMWorld.md)
     */

    let next_step_action = r##"(?P<next_step_action>^> \[.*\]\(.*?\).$)"##;

    /*
       comment, heading, code, image, checklist, selector, table, alert, list, link, row, column,
       extension, quote, other,

    */

    let regex = vec![
        comment,
        heading,
        code_block,
        code_ref,
        image,
        md_image,
        div,
        selector,
        checklist,
        next_step_action,
        table,
        alert,
        list,
        link,
        row,
        column,
        extension,
        quote,
        other,
    ];

    // try to compile each individual regex to validate the syntax

    for r in regex.iter() {
        let _ = Regex::new(r).unwrap();
    }

    print!("{}", &format!("(?m){}", regex.join("|")));

    let regex = Regex::new(&format!("(?m){}", regex.join("|"))).unwrap();

    let text = fs::read_to_string("test.md").unwrap();

    let mut comments = Vec::new();
    let mut headings = Vec::new();
    let mut codes = Vec::new();
    let mut images = Vec::new();
    let mut checklists = Vec::new();
    let mut selectors = Vec::new();
    let mut tables = Vec::new();
    let mut alerts = Vec::new();
    let mut lists = Vec::new();
    let mut links = Vec::new();
    let mut rows = Vec::new();
    let mut columns = Vec::new();
    let mut extensions = Vec::new();
    let mut quotes = Vec::new();
    let mut others = Vec::new();
    let mut divs = Vec::new();
    let mut md_images = Vec::new();
    let mut code_refs = Vec::new();
    let mut next_step_actions = Vec::new();

    for cap in regex.captures_iter(&text) {
        if let Some(comment) = cap.name("comment") {
            comments.push(comment.as_str());
        }
        if let Some(heading) = cap.name("heading") {
            headings.push(heading.as_str());
        }
        if let Some(code) = cap.name("code_block") {
            codes.push(code.as_str());
        }
        if let Some(image) = cap.name("image") {
            images.push(image.as_str());
        }
        if let Some(checklist) = cap.name("checklist") {
            checklists.push(checklist.as_str());
        }
        if let Some(selector) = cap.name("selector") {
            selectors.push(selector.as_str());
        }
        if let Some(table) = cap.name("table") {
            tables.push(table.as_str());
        }
        if let Some(alert) = cap.name("alert") {
            alerts.push(alert.as_str());
        }
        if let Some(list) = cap.name("list") {
            lists.push(list.as_str());
        }
        if let Some(link) = cap.name("link") {
            links.push(link.as_str());
        }
        if let Some(row) = cap.name("row") {
            rows.push(row.as_str());
        }
        if let Some(column) = cap.name("column") {
            columns.push(column.as_str());
        }
        if let Some(extension) = cap.name("extension") {
            extensions.push(extension.as_str());
        }
        if let Some(quote) = cap.name("quote") {
            quotes.push(quote.as_str());
        }
        if let Some(other) = cap.name("other") {
            others.push(other.as_str());
        }
        // div

        if let Some(div) = cap.name("div") {
            divs.push(div.as_str());
        }

        if let Some(md_image) = cap.name("md_image") {
            md_images.push(md_image.as_str());
        }

        if let Some(code_ref) = cap.name("code_ref") {
            code_refs.push(code_ref.as_str());
        }

        if let Some(next_step_action) = cap.name("next_step_action") {
            next_step_actions.push(next_step_action.as_str());
        }
    }

    // each match type gets its own vector of matches and a file is written for each match type at the end in the dir called test and if a group matches it is put in the right vectpr

    fs::write("test/comments.md", comments.join("\n\n")).unwrap();
    fs::write("test/headings.md", headings.join("\n\n")).unwrap();
    fs::write("test/codes.md", codes.join("\n\n")).unwrap();
    fs::write("test/images.md", images.join("\n\n")).unwrap();
    fs::write("test/checklists.md", checklists.join("\n\n")).unwrap();
    fs::write("test/selectors.md", selectors.join("\n\n")).unwrap();
    fs::write("test/tables.md", tables.join("\n\n")).unwrap();
    fs::write("test/alerts.md", alerts.join("\n\n")).unwrap();
    fs::write("test/lists.md", lists.join("\n\n")).unwrap();
    fs::write("test/links.md", links.join("\n\n")).unwrap();
    fs::write("test/rows.md", rows.join("\n\n")).unwrap();
    fs::write("test/columns.md", columns.join("\n\n")).unwrap();
    fs::write("test/extensions.md", extensions.join("\n\n")).unwrap();
    fs::write("test/quotes.md", quotes.join("\n\n")).unwrap();
    fs::write("test/others.md", others.join("\n\n")).unwrap();
    fs::write("test/divs.md", divs.join("\n\n")).unwrap();
    fs::write("test/md_images.md", md_images.join("\n\n")).unwrap();
    fs::write("test/code_refs.md", code_refs.join("\n\n")).unwrap();
    fs::write("test/next_step_actions.md", next_step_actions.join("\n\n")).unwrap();
}
