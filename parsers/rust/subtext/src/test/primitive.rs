use tendril::StrTendril;

use crate::primitive::{parse_slash_link, parse_text, parse_wiki_link, Entity};

#[test]
fn it_parses_a_slash_link_blap() {
    let input = StrTendril::try_from_byte_slice(b"/foo").unwrap();
    let (entity, steps) = parse_slash_link(input).unwrap();

    assert_eq!(steps, 4);
    assert_eq!(entity.to_string(), "/foo");
}

#[test]
fn it_parses_a_wiki_link() {
    let input = StrTendril::try_from_byte_slice(b"[[foo bar baz]]").unwrap();
    let (entity, steps) = parse_wiki_link(input).unwrap();

    assert_eq!(steps, 15);
    assert_eq!(entity.to_string(), "[[foo bar baz]]");
}

#[test]
fn it_parses_a_text_span() {
    let input = StrTendril::try_from_byte_slice(b"foo bar baz").unwrap();
    let (entities, steps) = parse_text::<Entity>(input).unwrap();

    assert_eq!(steps, 11);
    assert_eq!(entities.len(), 1);
    assert_eq!(entities.get(0).unwrap().to_string(), "foo bar baz");
}

#[test]
fn it_parses_a_text_span_delimited_by_a_newline() {
    let input = StrTendril::try_from_byte_slice(b"foo bar baz\n").unwrap();
    let (entities, steps) = parse_text::<Entity>(input).unwrap();

    assert_eq!(steps, 11);
    assert_eq!(entities.len(), 1);
    assert_eq!(entities.get(0).unwrap().to_string(), "foo bar baz");
}

#[test]
fn it_parses_a_slash_link_in_a_text_span() {
    let input = StrTendril::try_from_byte_slice(b"foo /bar baz").unwrap();
    let (entities, steps) = parse_text::<Entity>(input).unwrap();

    assert_eq!(steps, 12);
    assert_eq!(entities.len(), 3);
    assert_eq!(entities.get(0).unwrap().to_string(), "foo ");
    assert_eq!(entities.get(1).unwrap().to_string(), "/bar");
    assert_eq!(entities.get(2).unwrap().to_string(), " baz");
}

#[test]
fn it_parses_a_wiki_link_in_a_text_span() {
    let input = StrTendril::try_from_byte_slice(b"foo [[bar]] baz").unwrap();
    let (entities, steps) = parse_text::<Entity>(input).unwrap();

    assert_eq!(steps, 15);
    assert_eq!(entities.len(), 3);
    assert_eq!(entities.get(0).unwrap().to_string(), "foo ");
    assert_eq!(entities.get(1).unwrap().to_string(), "[[bar]]");
    assert_eq!(entities.get(2).unwrap().to_string(), " baz");
}

#[test]
fn it_parses_a_slash_link_following_a_text_span() {
    let input = StrTendril::try_from_byte_slice(b"foo bar /baz").unwrap();
    let (entities, steps) = parse_text::<Entity>(input).unwrap();

    assert_eq!(steps, 12);
    assert_eq!(entities.len(), 2);
    assert_eq!(entities.get(0).unwrap().to_string(), "foo bar ");
    assert_eq!(entities.get(1).unwrap().to_string(), "/baz");
}
