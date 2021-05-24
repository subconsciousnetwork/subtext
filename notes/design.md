# Design principles

Some of the design ethos behind Subtext.

## Small alphabet, wide expression

We want to choose the smallest set of primitives with the widest range of expression.

Remember: DNA has just four basepairs.

## Keep it simple

Do the simplest thing that could possibly work.

> You cannot get a simple system by adding simplicity to a complex system. <http://erlang.org/pipermail/erlang-questions/2012-March/065087.html>

And:

> A complex system that works is invariably found to have evolved from a simple system that worked. A complex system designed from scratch never works and cannot be patched up to make it work. You have to start over with a working simple system. (Gall's Law)

So, keep it simple.

## Simple for people, simple for computers

Try to keep it simple for both people and computers. That means intuitive syntax. It also means simple implementation.

- Keeping the markup intuitive makes it easy for people to learn and use Subtext.
- Keeping the implementation simple for computers means it will be easier to write and use Subtext implementations in many different languages and contexts.

In case of a conflict:

1. First try to resolve the conflict by avoiding it (don't add the feature).
2. Then try to resolve the conflict by reframign the problem (rethink the feature).
3. Finally, favor people over computers.

## YAGNI

You Ain't Gonna Need It. A principle from Extreme Programming. XP co-founder Ron Jeffries has written: "Always implement things when you actually need them, never when you just foresee that you need them.".