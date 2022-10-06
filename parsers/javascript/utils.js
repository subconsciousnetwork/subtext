import CharIterator from "./CharIterator.js";

const BlockType = {
  HEADER: "header",
  BLANK: "blank",
  PARAGRAPH: "paragraph",
  LIST: "list",
  LINK: "link",
  QUOTE: "quote",
}


const TokenType = {
  SIGIL: "sigil",
  EMPTY_SPACE: "empty-space",
  TEXT_SPAN: "text-span",
  HYPERLINK: "hyperlink",
  SLASHLINK: "slashlink",
}


const isWhiteSpace = (string) => {
  return string.trim().length === 0;
};


const parseText = (text) => {
  const spans = [];
  const iterator = new CharIterator(text);
  let currentSpanType;
  let currentSpanText = "";

  while (true) {
    const step = iterator.next();
    if (step.done) {
      spans.push({
        type: currentSpanType,
        content: currentSpanText,
      });
      break;
    }
    const char = step.value;
    const lastChar = iterator.peekBack();

    if (typeof lastChar !== "string" || isWhiteSpace(lastChar)) {
      if (
        char === "h"
        && (
          iterator.peek(5).join("") === "ttp:/"
          || iterator.peek(6).join("") === "ttps:/"
        )
      ) {
        currentSpanType && spans.push({
          type: currentSpanType,
          content: currentSpanText,
        });
  
        currentSpanText = "";
        currentSpanType = TokenType.HYPERLINK;
      } else if (
        char === "/"
      ) {
        currentSpanType && spans.push({
          type: currentSpanType,
          content: currentSpanText,
        });
  
        currentSpanText = "";
        currentSpanType = TokenType.SLASHLINK;
      } else {
        currentSpanType = TokenType.TEXT_SPAN;
      }
    }

    if (
      isWhiteSpace(char) && currentSpanType !== TokenType.TEXT_SPAN
    ) {
      currentSpanType && spans.push({
        type: currentSpanType,
        content: currentSpanText,
      });
      currentSpanText = "";
      currentSpanType = TokenType.TEXT_SPAN;
    }

    currentSpanText += char;
  }

  return spans;
};


export const toString = (object) => { console.log(object)
  if (typeof object.content === "string") {
    return object.content;
  }
  if (Array.isArray(object)) {
    return object.reduce((a, b) => a + toString(b), "");
  }
  return toString(object.content);
}


export const blocksToString = (blocks) => {
  return blocks.map(toString).join("\n");
}


const parseExtraSpace = (line, content) => {
  return line.substring(0, line.indexOf(toString(content)));
}


const parseSigilPrefixedLine = (line, blockType) => {
  const block = {
    type: blockType,
    content: [
      {
        type: TokenType.SIGIL,
        content: line.substring(0, 1),
      },
    ]
  };

  const content = parseText(line.substring(1).trimStart());
  const emptySpace = parseExtraSpace(line.substring(1), content);

  if (emptySpace.length > 0) {
    block.content.push({
      type: TokenType.EMPTY_SPACE,
      content: emptySpace,
    })
  }

  if (toString(content).length > 0) {
    block.content.push(...content);
  }

  return block;
}


const parseBlockWithoutSigil = (line) => {
  if (line.trim().length === 0) {
    return {
      type: BlockType.BLANK,
      content: {
        type: TokenType.EMPTY_SPACE,
        content: line,
      },
    };
  } else {
    const block = {
      type: BlockType.PARAGRAPH,
      content: [],
    };
  
    const content = parseText(line.trimStart());
    const emptySpace = parseExtraSpace(line, content);
  
    if (emptySpace.length > 0) {
      block.content.push({
        type: TokenType.EMPTY_SPACE,
        content: emptySpace,
      })
    }
  
    if (toString(content).length > 0) {
      block.content.push(...content);
    }

    return block;
  }
}


export const parseLine = (line) => {
  if (line.startsWith("#")) {
    return parseSigilPrefixedLine(line, BlockType.HEADER);
  } else if (line.startsWith("-")) {
    return parseSigilPrefixedLine(line, BlockType.LIST);
  } else if (line.startsWith(">")) {
    return parseSigilPrefixedLine(line, BlockType.QUOTE);
  } else {
    return parseBlockWithoutSigil(line);
  }
};
