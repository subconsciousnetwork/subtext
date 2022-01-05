- Feature: 2021-10-07-slashlinks
- RFC PR: https://github.com/gordonbrander/subtext/pull/18

# Summary

Rethink links in Subtext. Introduce a simpler syntax that works inline, block-level, and removes ambiguity from syntax.

# Motivation

In the design of Subtext, we have considered two kinds of links: block links, and wikilinks.

Block links:

```
& https://example.com Example page
& ./example file.txt Example page 
& Example page
```

Wikilinks:

```
[[https://example.com | Example page]]
[[./example file.txt | Example page]]
[[Example page | Example page synonym]]
```

Block links are simpler, and in keeping with Subtext's block-level design. Wikilinks are more common. But both have the same subtle issue: path ambiguity. Each of these markups supports 3 use-cases in one syntax:

- URL with optional label
- Path with optional label
- Label with optional alternate label (wikilink)

Because they support multiple cases with the same syntax, parsers must differentiate the cases in order to use the data they contain.

Paths may be disambiguated from labels by requiring a `/` or `./` at the beginning, and an extension at the end, and this is currently what Subtext does. It is tricky, but doable with regular expressions, or a parser.

However, URLs cannot be reasonably disambiguated from labels without either getting false-positives, or resorting to hard-coding protocols (`http://`). This is because [general URL syntax](https://datatracker.ietf.org/doc/html/rfc1738), beyond HTTP, is extremely open-ended, and more or less just defined by `<protocol>:<body>`.

Having to hardcode protocols as part of the language specification would be unfortunate. It prevents Subtext from being able to reference new and experimental protocols, such as p2p protocols like `ipfs:` and `dat:`.

Other motivations that inspired this rethink:

- Block links are unfamiliar, and not necessarily the easiest to visually scan.
- Wikilinks are visually noisy, albeit common.

Is there no simpler way forward?

# Proposed solution

A quick preview:

```
Bare http links https://example.com will be automatically linked.

You can also wrap links in angle brackets <https://example.com>.

This is useful for exotic protocols like <ipfs://asdfasdfasdfasdf>.

This is a /slashlink, it is a shortcut for linking to an /internal-page.
```

This proposal does a radical rethink of Subtext link syntax, proposing to replace the current block links with something more limited, but also more obvious, easier to parse, and unambiguous.

2. URLs `https://example.com` can be embedded anywhere in text, list, and quote blocks. Parsers MUST autolink bare `http` and `https` links.
1. Links can also be wrapped in angle brackets `<https://example.com>` and embedded anywhere in text, list, and quote blocks. This allows you to write non-http URLs unambiguously, such as `<doi:10.1000/182>`.
3. Internal pages can be linked to via a shortcut syntax, `/slashlink`, that is, a slash, followed by the path you wish to link to.

# Guide-level explanation

Subtext will detect any http URLs, and automatically link them.

```
Links like https://example.com will be automatically linked.
```

Note you must include the protocol (the `http://` bit) for Subtext to sniff out the URL.

Subtext parsers can automatically detect `http` and `https` links this way, but not every protocol will work. For exotic protocols wrap the URL in `<` `>` angle brackets.

```
Link unusual URLs like <doi:10.1000/182> by wrapping them in angle brackets. 

This works for any kind of URL <https://example.com>.
```

Subtext also has a shortcut for linking to local pages, called "slashlinks". Slashlinks are like `#hashtags`, or `@mentions` except instead of starting with a `#` or an `@`, they start with a `/`, like this: `/link`

Here are some examples of slashlinks:

```
This is a /slashlink. A few more:

/evolution
/requisite-variety
/DesignPatterns
```

Slashlinks can't include spaces. It is considered good practice to use dashes `-` instead. Also, slashlinks are case-insensitive. That is, `/Evolution` and `/evolution` reference the same thing. Feel free to write whatever looks better to you.

```
/whole-earth-catalog, a collection of /DIY books and tools.
```

Slashlinks can also include hierarchical sub-parts, just like a URL. This can be useful when two concepts from different fields share the same name.

```
/journal/2021-10-09
/vaclav-smil/energy-and-civilization
/climate/carbon-sinks
```

# Reference-level explanation

## URLs

URLs are wrapped in angle brackets, and can appear anywhere within a text, link, or quote block:

```
Links are wrapped in angle brackets, like this <https://example.com>, and can appear anywhere in text.

You can also reference links with exotic protocols like <doi:10.1000/182>.
```

### Parsing

Grammar:

```abnf
link = WB "<" url ">" WB
WB = SP / NL / BOF / EOF
SP = "\s" / "\t"
NL = CRLF / LF / CR
```

Where:

- `url` is conceptually a URL as defined by [RFC1738 Uniform Resource Locators (URL)](https://datatracker.ietf.org/doc/html/rfc1738), However implementations MAY use a simplified parsing strategy for URLs, described below.
- `EOF` is a conceptual code point that signifies the end of a string, or input stream.
- `BOF` is a conceptual code point that signifies the beginning of a string, or input stream.

A simplified parsing strategy MAY be used for parsing URLs. Implementations that use a simplified parsing strategy to identify bare URLs SHOULD use the following strategy, described here as a regular expression:

```regex
(^|\s)<([^<>\s]+)>($|\s)
```

## Bare URLs

Subtext parsers MUST implement automatic linking for certain URLs that are not in brackets.

```
You can also just paste bare links, like https://example.com, and Subtext will try to sniff them out and automatically link them.
```

The grammar for URLs defined by [RFC1738 Uniform Resource Locators (URL)](https://datatracker.ietf.org/doc/html/rfc1738) is extremely general, and not practical to sniff out without false positives. To avoid ambiguities and false positives, autolinking is restricted to a few well-known protocols that can more easily be identified:

- `http`
- `https`

### Parsing

`http` and `https` bare URLs are conceptually valid URIs as defined by [Uniform Resource Identifier (URI): Generic Syntax](https://datatracker.ietf.org/doc/html/rfc3986).

Parsing grammar:

```abnf
link = WB url WB
url = http-url / https-url
http-url = "http" ":" "/" url-body
https-url = "https" ":" "/" url-body
WB = SP / NL / BOF / EOF
SP = "\s" / "\t"
NL = CRLF / LF / CR
```

Where,

- `url` is conceptually a valid URI, as defined by [Uniform Resource Identifier (URI): Generic Syntax](https://datatracker.ietf.org/doc/html/rfc3986), with the grammar described in that document. However implementations MAY use a simplified strategy for identifying and parsing URLs, described below.
- `url-body` is conceptually a sequence of characters that are valid in URIs, as defined by [Uniform Resource Identifier (URI): Generic Syntax](https://datatracker.ietf.org/doc/html/rfc3986). However implementations MAY use a simplified strategy, described below.
- `BOF` is a conceptual code point that signifies the end of a string, or input stream.
- `EOF` is a conceptual code point that signifies the beginning of a string, or input stream.

A simplified parsing strategy MAY be used for identifying bare URLs. Implementations that use a simplified parsing strategy to identify bare URLs SHOULD use the following strategy, described here as a regular expression:

```regex
(^|\s)(https?://[^\s>]+)[\.,;]?
```

## Slashlinks

Slashlinks are a shorthand markup meant to be used for linking to same-origin pages. To reduce ambiguity, slashlinks do not use full URL or path syntax, but instead use a restricted syntax that is easier to parse and identify.

Generally, a slashlink is a `/` followed by any number of alphanumeric characters, dashes `-`, underscores `_`.

Implementations are free to interpret the slashlink in whatever way works best for their goals. For example, the slashlink `/foo/bar` does not have to reference a file at path `/foo/bar`. For example, it could be used as a slug for a database lookup, or expanded into a file path, such as `~/Documents/Subconscious/foo/bar.subtext`. These are just examples.

### Parsing

```abnf
slashlink = "/" hier-part [sub-hier-part]
hier-part = ALPHA / DIGIT / "-" / "_"
sub-hier-part = "/" hier-part
```

Parsing slashlinks can be achieved via the following regular expression:

```regex
(^|\s)(/[a-zA-Z0-9/\-\_]+)($|\s)
```

# Rationale

This is a radical refactoring of Subtext, and presents a different set of tradeoffs, with, I think some meaningful advantages for our goals:

- Markup for notes
- YAGNI
- Do the simplest thing that could possibly work

Importantly, ordinary URLs *just work*, without any special syntax.

- For well-known protocols, you can just paste URLs in verbatim.
- For exotic protocols, you can wrap in angle-brackets. This is a common convention for disambiguating URLs and addresses in plain text, used in email `to:` fields and elsewhere.

The biggest change is in removing the notion of label-only links, or wikilinks, and introducing slashlinks.

To recap, wikilinks wrap a range of plain text, marking it as something that should be linked. It is up to clients to normalize this text in order to make it into a useful URL or slug. Since the wikilink is doing two jobs (prose text and address), there are ambiguities around things like pluralization and synonyms. This is often solved with `|` which allows an alternate view-facing label for the wikilink. However, Wikilinks share the same syntax for external links. This leads to ambiguities distinguishing between URLs, paths, and wikilinks. The only reasonable way out is to hard-code protocols into the sniffing logic.

By contrast, slashlinks just represent a path. It is an unambiguous slug, or ID for a local resource.

Pros:

- Extremely simple syntax to write and to parse
- It is unambiguous what resource you are referencing
- It can be used as a primary key in a database
- Since it isn't prose, you don't have to worry about pluralizations and synonyms
- You can efficiently search for it in text, without word stemming
- Works the same way as a URL path.

Cons:

- Not ordinary prose

My sense is that this con is a worthy tradeoff, as it leads to most of the useful pros.

The parallel to draw is not to `[[wikilinks]]`, but to `#hashtags` and `@mentions`, which have been a successful pattern on Twitter and also many other products like Slack, Microsoft Teams, Discord, etc.

This leads to a different style of note-taking focused on simple prose, with `/slashlinks` peppered throughout the text, at the end of paragraphs, or on their own line.

The success of `#hashtags` and `@mentions` lies in there simplicity. They are simple to write, simple to understand, and simple to implement anywhere. `/slashlinks` share this simplicity. They're also visually simple, and easy to scan with your eyeball parsers.

There's also an obvious visual connection between `/slashlinks` and full URLs... they're similar, and they work in similar ways. If the path is too complex to describe with a slashlink, the syntax suggests you can just use a full URL instead, and this is true!

# Drawbacks

The downside of `#hashtags` and `@mentions` is that they "look like code". Well, so do `[[wikilinks]]` and other syntaxes, unless you differentiate between "view mode" and "write mode", a distinction we're trying to avoid in a markup designed for note-taking.

Like Twitter, clients can visually enhance both URLs and `/slashlinks` in a variety of ways:

- Transforming them into links (of course)
- Rendering transcludes below
- Hiding them in some cases, when in view mode


# Unresolved questions

...

# Future possibilities

The introduction of slashlinks suggests that we might also introduce `#hashtags` and `@mentions`. I think this is a neat idea, but want to consider value add before committing to this approach.

Two more extensions could expand the capabilities of the syntax to something closer to the current approach:

Angle-bracket links could allow for labels after a pipe `<https://example.com | Example>`. This would enable nice "prose links" for rendering and publishing, when you want to make a distinction between view-mode and edit-mode. I lean toward introducing this, but don't want to start here, since it doesn't add much for note-taking usecases.

Angle-bracket links could allow relative URLs, or more complex slashlinks: `</example | Example>`. For example, we could allow spaces, or other percent-encodable characters to be written verbatim when in angle brackets.

# References

