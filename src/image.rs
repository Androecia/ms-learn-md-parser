use url;
use super::MsMarkdown;


#[derive(Debug, Clone, PartialEq)]
pub enum Image {
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
