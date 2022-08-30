# Design principles

## Subtext principles

These principles guide Subtext's design, and should frame our design solution:

### Line-oriented

- Subtext represents block-oriented documents as line-oriented markup.
- One line = one block
- Documents parse to a flat list of blocks

This keeps parsing simple, and enables easy streaming parsing.

### What you would write anyway

As much as possible, we should aspire to choose syntax that is what you might write anyway.

The best syntax is no syntax. The second-best syntax is what you would write anyway. The third best is familiar syntax.

We try to avoid syntax that looks like "code". This is a delicate design goal that must be navigated through intuition and carefully balanced tradeoffs. Examples of syntax like this are headers in email/HTTP, or dashes for list items.

### Simple to parse

We try to keep Subtext simple for both people and computers. That means intuitive syntax. It also means simple implementation.

- Prefer context-free and regular grammars, such as balanced brackets.
- Prefer forms that can be easily implemented through regular expressions.
- Aim for minimal backtracking.

Keeping the implementation simple for computers means it will be easier to write and use Subtext implementations in many different languages and contexts.

This principle is sometimes in tension with "what you would write anyway". In case of a conflict:

1. First try to resolve the conflict by avoiding it (don't add the feature).
2. Then try to resolve the conflict by reframign the problem (rethink the feature).
3. Finally, favor people over computers.

Ultimately, we must navigate through intution and taste.

## General design principles

### Small alphabet, wide expression

We want to choose the smallest set of primitives with the widest range of expression.

Remember: DNA has just four basepairs.

### Keep it simple

Do the simplest thing that could possibly work.

> You cannot get a simple system by adding simplicity to a complex system. <http://erlang.org/pipermail/erlang-questions/2012-March/065087.html>

And:

> A complex system that works is invariably found to have evolved from a simple system that worked. A complex system designed from scratch never works and cannot be patched up to make it work. You have to start over with a working simple system. (Gall's Law)

So, keep it simple.

### YAGNI

You Ain't Gonna Need It. A principle from Extreme Programming. XP co-founder Ron Jeffries has written: "Always implement things when you actually need them, never when you just foresee that you need them.".