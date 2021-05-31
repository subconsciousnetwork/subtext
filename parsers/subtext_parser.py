"""
Example Subtext markup parser
"""
from functools import reduce
from collections import namedtuple
from itertools import groupby
from io import StringIO


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


Block = namedtuple("Block", ("type", "value"))


def _strip_markup_line(line, sigil):
    """Strip sigil and whitespace from a line of markup"""
    return line.lstrip(sigil).rstrip()


def markup_to_blocks(lines):
    """
    Parse lines in a file-like iterator, yielding blocks.
    """
    for line in lines:
        if line.startswith("# "):
            yield Block("heading", _strip_markup_line(line, "# "))
        elif line.startswith("- "):
            yield Block("list", _strip_markup_line(line, "- "))
        elif line.startswith("> "):
            yield Block("quote", _strip_markup_line(line, "> "))
        elif line.startswith("& "):
            yield Block("link", _strip_markup_line(line, "& "))
        elif line.strip() == "":
            yield Block("blank", "")
        else:
            yield Block("text", line)


def blocks_to_markup(blocks):
    """
    Render an iterable of blocks to an iterable of markup lines
    """
    for block in blocks:
        if block.type == "heading":
            yield f"# {block.value}"
        elif block.type == "list":
            yield f"- {block.value}"
        elif block.type == "quote":
            yield f"> {block.value}"
        elif block.type == "link":
            yield f"& {block.value}"
        elif block.type == "blank":
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


BlockGroup = namedtuple("BlockGroup", ("type", "value"))


def _get_block_type(block):
    return block.type


def group_blocks(blocks):
    """
    Group contiguous blocks by type.
    This may be useful if you want to move or manipulate contiguous
    ranges of blocks together, such as a series of list items.
    """
    for block_type, block_group in groupby(blocks, _get_block_type):
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


render = compose(
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