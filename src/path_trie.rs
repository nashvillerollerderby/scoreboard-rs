use lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

lazy_static::lazy_static! {
    static ref FIND_DEL: Regex = {
        let r = Regex::new(r"[\.\(]").unwrap();
        r
    };
}

const STAR_DELIMITER: &str = "*)";

/// *Please* refers to the test cases to understand what this DS allows and
/// the special cases.
/// In general, we allow to "register" patterns using the "add" function,
/// and to verify if some other pattern is "covered" (using - "covers")
///
/// # Example
/// ```
/// # use scoreboard_rs::path_trie::PathTrie;
/// let mut pt = PathTrie::default();
/// pt.add("ScoreBoard.Period(*).Bar");
///
/// assert!(pt.covers("ScoreBoard.Period(1).Bar"));
/// assert!(!pt.covers("ScoreBoard.Period"));
/// assert!(!pt.covers("ScoreBoard.Period(2).Baz"));
/// ```
#[derive(Default, Debug)]
pub struct PathTrie {
    root: PathTrieChildren,
}

#[derive(Default, Debug)]
struct PathTrieChildren {
    is_path: bool,
    children: HashMap<String, PathTrieChildren>,
}

impl PathTrie {
    // When we add, we split by `.` and `(`.
    // We add the pieces as children.
    // Note that * has a different meaning inside and outside the parenthesis:
    // the idea is that within an identifier, * counts as a character,
    // while within the `()` it means - in case it is at the end of a String,
    // that we allow everything that comes after.
    pub fn add(&mut self, entry: &str) {
        let mut rem = entry;
        let mut node = &mut self.root;
        loop {
            log::debug!("PathTrie: adding `{}` to node {:?}", rem, node);
            let found = FIND_DEL.find(rem);
            match found {
                Some(a) => {
                    let start = rem.split_at(a.start()).0;
                    let end = rem.split_at(a.end()).1;
                    if !node.children.contains_key(start) {
                        log::debug!(
                            "PathTrie: found no node with key {}, creating a new one",
                            start
                        );
                        let new_node = PathTrieChildren::default();
                        node.children.insert(start.to_owned(), new_node);
                    }
                    node = node
                        .children
                        .get_mut(start)
                        .expect("we either had already a child, or we added it");
                    rem = end;
                }
                None => {
                    // There is nothing more we can match against. As such, we check whether the
                    // remainder is there, and we quit.
                    if !node.children.contains_key(rem) {
                        log::debug!(
                            "PathTrie: found no node with key {}, creating a new one",
                            rem
                        );

                        let new_node = PathTrieChildren::default();
                        node.children.insert(rem.to_owned(), new_node);
                    }
                    let node = node
                        .children
                        .get_mut(rem)
                        .expect("we either had already a child, or we added it");
                    node.is_path = true;
                    break;
                }
            }
        }
    }

