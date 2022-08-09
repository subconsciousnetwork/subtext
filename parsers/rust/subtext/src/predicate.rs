use crate::sequence::Sequence;

pub const CARRIAGE_RETURN_NEW_LINE_SEQUENCE: &[char] = &['\r', '\n'];
pub const SLASH_LINK_SEQUENCE: &[char] = &['/'];
pub const WIKI_LINK_OPEN_SEQUENCE: &[char] = &['[', '['];
pub const WIKI_LINK_CLOSE_SEQUENCE: &[char] = &[']', ']'];
pub const HTTPS_LINK_SEQUENCE: &[char] = &['h', 't', 't', 'p', 's', ':', '/', '/'];
pub const HTTP_LINK_SEQUENCE: &[char] = &['h', 't', 't', 'p', ':', '/', '/'];
pub const IPFS_LINK_SEQUENCE: &[char] = &['i', 'p', 'f', 's', ':', '/', '/'];

pub fn sequence_to_predicate(mut sequence: Sequence) -> impl FnMut(&char) -> Option<usize> + '_ {
    move |token: &char| {
        sequence.go_to(token);
        match sequence.is_complete() {
            true => Some(sequence.len()),
            false => None,
        }
    }
}

pub fn first_predicate(
    mut predicates: Vec<impl FnMut(&char) -> Option<usize>>,
) -> impl FnMut(&char) -> Option<usize> {
    move |token: &char| {
        for predicate in predicates.iter_mut() {
            if let result @ Some(_) = predicate(&token) {
                return result;
            }
        }

        None
    }
}

pub fn tab_or_space_predicate() -> impl FnMut(&char) -> Option<usize> {
    move |token: &char| match token {
        ' ' | '\t' => Some(1usize),
        _ => None,
    }
}

pub fn new_line_predicate() -> impl FnMut(&char) -> Option<usize> {
    let mut is_carriage_return =
        sequence_to_predicate(Sequence::new(CARRIAGE_RETURN_NEW_LINE_SEQUENCE, None));

    move |token: &char| {
        if let matched @ Some(_) = is_carriage_return(token) {
            return matched;
        }

        if *token == '\n' {
            return Some(1usize);
        }

        None
    }
}

pub fn white_space_predicate() -> impl FnMut(&char) -> Option<usize> {
    let mut is_new_line = new_line_predicate();
    let mut is_white_space = tab_or_space_predicate();

    move |token: &char| {
        if let matched @ Some(_) = is_new_line(token) {
            return matched;
        }

        if let matched @ Some(_) = is_white_space(token) {
            return matched;
        }

        None
    }
}

pub fn wiki_link_delimiter_predicate() -> impl FnMut(&char) -> Option<usize> {
    let mut is_new_line = new_line_predicate();
    let mut is_wiki_link_close =
        sequence_to_predicate(Sequence::new(WIKI_LINK_CLOSE_SEQUENCE, None));

    move |token: &char| {
        if let matched @ Some(_) = is_new_line(token) {
            return matched;
        }

        if let matched @ Some(_) = is_wiki_link_close(token) {
            return matched;
        }

        None
    }
}

pub fn hyper_link_predicate() -> impl FnMut(&char) -> Option<usize> {
    first_predicate(Vec::from([
        sequence_to_predicate(Sequence::new(HTTP_LINK_SEQUENCE, Some(' '))),
        sequence_to_predicate(Sequence::new(HTTPS_LINK_SEQUENCE, Some(' '))),
        sequence_to_predicate(Sequence::new(IPFS_LINK_SEQUENCE, Some(' '))),
    ]))
}

pub fn wiki_link_open_predicate() -> impl FnMut(&char) -> Option<usize> {
    sequence_to_predicate(Sequence::new(WIKI_LINK_OPEN_SEQUENCE, Some(' ')))
}

pub fn slash_link_predicate() -> impl FnMut(&char) -> Option<usize> {
    sequence_to_predicate(Sequence::new(SLASH_LINK_SEQUENCE, Some(' ')))
}

#[derive(Debug)]
pub enum ParseLinkAs {
    SlashLink,
    HyperLink,
    WikiLink,
}

pub fn link_predicate() -> impl FnMut(&char) -> Option<(usize, ParseLinkAs)> {
    let mut is_hyper_link = hyper_link_predicate();
    let mut is_slash_link = slash_link_predicate();
    let mut is_wiki_link_open = wiki_link_open_predicate();

    move |token: &char| {
        if let Some(steps) = is_hyper_link(token) {
            return Some((steps, ParseLinkAs::HyperLink));
        }

        if let Some(steps) = is_slash_link(token) {
            return Some((steps, ParseLinkAs::SlashLink));
        }

        if let Some(steps) = is_wiki_link_open(token) {
            return Some((steps, ParseLinkAs::WikiLink));
        }

        None
    }
}
