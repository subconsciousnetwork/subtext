use crate::{
    block::{self, Block},
    parse,
    primitive::{self, Entity},
};

#[test]
fn empty_space() {
    let input = r#"  

          "#;

    let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

    assert_eq!(blocks.len(), 3);

    match &blocks.as_slice()[..] {
        [block::Block::Blank(first), block::Block::Blank(second), block::Block::Blank(third)] => {
            // assert_eq!(primitive.to_string(), input);
            assert_eq!(first.to_string(), "  ");
            assert_eq!(second.to_string(), "");
            assert_eq!(third.to_string(), "          ");
        }
        _ => panic!("Incorrect block type!"),
    }
}

#[test]
fn basic_slash_links() {
    let input = r#"/foo/bar"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

    assert_eq!(blocks.len(), 1);

    match blocks.first() {
        Some(block::Block::Paragraph(parts)) => {
            assert_eq!(parts.len(), 1);
            assert_eq!(parts.first().unwrap().to_string(), input);
        }
        _ => panic!("Incorrect block or primitive type!"),
    }
}

#[test]
fn basic_headers() {
    let input = r#"# Hello, world!"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

    assert_eq!(blocks.len(), 1);

    match blocks.first() {
        Some(block::Block::Header(primitives)) => {
            assert_eq!(primitives.len(), 3);
            assert_eq!(primitives.first().unwrap().to_string(), "#");
            assert_eq!(primitives.get(1).unwrap().to_string(), " ");
            assert_eq!(primitives.get(2).unwrap().to_string(), "Hello, world!");
        }
        _ => panic!("Incorrect block type!"),
    }
}

#[test]
fn basic_paragraphs() {
    let input = r#"This is a paragraph"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

    assert_eq!(blocks.len(), 1);

    match blocks.first() {
        Some(block::Block::Paragraph(parts)) => {
            assert_eq!(parts.len(), 1);
            assert_eq!(parts.first().unwrap().to_string(), "This is a paragraph");
        }
        _ => panic!("Incorrect block type!"),
    }
}

#[test]
fn basic_hyper_links() {
    let input = r#"http://example.com/foo?bar=baz#zot"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

    assert_eq!(blocks.len(), 1);

    match blocks.first() {
        Some(block::Block::Paragraph(parts)) => {
            assert_eq!(parts.len(), 1);
            assert_eq!(parts.first().unwrap().to_string(), input);
        }
        _ => panic!("Incorrect block type!"),
    }
}

#[test]
fn basic_lists() {
    let input = r#"- One
- Two
- Three"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

    assert_eq!(blocks.len(), 3);

    match blocks.as_slice() {
        [Block::List(one), Block::List(two), Block::List(three)] => {
            match (one.get(2), two.get(2), three.get(2)) {
                (Some(one), Some(two), Some(three)) => {
                    assert_eq!(one.to_string(), "One");
                    assert_eq!(two.to_string(), "Two");
                    assert_eq!(three.to_string(), "Three");
                }
                _ => panic!("Unexpected list items!"),
            }
        }
        _ => panic!("Incorrect set of blocks!"),
    }
}

mod headers {

    mod with_hyperlinks {
        use crate::{
            block::{self, Block},
            parse,
            primitive::{self, Entity},
        };

        #[test]
        fn at_the_beginning() {
            let input = r#"# http://example.com/foo?bar=baz#zot for example"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(Block::Header(primitives)) => {
                    assert_eq!(primitives.len(), 4);

                    match primitives.get(2) {
                        Some(primitive::Entity::HyperLink(value)) => {
                            assert_eq!(value.to_string(), "http://example.com/foo?bar=baz#zot");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                    match primitives.get(3) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), " for example")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }

        #[test]
        fn in_the_middle() {
            let input = r#"# See http://example.com/foo?bar=baz#zot for example"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Header(primitives)) => {
                    assert_eq!(primitives.len(), 5);

                    match primitives.get(2) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), "See ");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };

                    match primitives.get(3) {
                        Some(primitive::Entity::HyperLink(value)) => {
                            assert_eq!(value.to_string(), "http://example.com/foo?bar=baz#zot");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                    match primitives.get(4) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), " for example")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }

        #[test]
        fn at_the_end() {
            let input = r#"# Example link: http://example.com/foo?bar=baz#zot"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Header(primitives)) => {
                    assert_eq!(primitives.len(), 4);

                    match primitives.get(2) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), "Example link: ")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };

                    match primitives.get(3) {
                        Some(primitive::Entity::HyperLink(value)) => {
                            assert_eq!(value.to_string(), "http://example.com/foo?bar=baz#zot");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }
    }
}

