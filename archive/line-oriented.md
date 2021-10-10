- Feature: 2021-05-24 Line-oriented
- RFC PR: (fill me in with link to PR)
- Issue: (fill me in with link to issue, if any)

# Summary

Delimit blocks with newlines. Subtext, as currently specified, is line-oriented. The document is composed of a list of logical blocks, and blocks are delimited by a line break. This rfc outlines the pros and cons of this approach, explores alternatives, and how they might interact with other language features.

# Motivation

Subtext is line-oriented as currently specified. Some things to appreciate about that:

- Breaking by line is an elegant balance that is [simple for people and simple for computers](../notes/design.md).
- It is more or what you would do anyway.
- It is impossible to write broken syntax (e.g. no closing tag to forget).
- Each line is a stand-alone block, and does not rely on previous or subsequent lines to determine its block type.
- Parsing is extremely simple. Just a map operation over lines, rather than a fold operation, or something more complex. It's extremely fast and you can write a parser in perhaps a dozen lines of code or less.
- Many other Unix tools operate over lines (think stdin/stdout). That means Subtext gets a whole host of interoperable existing tools for free! `grep`, `cat`, `head`, `tail`, etc. They don't need to understand Subtext markup, only how to work with lines.
- Because each line is a stand-alone block, it is possible to stream Subtext, and begin rendering blocks as soon as you receive them. This is a very desirable property for a networked hypertext format (though not necessarily a deal-breaker if we have to lose it).

Some other small-but-nice properties of line-orientation (but not necessarily deal breakers to lose):

- Editors can rearrange blocks without parsing the text
- You can naively concatenate Subtext files, and get valid Subtext (also happens to be true of blank-line delimited text, but not necessarily other kinds of non-line-oriented markup).

So, my sense is that line-orientation is very desirable, but this is one area where I think we want to carefully consider tradeoffs and experiment with prototypes.

# Guide-level explanation

Explain the proposal as if it was already included in the language and you were teaching it to someone who is new to the language. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how users should *think* about the feature, and how it should impact the way they use Subtext. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing Subtext users and new Subtext users.

# Reference-level explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

# Drawbacks

- 

# Rationale and alternatives

- Why is this design the best in the space of possible designs?
- What are the values embodied by this design. State them literally.
- What are the set of tradeoffs this design chooses? What other, different sets of tradeoffs might be disireable? Why?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?


For example, why not delimit blocks with a blank line?


# Prior art

## Runic

[Runic](https://wiki.xxiivv.com/site/runic.html) is a small line-oriented wiki markup language.

Plus: Runic is simple to understand, and simple to implement. The elegance of the approach is compelling.

Minus: However, Runic's strict adherance to always prefixing every line with a Rune is less practical when writing text by hand.

As much as possible, we want valid Subtext to look like what you might write in a plain text file before you knew about any syntax. That means text blocks should be unprefixed.

## Project Gemini

[Gemini](https://gemini.circumlunar.space/) is an internet protocol which aims to be "heavier than Gopher and lighter than the web." [Gemini features a line-oriented markup language](https://gemini.circumlunar.space/docs/specification.gmi).

I discovered Gemini about a month after roughing out the outline for Subtext. I was excited about the idea of a minimal line-oriented syntax. Gemini gave me further confidence that line-orientated markup could be a solid basis for a hypertext format.

- Like Subtext, Gemini is strictly line-oriented.
- Also like Subtext, Gemini treats unprefixed blocks as text blocks.

## Emacs Org Mode

https://orgmode.org/

## Emacs Outline Mode

https://www.emacswiki.org/emacs/OutlineMode
https://ftp.gnu.org/old-gnu/Manuals/emacs-21.2/html_node/emacs_246.html


Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, tools, and parser proposals: Does this feature exist in other markup languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.

Please also take into consideration that Subtext sometimes intentionally diverges from common language features.

# Unresolved questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

# Future possibilities

- Line indentation

Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.

Also consider how this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.