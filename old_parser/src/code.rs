use serde::{Deserialize, Serialize};

use crate::Indent;

#[derive(Debug, Clone, PartialEq)]
pub enum Code {

    Block {
        content: String,
        /// If not language is specified, this is set to Language::None

        language: Language,
        indent: u8,
    },
    Reference {
        content: String,
        source: String,
        /// If not language is specified, this is set to Language::None
        language: Language,

    //TODO finish this based on ms md reference
    //:::code language="csharp" source="intro/samples/cu/Controllers/StudentsController.cs" range="2-24,26":::
    //This example displays only lines 2-24 and 26 of the *StudentController.cs* code file.

        range: Option<[u32; 2]>,
        id: Option<String>,
        highlight: Option<String>,
        interactive: Option<Interactive>,
        indent: u8,
    },

}


impl Code {


    pub fn new_block(content: String, language: Language, indent: u8) -> Self {
        Code::Block {
            content,
            language,
            indent,
        }
    }

    pub fn new_reference(
        content: String,
        source: String,
        language: Language,
        indent: u8,
    ) -> Self {
        Code::Reference {
            content,
            source,
            language,
            range: None,
            id: None,
            highlight: None,
            interactive: None,
            indent,
        }
    }


    pub fn get_code(&self) -> &str {
        match self {
            Code::Block { content, .. } => content,
            Code::Reference { content, .. } => content,
        }
    }

    pub fn get_language(&self) -> &Language {
        match self {
            Code::Block { language, .. } => language,
            Code::Reference { language, .. } => language,
        }
    }

    pub fn get_source(&self) -> Option<&str> {
        match self {
            Code::Block { .. } => None,
            Code::Reference { source, .. } => Some(source),
        }
    }

    pub fn get_range(&self) -> Option<&[u32; 2]> {
        match self {
            Code::Block { .. } => None,
            Code::Reference { range, .. } => range.as_ref(),
        }
    }

    pub fn get_id(&self) -> Option<&str> {
        match self {
            Code::Block { .. } => None,
            Code::Reference { id, .. } => id.as_deref(),
        }
    }

    pub fn get_highlight(&self) -> Option<&str> {
        match self {
            Code::Block { .. } => None,
            Code::Reference { highlight, .. } => highlight.as_deref(),
        }
    }

    pub fn get_interactive(&self) -> Option<&Interactive> {
        match self {
            Code::Block { .. } => None,
            Code::Reference { interactive, .. } => interactive.as_ref(),
        }
    }

    pub fn set_range(&mut self, range: Option<[u32; 2]>) {
        match self {
            Code::Block { .. } => {}
            Code::Reference { range: r, .. } => *r = range,
        }
    }

    pub fn set_id(&mut self, id: Option<String>) {
        match self {
            Code::Block { .. } => {}
            Code::Reference { id: i, .. } => *i = id,
        }
    }

    pub fn set_highlight(&mut self, highlight: Option<String>) {
        match self {
            Code::Block { .. } => {}
            Code::Reference { highlight: h, .. } => *h = highlight,
        }
    }

    pub fn set_interactive(&mut self, interactive: Option<Interactive>) {
        match self {
            Code::Block { .. } => {}
            Code::Reference { interactive: i, .. } => *i = interactive,
        }
    }

    pub fn set_source(&mut self, source: String) {
        match self {
            Code::Block { .. } => {}
            Code::Reference { source: s, .. } => *s = source,
        }
    }

    pub fn set_language(&mut self, language: Language) {
        match self {
            Code::Block { language: l, .. } => *l = language,
            Code::Reference { language: l, .. } => *l = language,
        }
    }

    pub fn set_content(&mut self, content: String) {
        match self {
            Code::Block { content: c, .. } => *c = content,
            Code::Reference { content: c, .. } => *c = content,
        }
    }

    pub fn set_indent(&mut self, indent: u8) {
        match self {
            Code::Block { indent: i, .. } => *i = indent,
            Code::Reference { indent: i, .. } => *i = indent,
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            Code::Block { .. } => true,
            Code::Reference { .. } => false,
        }
    }

    pub fn is_reference(&self) -> bool {
        match self {
            Code::Block { .. } => false,
            Code::Reference { .. } => true,
        }
    }



}


impl Indent for Code {
    fn get_indent(&self) -> u8 {
        match self {
            Code::Block { indent, .. } => *indent,
            Code::Reference { indent, .. } => *indent,
        }
    }
}








//TODO finish this based on ms md reference
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Bat,
    Cmd,
    PowerShell,
    Config,
    CSharp,
    CMake,
    JavaScript,
    Java,
    TypeScript,
    JSON,
    YAML,
    CPP,
    Http,
    Html,
    XML,
    Markdown,
    None,
    Other(String),
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "bat" | "batch" => Language::Bat,
            "csharp" | "cs" | "c#" => Language::CSharp,
            "javascript" | "js" | "jsx" => Language::JavaScript,
            "typescript" | "ts" | "tsx" => Language::TypeScript,
            "json" => Language::JSON,
            "yaml" | "yml" => Language::YAML,
            "cpp" | "c++" => Language::CPP,
            "http" | "https" | "uri" => Language::Http,
            "xml" | "xhtml" | "rss" | "atom" | "xjb" | "xsd" | "xsl" | "plist" => Language::XML,
            "cmd" => Language::Cmd,
            "config" => Language::Config,
            "java" | "jsp" => Language::Java,
            "html" /*| "xhtml"*/ => Language::Html,
            "markdown" | "md" | "mkdown" | "mkd" => Language::Markdown,
            "cmake" | "cmake.in" => Language::CMake,
            "powershell" | "ps" => Language::PowerShell,
            _ => {
                if s.is_empty() {
                    return Language::None;
                }

                println!("Unknown language: {}", s);
                Language::Other(s.to_string())
            }
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Bat => "bat".to_string(),
            Language::Cmd => "cmd".to_string(),
            Language::Config => "config".to_string(),
            Language::CSharp => "csharp".to_string(),
            Language::JavaScript => "javascript".to_string(),
            Language::Java => "java".to_string(),
            Language::TypeScript => "typescript".to_string(),
            Language::JSON => "json".to_string(),
            Language::YAML => "yaml".to_string(),
            Language::CPP => "cpp".to_string(),
            Language::Http => "http".to_string(),
            Language::XML => "xml".to_string(),
            Language::Other(s) => s.to_string(),
            Language::Html => "html".to_string(),
            Language::Markdown => "markdown".to_string(),
            Language::CMake => "cmake".to_string(),
            Language::PowerShell => "powershell".to_string(),
            Language::None => "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Interactive {
    #[serde(rename = "cloudshell-powershell")]
    CloudShellPowerShell,
    #[serde(rename = "cloudshell-bash")]
    CloudShellBash,
    #[serde(rename = "try-dotnet")]
    TryDotNet,
    #[serde(rename = "try-dotnet-class")]
    TryDotNetClass,
    #[serde(rename = "try-dotnet-method")]
    TryDotNetMethod,
}

impl From<String> for Interactive {
    fn from(s: String) -> Self {
        match s.as_str() {
            "cloudshell-powershell" => Interactive::CloudShellPowerShell,
            "cloudshell-bash" => Interactive::CloudShellBash,
            "try-dotnet" => Interactive::TryDotNet,
            "try-dotnet-class" => Interactive::TryDotNetClass,
            "try-dotnet-method" => Interactive::TryDotNetMethod,
            _ => panic!("Unknown interactive type: {}", s),
        }
    }
}
