use crate::{block::Block, parse, primitive::Entity};

#[test]
fn it_skips_leading_whitespace_in_paragraphs() {
    let input = "  Hello, world!";
    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();
    let text_content = blocks.first().unwrap().to_text_content();

    assert_eq!(text_content, "Hello, world!");
}

#[test]
fn it_skips_the_sigil_and_leading_whitespace_for_headers() {
    let input = "# Hello, world!";
    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();
    let text_content = blocks.first().unwrap().to_text_content();

    assert_eq!(text_content, "Hello, world!");
}

#[test]
fn it_skips_the_sigil_and_leading_whitespace_for_lists() {
    let input = "- Hello, world!";
    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();
    let text_content = blocks.first().unwrap().to_text_content();

    assert_eq!(text_content, "Hello, world!");
}

#[test]
fn it_skips_the_sigil_and_leading_whitespace_for_quotes() {
    let input = "> Hello, world!";
    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();
    let text_content = blocks.first().unwrap().to_text_content();

    assert_eq!(text_content, "Hello, world!");
}

#[test]
fn it_yields_an_empty_string_for_blanks() {
    let input = "   ";
    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();
    let text_content = blocks.first().unwrap().to_text_content();

    assert_eq!(text_content, "");
}
