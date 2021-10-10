# Subtext guide

- **Status**: Draft
- **Authors**: Gordon Brander

Subtext is markup for note-taking. It aims to add just the tiniest bit of structure on top of plain text, to make notes more useful.

**Warning**: Subtext is an experiment, NOT a finished language proposal. We are actively putting it through its paces using practical prototypes. It may change in radical and breaking ways between now and a first stable release. 

## A bit of Subtext

Subtext is markup that is barely there—just the *tiniest* bit of structure on top of plain text, to make it more useful. Simple for people, simple for computers, and meaningful for both.

```
# Heading

Plain text.

URLs like https://example.com are automatically linked.

You can also link to other notes using /slashlinks.

- List item
- List item

> Quoted text
```

## Introduction

Subtext represents block-oriented documents as line-oriented markup. Each line is treated as a distinct block of content. The type of a line is determined by a "sigil character", like `#`, or `>`, at the front of the line. If a line doesn’t have a sigil character, it is treated as plain text.

Some of the earliest hypertext proposals were block-oriented, including Ted Nelson's ELF (Nelson, 1965). Block-oriented documents have also independently evolved within many contemporary tools-for-thought, including Notion, Roam, and Ward Cunningham's Federated Wiki. Why does this pattern keep re-emerging?

Blocks are like thought legos. They turn ideas into data that can be remixed into new ideas. This is very valuable when building "Tools for Thought" — tools that augment our note-taking and writing. Using Subtext you can do things like:

- Excerpt a document by taking the first text block
- Select all quotes from a collection of documents
- Select all links, and generate a link graph for a collection of documents
- Find all backlinks and append them to the document as links

## Blocks

### Text blocks

One line = one block in Subtext. This means you write text extactly the way you would anyway.

Here are two blocks:

```
Delays in systems cause waves.

You can only smooth them out with stocks that match the scale of the delay-waves.
```

One line = one block also means that blank lines are optional. This is also two blocks.

```
Block 1
Block 2
```

It's nice to place a blank line between text blocks for readability, though.

### Heading blocks

Heading blocks start with a `#`

```
# This is a heading

Text underneath.
```

The space after the `#` is optional, but it's nice to include it for readability.

```
#This is also a heading
```

### List blocks

List blocks start with a `-`

```
- List item
- List item
- List item
```

The space after the `-` is optional, but it's nice to include it for readability.

### Quote blocks

Quote blocks start with a `>`

```
> Life is not a problem to be solved, but a reality to be experienced.
```

The space after the `>` is optional, but it's nice to include it for readability.

## Links

Subtext will detect any http URLs, and automatically link them.

```
Links like https://example.com will be automatically linked.
```

Note you must include the protocol (the `http://` bit) for Subtext to sniff out the URL.

Subtext parsers can automatically detect `http` links this way, but not every protocol will work. For exotic protocols wrap the URL in `<` `>` angle brackets.

```
Link unusual URLs like <doi:10.1000/182> by wrapping them in angle brackets. 

This works for any kind of URL <https://example.com>.
```

## Slashlinks

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