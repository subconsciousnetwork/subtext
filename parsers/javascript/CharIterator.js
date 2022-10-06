export default class CharIterator {
  #chars;
  #index;

  constructor(input) {
    this.#chars = Array.from(input);
    this.#index = -1;
  }

  next() {
    this.#index++;
    const done = this.#index === this.#chars.length;
    return done ? {
      done,
      value: null,
    } : {
      done,
      value: this.#chars[this.#index],
    };
  }

  peek(numberOfChars) {
    return this.#chars.slice(this.#index + 1, this.#index + 1 + numberOfChars);
  }

  peekBack() {
    return this.#chars[this.#index - 1];
  }
}
