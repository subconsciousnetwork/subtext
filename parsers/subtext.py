"""
Example Subtext markup parser
"""
from functools import reduce
from collections import namedtuple
from itertools import groupby
from io import StringIO
import re
import html


def id(x):
    """
    The id function.
    """
    return x


def _compose2(b, a):
    """Compose 2 functions"""
    def composed(x):
        """Composed function"""
        return b(a(x))
    return composed


def compose(*funcs):
    """Compose n functions from right to left"""
    return reduce(_compose2, funcs, id)


def splitlines(str):
    """
    Subtext-conforming implementation of splitlines.

    Python's native splitlines splits on a set of Unicode line-endings,
    and so does not match the line-ending behavior specified by Subtext.

    OTOH, Python's file wrapper splits on `\n`, `\r\n`, and `\r`,
    exactly as is specified in Subtext. We use it instead.

    More notes:
    https://github.com/gordonbrander/subtext/blob/main/rfcs/2021-05-24-newlines.md#prior-art
    """
    filewrapper = StringIO(str)
    for line in filewrapper:
        yield line.rstrip("\n\r")


def joinlines(lines):
    """
    Join lines using Unix-style newlines
    """
    return "\n".join(lines)


TextBlock = namedtuple("TextBlock", ("value",))
HeadingBlock = namedtuple("HeadingBlock", ("value",))
ListBlock = namedtuple("ListBlock", ("value",))
QuoteBlock = namedtuple("QuoteBlock", ("value",))
BlankBlock = namedtuple("BlankBlock", tuple())


def _strip_line_markup(line, sigil):
    """Strip sigil and whitespace from a line of markup"""
    chars = f"{sigil} "
    return line.lstrip(chars)


def markup_to_blocks(lines):
    """
    Parse lines in a file-like iterator, yielding blocks.
    """
    for line in lines:
        if line.startswith("#"):
            yield HeadingBlock(_strip_line_markup(line, "#"))
        elif line.startswith("-"):
            yield ListBlock(_strip_line_markup(line, "-"))
        elif line.startswith(">"):
            yield QuoteBlock(_strip_line_markup(line, ">"))
        elif line == "":
            yield BlankBlock()
        else:
            yield TextBlock(line)


def blocks_to_markup(blocks):
    """
    Render an iterable of blocks to an iterable of markup lines
    """
    for block in blocks:
        block_type = type(block)
        if block_type is HeadingBlock:
            yield f"# {block.value}"
        elif block_type == ListBlock:
            yield f"- {block.value}"
        elif block_type == QuoteBlock:
            yield f"> {block.value}"
        elif block_type == BlankBlock:
            yield ""
        else:
            yield f"{block.value}"


def blocks_to_plain(blocks):
    """
    Render block content without markup.
    This is a lossy process. You lose block types.
    """
    for block in blocks:
        yield f"{block.value}"


_SLASHLINK = re.compile(
    "(^|\s)(/[a-zA-Z0-9/\-\_]+)"
)

_BARELINK = re.compile(
    "(^|\s)(https?://[^\s>]+)[\.,;]?"
)

_BRACKETLINK = re.compile(
    "<([^>\s]+)>"
)


def _repl_barelink(matchobj):
    url = matchobj.group(2)
    text = html.escape(url)
    return f'<a href="{url}">{text}</a>'


def _repl_bracketlink(matchobj):
    url = matchobj.group(1)
    text = html.escape(url)
    return f'<a href="{url}">{text}</a>'


def render_inline_html(line, slashlink):
    def _repl_slashlink(matchobj):
        return slashlink(matchobj.group(2))
    line = _SLASHLINK_RE.sub(_repl_slashlink, line)
    line = _BARELINK.sub(_repl_barelink, link)
    line = _BARELINK.sub(_repl_bracketlink, link)


BlockGroup = namedtuple("BlockGroup", ("type", "value"))


def group_blocks(blocks):
    """
    Group contiguous blocks by type.
    This may be useful if you want to move or manipulate contiguous
    ranges of blocks together, such as a series of list items.
    """
    for block_type, block_group in groupby(blocks, type):
        yield BlockGroup(block_type, tuple(block_group))


def find_first_text(blocks, default=""):
    """
    Find text of first text block in an iterable of blocks.
    Returns that text, or default, if there are no text blocks.
    """
    for block in blocks:
        if block.type == "text":
            return block.value
    return default


parse = compose(
    markup_to_blocks,
    splitlines
)


markup = compose(
    joinlines,
    blocks_to_markup
)


strip = compose(
    joinlines,
    blocks_to_plain,
    parse
)


excerpt = compose(
    find_first_text,
    parse
)
