import assert from 'node:assert';
import { it } from 'node:test';
import { parseAtOnce } from "../index.js";
import { blocksToString } from '../utils.js';

const assertRoundTrip = (input) => {
  const blocks = parseAtOnce(input); console.log(blocks)
  const stringFromBlocks = blocksToString(blocks);
  assert.strictEqual(input, stringFromBlocks);
}

it("coverts a list block to bytes", () => {
  const input = `- List item one
- List item two
- List /with_link`;

  assertRoundTrip(input);
});

it("coverts a paragraph block to bytes", () => {
  const input = `URLs like https://example.com are automatically linked.`;

  assertRoundTrip(input);
});

it("coverts a header block to bytes", () => {
  const input = "# This is a header";

  assertRoundTrip(input);
});

it("coverts a slashlink block to bytes", () => {
  const input = "/foo/bar";

  assertRoundTrip(input);
});

it("coverts a hyperlink block to bytes", () => {
  const input = "https://foo.example.com?bar#baz";

  assertRoundTrip(input);
});

it("coverts whitespace to bytes", () => {
  const input = `
       
  `;

  assertRoundTrip(input);
});