    pub fn covers(&self, entry: &str) -> bool {
        let mut possible_subtrees = VecDeque::from(vec![(&self.root, entry)]);
        while let Some((node, rem)) = possible_subtrees.pop_front() {
            log::debug!("PathTrie: searching for {} in node {:?}", rem, node);
            let found = FIND_DEL.find(rem);
            match found {
                Some(a) => {
                    let start = rem.split_at(a.start()).0;
                    let end = rem.split_at(a.end()).1;
                    // if we have "*)" in the children, we need to also add that one.
                    let star_node = node.children.get(STAR_DELIMITER);
                    if let Some(star_node) = star_node {
                        log::debug!("PathTrie: found a star node");
                        possible_subtrees.push_back((star_node, end));
                    }
                    let next_node = node.children.get(start);
                    if let Some(next_node) = next_node {
                        log::debug!("PathTrie: {} found", start);
                        if next_node.is_path {
                            return true;
                        }
                        possible_subtrees.push_back((next_node, end));
                    }
                }
                None => {
                    // If there is no reminder, then we quit.
                    if rem.is_empty() {
                        return false;
                    }
                    // There is nothing more we can match against. As such, we check whether the
                    // remainder is there, and we quit.
                    if let Some(node) = node.children.get(rem) {
                        log::debug!("PathTrie: {} has been found", rem);
                        if node.is_path {
                            return true;
                        }
                    }
                    if let Some(node) = node.children.get(STAR_DELIMITER) {
                        // if we still have children, we cannot possibly match.
                        if !node.children.is_empty() {
                            log::debug!("PathTrie: star remaider has children");
                            return false;
                        }
                        log::debug!("PathTrie: star remainder has been found");
                        return true;
                    }
                    log::debug!("PathTrie: {} has not been found", rem);
                    // if we "are" a path, then this is correct.
                    if node.is_path {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_covers() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Period");

        assert!(pt.covers("ScoreBoard.Period"));
        assert!(pt.covers("ScoreBoard.Period(1)"));
        assert!(pt.covers("ScoreBoard.Period.Jam"));

        assert!(!pt.covers("ScoreBoard.PeriodFoo"));
        assert!(!pt.covers("ScoreBoard.Perioc"));
        assert!(!pt.covers("ScoreBoard.Perioe"));
        assert!(!pt.covers("ScoreBoard(1).Perioe"));
        assert!(!pt.covers("ScoreBoard.Perio"));
        assert!(!pt.covers("Scoreboard.Period"));
    }

    #[test]
    fn stars_are_valid_names() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Period*");

        assert!(pt.covers("ScoreBoard.Period*"));

        assert!(!pt.covers("ScoreBoard.Period*a"));
        assert!(!pt.covers("ScoreBoard.Period"));
        assert!(!pt.covers("ScoreBoard.Period("));
        assert!(!pt.covers("ScoreBoard.Period(1).Jam(2).Foo(2).Bar"));
    }

    #[test]
    fn partial_and_fully_specified() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Period(1).Jam(1).StarPass");
        pt.add("ScoreBoard.Period");

        assert!(pt.covers("ScoreBoard.Period"));
        assert!(pt.covers("ScoreBoard.Period(1)"));
        assert!(pt.covers("ScoreBoard.Period(1).Jam(1).StarPass"));
    }

    #[test]
    fn star_in_brackets() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Period(1).Foo");
        pt.add("ScoreBoard.Period(*).Bar");

        assert!(pt.covers("ScoreBoard.Period(1).Foo"));
        assert!(pt.covers("ScoreBoard.Period(1).Bar"));
        assert!(pt.covers("ScoreBoard.Period(2).Bar"));

        assert!(!pt.covers("ScoreBoard.Period(2).Foo"));
    }

    #[test]
    fn long_chain_does_not_allow_partial_spec() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Period(*).Jam(1).Foo(*).Bar");

        assert!(pt.covers("ScoreBoard.Period(1).Jam(1).Foo(2).Bar"));
        assert!(pt.covers("ScoreBoard.Period(1).Jam(1).Foo(2).Bar.Baz"));
        assert!(pt.covers("ScoreBoard.Period(1).Jam(1).Foo(2).Bar(zzz)"));

        assert!(!pt.covers("ScoreBoard.Period"));
        assert!(!pt.covers("ScoreBoard.Period("));
        assert!(!pt.covers("ScoreBoard.Period(1)"));
        assert!(!pt.covers("ScoreBoard.Period(1).Jam(2).Foo(2).Bar"));
        assert!(!pt.covers("ScoreBoard.Period(1).Jam(1).Foo(2)"));
        assert!(!pt.covers("ScoreBoard.Period(1).Jam(1).TeamJam(1).Foo(2)"));
    }

    #[test]
    fn paths_in_parenthesis_follow_the_rules() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Rulesets.Rule(Period.Duration)");
        pt.add("ScoreBoard.Rulesets.Rule(Jam.*)");
        pt.add("ScoreBoard.Rulesets.Rule(Intermission*)");

        assert!(pt.covers("ScoreBoard.Rulesets.Rule(Period.Duration)"));
        assert!(pt.covers("ScoreBoard.Rulesets.Rule(Jam.Foo)"));
        assert!(pt.covers("ScoreBoard.Rulesets.Rule(Jam.Foo.Bar)"));

        assert!(!pt.covers("ScoreBoard.Rulesets.Rule(Period.Direction)"));
        assert!(!pt.covers("ScoreBoard.Rulesets.Rule(Jam)"));
        assert!(!pt.covers("ScoreBoard.Rulesets.Rule(Intermission.Direction)"));

        pt.add("ScoreBoard.Rulesets.Rule(*)");

        assert!(pt.covers("ScoreBoard.Rulesets.Rule(Period.Direction)"));
        assert!(pt.covers("ScoreBoard.Rulesets.Rule(Jam)"));
        assert!(pt.covers("ScoreBoard.Rulesets.Rule(Intermission.Direction)"));
    }
}
