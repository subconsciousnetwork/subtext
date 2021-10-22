use crate::{block, util::cut};
use tendril::StrTendril;

/**
 * Parse a raw buffer as a chunk of subtext.
 */
pub fn parse(input: &[u8]) -> Vec<block::Entity> {
    let input = StrTendril::try_from_byte_slice(input).unwrap();
    let mut iter = input.char_indices().peekable();
    let mut blocks = Vec::<block::Entity>::new();

    while let Some(&(index, token)) = iter.peek() {
        let parse_block = match token {
            '\r' | '\n' | ' ' | '\t' => block::parse_empty_space,
            '#' => block::parse_header,
            '-' => block::parse_list,
            '>' => block::parse_quote,
            _ => block::parse_paragraph,
        };

        let advance = match cut(&input, index) {
            Ok(slice) => match parse_block(slice) {
                Ok((block, steps)) => {
                    blocks.push(block);
                    steps
                }
                Err(error) => {
                    println!("Failed to parse block: {:?}", error);
                    break;
                }
            },
            Err(error) => {
                println!("Failed to slice input: {:?}", error);
                break;
            }
        };

        iter.nth(advance);
    }

    blocks
}
