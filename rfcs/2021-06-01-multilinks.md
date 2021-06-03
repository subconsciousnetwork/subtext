- Feature: 2021-06-01-multilinks
- RFC PR: https://github.com/gordonbrander/subtext/pull/14
- Issue: (fill me in with link to issue, if any)

# Summary

Support links with labels, wikilinks, and links with multiple URLs.

# Motivation

Currently Subtext underspecifies the contents of link blocks. This RFC expands on Subtext's links by specifying several important types of link:

1. URL-only link blocks
2. URL link blocks with text labels
3. Wikilinks (links that have a text label and no URL)
4. Multilinks (links that specify multiple URLs that the client can use to retreive the resource)

Examples of a URL-only link block, URL with label, wikilink, and multilink with label, in order:

```
& http://example.com
& http://example.com Label text
& Label text
& http://example.com|ipns://example.com Label text
```

## URL labels

Labels serve multiple use-cases:

- They can be used as a text-based fallback when a client doesn't know how to transclude a given type of file.
- Clients can use label-only links as wikilinks, or search links.
- An edge label for constructing [knowledge graphs](http://ai.stanford.edu/blog/introduction-to-knowledge-graphs/) from links.

## Wikilinks

Wikilinks are a critical feature for Wikis and networked note-taking. Wikipedia, Roam, Federated Wiki, and many other systems have a feature analogous to wikilinks.

In Subtext, a wikilink takes the form of a link block without a URL.

```
& Label text
```

The client is free to interpret this label in whatever way it chooses, and select the best resource (if any) that matches the query.

## Multilinks

Numerous HTML features support the ability to specify multiple URLS for the same resource, or the same conceptual object (see [prior art](#prior-art)). This solves a few practical problems:

- Offering resources through more than one protocol (e.g. `http`, `ipfs`) and allowing client to choose the protocol it supports.
- Offering several resources and allowing client to choose the one best suited for the task at hand.
- Mirroring content for performance or robustness.

This doc explores introducing multiple link sources to Subtext. Practically, Subconscious is likely to leverage distributed protocols, such as IPFS or Dat. We want to support linking to these distributed protocols, along with an equivalent HTTP gateway. That way, the content remains useful in clients that do not support these protocols.

# Guide-level explanation

Links are the most important feature in Subtext. By allowing you to reference other documents, you can compose hypertext documents from many smaller documents.

Subtext link blocks allow you to reference other files by path, or by URL. Files can be of any type, even other Subtext documents! Subtext clients typically render a link block as a transclude, displaying all or part of the content of the link in-place. What this looks like will depend on the file type and the Subtext client. For example:

- A link to an image may be displayed in-place
- A link to a video may be displayed with playback controls
- A link to CSV file may be rendered as a table
- A link to a Subtext file may render all or some of the Subtext file within another Subtext file.

Think of Subtext as hypertext glue that lets you compose larger documents from smaller documents.

A basic link is just a link sigil (`&`) followed by a path or URL. These are all valid link blocks:

```
& example.png
& ../example.png
& http://example.com
```

Links may also include a label. Labels are a people-friendly discription of the link. They can be used as a text-based fallback when a client doesn't know how to transclude a given type of file. Labels come after the URL, like this:

```
http://example.com Example label
```

You can also write a link with only a label. We call this a wikilink. Clients may use the label to decide on a resource they think matches best. This is analogous to the way `[[wiklink]]` tags work in [Wiki markup](https://en.wikipedia.org/wiki/Help:Wikitext).

```
& Example label
```

Links can also contain multiple URLs. This allows you to describe multiple places where a resource may be found. This can be useful when sharing content from newer protocols that may not be widely supported yet, such as `ifps` or `dat`. In these cases you can offer multiple URLs and the client can choose whichever one it understands.

Multiple urls are separated by a pipe character (`|`).

```
& http://example.com|ipfs://example|dat://example
```

You can also have multiple URLs and a label:

```
& http://example.com|ipfs://example|dat://example Example label
```

# Reference-level explanation

Link blocks start with `& `. They reference other files within the flow of a Subtext document. Any kind of file can be linked, including other Subtext documents.

Links are the most important feature in Subtext. By allowing you to reference other documents, you can compose hypertext documents from many smaller documents.

Links can include zero or more pipe-delimited URLs, and an optional text label. They often take one of the following forms:

- URL
- URL with label
- Multiple URLs
- Multiple URLs with label
- Label only (wikilink)

The following are all valid URLs

```
& example.png
& http://example.com
& http://example.com Example label
& http://example.com|ipfs://example|dat://example Example label
& Example label
```

The client MUST render some kind of user-interactable link to the document. The client MAY choose the form this link takes.

When more than one URL is present, clients MAY choose which URL to use when presenting to the user, when navigating, or when transcluding content.

When no URL is present, but a text label is present, clients SHOULD choose a resource that best matches the text when navigating to the link, or when transcluding content.

For file types it supports, the client MAY render all or part of the linked document in-place (e.g. transclude). For example:

- A linked image file (`.png`, `.jpg`, `.gif`, etc) MAY be rendered in-place by the client and sized to fit its context.
- A linked video file MAY be rendered in-place, together with playback controls.
- A linked `.csv` file MAY be rendered in-place as a table.
- A linked Subtext file MAY be transcluded (linked while rendered in-place), rendered in-place in full, or excerpted and rendered in-place, depending on the use-case.

Rather than extending the syntax of Subtext to include features like tables, videos, or deeply nested lists, our sense is that a hypertext format allows these special types to be represented in their native file containers. Clients that understand these other file types MAY embed them, or even allow you to edit them in-place. This keeps Subtext simple, and allows data sources like `.csv`, or `.png` to be OPTIONALLY embedded in-place, while remaining valid file types that can be opened, edited, and used in best-of-breed applications.

## Syntax

Each link form is some combination of zero or more pipe-delimited URLs, followed by optional label text.

A link block has the following syntax in ABNF notation, as described in [IETF RFC 5234](https://tools.ietf.org/rfc/rfc5234):

```abnf
link-block = "&" *space link-block-body universal-newline
link-block-body = url / labeled-url / url-group / labeled-url-group / text
labeled-url = url 1*space text
url-group = *(url "|") url
labeled-url-group = url-group 1*space text
universal-newline = "\r\n" / "\n" / "\r"
space = "\s"
```

Where:

- `url` may be either a relative file path, or a URL that conforms to the URI syntax described in [IETF RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986).
- `text` is any number of unicode characters, as described in [IETF RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629), excluding [Universal Newline](../specification.md#universal-newlines).


# Rationale and alternatives

The challenge:

- Disambiguating between free-range text and URLs is tricky without introducing a separator syntax. Plain text includes spaces. OTOH spaces are one reliable way to delimit multiple URLs.
- Ideally, we want to avoid introducing syntax with closing tags, since it is possible to break this kind of syntax.
- We want a syntax that feels somewhat natural.
- Ideally, we want to support URL with label, URL-only, and label-only (wikilinks!)

## Multiple URLs delimited by pipe

```
& http://example.com|ipfs://example Example
```

Advantages:

- Analogous to "or" pipe
- Easy to parse. Split on the first space, then split on `|`.
- Pipe is never a valid URL character
- Keeps simple things simple. Basic wikilinks are easy. Single URLs are easy.

Disadvantages:

- Complex things are a bit more complex. A bit harder to read.

## Multiple URLs delimited by comma

```
& http://example.comm, ipfs://example, Example
```

Advanatages:

- Intuitive! Comma is how you list things.
- Similar to srcset
- In future, this syntax has room for additional qualifiers after URL (consider srcset flags in [prior art](#prior-art))

Disadvantages:

- Comma is a [reserved sub-delimiter character in URLs](https://datatracker.ietf.org/doc/html/rfc3986#page-11). That means we MUST delimit by comma AND space. Even so, this ambiguity causes practical problems with parsing in practice ([example](https://github.com/simplecrawler/simplecrawler/issues/413)). 
- Minor: the delimiter between URL and text is the same, so you have to test the last item to determine if it is a URL.

## Fancy space disambiguation

I could imagine disambiguating text from URLs by splitting on space, and then testing each chunk for a protocol. All chunks after the first chunk that does not begin with a protocol are treated as text.

So the following

```
& http://example.com http://example.mirror.com ipfs://example dat://example Example
```

Parses to to pseudocode:

```
((urls
  "http://example.com"
  "http://example.mirror.com"
  "ipfs://example"
  "dat://example")
  (text "Example"))
```

Here's a pathological case:

```
& http://example.com http://example.mirror.com ipfs://example dat://example Example http://thisistext.com
```

Parses to:

```
((urls
  "http://example.com"
  "http://example.mirror.com"
  "ipfs://example"
  "dat://example")
  (text "Example http://thisistext.com"))
```

Advantages:

- The most common cases are obvious

Disadvantages:

- Parsing is a bit tricky
- Pathological cases pathological

## Parts delimited by spaces and a `|`

```
& Example | http://example.com | http://example.mirror.com | ipfs://example | dat://example
```

Alternatively:

```
& Example / http://example.com / http://example.mirror.com / ipfs://example / dat://example
```

Advantages:

- Readable
- No closing tag

Disadvantages:

- Pipe character is not really the first thing you might guess, but then again multiple URLs are a slightly more advanced feature.

## No-gos

### Required labels

We could require links to begin with labels. This would be useful for types that clients don't know how to transclude.

```
& Example | http://example.com http://example.mirror.com ipfs://example dat://example
```

Advantages:

- Simple and unambiguous to parse

Disadvantages:

- Annoying to type labels for all links
- Would lead to people putting nothing to the left of pipe. Syntax noise!

### Text wrapped in quotes

```
& "Example" http://example.com http://example.mirror.com ipfs://example dat://example
```

Advantages:

- Obvious

Disadvantages:

- Closing tag (possible to forget closing quote)
- Requires you to quote wikilinks when there are no URLs.

### Bracket URLs

```
& Example <http://example.com> <http://example.mirror.com> <ipfs://example> <dat://example>
```

Or, alternatively:

```
& Example (http://example.com) (http://example.mirror.com) (ipfs://example) (dat://example)
```

Advantages:

- Familiar if you've used Markdown

Disadvantages:

- Closing tag
- Fiddly for multiple URLs.
- An annoying amount of typing if all you want to do is link to a URL.

# Drawbacks

...


# Prior art

- HTML5 srcset attribute. See [srcset spec](https://html.spec.whatwg.org/multipage/images.html#srcset-attribute), [srcset explainer on MDN](https://developer.mozilla.org/en-US/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images#resolution_switching_different_sizes)
  - Note that srcset delimits by comma, which is a reserved URL character. This ambiguity causes practical problems with parsing ([example](https://github.com/simplecrawler/simplecrawler/issues/413)). If we wanted to use a comma, we could require the comma be followed by a space.
- [source tag](https://html.spec.whatwg.org/multipage/embedded-content.html#the-source-element) in picture tag
- [source tag](https://html.spec.whatwg.org/multipage/embedded-content.html#the-source-element) in video tag
- [source tag](https://html.spec.whatwg.org/multipage/embedded-content.html#the-source-element) in audio tag

# Unresolved questions

- Should we establish an upper limit on number of URLs, or leave that up to client?

# Future possibilities

- What about markup for MIME type?
  - What if the different URLs have different resources with different MIME types?
  - What happens when MIME in markup and MIME in request don't match? If the actual response doesn't match you probably have to throw it away, or specify some fallback behavior.
  - Is this necessary? When links are followed eagerly, as with audio/video/transcludes, it does allow you to avoid fetching files when you don't understand the MIME type.