- Feature: 2021-05-28-multilinks
- RFC PR: (fill me in with link to PR)
- Issue: (fill me in with link to issue, if any)

# Summary

Support links with multiple references and labels.

# Motivation

Numerous HTML features support the ability to specify multiple URLS for the same resource, or the same conceptual object (see [prior art](#prior-art)). This solves a few practical problems:

- Offering resources through more than one protocol (e.g. `http`, `ipfs`) and allowing client to choose the protocol it supports.
- Offering several resources and allowing client to choose the one best suited for the task at hand.
- Mirroring content for performance or robustness.

This doc explores introducing multiple link sources to Subtext. Practically, Subconscious is likely to leverage distributed protocols, such as IPFS or Dat. We want to support linking to these distributed protocols, along with an equivalent HTTP gateway. That way, the content remains useful in clients that do not support these protocols.

This doc also explores introducing labels to link blocks. Labels serve multiple use-cases:

- They can be used as a text-based fallback when a client doesn't know how to transclude a given type of file.
- Clients can use label-only links as wikilinks, or search links.
- An edge label for constructing [knowledge graphs](http://ai.stanford.edu/blog/introduction-to-knowledge-graphs/) from links.

# Guide-level explanation

Subtext links may include a label. Labels are a people-friendly discription of the link.

They can be used as a text-based fallback when a client doesn't know how to transclude a given type of file. They can also be used without URLs as wikilinks.

Labels come after the URL (if any). These are all valid links:

```
& http://example.com
& http://example.com Example label
& Example label
```

That last link is what we call a wikilink. It doesn't have a URL associated with it, but clients may use the label to decide on a resource they think matches best. This is analogous to the way simple `[[wiklinks]]` work on Wikipedia.

Links can also contain multiple URLs. This allows you to describe multiple places where a resource may be found.

- This can be useful when sharing content from newer protocols that may not be widely supported yet, such as `ifps` or `dat`. Multiple sources allow you to specify an `http` gateway where clients may alternatively go to find the content.
- Multiple sources can also be used for robustness or performance reasons. For example, you might link to a source, along with its [Internet Archive mirror](https://archive.org/).

Clients are free to choose whichever URL works for their needs.

We're still working out the syntax for multiple URLs (see [approaches](#approaches). They might be pipe-delimeted:

```
& http://example.com|http://example.mirror.com|ipfs://example|dat://example
```

...or, to follow the lead of HTML's srcset attribute, we could delimit with a comma and a space (see [prior art](#prior-art)).

```
& http://example.com, http://example.mirror.com, ipfs://example, dat://example
```

See [approaches](#approaches) below for more info.

# Reference-level explanation

TODO

Parsing URLs, roughly:

```
[a-zA-Z]+://[^\s]+
```

This may result in parsing some invalid URLs, but it keeps parsing simple.

# Approaches

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

- Comma is a [reserved sub-delimiter character in URLs](https://datatracker.ietf.org/doc/html/rfc3986#page-11). That means we MUST delimit by comma AND space.
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
- picture tag
- video tag

# Unresolved questions

- Should we establish an upper limit on number of URLs, or leave that up to client?

# Future possibilities

- What about markup for MIME type?
  - What if the different URLs have different resources with different MIME types?
  - Is this useful? What happens when MIME in markup and MIME in request don't match?
  - Is this necessary? You have to fetch the resource to be able to transclude it anyway.