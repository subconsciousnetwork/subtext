# Subtext

## Speculative specification

- **Version**: 2021.05.25.dev
- **Status**: Draft
- **Authors**: Gordon Brander

This is a rough sketch of an actual spec for Subtext. It is becoming increasingly less rough over multiple passes. This doc is being shared in the spirit of working with the garage door open. Feedback is welcome!

**Warning to implementors**: Subtext is a hypothesis, an experiment, NOT a finished language proposal. We are actively putting it through its paces using practical prototypes. It may change in radical and breaking ways between now and a first stable release. 

## Overview

Subtext is a text-based, line-oriented hypertext format. It is designed with note-taking in mind. It has a simple, people-friendly syntax that is easy to read, and difficult to mess up.

Subtext markup is made up of a sequence of lines. Lines that are prefixed with "magic" characters are treated as specially tagged blocks. Lines without leading special characters are tagged as text blocks. Empty lines are ignored. Here's a quick sample:

```
# Heading

Plain text

- List item
- List item

> Quoted text

& example.csv
& https://example.com
```

Link blocks (lines starting with `&`) allow you to link to other files within the flow of a Subtext document. Any kind of file can be linked, including other Subtext documents! This makes Subtext composable, and gives it hypertext properties similar to Ted Nelson's ELF format (Nelson, 1965).

Subtext is designed to be used in a wide range of settings, from simple command-line utilities, to advanced clients. It is extremely easy to parse, and can be rendered in a single pass, or even streamed.

## Markup for note-taking

Subtext is designed with hypertext note-taking in mind. It represents _block-oriented documents_ as _line-oriented markup_.

A typical block-oriented document is made up of a list of blocks of different types (or occasionally, a tree of blocks). Each block type may be displayed differently. For example, a quote block may render as quote-formatted text, while an image block may render an image in-place.

