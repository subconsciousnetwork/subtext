# Subtext FAQ

About Subtext:

- [README](README.md)
- [Guide](guide.md)
- [Specification](specification.md)

## What about deep linking?

This was a considered design choice! My sense is that it's best to nudge each linkable thing into its own flat file. Big things can be composed by linking to small things. If we really need deep links into one file, we could do something like [text fragment linking](https://wicg.github.io/scroll-to-text-fragment/) or [Xanadu EDL](https://xanadu.com/xuEDL.html).

See [explorations/deep-links.md](explorations/deep-links.md) for more.

## What about bold, italic, and other inline markup?

Maybe! I asked myself this same question, and came away with 3 provocations to myself:

1. [YAGNI](notes/design.md).
2. It's markup for note taking.
3. I never seem to miss bold/italic when I'm taking notes by hand.

Introducing inline markup is imaginable, and yet, we would prefer to start with the simplest thing that could possibly work. The design goal being the smallest amount of structural markup that would help you and your computer think more meaningfully with text.

One other way to see Subtext is as a hypertext glue layer. If you really need complex text formatting, you can always transclude a Markdown/LaTex file. Not every client may support rendering every format in-place, but covering handful of common ones makes sense!

See [explorations/inline.md](explorations/inline.md) for more.

## What about nested outlines?

I really appreciate Ted Nelson's YAGNI attitude in Nelson, 1965. Basically “it could be a tree but a list is just fine.”

When it's flat, you just concat. And trees are still there... as links! In the editor this could be represented as nested lists, even if the result is discrete flat files.

## What about tags?

Hashtags perhaps? On the other hand, a tag is just a link to a page that doesn’t exist yet. Backlinks get you all of the practical features of tags.

See [explorations/hashtags.md](explorations/hashtags.md), [explorations/wiki-link.md](explorations/2021-05-28-multilinks.md) and [explorations/2021-05-28-multilinks.md](explorations/2021-05-28-multilinks.md) for more.

## Why line-oriented?

For example, why not delimit blocks with a blank line?

Subtext was partially sparked by discovering [Runic](https://wiki.xxiivv.com/site/runic.html) some years ago. This clever little language made me fall in love with the idea of line-oriented markup. Discovering [Project Gemini](https://gemini.circumlunar.space/docs/specification.gmi) was further confirmation that line-oriented markup could be a solid basis for hypertext.

One reason to appreciate line-orientation is that it makes parsing extremely simple. Breaking by line is an elegant balance that is [simple for people and simple for computers](notes/design.md).

Each line is a stand-alone block, and does not rely on previous or subsequent lines to determine its block type. Parsing is a simple map operation, rather than a fold operation. It's extremely fast.

Many other Unix tools operate over lines (think stdin/stdout). That means Subtext gets a whole host of interoperable existing tools for free! `grep`, `cat`, `head`, `tail`, etc. They don't need to understand Subtext markup, only how to work with lines.

Because each line is a stand-alone block, it is possible to stream Subtext, and begin rendering blocks as soon as you receive them. This is a very desirable property for a networked hypertext format (though not necessarily a deal-breaker if we have to lose it).

Some other small-but-nice properties of line-orientation (but not necessarily deal breakers to lose):

- Editors can rearrange blocks without parsing the text
- You can naively concatenate Subtext files, and get valid Subtext (also happens to be true of blank-line delimited text, but not necessarily other kinds of non-line-oriented markup).

So, my sense is that line-orientation is very desirable, but this is one area where we are doing some practical experimentation.