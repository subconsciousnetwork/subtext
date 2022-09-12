use std::marker::PhantomData;

use crate::{
    block::{parse_blank, parse_header, parse_list, parse_paragraph, parse_quote, Block},
    primitive::Entity,
    util::cut,
};
use anyhow::{anyhow, Result};
// use tendril::SharedString;
use crate::str::SharedString;

/// Parse a raw buffer as a chunk of subtext. The iterator yields the parsed
/// subtext one block at a time.
pub fn parse<B, E>(input: &[u8]) -> Result<SubtextIterator<B, E>>
where
    E: From<Entity> + AsRef<Entity>,
    B: From<Block<E>>,
{
    let input = SharedString::try_from_byte_slice(input)
        .map_err(|_| anyhow!("Could not interpret bytes as UTF-8"))?;
    Ok(SubtextIterator::new(input))
}

pub struct SubtextIterator<B, E>
where
    E: From<Entity> + AsRef<Entity>,
    B: From<Block<E>>,
{
    input: SharedString,
    output_type: PhantomData<(B, E)>,
}

impl<B, E> SubtextIterator<B, E>
where
    E: From<Entity> + AsRef<Entity>,
    B: From<Block<E>>,
{
    pub fn new(input: SharedString) -> Self {
        SubtextIterator {
            input,
            output_type: PhantomData {},
        }
    }
}

impl<B, E> Iterator for SubtextIterator<B, E>
where
    E: From<Entity> + AsRef<Entity>,
    B: From<Block<E>>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.input.chars().next() {
            let parse_block = match token {
                '\r' | '\n' | ' ' | '\t' => parse_blank,
                '#' => parse_header,
                '-' => parse_list,
                '>' => parse_quote,
                _ => parse_paragraph,
            };

            match parse_block(self.input.clone()) {
                Ok((block, steps)) => {
                    let steps = usize::min(steps, self.input.len());
                    self.input = match cut(&self.input, steps) {
                        Ok(input) => input,
                        _ => "".into(),
                    };

                    Some(B::from(block))
                }
                Err(error) => {
                    warn!("Failed to parse block: {:?}", error);
                    None
                }
            }
        } else {
            None
        }
    }
}