Some of the earliest hypertext proposals were block-oriented, including Ted Nelson's ELF (Nelson, 1965). Block-oriented documents have also independently evolved within many contemporary tools-for-thought, including [Notion](https://www.notion.so/), [Roam](https://roamresearch.com/), and [Ward Cunningham's Federated Wiki](http://fed.wiki.org/view/federated-wiki).

Block-oriented documents allow for a rich mix of text, images, and other formatted content, while remaining easy to understand. Block-oriented documents are also composable (and decomposable). You can break them apart into component blocks, filter down to blocks of a particular type, merge documents, pluck out blocks, link to specific blocks, etc. Working with documents programmatically is easy because the document structure is linear. Try  meaningfully merging two HTML files... tag soup!

Subtext takes the magic of _block-oriented documents_ and represents it as _line-oriented markup_, with each line describing a block. Plain text is simple, human-readable, and most importantly, it will never go away. You can work with plain text using a wide range of tools, from GUI editors, to grep. Subtext takes universal plain text, and layers a bit of structure on top, in a way that still make sense to read as plain text.

I'm developing Subtext as part of a tool-for-thought, called [Subconscious](https://subconscious.substack.com/). I expect Subtext could be broadly useful for other note apps, wikis, [Zettelkasten](https://en.wikipedia.org/wiki/Zettelkasten), or plain-text note-taking. Subtext is useful anywhere a bit of structure might help you (and your computer) think more meaningfully with text.


## Design goals

### People goals

- **People-friendly**. We've settled on a syntax that is as close to plain text as we could manage. It has a passing resemblance to Markdown.
- **Hand-writeable**. We want to avoid imposing too much syntax on the user who is hand-editing Subtext. It should be possible to write ordinary lines of plain text, prose, and get a working document out of it.
- **Simple and mistake-proof**. At this time, it is not possible to write invalid Subtext documents by hand. There are no brackets to mis-type or closing tags to omit. We would like to retain this property as much as possible, even if Subtext's features grow in the future.

### Computer goals

- **Easy to implement**. Clients can implement features that make sense for their use-cases, and fall back to plain text for unsupported features.
- **Composable and decomposable into component blocks**. You can compose documents from many small pieces, including from other Subtext documents.
- **Simple to meaningfully manipulate with software**. With a linear data model, the range of meaningful document structures is narrowed. this means you can make complex, yet meaningful programmatic decisions about how to manipulate it without having to understand as much about the structure and context of the specific document.

Using Subtext, you should be able to programmatically do things like:

- Excerpt documents by showing only the first text block
- Append this document to that one
- Select all quotes from a collection of documents
- Autogenerate a table of contents from all heading blocks in a document
- Select all links, and generate a graph for a collection of documents
- Find all backlinks and append them to the document as links

### Non-goals

Subtext is not a layout, presentation, or word processing format. The analogy to draw here is index cards, not books or manuscripts. Subtext deliberately avoids the kind of complex presentation features offered by publishing formats like HTML, PDF, and LaTex. It has no opinions about fonts, colors, sizes. It does not deal in scripting behavior.

We picture Subtext being used for note-taking systems, like as [Zettelkasten](https://en.wikipedia.org/wiki/Zettelkasten). A publishing workflow might look like copy-pasting Subtext into a publishing tool to create an artifact, or perhaps exporting Subtext into another format, like HTML, and then editing it into shape.

This puts Subtext in a different category than complex markup languages like HTML5. It is closer in spirit to something like RSS or OPML. Subtext brings a minimal amount of semantic structure to linear documents. It is up to clients to decide how that structure is used and displayed.

## Sample document

```
# Overview

Evolution is a behavior that emerges in any system with:

- Mutation
- Heredity
- Selection

Evolutionary systems often generate unexpected solutions. Nature selects for good enough.

> There is no such thing as advantageous in a general sense. There is only advantageous for the circumstances you’re living in. (Olivia Judson, Santa Fe Institute)

Evolving systems exist in punctuated equilibrium.

& punctuated-equilibrium.st

# Questions

- What systems (beside biology) exhibit evolutionary behavior? Remember, evolution happens in any system with mutation, heredity, selection.
- What happens to an evolutionary system when you remove mutation? Heredity? Selection?
- Do you see a system with one of these properties? How can you introduce the other two?

# See also

& https://en.wikipedia.org/wiki/Evolutionary_systems
```

## Line-orientation

Subtext is line-oriented. Lines can be parsed to blocks by checking if the leading characters of the line match one of a set of special character sequences. These special character sequences are called "sigils", and determine the block type of the line.

All sigils currently happen to be two characters: one character plus a space. In future, we may introduce multi-character sigils, so implementors MUST NOT determine block type by sampling only first two characters. However, we will try to avoid multi-character sigils for reasons of aesthetics.

**TODO**: fix an upper limit on sigil length?

Note that the space after sigil characters is part of the sigil and is not optional. If the sigil character does not end in a space, it is not a sigil character.

```
# This is a heading
#This is just text with a weird character in front.
```

## Core block types

### Blank lines

Each blank line in Subtext is parsed to a blank block. Blank blocks are for presentational purposes, and have no structural meaning.

For example, the following Subtext document:

```
I have eaten
the plums
that were in
the icebox

and which
you were probably
saving
for breakfast
```

Parses to blocks (expressed as Lisp pseudocode):

```
((text "I have eaten")
 (text "the plums")
 (text "that were in")
 (text "the icebox")
 (blank)
 (text "and which")
 (text "you were probably")
 (text "saving")
 (text "for breakfast"))
```

When parsing Subtext, blank lines are defined as any number of `\s` or `\t` characters, followed by a Universal Newline.

The following sequences of characters are all valid blank lines for the purpose of parsing. (Since space characters are invisible, the following examples are described using escape characters):

```
\n
\s\n
\t\t\r\n
```

Any `\s` or `\t` characters that may have been part of a blank line during parsing are discarded. Subtext renderers MUST render blank lines as an empty string, joined with a newline `\n` character.

### Text blocks

Text blocks are the fundamental line type. Any line which does not match the definition of another line type defaults to being a text line.

Text lines SHOULD be presented to the user in a manner suitable for general reading. This SHOULD include wrapping text, where appropriate. Text lines MAY be formatted for reading at the client's discretion. For example, clients MAY choose to display text with variable fonts, leading and font-size appropriate to the context. 

### Link blocks

Link blocks start with `& `. They reference other files within the flow of a Subtext document. Any kind of file can be linked, including other Subtext documents.

The client MUST render some kind of user-interactable link to the document. The client MAY choose what form this link takes.

For file types it understands, the client MAY render all or part of the linked document in-place (e.g. transclude). For example:

- A linked image file (`.png`, `.jpg`, `.gif`, etc) MAY be rendered in-place by the client and sized to fit its context.
- A linked video file MAY be rendered in-place, together with playback controls.
- A linked `.csv` file MAY be rendered in-place as a table.
- A linked Subtext file MAY be transcluded (linked while rendered in-place), rendered in-place in full, or excerpted and rendered in-place, depending on the use-case.

Links are the most important feature in Subtext. By allowing you to reference other documents, you can compose hypertext documents from many smaller documents.

Rather than extending the syntax of Subtext to include features like tables, videos, or deeply nested lists, our sense is that a hypertext format allows these special types to be represented in their native file containers. Clients that understand these other file types MAY embed them, or even allow you to edit them in-place. This keeps Subtext simple, and allows data sources like `.csv`, or `.png` to be OPTIONALLY embedded in-place, while remaining valid file types that can be opened, edited, and used in best-of-breed applications.

**For future exploration**:

- [Multiple reference links](explorations/2021-05-28-multilinks.md)
- [Generating citations via DOI links](explorations/doi.md)
- [Wiki links and search links](explorations/2021-05-28-multilinks.md)

### Heading blocks

Heading blocks start with `# `. Heading blocks SHOULD be presented in a manner denotes that they are hierarchically "above", and "label" blocks below. This MAY mean typical typographic heading treatment in visual clients, such as increasing the font size as compared to text blocks, or displaying the text in bold. In non-visual clients, such as screen readers, this MAY mean announcing the block using a different voice style.

Clients MAY also create navigational affordances for headings, such as deriving a Table of Contents from heading blocks, or creating jump points in the scroll bar.

Subtext currently supports only one level of heading. This is a deliberate design choice, since a deep heading hierarchy is probably a sign your note needs to be refactored or unbundled into multiple notes. To construct deeply nested documents, you can link to other notes using a link block, instead. Supporting clients MAY read in these nested documents, embed them in-place, and adjust heading sizes as necessary, to denote heading level. Solving hierarchy through links encourages a hypertext method of writing, and allows pieces of a document to be used in more than one place.

### List blocks

List blocks start with `- `. List blocks SHOULD be presented in a manner that denotes they are lists. Visual clients MAY render list blocks with a bullet preceding. They MAY also choose to visually group contiguous list blocks together, for example, by removing the margins between them.

Subtext currently supports only one level of list. Our sense at this time is that the benefits of deep lists do not outweigh the costs of complicating the document format by introducing hierarchy. Clients MAY consider transcluding links to formats like `.yaml`, `.json`, or `.opml` to display deep lists.

### Quote blocks

Quote blocks start with `> `. Quote blocks SHOULD be presented in a manner that denotes they are quoted text. Visual clients MAY render quote blocks by indenting them, or by rendering them with a line to the left, as seen in many email clients. Non-visual clients, such as screen readers MAY read quote blocks in a different voice style.

## Sigils reserved for future use

The following sigils are reserved for possible future use:

- `$ `
- `@ `
- `! `
- `% `
- `~ `
- `| `
- `: `
- `* `
- `+ `
- `= `
- `\ `
- `λ `
- `\s\s` (two or more contiguous space characters leading a line)
- `\t` (one or more contiguous horizontal tab characters leading a line)
- `---`

Note, again, that the sigil includes the space character.

## Line breaks

Different systems use different character(s) for breaking lines. For example, Unix systems use `\n` (LF), while Windows uses `\r\n` (CRLF) (See [Wikipedia entry on newlines](https://en.wikipedia.org/wiki/Newline#Representation)). Because Subtext is line-oriented, it specifies a strategy for normalizing line breaks.

The canonical line-break for Subtext is `\n` (LF).

Human authors of Subtext SHOULD use `\n` to delimit lines.

Software that writes Subtext MUST use `\n` to delimit lines.

Software that parses Subtext will accept several newline characters as line breaks: `\n`, `\r`, or `\r\n` (referred to as "Universal Newlines" elswhere in the spec). Subtext parsers MUST normalize newlines by interpreting `\r\n` (CRLF) and `\r` (CR) as equivalent to `\n`, using the following sequence of steps:

1. If `\r\n` is encountered, it is normalized to be equivalent to `\n` by the parser.
2. If `\r` is encountered, it is normalized to be equivalent to `\n` by the parser.

Other line break characters, including Unicode line-break characters such as `PS` and `LS` MUST NOT be treated as line delimiters for the purpose of Subtext parsing.

This strategy follows [Postel's Robustness Principle](https://en.wikipedia.org/wiki/Robustness_principle), to a degree. We standardize on a single line-break character, but will parse the most widely used alternatives.

## Mime Type and extension

The preferred file extension for Subtext is `.st`.

The Mime Type for Subtext is `text/subtext`. As a subtype of the top-level media type "text", "text/subtext" inherits the "charset" parameter defined in [RFC 2046](https://tools.ietf.org/html/rfc2046). However, the default value of "charset" is "UTF-8" for Subtext "text" content.

Subtext is valid plain text and plain text is, in most cases, valid Subtext. In most cases it should be practically possible to read `.txt` files as Subtext, if a client wishes to.

## Appendix 1: Terminology

### Specification requirement levels

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).

### Universal Newlines

When parsing, Subtext normalizes several line break characters. The following are all treated as line breaks when parsing: `\n`, `\r`, `\r\n`. See [line breaks](#line-breaks) for specific normalization steps.

Note that while Subtext parsers accept Universal Newlines, Subtext renderes only ever write Unix newlines (`\n`).

## Appendix 2: Further Resources

- [Subtext overview and motivation](README.md)
- [FAQ](faq.md)
- [Future language explorations](./explorations/): this section is non-normative and exploratory. It captures some thinking about possible language additions or alternatives.
- [Subtext parsers](./parsers/): code you can use to parse and work with Subtext.
- Design notes
  - [Hypertext Montage](https://subconscious.substack.com/p/hypertext-montage)
  - [Thought Legos](https://subconscious.substack.com/p/thought-legos)

## Appendix 3: References

### Prior art

#### Ted Nelson's ELF

By an accident of convergent evolution, Subtext happens to have some structural resemblances to Ted Nelson's ELF format (Nelson, 1965).

#### Runic

[Runic](https://wiki.xxiivv.com/site/runic.html) is a small line-oriented wiki markup language. The simplicity and cleverness of Runic made me fall in love with the idea of line-oriented markup.

#### Project Gemini

[Gemini](https://gemini.circumlunar.space/) is an internet protocol which aims to be "heavier than Gopher and lighter than the web." [Gemini features a line-oriented markup language](https://gemini.circumlunar.space/docs/specification.gmi).

I discovered Gemini about a month after roughing out the outline for Subtext. I was excited about the idea of a minimal line-oriented syntax. Gemini gave me further confidence that line-orientated markup could be a solid basis for a hypertext format.

#### Markdown

Subtext has a passing resemblance to [Markdown](https://daringfireball.net/projects/markdown/). Markdown is a popular plain text formatting systems among programmers. It features a familiar syntax that looks similar to the organic conventions of plain text email.

Markdown is meant to have HTML semantics, and compile to HTML. You can also embed arbitrary HTML within Markdown. By contrast, Subtext can be compiled to HTML, but is not specifically a "compile to HTML" language. Note [Subconscious](https://subconscious.substack.com/) will be shipping a native (non-browser) renderer for Subtext. It's important that we don't drag the whole web specification in behind us accidentally.

The original implementation of Markdown used regular expression string replacement, rather than parsing an AST. Many Markdown libraries take this approach. Few generate an AST.

Markdown does not have an official formal specification, only a reference implementation. Aspects of the syntax are ambiguous. This is a fine choice for a personal tool, and has encouraged organic extensions to the syntax, such as [GitHub-flavored Markdown](https://github.github.com/gfm/). On the other hand, it makes it difficult to treat Markdown itself as a standardized format intended to work across multiple clients. 

Specifications for various dialects:

- [GitHub-flavored Markdown](https://github.github.com/gfm/)
- [CommonMark](https://commonmark.org/)

#### Emacs Org Mode and Outline Mode

[Outline Mode](https://www.gnu.org/software/emacs/manual/html_node/emacs/Outline-Mode.html) and [Org Mode](https://orgmode.org/) are two popular plain text syntax flavors in Emacs.

[Syntax](https://www.gnu.org/software/emacs/manual/html_node/emacs/Outline-Format.html): Outline Mode is made up of headings (prefixed with one or more `*`), and text (unprefixed). The number of `*` determines the depth of the heading. Text below a heading is considered "belonging to" that heading, and together they make up an "entry".

Org Mode builds on Outline Mode, adding additional syntax for metadata, `[[wikilinks]]` and other features.

Things to appreciate about Outline Mode:

- It gets a lot of mileage out of a single primitive (`*`).
- It is a structured format. You can parse it into logical software-manipulable blocks.

Con: Forcing titles for logical blocks imposes a fair amount of abstract thinking when taking rough notes.

Note Org Mode is not strictly line-oriented. [It breaks paragraphs by one or more blank lines](https://orgmode.org/manual/Paragraphs.html).

### References

- Nelson, Theodore "A File Structure for the Complex, the Changing, and the Indeterminate", Association for Computing Machinery, Proceedings of the 20th National Conference, 84-100. Ed. Lewis Winner, 1965.
