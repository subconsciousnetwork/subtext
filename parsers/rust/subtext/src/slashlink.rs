use anyhow::anyhow;
use std::{fmt::Display, str::FromStr};

/// The various forms that the "peer" part of a slashlink may take
#[derive(Debug, PartialEq, Clone)]
pub enum Peer {
    Name(Vec<String>),
    Did(String),
    None,
}

/// A slashlink is form of reference to content in a Sphere. It consists of a
/// peer part and a slug part. A slashlink with just the slug looks like:
/// `/foo`. A slashlink with just the peer looks like: `@cdata`. With both
/// parts, the link would look like: `@cdata/foo`.
///
/// This struct makes it easier to parse a slashlink from a string.
#[derive(Debug, PartialEq, Clone)]
pub struct Slashlink {
    pub peer: Peer,
    pub slug: Option<String>,
}

impl FromStr for Slashlink {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsing_peer = false;
        let mut parsing_link = false;

        let mut raw_peer = String::new();
        let mut slug = None;

        for (index, character) in s.char_indices() {
            match character {
                '@' if index == 0 => {
                    parsing_peer = true;
                }
                '/' if index == 0 || parsing_peer => {
                    parsing_peer = false;
                    parsing_link = true;
                }
                _ if parsing_peer => raw_peer.push(character),
                _ if parsing_link => {
                    slug = Some(s[index..].to_string());
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        let peer = if raw_peer.len() > 0 {
            if raw_peer.starts_with("did:") {
                Peer::Did(raw_peer)
            } else {
                Peer::Name(raw_peer.split('.').map(|s| s.to_owned()).collect())
            }
        } else {
            Peer::None
        };

        if peer == Peer::None && slug == None {
            Err(anyhow!("Could not parse {} as SlashLink", s))
        } else {
            Ok(Slashlink { peer, slug })
        }
    }
}

impl Display for Slashlink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.peer {
            Peer::Name(names) => write!(f, "@{}", names.join(".")),
            Peer::Did(did) => write!(f, "@{}", did),
            Peer::None => Ok(()),
        }?;

        match &self.slug {
            Some(slug) => write!(f, "/{}", slug),
            None => Ok(()),
        }?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::slashlink::{Peer, Slashlink};

    #[test]
    fn it_can_parse_a_basic_slashlink() {
        let slashlink = Slashlink::from_str("/foo-bar").unwrap();

        assert_eq!(slashlink.peer, Peer::None);
        assert_eq!(slashlink.slug, Some("foo-bar".into()));
    }

    #[test]
    fn it_can_parse_a_basic_slashlink_with_a_short_peer_name() {
        let slashlink = Slashlink::from_str("@ben/foo-bar").unwrap();

        assert_eq!(slashlink.peer, Peer::Name(vec!["ben".into()]));
        assert_eq!(slashlink.slug, Some("foo-bar".into()));
    }

    #[test]
    fn it_can_parse_a_basic_slashlink_with_a_peer_name() {
        let slashlink = Slashlink::from_str("@cdata/foo-bar").unwrap();

        assert_eq!(slashlink.peer, Peer::Name(vec!["cdata".into()]));
        assert_eq!(slashlink.slug, Some("foo-bar".into()));
    }

    #[test]
    fn it_can_parse_a_slashlink_with_a_peer_did() {
        let slashlink = Slashlink::from_str("@did:test:alice/foo-bar").unwrap();
        assert_eq!(slashlink.peer, Peer::Did("did:test:alice".into()));
        assert_eq!(slashlink.slug, Some("foo-bar".into()));
    }

    #[test]
    fn it_can_parse_a_slashlink_with_a_peer_name_chain() {
        let slashlink = Slashlink::from_str("@jordan.gordon.morgon/foo-bar").unwrap();

        assert_eq!(
            slashlink.peer,
            Peer::Name(vec!["jordan".into(), "gordon".into(), "morgon".into()])
        );
        assert_eq!(slashlink.slug, Some("foo-bar".into()));
    }

    #[test]
    fn it_can_parse_a_slashlink_with_only_a_peer() {
        let slashlink = Slashlink::from_str("@cdata").unwrap();
        assert_eq!(slashlink.peer, Peer::Name(vec!["cdata".into()]));
        assert_eq!(slashlink.slug, None);
    }

    #[test]
    fn it_can_parse_a_slashlink_with_only_a_peer_did() {
        let slashlink = Slashlink::from_str("@did:test:alice").unwrap();

        assert_eq!(slashlink.peer, Peer::Did("did:test:alice".into()));
        assert_eq!(slashlink.slug, None);
    }

    #[test]
    fn it_can_parse_a_slashlink_with_only_a_peer_name_chain() {
        let slashlink = Slashlink::from_str("@jordan.gordon.morgon").unwrap();

        assert_eq!(
            slashlink.peer,
            Peer::Name(vec!["jordan".into(), "gordon".into(), "morgon".into()])
        );
        assert_eq!(slashlink.slug, None);
    }

    #[test]
    fn it_will_not_parse_a_non_slashlink() {
        let non_slashlinks = vec!["cdata", "@", "/", "@/", "foo/bar"];
        for test_case in non_slashlinks {
            println!("Checking {}", test_case);
            assert!(Slashlink::from_str(test_case).is_err())
        }
    }

    #[test]
    #[ignore = "TODO(subconsciousnetwork/subtext#36)"]
    fn it_can_parse_a_slashlink_that_is_a_cid() {}
}
