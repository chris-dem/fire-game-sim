use rand_derive2::RandGen;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
enum RuleCase {
    AllCoop,
    AllButOneCoop,
    Argument,
}

#[derive(Debug, PartialEq, Eq, RandGen, Clone, Copy, Deserialize)]
pub enum Strategy {
    Competitive,
    Cooperative,
}

impl Strategy {
    fn game_rules(&self, neigh: &[Self]) -> RuleCase {
        let self_val = if *self == Self::Competitive { 1 } else { 0 };
        match neigh.iter().filter(|x| **x == Self::Competitive).count() + self_val {
            0 => RuleCase::AllCoop,
            1 => RuleCase::AllButOneCoop,
            _ => RuleCase::Argument,
        }
        // RuleCase::AllCoop
    }
}
impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_coop() {
        let curr_evac = Strategy::Cooperative;
        let others = [
            Strategy::Cooperative,
            Strategy::Cooperative,
            Strategy::Cooperative,
        ];
        assert_eq!(curr_evac.game_rules(&others), RuleCase::AllCoop);
    }

    #[test]
    fn test_all_but_one_coop() {
        let curr_evac = Strategy::Cooperative;
        let others = [
            Strategy::Cooperative,
            Strategy::Competitive,
            Strategy::Cooperative,
        ];
        assert_eq!(curr_evac.game_rules(&others), RuleCase::AllButOneCoop);
    }

    #[test]
    fn test_more_than_one_coop() {
        let curr_evac = Strategy::Competitive;
        let others = [
            Strategy::Cooperative,
            Strategy::Competitive,
            Strategy::Cooperative,
        ];
        assert_eq!(curr_evac.game_rules(&others), RuleCase::Argument);
    }
}
