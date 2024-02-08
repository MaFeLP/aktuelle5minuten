#!/usr/bin/env python3

import requests
import json
import logging
from bs4 import BeautifulSoup, Tag
from sys import exit
from flask import abort

_WOCHENRUECKBLICK_URL = "https://www.deutschlandfunk.de/nachrichten/wochenueberblick"

_LOGGER = logging.getLogger(__name__)

_NON_ARTICLES_URLS = [
    "https://www.deutschlandfunk.de/nachrichten/nachlesen",
    "https://www.deutschlandfunk.de/suche",
    "https://www.deutschlandfunk.de/nachrichten/barrierefrei",
]


def download_wochenrueckblick() -> str:
    r = requests.get(_WOCHENRUECKBLICK_URL)
    if r.status_code != 200:
        print("Could not download the Wochenrückblick from Deutschlandfunk!")
        exit(1)
    return r.text


def parse_wochenrueckblick(wr: str) -> list[dict[str, str]]:
    soup = BeautifulSoup(wr, "lxml")
    main = soup.main

    if main is None:
        raise ValueError("<main> not found in the html test!")

    articles = []
    _LOGGER.info("Parsing Wochenrückblick...")
    for script in main.find_all("script"):
        j = json.loads(script["data-json"])
        key = j["key"]
        j = j["value"]

        if "__typename" not in j:
            if "data" in j and "newsByWeek" in j["data"]:
                # One extra element that is not an article
                continue
            _LOGGER.warning(
                "Found data element that does not fit scheme! See debug output for more information"
            )
            _LOGGER.debug(json.dumps(j))
            continue

        if j["__typename"] == "Teaser" and j["path"] not in _NON_ARTICLES_URLS:
            _LOGGER.info(f"  -> Found Article '{j['title']}'")
            articles.append(
                {
                    "key": key,
                    "title": j["title"],
                    "teaserHeadline": j["teaserHeadline"],
                    "teaserText": j["teasertext"],
                    "date": j["firstPublicationDate"],
                    "localeDate": j["dateLocalizedFormatted"],
                    "href": j["path"],
                }
            )
        else:
            _LOGGER.debug(f"Not a Teaser/News Article: {j}")
    return articles


def download_article(href: str) -> str:
    assert href is not None
    assert href.startswith("https://www.deutschlandfunk.de/")
    r = requests.get(href)
    if r.status_code != 200:
        abort(r.status_code)
    return r.text


def parse_article(html: str) -> dict[str, str]:
    soup: Tag = BeautifulSoup(html, "lxml")
    article: Tag = soup.find("article", {"class": "b-article"})
    header: Tag = article.header
    section: Tag = article.section
    assert section is not None

    figures: list[dict[str, str]] = []
    for figure in section.find_all("figure"):
        img: Tag = figure.img
        figures.append({"image": img.decode(), "caption": figure.figcaption.text})
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
    }
