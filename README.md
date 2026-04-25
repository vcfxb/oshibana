# oshibana, a magic the gathering collection tracker for the modern era.

## About

## April/May 2026 rewrite
This project is actively being rewritten in rust as a desktop native application.

It's core goals are as follows:
- Track which cards are allocated/assigned to which decks you own.
- Rich deck history -- revisions, versions, etc.
- Scryfall liveness independence -- Scryfall is the best source for card data, but that doesn't mean we have to depend 
on their website & search engine being up for ours to work.
    - To the best of our ability, we will pull scryfall data once a day, and self-host our own archive of it. In doing 
  so we also will reimplement as much of their search engine functionality as is reasonable/necessary.

Explicit non-goals of the project:
- Per-card price history -- check tcgplayer if you want this, or another tool. It's out of scope.
- Game simulation -- there are other tools you can use to play with the decks you build here.
