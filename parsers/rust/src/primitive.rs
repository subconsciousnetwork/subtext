use tendril::{StrTendril, SubtendrilError};

use crate::{sequence::Sequence, util::cut};

#[derive(Debug, Clone)]
pub enum Entity {
    Sigil(StrTendril),
    TextSpan(StrTendril),
    EmptySpace(StrTendril),
    SlashLink(StrTendril),
    HyperLink(StrTendril),
}

impl Entity {
    pub fn to_string(&self) -> String {
        match self {
            Entity::TextSpan(tendril) => tendril.into(),
            Entity::Sigil(tendril) => tendril.into(),
            Entity::EmptySpace(tendril) => tendril.into(),
            Entity::SlashLink(tendril) => tendril.into(),
            Entity::HyperLink(tendril) => tendril.into(),
        }
    }
}

pub fn parse_empty_space(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut end = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        match token {
            '\r' | '\n' | '\t' | ' ' => iter.next(),
            _ => break,
        };
        end = index;
    }

    Ok((
        Entity::EmptySpace(input.try_subtendril(0, end as u32 + 1)?),
        end,
    ))
}

fn parse_until_empty_space(input: StrTendril) -> Result<(StrTendril, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut end = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        match token {
            '\r' | '\n' | '\t' | ' ' => break,
            _ => iter.next(),
        };

        end = index;
    }

    Ok((input.try_subtendril(0, end as u32 + 1)?, end))
}

pub fn parse_slash_link(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until_empty_space(input) {
        Ok((value, steps)) => Ok((Entity::SlashLink(value), steps)),
        Err(error) => Err(error),
    }
}

pub fn parse_hyper_link(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until_empty_space(input) {
        Ok((value, steps)) => Ok((Entity::HyperLink(value), steps)),
        Err(error) => Err(error),
    }
}

enum ParseLinkAs {
    SlashLink,
    HyperLink,
}

pub fn parse_text(input: StrTendril) -> Result<(Vec<Entity>, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut start = 0usize;
    let mut end = 0usize;
    let mut entities = Vec::<Entity>::new();

    let mut link_sequences = [
        (
            Sequence::new("http://".chars().collect(), ' '),
            ParseLinkAs::HyperLink,
        ),
        (
            Sequence::new("https://".chars().collect(), ' '),
            ParseLinkAs::HyperLink,
        ),
        (
            Sequence::new("ipfs://".chars().collect(), ' '),
            ParseLinkAs::HyperLink,
        ),
        (
            Sequence::new("/".chars().collect(), ' '),
            ParseLinkAs::SlashLink,
        ),
    ];

    'parse: while let Some(&(index, token)) = iter.peek() {
        for (sequence, parse_mode) in link_sequences.as_mut() {
            sequence.go_to(token);

            if sequence.is_complete() {
                end = index - (sequence.len() - 1);

                let link_input = cut(&input, end)?;
                let (special_entity, steps) = match parse_mode {
                    ParseLinkAs::HyperLink => parse_hyper_link(link_input)?,
                    ParseLinkAs::SlashLink => parse_slash_link(link_input)?,
                };
                if end > start {
                    let text_span_entity = Entity::TextSpan(
                        input.try_subtendril(start as u32, end as u32 - start as u32)?,
                    );
                    entities.push(text_span_entity);
                }
                start = end + steps + 1;
                end = start;

                entities.push(special_entity);
                iter.nth(start - index);
                continue 'parse;
            }
        }

        match token {
            '\r' | '\n' => break,
            _ => iter.next(),
        };

        end = index;
    }

    if end > start {
        let value = input.try_subtendril(start as u32, end as u32 - start as u32 + 1)?;
        entities.push(Entity::TextSpan(value));
    }

    Ok((entities, end))
}
