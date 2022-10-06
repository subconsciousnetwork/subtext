import assert from 'node:assert';
import { it } from 'node:test';
import { parseAtOnce } from "../index.js";

it("parses empty space", () => {
  const input = `  

          `;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "blank",
      content: {
        content: "  ",
        type: "empty-space",
      },
    },
    {
      type: "blank",
      content: {
        content: "",
        type: "empty-space",
      },
    },
    {
      type: "blank",
      content: {
        content: "          ",
        type: "empty-space",
      },
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("parses basic slash links", () => {
  const input = `/foo/bar`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "/foo/bar",
          type: "slashlink",
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("parses basic headers", () => {
  const input = `# Hello, world!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "header",
      content: [
        {
          content: "#",
          type: "sigil",
        },
        {
          content: " ",
          type: "empty-space"
        },
        {
          content: "Hello, world!",
          type: "text-span"
        }
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("parses basic paragraphs", () => {
  const input = `This is a paragraph`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "This is a paragraph",
          type: "text-span"
        }
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("parses basic hyperlinks", () => {
  const input = `http://example.com/foo?bar=baz#zot`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "http://example.com/foo?bar=baz#zot",
          type: "hyperlink"
        }
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("parses basic lists", () => {
  const input = `- One
- Two
- Three`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      "type": "list",
      "content": [
        {
          "type": "sigil",
          "content": "-"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "text-span",
          "content": "One"
        }
      ]
    },
    {
      "type": "list",
      "content": [
        {
          "type": "sigil",
          "content": "-"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "text-span",
          "content": "Two"
        }
      ]
    },
    {
      "type": "list",
      "content": [
        {
          "type": "sigil",
          "content": "-"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "text-span",
          "content": "Three"
        }
      ]
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("headers with hyperlinks at the beginning", () => {
  const input = `# http://example.com/foo?bar=baz#zot for example`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      "type": "header",
      "content": [
        {
          "type": "sigil",
          "content": "#"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "hyperlink",
          "content": "http://example.com/foo?bar=baz#zot"
        },
        {
          "type": "text-span",
          "content": " for example"
        },
      ]
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("headers with hyperlinks in the middle", () => {
  const input = `# See http://example.com/foo?bar=baz#zot for example`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      "type": "header",
      "content": [
        {
          "type": "sigil",
          "content": "#"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "text-span",
          "content": "See "
        },
        {
          "type": "hyperlink",
          "content": "http://example.com/foo?bar=baz#zot"
        },
        {
          "type": "text-span",
          "content": " for example"
        },
      ]
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("headers with hyperlinks at the end", () => {
  const input = `# Example link: http://example.com/foo?bar=baz#zot`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      "type": "header",
      "content": [
        {
          "type": "sigil",
          "content": "#"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "text-span",
          "content": "Example link: "
        },
        {
          "type": "hyperlink",
          "content": "http://example.com/foo?bar=baz#zot"
        },
      ]
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("list: one item is a sublink", () => {
  const input = `- One
- /two
- Three`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      "type": "list",
      "content": [
        {
          "type": "sigil",
          "content": "-"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "text-span",
          "content": "One"
        },
      ],
    },
    {
      "type": "list",
      "content": [
        {
          "type": "sigil",
          "content": "-"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "slashlink",
          "content": "/two"
        },
      ],
    },
    {
      "type": "list",
      "content": [
        {
          "type": "sigil",
          "content": "-"
        },
        {
          "type": "empty-space",
          "content": " "
        },
        {
          "type": "text-span",
          "content": "Three"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("paragraph with slashlink at the beginning", () => {
  const input = `/foo/bar for example`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "/foo/bar",
          type: "slashlink"
        },
        {
          content: " for example",
          type: "text-span"
        }
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("paragraph with slashlink in the middle", () => {
  const input = `See /foo/bar for example`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "See ",
          type: "text-span"
        },
        {
          content: "/foo/bar",
          type: "slashlink"
        },
        {
          content: " for example",
          type: "text-span"
        }
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("paragraph with slashlink at the end", () => {
  const input = `Example link: /foo/bar`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "Example link: ",
          type: "text-span"
        },
        {
          content: "/foo/bar",
          type: "slashlink"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("paragraph with hyperlink at the beginning", () => {
  const input = `http://example.com/foo?bar=baz#zot for example`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "http://example.com/foo?bar=baz#zot",
          type: "hyperlink"
        },
        {
          content: " for example",
          type: "text-span"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("paragraph with hyperlink in the middle", () => {
  const input = `See http://example.com/foo?bar=baz#zot for example`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "See ",
          type: "text-span"
        },
        {
          content: "http://example.com/foo?bar=baz#zot",
          type: "hyperlink"
        },
        {
          content: " for example",
          type: "text-span"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("paragraph with hyperlink at the end", () => {
  const input = `Example link: http://example.com/foo?bar=baz#zot`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "Example link: ",
          type: "text-span"
        },
        {
          content: "http://example.com/foo?bar=baz#zot",
          type: "hyperlink"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("parses complex multiline subtext", () => {
  const input = `# Html

It is a /markup language.
Based around the concept of [[Blocks]].

http://www.google.com

 - One
 - /two
 - I bet [[you thought]] I would write three`;

  const blocks = parseAtOnce(input);

  assert.strictEqual(blocks.length, 10);
});