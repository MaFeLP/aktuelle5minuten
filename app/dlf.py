#!/usr/bin/env python3

import requests
import json
import logging
from bs4 import BeautifulSoup, Tag
from sys import exit
from flask import abort

PREFIX = "https://www.deutschlandfunk.de/"

_WOCHENRUECKBLICK_URL = PREFIX + "nachrichten/wochenueberblick"

_LOGGER = logging.getLogger(__name__)

_NON_ARTICLES_URLS = [
    PREFIX + "nachrichten/nachlesen",
    PREFIX + "suche",
    PREFIX + "nachrichten/barrierefrei",
]


def download_wochenrueckblick() -> str:
    r = requests.get(_WOCHENRUECKBLICK_URL)
    if r.status_code != 200:
        print("Could not download the Wochenrückblick from Deutschlandfunk!")
        exit(1)
    return r.text


def _parse_partial_article(script: Tag) -> dict | None:
    j = json.loads(script["data-json"])["value"]

    if "__typename" not in j:
        if "data" in j and "newsByWeek" in j["data"]:
            # One extra element that is not an article
            return None
        _LOGGER.warning(
            "Found data element that does not fit scheme! See debug output for more information"
        )
        _LOGGER.debug(json.dumps(j))
        return None

    if j["__typename"] == "Teaser" and j["path"] not in _NON_ARTICLES_URLS:
        _LOGGER.info(f"  -> Found Article '{j['title']}'")
        if not j["path"].startswith(PREFIX):
            return None
        return {
            "key": j["path"][len(PREFIX) :],
            "title": j["title"],
            "teaserHeadline": j["teaserHeadline"],
            "teaserText": j["teasertext"],
            "date": j["firstPublicationDate"],
            "localeDate": j["dateLocalizedFormatted"],
        }
    else:
        _LOGGER.debug(f"Not a Teaser/News Article: {j}")
    return None


def parse_wochenrueckblick(wr: str) -> list[dict[str, str]]:
    soup = BeautifulSoup(wr, "lxml")
    main = soup.main

    if main is None:
        raise ValueError("<main> not found in the html test!")

    articles = []
    _LOGGER.info("Parsing Wochenrückblick...")
    for script in main.find_all("script"):
        article = _parse_partial_article(script)
        if article is not None:
            articles.append(article)
    return articles


def download_article(href: str) -> str:
    assert href is not None
    assert href.startswith("https://www.deutschlandfunk.de/")
    r = requests.get(href)
    if r.status_code != 200:
        raise ConnectionError(f"Could not download the article: {r.status_code}")
    return r.text


def parse_article(html: str) -> dict[str, str]:
    soup: Tag = BeautifulSoup(html, "lxml")

    # Parse Head for Metadata
    head: Tag = soup.head
    metadata: None | dict = None
    for script in head.find_all("script", {"class": "js-client-queries"}):
        metadata = _parse_partial_article(script)
        if metadata is not None:
            break

    # Parse content
    article: Tag = soup.find("article", {"class": "b-article"})
    header: Tag = article.header
    section: Tag = article.section
    assert section is not None

    figures: list[dict[str, str]] = []
    for figure in section.find_all("figure"):
        img: Tag = figure.img
        figures.append(
            {
                "image": {
                    "src": img["src"],
                    "alt": img["alt"],
                    "srcset": img["srcset"],
                    "title": img["title"],
                },
                "caption": (
                    figure.figcaption.text
                    if figure.figcaption is not None
                    else img["alt"]
                ),
            }
        )
        figure.decompose()
    for script in section.find_all("script"):
        script.decompose()

    return {
        "kicker": header.find("span", {"class": "headline-kicker"}).text,
        "title": header.find("span", {"class": "headline-title"}).text,
        "description": header.find("p", {"class": "article-header-description"}).text,
        "author": header.find("div", {"class": "article-header-author"}).span.text,
        "content": {
            "html": section.decode_contents().strip(),
            "plaintext": section.text.strip(),
        },
        "figures": figures,
        "key": metadata["key"],
        "date": metadata["date"],
        "localeDate": metadata["localeDate"],
    }
