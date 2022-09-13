use crate::{block::Block, parse, primitive::Entity};

#[test]
fn it_dissolves_a_terminating_newline() {
    let input = r#"Hello,
World!"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();

    match blocks.as_slice() {
        [Block::Paragraph(hello), Block::Paragraph(world)] => {
            assert_eq!(hello.first().unwrap().to_string(), "Hello,");
            assert_eq!(world.first().unwrap().to_string(), "World!");
        }
        _ => panic!("Unexpected block(s) or primitive(s): {:#?}", blocks),
    }
}

#[test]
fn it_captures_extra_empty_space_in_a_blank() {
    let input = r#"Hello,
  
World!"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();

    match blocks.as_slice() {
        [Block::Paragraph(hello), Block::Blank(empty), Block::Paragraph(world)] => {
            assert_eq!(hello.first().unwrap().to_string(), "Hello,");
            assert_eq!(empty.to_string(), "  ");
            assert_eq!(world.first().unwrap().to_string(), "World!");
        }
        _ => panic!("Unexpected block(s) or primitive(s): {:#?}", blocks),
    }
}

#[test]
fn it_recognizes_zero_length_lines_as_blanks() {
    let input = r#"Hello,
  

     
World!"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();

    match blocks.as_slice() {
        [Block::Paragraph(hello), Block::Blank(empty_one), Block::Blank(empty_two), Block::Blank(empty_three), Block::Paragraph(world)] =>
        {
            assert_eq!(hello.first().unwrap().to_string(), "Hello,");
            assert_eq!(empty_one.to_string(), "  ");
            assert_eq!(empty_two.to_string(), "");
            assert_eq!(empty_three.to_string(), "     ");
            assert_eq!(world.first().unwrap().to_string(), "World!");
        }
        _ => panic!("Unexpected block(s) or primitive(s): {:#?}", blocks),
    }
}

#[test]
fn it_does_not_absorb_leading_whitespace_into_a_preceding_blank() {
    let input = r#"Hello,

 - World!"#;

    let blocks: Vec<Block<Entity>> = parse(input.as_ref()).unwrap().collect();

    match blocks.as_slice() {
        [Block::Paragraph(hello), Block::Blank(empty), world @ Block::Paragraph(world_entities)] => {
            assert_eq!(hello.first().unwrap().to_string(), "Hello,");
            assert_eq!(empty.to_string(), "");
            assert_eq!(world_entities.first().unwrap().to_string(), " ");
            assert_eq!(world.to_string(), " - World!");
        }
        _ => panic!("Unexpected block(s) or primitive(s): {:#?}", blocks),
    }
}
