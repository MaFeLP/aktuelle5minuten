use crate::regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const PREFIX: &str = "https://www.deutschlandfunk.de/";
const WOCKENRUECKBLICK_URL: &str = "https://www.deutschlandfunk.de/nachrichten/wochenueberblick";
const NON_ARTICLE_URLS: [&str; 3] = ["nachrichten/nachlesen", "suche", "nachrichten/barrierefrei"];

macro_rules! select_one {
    ($parent:expr, $selector:expr) => {
        $parent
            .select(
                &::scraper::Selector::parse($selector)
                    .expect(&format!("Failed to parse the selector '{}'", $selector)),
            )
            .next()
            .ok_or(ParseError::MissingKey($selector))?
    };
}

macro_rules! select_one_text {
    ($parent:expr, $selector:expr, $default:expr) => {
        match $parent
            .select(
                &::scraper::Selector::parse($selector)
                    .expect(&format!("Failed to parse the selector '{}'", $selector)),
            )
            .next()
        {
            Some(element) => element.text().collect::<String>().trim().to_string(),
            None => $default.to_string(),
        }
    };
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PartialArticle {
    pub key: String,
    pub title: String,
    #[serde(rename = "teaserHeadline")]
    pub teaser_headline: Option<String>,
    #[serde(rename = "teasertext")]
    pub teaser_text: Option<String>,
    #[serde(rename = "firstPublicationDate")]
    pub date: String,
    #[serde(rename = "dateLocalizedFormatted")]
    pub locale_date: String,
}

#[derive(Error, Debug)]
pub enum ParseError {
    /// Error parsing JSON
    #[error("Error parsing JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// The JSON type is not the expected one
    #[error("Expected JSON type {0}")]
    WrongJsonType(&'static str),
    /// The JSON is missing a key
    #[error("Missing key: {0}")]
    MissingKey(&'static str),
    /// The path in the URL does not have a prefix
    #[error("Missing prefix")]
    MissingPrefix,
}

fn parse_partial_article(json: &str) -> Result<Option<PartialArticle>, ParseError> {
    let json: serde_json::Value = serde_json::from_str(json)?;
    let value = json.get("value").ok_or(ParseError::MissingKey("value"))?;
    match value.get("__typename") {
        None => {
            match json.get("data") {
                Some(data) => {
                    if data.get("newsByWeek").is_some() {
                        return Ok(None);
                    }
                    debug!(
                        "Found data element that does not fit scheme! See debug output for more information: {:?}",
                        &json
                    );
                }
                None => debug!(
                    "Found data element that does not fit scheme! See debug output for more information: {:?}",
                    &json
                ),
            }
            Ok(None)
        }
        Some(typename) => {
            let typename = typename
                .as_str()
                .ok_or(ParseError::WrongJsonType("__typename"))?;
            if typename != "Teaser" {
                debug!("Not a teaser article");
                return Ok(None);
            }
            let path = value
                .get("path")
                .ok_or(ParseError::MissingKey("value.path"))?
                .as_str()
                .unwrap();

            for url in NON_ARTICLE_URLS.iter() {
                if path.starts_with(&format!("{}{}", PREFIX, url)) {
                    debug!("Not a news article!");
                    return Ok(None);
                }
            }

            // Add key from main json node to the value node
            let mut value = value.clone();
            value["key"] = path
                .strip_prefix(PREFIX)
                .ok_or(ParseError::MissingPrefix)?
                .into();
            let parsed: PartialArticle = serde_json::from_value(value.clone())?;
            Ok(Some(parsed))
        }
    }
}

#[derive(Error, Debug)]
pub enum DlfError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("Response error: {0}")]
    Response(&'static str),
    #[error("Prefix error")]
    Prefix,
}

pub async fn wochenrueckblick() -> Result<Vec<PartialArticle>, DlfError> {
    let body = reqwest::get(WOCKENRUECKBLICK_URL).await?.text().await?;

    let document = Html::parse_document(&body);
    let script_selector = Selector::parse("script").unwrap();

    let mut articles: Vec<PartialArticle> = vec![];
    let main = select_one!(document, "main");
    for script in main.select(&script_selector) {
        let parsed_article = parse_partial_article(script.attr("data-json").unwrap())?;
        if let Some(article) = parsed_article {
            articles.push(article);
        }
    }

    Ok(articles)
}

#[derive(Debug)]
pub struct Article {
    pub kicker: String,
    pub title: String,
    pub description: String,
    pub html: String,
    pub plaintext: String,
    pub figures: Vec<Figure>,
    pub key: String,
    pub date: String,
    pub locale_date: String,
}

#[derive(Debug)]
pub struct Figure {
    pub src: String,
    pub alt: String,
    pub srcset: String,
    pub title: String,
    pub caption: String,
}

pub async fn article(href: &str) -> Result<Article, DlfError> {
    if !href.starts_with(PREFIX) {
        return Err(DlfError::Prefix);
    }

    let response = reqwest::get(href).await?;
    if response.status() != 200 {
        return Err(DlfError::Response("Could not download article!"));
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Parse head
    let head = select_one!(document, "head");
    let mut metadata: Option<PartialArticle> = None;
    for script in head.select(&Selector::parse("script.js-client-queries").unwrap()) {
        metadata = parse_partial_article(
            script
                .attr("data-json")
                .ok_or(ParseError::MissingKey("data-json"))?,
        )?;
        if metadata.is_some() {
            break;
        }
    }
    let metadata = metadata.unwrap();

    let article = select_one!(document, "article.b-article");
    let header = select_one!(article, "header");
    let section = select_one!(article, "section");

    let mut figures: Vec<Figure> = vec![];
    for figure in section.select(&Selector::parse("figure").unwrap()) {
        let image = select_one!(figure, "img");
        figures.push(Figure {
            src: String::from(image.attr("src").unwrap()),
            alt: String::from(image.attr("alt").unwrap()),
            srcset: String::from(image.attr("srcset").unwrap()),
            title: String::from(image.attr("title").unwrap()),
            caption: {
                if let Some(figcaption) = figure
                    .select(&Selector::parse("figcaption").unwrap())
                    .next()
                {
                    figcaption.text().collect()
                } else {
                    String::from(image.attr("alt").unwrap())
                }
            },
        });
    }

    // Remove figure and script tags from the article, as well as whitespace
    let content_html = Html::parse_fragment(
        regex!("(<figure.*?</figure>|<script.*?</script>|\\s{2,})")
            .replace_all(&section.inner_html().replace("\n", ""), " ")
            .to_string()
            .trim(),
    );

    Ok(Article {
        kicker: select_one_text!(header, "span.headline-kicker", ""),
        title: select_one_text!(header, "span.headline-title", ""),
        description: select_one_text!(header, "p.article-header-description", ""),
        html: content_html.root_element().inner_html(),
        plaintext: content_html.root_element().text().collect(),
        figures,
        key: metadata.key,
        date: metadata.date,
        locale_date: metadata.locale_date,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn download_and_parse_wochenrueckblick() {
        wochenrueckblick().await.unwrap();
    }

    #[tokio::test]
    async fn download_and_parse_article() {
        article("https://www.deutschlandfunk.de/bundesregierung-will-asylrecht-weiter-verschaerfen-100.html").await.unwrap();
    }
}
