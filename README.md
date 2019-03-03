# license-store

`license-store` is a library to detect license text within a supplied document.  It is a fork of `[askalono](https://github.com/amzn/askalono)`.

Some goals of `license-store` compared to `askalono`:

* Allow tagging licenses in the store with arbitrary (textual) metadata
* Integrate into a larger ecosystem for comprehensive auditing, review, and approval

## Notice

This tool does not provide legal advice and it is not a lawyer. It endeavors to match your input to a database of similar license texts, and tell you what it thinks is a close match. But, it can't tell you that the given license is authoritative over a project. Nor can it tell you what to do with a license once it's identified. You are not entitled to rely on the accuracy of the output of this tool, and should seek independent legal advice for any licensing questions that may arise from using this tool.

## Usage

At the moment, `Store` and `LicenseContent` are exposed for usage.

The best way to get an idea of how to use `license-store` in its early state is to look at the [example](./examples/basic.rs). Some examples are also available in the [documentation](https://docs.rs/license-store).

## Details

### Implementation

Quoting from the [`askalono` README](https://github.com/amzn/askalono/blob/0.3.0/README.md):

> **tl;dr**: Sørensen–Dice scoring, multi-threading, compressed cache file
> 
> At its core, `license-store` builds up bigrams (word pairs) of input text, and compares that with other license texts it knows about to see how similar they are. It scores each match with a [Sørensen–Dice](https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient) coefficient and looks for the highest result. There is some minimal preprocessing happening before matching, but there are no hand-maintained regular expressions or curations used to determine a match.
> 
> In detail, the matching process:
> 
> 1. Reads in input text
> 1. Normalizes everything it reasonably can -- Unicode characters, whitespace, quoting styles, etc. are all whittled down to something common.
>     * Lines that tend to change a lot in licenses, like "Copyright 20XX Some Person", are additionally removed.
> 1. Tokenizes normalized text into a set of bigrams.
> 1. In parallel, the bigram set is compared with all of the other sets `license-store` knows about.
> 1. The resulting list is sorted, the top match identified, and result returned.
> 
> To optimize startup, `license-store` allows building up a database of license texts (applying the same normalization techniques described above), and persists this data to a MessagePack'd & gzip'd cache file. This cache can be loaded at startup, and optionally be embedded in the binary itself.

## License

This library is licensed under the [Apache 2.0 License](LICENSE).
