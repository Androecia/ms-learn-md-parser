use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    /// This is read from the file referenced in the source attribute when parsing the :::code syntax
    pub content: String,
    /// If not language is specified, this is set to Language::None
    pub language: Language,

    // Always available for :::code syntax
    pub source: Option<String>,

    //TODO finish this based on ms md reference
    //:::code language="csharp" source="intro/samples/cu/Controllers/StudentsController.cs" range="2-24,26":::
    //This example displays only lines 2-24 and 26 of the *StudentController.cs* code file.
    pub range: Option<[u32; 2]>,

    pub id: Option<String>,

    pub highlight: Option<String>,

    pub interactive: Option<Interactive>,

    pub indent: u8,
}

impl Code {
    pub fn get_source(&self) -> Option<&str> {
        self.source.as_deref()
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
            "html" | "xhtml" => Language::Html,
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
