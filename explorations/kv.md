# Key-value blocks

We could explore expanding Subtext to support markup for [key-value pairs](https://en.wikipedia.org/wiki/Attribute%E2%80%93value_pair).

```
Q: What is Subtext?
A: Subtext is a markup language for note-taking.
```

A key-value block is any alphanumeric string followed by a `:`. The alphanumeric string before the `:` becomes the sigil type for the block.

Sigil, described as a regular expression:

```
^[a-zA-Z0-9_]+:\s
```

Key-value pairs are a fundamental primitive with a wide range of potential use-cases for tooling. Like any other type of block, key-value blocks could be gathered by key into lists, concatenated, or collected using a first- or last-key-wins to get simple key/value data.

- You could execute queries such as: “list all questions (`Q:` blocks) in my notes”.
- You could transform a collection of notes into a sparse table by treating each note as a row, and treating keys as columns. Denser tabular data can be had by filtering notes to only include those with a particular set of keys, and then concatenating, JSON-encoding, or dropping duplicate keys. Tada! CSV.
- You could include headmatter in the body of a note. This can make it easier to integrate notes with static site generators, or other tools.

**Open question**: what are the implications for parsing? It would require us to run a search on across a string for an unbounded number of characters, until we encounter a space character, before defining the block as a text block. That means this search must happen to every block before it can be found to be a text block. Is this a problem in practice? Are there ways we could simplify this algorithmically?

## Alternatives

```
@Q What is Subtext
@A Subtext is a markup language for note-taking
```

or

```
$Q What is Subtext
$A Subtext is a markup language for note-taking
```

Pros: can determine block type based on first character.

Cons: less natural to type.