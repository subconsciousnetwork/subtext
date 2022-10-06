import assert from 'node:assert';
import { it } from 'node:test';
import { parseAtOnce } from "../index.js";

it("skips leading whitespace in paragraphs", () => {
  const input = `  Hello, world!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          content: "  ",
          type: "empty-space"
        },
        {
          content: "Hello, world!",
          type: "text-span"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("skips and leading whitespace for headers", () => {
  const input = `# Hello, world!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "header",
      content: [
        {
          content: "#",
          type: "sigil"
        },
        {
          content: " ",
          type: "empty-space"
        },
        {
          content: "Hello, world!",
          type: "text-span"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("skips and leading whitespace for lists", () => {
  const input = `- Hello, world!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "list",
      content: [
        {
          content: "-",
          type: "sigil"
        },
        {
          content: " ",
          type: "empty-space"
        },
        {
          content: "Hello, world!",
          type: "text-span"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("skips and leading whitespace for quotes", () => {
  const input = `> Hello, world!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "quote",
      content: [
        {
          content: ">",
          type: "sigil"
        },
        {
          content: " ",
          type: "empty-space"
        },
        {
          content: "Hello, world!",
          type: "text-span"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("yields an empty string for blanks", () => {
  const input = `   `;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "blank",
      content: {
        content: "   ",
        type: "empty-space"
      },
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});