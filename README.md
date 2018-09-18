I have an app which needs to have an embedded english (mand more languages should be added later) dictionary. Currently, I'm using postgres to store the words. `pg_trgm` is used to enable "fuzzy" search.

This repo looks into more efficient ways of having an indexed dictionary inside the app:

- [FSTs](https://en.wikipedia.org/wiki/Finite-state_transducer) via [fst](https://github.com/BurntSushi/fst)
- [tantivy](https://github.com/tantivy-search/tantivy) using trigram tokenizers.
- sqlite with [trilite](https://github.com/jonasfj/trilite)
- something else?

The current approach (postgres + `pg_trgm`) is the control.
