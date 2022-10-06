import { parseLine } from "./utils.js";

export const parseAtOnce = (input) => {
  return input.split("\n").map(parseLine);
}

export default () => {
  let buffer = "";

  const parseLinesFromBuffer = (controller) => {
    while (buffer.includes("\n")) {
      const indexOfNewLine = buffer.indexOf("\n");
      const line = buffer.substring(0, indexOfNewLine);
      const parsedBlock = parseLine(line);
      controller.enqueue(parsedBlock);
      buffer = buffer.substring(indexOfNewLine + 1);
    }
  }

  return new TransformStream({
    transform(chunk, controller) {
      buffer += chunk;
      parseLinesFromBuffer(controller);
    },
    flush(controller) {
      parseLinesFromBuffer(controller);
      if (buffer.length > 0) {
        const parsedBlock = parseLine(buffer);
        controller.enqueue(parsedBlock);
        buffer = "";
      }
    }
  });
}