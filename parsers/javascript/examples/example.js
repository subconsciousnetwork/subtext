/*
  Notice:
  This example must be run with Node.js as it uses the Node.js API to open
  the example source file. The main parser however is runtime-agnostic.
*/

import fs from "node:fs";
import * as path from "node:path";
import * as url from "node:url";
import { Readable } from 'node:stream';
import subtextStreamingParser from "../index.js";

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
const nodeReadable = fs.createReadStream(
  path.join(__dirname, "example.subtext"),
  {
    encoding: "utf-8",
  }
);
const readableStream = Readable.toWeb(nodeReadable);
const streamOfParsedData = readableStream.pipeThrough(
  subtextStreamingParser(),
);

// Let's collect the incoming blocks from the stream
const blocks = [];
for await (const block of streamOfParsedData) {
  blocks.push(block);
}

console.log("Parsed blocks:");
console.log(JSON.stringify(blocks, null, "  "));