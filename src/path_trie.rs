use lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref FIND_DEL: Regex = {
        let r = Regex::new(r"[\.\(]").unwrap();
        r
    };
}

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
    pub fn add(&mut self, entry: &str) {
        let mut rem = entry;
        let mut node = &mut self.root;
        loop {
            let found = FIND_DEL.find(rem);
            match found {
                Some(a) => {
                    let start = rem.split_at(a.start()).0;
                    let end = rem.split_at(a.end()).1;
                    if !node.children.contains_key(start) {
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
                        let new_node = PathTrieChildren::default();
                        node.children.insert(rem.to_owned(), new_node);
                    }
                    let mut node = node
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
        let mut rem = entry;
        let mut node = &self.root;
        loop {
            let found = FIND_DEL.find(rem);
            match found {
                Some(a) => {
                    let start = rem.split_at(a.start()).0;
                    let end = rem.split_at(a.end()).1;
                    if !node.children.contains_key(start) {
                        eprintln!(" => `{}` not in the childrens", start);
                        return false;
                    }
                    node = node
                        .children
                        .get(start)
                        .expect("we either had already a child, or we added it");
                    rem = end;
                }
                None => {
                    // There is nothing more we can match against. As such, we check whether the
                    // remainder is there, and we quit.
                    if let Some(node) = node.children.get(rem) {
                        return node.is_path;
                    }
                    eprintln!(" => `{}` last level, not in children", rem);
                    // if we "are" a path, then this is correct.
                    return node.is_path;
                }
            }
        }
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
    fn covers1() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Period(1).Jam(1).StarPass");
        pt.add("ScoreBoard.Period");

        assert!(pt.covers("ScoreBoard.Period"));
        assert!(pt.covers("ScoreBoard.Period(1)"));
        assert!(pt.covers("ScoreBoard.Period(1).Jam(1).StarPass"));
    }

    #[test]
    fn covers2() {
        let mut pt = PathTrie::default();
        pt.add("ScoreBoard.Period(1).Foo");
        pt.add("ScoreBoard.Period(*).Bar");

        assert!(pt.covers("ScoreBoard.Period(1).Foo"));
        assert!(pt.covers("ScoreBoard.Period(1).Bar"));
        assert!(pt.covers("ScoreBoard.Period(2).Bar"));

        assert!(!pt.covers("ScoreBoard.Period(2).Foo"));
    }

    #[test]
    fn covers3() {
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
    fn covers5() {
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
