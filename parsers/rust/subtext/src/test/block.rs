use crate::util::assert_round_trip;

#[test]
fn it_converts_a_list_block_to_bytes() {
    let input = r#"- List item one
- List item two
- List /with_link"#;

    assert_round_trip(input);
}

#[test]
fn it_converts_a_paragraph_block_to_bytes() {
    let input = r#"URLs like https://example.com are automatically linked."#;

    assert_round_trip(input);
}

#[test]
fn it_converts_a_header_block_to_bytes() {
    let input = r#"# This is a header"#;

    assert_round_trip(input);
}

#[test]
fn it_converts_a_slashlink_block_to_bytes() {
    let input = r#"/foo/bar"#;

    assert_round_trip(input);
}

#[test]
fn it_converts_a_hyperlink_block_to_bytes() {
    let input = r#"https://foo.example.com?bar#baz"#;

    assert_round_trip(input);
}

#[test]
fn it_converts_whitespace_to_bytes() {
    let input = r#"
       
  "#;

    assert_round_trip(input);
}
