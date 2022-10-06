import assert from 'node:assert';
import { it } from 'node:test';
import { parseAtOnce } from "../index.js";

it("dissolves a terminating newline", () => {
  const input = `Hello,
World!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          type: "text-span",
          content: "Hello,"
        },
      ],
    },
    {
      type: "paragraph",
      content: [
        {
          type: "text-span",
          content: "World!"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("captures extra empty space in a blank", () => {
  const input = `Hello,
  
World!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          type: "text-span",
          content: "Hello,"
        },
      ],
    },
    {
      type: "blank",
      content: {
        content: "  ",
        type: "empty-space",
      },
    },
    {
      type: "paragraph",
      content: [
        {
          type: "text-span",
          content: "World!"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});

it("recognizes zero length lines as blanks", () => {
  const input = `Hello,
  

     
World!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          type: "text-span",
          content: "Hello,"
        },
      ],
    },
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
        content: "     ",
        type: "empty-space",
      },
    },
    {
      type: "paragraph",
      content: [
        {
          type: "text-span",
          content: "World!"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});


it("does not absorb leading whitespace into a preceding blank", () => {
  const input = `Hello,
  
 - World!`;

  const actualOutput = parseAtOnce(input);
  const expectedOutput = [
    {
      type: "paragraph",
      content: [
        {
          type: "text-span",
          content: "Hello,"
        },
      ],
    },
    {
      type: "blank",
      content: {
        content: "  ",
        type: "empty-space",
      },
    },
    {
      type: "paragraph",
      content: [
        {
          type: "empty-space",
          content: " "
        },
        {
          type: "text-span",
          content: "- World!"
        },
      ],
    },
  ];

  assert.deepEqual(actualOutput, expectedOutput);
});