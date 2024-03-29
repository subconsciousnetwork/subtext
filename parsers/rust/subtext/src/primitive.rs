use crate::str::SharedString;
use tendril::SubtendrilError;

use crate::{
    predicate::{
        link_predicate, white_space_predicate, wiki_link_delimiter_predicate, ParseLinkAs,
    },
    util::cut,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Sigil(SharedString),
    TextSpan(SharedString),
    EmptySpace(SharedString),
    SlashLink(SharedString),
    HyperLink(SharedString),
    WikiLink(SharedString),
}

// TODO: Investigate the validity of this
unsafe impl Sync for Entity {}

impl AsRef<Entity> for Entity {
    fn as_ref(&self) -> &Entity {
        self
    }
}

impl Entity {
    pub fn to_string(&self) -> String {
        match self {
            Entity::TextSpan(tendril) => tendril.into(),
            Entity::Sigil(tendril) => tendril.into(),
            Entity::EmptySpace(tendril) => tendril.into(),
            Entity::SlashLink(tendril) => tendril.into(),
            Entity::HyperLink(tendril) => tendril.into(),
            Entity::WikiLink(tendril) => tendril.into(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Entity::TextSpan(tendril)
            | Entity::Sigil(tendril)
            | Entity::EmptySpace(tendril)
            | Entity::SlashLink(tendril)
            | Entity::HyperLink(tendril)
            | Entity::WikiLink(tendril) => tendril.as_bytes(),
        }
    }
}

pub fn parse_empty_space(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut end = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        match token {
            '\r' | '\n' | '\t' | ' ' => iter.next(),
            _ => break,
        };
        end = index;
    }

    end = end + 1;

    Ok((
        Entity::EmptySpace(input.try_subtendril(0, end as u32)?),
        end,
    ))
}

pub fn parse_until<P>(
    input: SharedString,
    mut predicate: P,
) -> Result<(SharedString, usize), SubtendrilError>
where
    P: FnMut(&char) -> Option<usize>,
{
    let mut iter = input.char_indices().peekable();
    let mut end = 0usize;
    while let Some(&(index, token)) = iter.peek() {
        end = index;

        let predicate_match = predicate(&token);

        if let Some(match_length) = predicate_match {
            end = end + match_length - 1;
            return Ok((input.try_subtendril(0, end as u32)?, end));
        }

        iter.next();
    }

    return Ok((input, end + 1));
}

pub fn parse_slash_link(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until(input, white_space_predicate()) {
        Ok((value, steps)) => Ok((Entity::SlashLink(value), steps)),
        Err(error) => Err(error),
    }
}

pub fn parse_hyper_link(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until(input, white_space_predicate()) {
        Ok((value, steps)) => Ok((Entity::HyperLink(value), steps)),
        Err(error) => Err(error),
    }
}

pub fn parse_wiki_link(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until(input, wiki_link_delimiter_predicate()) {
        Ok((value, steps)) => Ok((Entity::WikiLink(value), steps)),
        Err(error) => Err(error),
    }
}

/// Clips the next one or two entities with awareness of leading whitespace
fn clip<E>(
    input: &SharedString,
    leading_whitespace_index: u32,
    mut start: u32,
    end: u32,
) -> Result<Vec<E>, SubtendrilError>
where
    E: From<Entity> + AsRef<Entity>,
{
    let mut entities = Vec::new();

    if leading_whitespace_index > 0 {
        entities.push(
            Entity::EmptySpace(input.try_subtendril(start, leading_whitespace_index)?).into(),
        );
        start = leading_whitespace_index;
    }

    if end > start {
        entities.push(Entity::TextSpan(input.try_subtendril(start, end - start)?).into());
    }

    Ok(entities)
}

pub fn parse_text<E>(input: SharedString) -> Result<(Vec<E>, usize), SubtendrilError>
where
    E: From<Entity> + AsRef<Entity>,
{
    let mut iter = input.char_indices().peekable();
    let mut start = 0usize;
    let mut end = 0usize;
    let mut entities = Vec::<E>::new();

    let mut is_link = link_predicate();
    let mut leading_whitespace_index = 0usize;
    let mut leading_word_index = input.len();

    'parse: while let Some(&(index, token)) = iter.peek() {
        match token {
            ' ' | '\t' if leading_word_index > index => {
                leading_whitespace_index = index + 1;
            }
            _ => leading_word_index = index,
        };

        // Check if we met the link predicate criteria; if we did, make an
        // entity out of the text span we have seen so far and then parse
        // the link
        if let Some((match_length, parse_as)) = is_link(&token) {
            end = index - (match_length - 1);

            if end > start {
                entities.extend(clip(
                    &input,
                    leading_whitespace_index as u32,
                    start as u32,
                    end as u32,
                )?);
                leading_whitespace_index = 0;
            }

            let link_input = cut(&input, end)?;
            let (link_entity, steps) = match parse_as {
                ParseLinkAs::HyperLink => parse_hyper_link(link_input)?,
                ParseLinkAs::SlashLink => parse_slash_link(link_input)?,
                ParseLinkAs::WikiLink => parse_wiki_link(link_input)?,
            };

            start = end + steps;
            end = start;

            entities.push(link_entity.into());

            match iter.nth(start - index) {
                Some((_, '\n')) => {
                    break 'parse;
                }
                Some((_, any)) => {
                    is_link(&any);
                    continue 'parse;
                }
                _ => {
                    continue 'parse;
                }
            };
        }

        match token {
            '\n' => {
                end = index;
                break 'parse;
            }
            _ => (),
        };

        iter.next();

        end = index;

        if let None = iter.peek() {
            end = end + 1;
        }
    }

    if end > start {
        entities.extend(clip(
            &input,
            leading_whitespace_index as u32,
            start as u32,
            end as u32,
        )?);
    }

    Ok((entities, end))
}
