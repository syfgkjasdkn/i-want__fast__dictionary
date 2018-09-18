I have an app which needs to have an embedded english -- with more languages added later -- dictionary. Currently, I'm using postgres to store the words and their definitions. `pg_trgm` is used to enable "fuzzy" search over the words.

This repo looks into more efficient and straighforward ways of having an indexed dictionary inside the app:

- [FSTs](https://en.wikipedia.org/wiki/Finite-state_transducer) via [fst](https://github.com/BurntSushi/fst)
- [tantivy](https://github.com/tantivy-search/tantivy) using trigram tokenizers
- sqlite with [trilite](https://github.com/jonasfj/trilite)
- something else?

The current approach (postgres + `pg_trgm`) is the control.
