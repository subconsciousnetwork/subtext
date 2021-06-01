- Feature: 2021-06-01-blank-lines
- RFC PR: https://github.com/gordonbrander/subtext/pull/13

# Summary

Blank lines are useful should be preserved.

# Motivation

This RFC amends Subtext so that it treats blank lines as a new block type, "blank".

Subtext as currently specified maps line-oriented text to block-oriented markup. Linebreaks map to paragraph breaks. Empty lines are treated as non-markup. However,

- Consider: we prefer to "pretty print" text with empty lines between blocks, except list blocks, when rendering. This hints at a mismatch between Subtext's data model vs human-preferred writing style. The mismatch can be papered over via a "pretty-print" renderer, but the behavior should be specified, and deserves further consideration of tradeoffs.
- Consider: Subtext cannot presently present poetry. This isn't a dealbreaker, but it does seem unfortunate.

By specifying blank lines as a block type, we:

- Increase the expressive range of Subtext a bit.
- Simplify the parser/renderer logic.
- Get a cleaner roundtrip. `Subtext -> parse -> render -> Subtext` will result in the same document.

In short, it's the simplest thing that could possibly work, and it increases Subtext's expressiveness.


# Guide-level explanation

Blank lines can be used to separate blocks in subtext. You can use any number of newlines. They have no special meaning on their own, and how you use them is up to you.

Here's an example of several text lines and a blank line being used to jot down a bit of poetry:

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

Note this means line breaks in Subtext are significant. Hard-wrapped prose text separated by newlines will not be unwrapped and re-wrapped for different screen widths. If you mean to write flowing prose, you should use soft-wrapping.

# Reference-level explanation

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

# Drawbacks

Drawbacks:

- No "semantic grouping" for things like lists. I would say this is both a benefit and a drawback. Line-orientation FTW!
- Mapping to a tree-based AST language like HTML is a bit trickier, but not difficult. It can be accomplished by using something like Python's [groupby](https://docs.python.org/3/library/itertools.html#itertools.groupby) function to group contiguous blocks. Anyway, it makes the HTML renderer shoulder the complexity of HTML, rather than embodying AST complexity in the basic parser.

# Rationale and alternatives

There are several other ways we could approach:

- Leaving behavior as is, and specifying pretty-printing
- Grouped lines: Where most languages parse into an abstract syntax tree (AST), Subtext is currently a flat list of typed blocks. However, we could maintain this property, while assigning lines broken by blank lines into logical groups.
- Preformatting toggle: an opening/closing marker for preformatted text blocks. This could be paired with either approach above.

Taken in order...

## Leaving behavior as is, specifying pretty-printing

This approach has a couple downsides:

- Less expressive than specifying line breaks
- Requires pretty-printing renderer to embody a tiny bit of complexity by rendering list blocks differently from other blocks.

## Grouped lines

```
Some text
More text

- List item
- List item

> A quote
More text


The end
```

Would be parsed to the following AST pseudocode:

```
(text ("Some text" "More text"))
(list ("List item" "List item"))
(quote ("More text"))
(text ("More text"))
(text ("The end"))
```

The rule is that contiguous blocks of the same type are combined into one. Clients can choose how to render the block parts. For example

Benefits:

- Semantic grouping can be nice. Lists are now a unified block, for example.

Drawbacks:

- Text roundtripped through parse->render may not be the same. Multiple blank lines will be collapsed. I would say this is both a benefit and a drawback!
- You will not be able to use multiple contiguous line breaks, for example, in poetry.

Additional notes:

- Relatively simple implementation using the equivalent of Python's groupby, so implemenation complexity is not an impediment.
- Another consideration is how this might play with indentation later on. Certainly possible, but may require a simple lookahead shift-reduce parser. I am not fond of hierarchical indentation, but I also don't want to paint myself into a corner there.

The biggest argument against this approach is that you can easily build it on top of significant blank lines by using the equivalent of Python's [groupby](https://docs.python.org/3/library/itertools.html#itertools.groupby) function. A client that desires an AST can group contiguous blocks, and they've got an AST.

## Preformatting tag

Preformatting could be accomlished with a preformatting tag, perhaps three backticks, like Github-flavored Markdown. After encountering this tag, the parser would concatenate all lines it receives, verbatim, until it encounters a closing flag, at which point, it assigns the concatenated lines to a Preformatted block and returns to normal parsing.

The preformatting tag can be considered together with, or independently of the two other proposals.

Benefits:

- This would allow for maximal flexibility in formatting.
- Whitespace could be honored. It may also provide an escape hatch for embedded code, although we strongly prefer the approach of transcluding and pretty-printing real code files.

Drawbacks:

- Introduces a new concept to the language that is not line-oriented.
- It is possible to write broken preformatting, by forgetting the closing tag.

Notes:

- Unclear how this would play with indentation if we ever introduced it. Perhaps the flag would be required to be at indentation level 0.

All-in-all this feels orthogonal, and probably not necessary.


# Prior art

- Semantic blank lines is what [Gemini](https://gemini.circumlunar.space/docs/specification.gmi) does. Going this route would make Subtext very similar to Gemini, although there are differences in surface syntax, and, in future, probably a few features.
- [Gemini](https://gemini.circumlunar.space/docs/specification.gmi) also has a preformatting tag.


# Unresolved questions

Should we bother to specify blank lines as including whitespace-only lines? Pro: space characters are invisible. If we specify whitespace-only lines as blank lines, then every empty line which looks the same visually acts the same semantically.

# Future possibilities

Separately, we should consider specifying transcluded `.txt` to be rendered as preformatted text.
