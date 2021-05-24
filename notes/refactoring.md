# Hypertext refactoring

If you've ever maintained a Wiki, or another long-term body of hypertext, you've probably noticed a behavior pattern emerge that I call **refactoring**.

Hypertext _refactoring_ looks like:

- **Factoring out**: Parts of pages often start in one place, and end up split out into a separate page, and linked to from the original page.
- **Merging**: Other times, a piece of one page, or even an entire page, will be merged with some other existing page.

In addition to this large page level _refactoring_, there are also smaller-scale text-level changes:

- **Rearranging**: moving around paragraph-scale blocks, or items in a list.
- **Adding**: often appending a bit of information to the end of a document, or the end of a list.
- **Removing**: removing a bit of text that is no longer useful.
- **Rewriting** This one requires careful creative judgement. It's the Synthesize step in [Capture, Organize, Synthesize](https://subconscious.substack.com/p/unconscious-r-and-d) and can include re-organizing, adding, and removing as one big holistic action.

Hypertext is refactored constantly. Ideas get reframed. Perhaps the "zoom level" was wrong, or the concept misunderstood. Perhaps the page started in [Thread Mode](https://wiki.c2.com/?ThreadMode) and is being rewritten in [Pattern Mode](https://wiki.c2.com/?PatternMode).

Refactoring is an organic process, and a feature, not a bug, of hypertext. Hypertext wants to be refactored. Through this constent refactoring, knowledge finds the right "packets" for a given domain.

If hypertext wants to be refactored, are there ways we can [augment](https://subconscious.substack.com/p/the-knowledge-ecology) refactoring with software? I think so.

To do so, we need to make hypertext [legible](https://www.ribbonfarm.com/2010/07/26/a-big-little-idea-called-legibility/) to software. That means giving it some degree of structural meaning so that software can make sense of it. To think about what that might mean, I'd like to consider two extremes:

**An ordinary plain text file** is more or less a single blob. Without any formal structure, it is difficult for a computer to understand what can be re-arranged, what can be factored out. However, text is flat, so we can probably usefully merge two text files by concatenating them.

**HTML, or other publishing formats** are deep trees of nodes. The nodes describe structural relationships, and formatting properties. Often these nodes can be infinitely nested. These formats bring structure to the document. In theory, this should make them meaningful to a computer. In practice, since the structure is arbitrarily complex, it is difficult to meaninfully manipulate the document without foreknowledge of the human meanings given to the particular document structure. How do you merge two HTML documents? Without understanding the particulars of the document, it is unclear. The result is often tag soup. This is also why [web scraping](https://en.wikipedia.org/wiki/Web_scraping) is brittle in practice.

**Between these two extremes** could there be a format that is simple for people to understand, simple for computers to manipulate, and meaningful for both?

**This is the goal of [Subtext](https://github.com/gordonbrander/subtext)** — not to offer visual formatting, but to find the smallest surface-area of document structure, with the highest range of meaningful creative expression for both people and computers. Subtext attempts to resolve this dilemma by radically simplifying it.

Subtext is made of blocks. Blocks are roughly analogous to a paragraph, so 1 block = 1 idea. This breaks text apart into [idea legos](https://subconscious.substack.com/p/thought-legos) that can be easily composed and rearranged.

Subtext is flat. Paradoxically, by limiting the format to a flat list of blocks, we expand what software can usefully do with it. The set of document structures is narrower, so there are fewer ways to fail. For example:

- Concatenation is a "good enough" merging strategy for flat documents.
- Because prose is linear, software can assume a [BLUF](https://en.wikipedia.org/wiki/BLUF_(communication)) writing style, and take the first few blocks to get a useful excerpt.

With Subtext, both people and software should be able to easily:

- **Merge** documents
- **Factor out** a block in a document into a new document
- **Rearrange** blocks in a document
- **Filter** a document down to only those blocks meeting a particular condition
- **Combine** parts of old documents to create new documents