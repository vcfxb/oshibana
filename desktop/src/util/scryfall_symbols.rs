//! Utility to render symbols in place in text.

use regex::Regex;
use std::sync::LazyLock;
use storage::scryfall::ScryfallStorage;
use url::Url;

static SYMBOL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\{[A-Z0-9/]+}"#).unwrap());

#[derive(Debug)]
pub enum ResolvedPart<'a> {
    RenderSymbolUri(Url),
    Text(&'a str),
}

pub fn render<'a>(
    scryfall_store: &ScryfallStorage,
    text: &'a str,
) -> impl Iterator<Item = ResolvedPart<'a>> {
    let mut last_end = 0;
    let mut regex_matches = SYMBOL_REGEX.find_iter(text);
    let mut pending = None;

    let iter = std::iter::from_fn(move || {
        if pending.is_some() {
            return pending.take();
        }

        let Some(next_match) = regex_matches.next() else {
            if last_end < text.len() {
                let part = ResolvedPart::Text(&text[last_end..]);
                last_end = text.len();
                return Some(part);
            } else {
                return None;
            }
        };

        let match_str = next_match.as_str();
        pending = Some(
            scryfall_store
                .get_symbol_svg_uri(match_str)
                .map(ResolvedPart::RenderSymbolUri)
                .unwrap_or(ResolvedPart::Text(match_str)),
        );

        let this_part = ResolvedPart::Text(&text[last_end..next_match.start()]);
        last_end = next_match.end();
        Some(this_part)
    });

    iter.filter(|part| match part {
        ResolvedPart::RenderSymbolUri(_) => true,
        ResolvedPart::Text(s) => !s.is_empty(),
    })
}
