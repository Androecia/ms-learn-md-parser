use serde::{Deserialize, Serialize};

/// Struct representing the metadata for a document.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct MsMdMetadata {
    /// The page title. This is the page title that's displayed on the browser tab.
    /// It's the most important metadata for SEO.
    title: String,

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
    robots: Option<Robots>,

    /// A list of words in the article that should never be translated (localized).
    /// Use this metadata to prevent "overlocalization."
    #[serde(rename = "no-loc")]
    no_loc: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Robots {
    /// Causes the asset to not show up in search results.
    #[serde(rename = "NOINDEX")]
    NoIndex,
    /// Only when you archive an entire content set.
    #[serde(rename = "NOFOLLOW")]
    NoFollow,
}
