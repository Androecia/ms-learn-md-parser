use crate::{Content, Indent, MsMarkdown};

#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
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