mod lists {
    use crate::{block::Block, parse, primitive::Entity};

    #[test]
    fn one_item_is_a_sublink() {
        let input = r#"- One
- /two
- Three"#;

        let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

        assert_eq!(blocks.len(), 3);

        match blocks.as_slice() {
            [Block::List(one), Block::List(two), Block::List(three)] => {
                match (one.get(2), two.get(2), three.get(2)) {
                    (Some(one), Some(two), Some(three)) => {
                        assert_eq!(one.to_string(), "One");
                        assert_eq!(two.to_string(), "/two");
                        assert_eq!(three.to_string(), "Three");
                    }
                    _ => panic!("Unexpected list items!"),
                }
            }
            _ => panic!("Incorrect set of blocks!"),
        }
    }
}

mod paragraphs {
    mod with_slash_links {

        use crate::{
            block::{self, Block},
            parse,
            primitive::{self, Entity},
        };

        #[test]
        fn at_the_beginning() {
            let input = r#"/foo/bar for example"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Paragraph(primitives)) => {
                    assert_eq!(primitives.len(), 2);

                    match primitives.first() {
                        Some(primitive::Entity::SlashLink(value)) => {
                            assert_eq!(value.to_string(), "/foo/bar");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                    match primitives.get(1) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), " for example")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }

        #[test]
        fn in_the_middle() {
            let input = r#"See /foo/bar for example"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Paragraph(primitives)) => {
                    assert_eq!(primitives.len(), 3);

                    match primitives.first() {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), "See ");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };

                    match primitives.get(1) {
                        Some(primitive::Entity::SlashLink(value)) => {
                            assert_eq!(value.to_string(), "/foo/bar");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                    match primitives.get(2) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), " for example")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }

        #[test]
        fn at_the_end() {
            let input = r#"Example link: /foo/bar"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Paragraph(primitives)) => {
                    assert_eq!(primitives.len(), 2);

                    match primitives.first() {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), "Example link: ")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };

                    match primitives.get(1) {
                        Some(primitive::Entity::SlashLink(value)) => {
                            assert_eq!(value.to_string(), "/foo/bar");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }
    }

    mod with_hyper_links {
        use crate::{
            block::{self, Block},
            parse,
            primitive::{self, Entity},
        };

        #[test]
        fn at_the_beginning() {
            let input = r#"http://example.com/foo?bar=baz#zot for example"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Paragraph(primitives)) => {
                    assert_eq!(primitives.len(), 2);

                    match primitives.first() {
                        Some(primitive::Entity::HyperLink(value)) => {
                            assert_eq!(value.to_string(), "http://example.com/foo?bar=baz#zot");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                    match primitives.get(1) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), " for example")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }

        #[test]
        fn in_the_middle() {
            let input = r#"See http://example.com/foo?bar=baz#zot for example"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Paragraph(primitives)) => {
                    assert_eq!(primitives.len(), 3);

                    match primitives.first() {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), "See ");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };

                    match primitives.get(1) {
                        Some(primitive::Entity::HyperLink(value)) => {
                            assert_eq!(value.to_string(), "http://example.com/foo?bar=baz#zot");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                    match primitives.get(2) {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), " for example")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }

        #[test]
        fn at_the_end() {
            let input = r#"Example link: http://example.com/foo?bar=baz#zot"#;

            let blocks: Vec<Block<Entity>> = parse(input.as_bytes()).unwrap().collect();

            assert_eq!(blocks.len(), 1);

            match blocks.first() {
                Some(block::Block::Paragraph(primitives)) => {
                    assert_eq!(primitives.len(), 2);

                    match primitives.first() {
                        Some(primitive::Entity::TextSpan(value)) => {
                            assert_eq!(value.to_string(), "Example link: ")
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };

                    match primitives.get(1) {
                        Some(primitive::Entity::HyperLink(value)) => {
                            assert_eq!(value.to_string(), "http://example.com/foo?bar=baz#zot");
                        }
                        _ => panic!("Incorrect primitive type!"),
                    };
                }
                _ => panic!("Incorrect block type!"),
            }
        }
    }
}

#[test]
fn it_parses_complex_multiline_subtext() {
    let subtext = r#"# Html

It is a /markup language.
Based around the concept of [[Blocks]].

http://www.google.com

 - One
 - /two
 - I bet [[you thought]] I would write three"#;

    let blocks: Vec<Block<Entity>> = parse(subtext.as_bytes()).unwrap().collect();

    assert_eq!(blocks.len(), 10);
}
