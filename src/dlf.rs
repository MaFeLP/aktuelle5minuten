use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

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
            .expect(&format!(
                "No matching element found for selector '{}'",
                $selector
            ))
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

fn parse_partial_article(json: &str) -> Result<Option<PartialArticle>, &'static str> {
    let json: serde_json::Value = serde_json::from_str(json).unwrap();
    let value = json.get("value").unwrap();
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
                None =>
                    debug!(
                        "Found data element that does not fit scheme! See debug output for more information: {:?}",
                        &json
                    )
            }
            Ok(None)
        }
        Some(typename) => {
            let typename = typename.as_str().unwrap();
            if typename != "Teaser" {
                debug!("Not a teaser article");
                return Ok(None);
            }
            let path = value.get("path").unwrap().as_str().unwrap();

            for url in NON_ARTICLE_URLS.iter() {
                if path.starts_with(&format!("{}{}", PREFIX, url)) {
                    debug!("Not a news article!");
                    return Ok(None);
                }
            }

            // Add key from main json node to the value node
            let mut value = value.clone();
            value["key"] = path.strip_prefix(PREFIX).unwrap().into();
            let parsed: PartialArticle =
                serde_json::from_value(value.clone()).expect("Parsing the article went wrong!");
            Ok(Some(parsed))
        }
    }
}

pub async fn wochenrueckblick() -> Result<Vec<PartialArticle>, &'static str> {
    let body = reqwest::get(WOCKENRUECKBLICK_URL)
        .await
        .map_err(|_| "Could not retrieve the webpage")?
        .text()
        .await
        .map_err(|_| "Could not retrieve the webpage's text")?;

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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub struct Figure {
    pub src: String,
    pub alt: String,
    pub srcset: String,
    pub title: String,
    pub caption: String,
}

pub async fn article(href: &str) -> Result<Article, &'static str> {
    if !href.starts_with(PREFIX) {
        return Err("Is not a DLF Article");
    }

    let response = reqwest::get(href).await.unwrap();
    if response.status() != 200 {
        return Err("Could not download article!");
    }

    let html = response.text().await.unwrap();
    let document = Html::parse_document(&html);

    // Parse head
    let head = select_one!(document, "head");
    let mut metadata: Option<PartialArticle> = None;
    for script in head.select(&Selector::parse("script.js-client-queries").unwrap()) {
        metadata = parse_partial_article(script.attr("data-json").unwrap()).unwrap();
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

    Ok(Article {
        kicker: String::from(
            select_one!(header, "span.headline-kicker")
                .text()
                .collect::<String>()
                .trim(),
        ),
        title: String::from(
            select_one!(header, "span.headline-title")
                .text()
                .collect::<String>()
                .trim(),
        ),
        description: String::from(
            select_one!(header, "p.article-header-description")
                .text()
                .collect::<String>()
                .trim(),
        ),
        html: section.inner_html(),
        plaintext: section.text().collect(),
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
