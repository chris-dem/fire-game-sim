use rand_derive2::RandGen;

#[derive(Debug, PartialEq, Eq)]
enum RuleCase {
    AllCoop,
    AllButOneCoop,
    Argument,
}

// Add todo

#[derive(Debug, PartialEq, Eq, RandGen)]
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
