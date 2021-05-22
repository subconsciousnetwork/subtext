"""
Example Subtext markup parser
"""
from functools import reduce


def daisychain(iterable, last=None):
    """
    Iterate over an iterable by pairs `((1, 2), (2, 3), ...)`.
    """
    items = iter(iterable)
    n = next(items, None)
    if n is None:
        return
    for n1 in items:
        yield (n, n1)
        n = n1
    yield (n, last)


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
    Func version of splitlines
    """
    return str.splitlines()


def join(strings):
    return "".join(strings)


BLOCK_TYPES = ("heading", "list", "quote", "link", "text", "eof")


def guard_block_type(type):
    if type not in BLOCK_TYPES:
        raise ValueError(f"Unknown type for block: {type}")
    return type


def guard_block_value(value):
    if not isinstance(value, str):
        raise ValueError(
            f"Block value must be instance of str. Given: {type(value)}"
        )
    return value


class Block:
    def __init__(self, type, value):
        self.type = guard_block_type(type)
        self.value = guard_block_value(value)


eof = Block("eof", "")


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
            pass
        else:
            yield Block("text", line)


def blocks_to_markup(blocks):
    """
    Render an iterable of blocks to an iterable of markup lines
    """
    for curr, next in daisychain(blocks, last=eof):
        if curr.type == "heading":
            yield f"# {curr.value}\n\n"
        elif curr.type == "list":
            if next.type == "list":
                yield f"- {curr.value}\n"
            else:
                yield f"- {curr.value}\n\n"
        elif curr.type == "quote":
            yield f"> {curr.value}\n\n"
        elif curr.type == "link":
            yield f"& {curr.value}\n\n"
        else:
            yield f"{curr.value}\n\n"


def blocks_to_plain(blocks):
    """
    Render block content without markup.
    This is a lossy process. You lose block types.
    """
    for block in blocks:
        yield f"{block.value}"


def dict_to_block(block_dict):
    """
    Transform a dict into a Block, raising an exception if dict is malformed.
    """
    return Block(
        block_dict["type"],
        block_dict["value"]
    )


def block_to_dict(block):
    """
    Serialize a block as a dict.
    """
    return {
        "type": block.type,
        "value": block.value
    }


def find_first_text(blocks, default=""):
    """
    Find text of first text block in an iterable of blocks.
    Returns that text, or default, if there are no text blocks.
    """
    for block in blocks:
        if block.type === "text":
            return block.value
    return default


parse = compose(
    markup_to_blocks,
    splitlines
)


render = compose(
    join,
    blocks_to_markup
)


strip = compose(
    join,
    blocks_to_plain,
    markup_to_blocks,
    splitlines
)


excerpt = compose(
    find_first_text,
    markup_to_blocks,
    splitlines
)