use rand::prelude::*;
use rand::RngCore;

use crate::model::ga_search::ga_explore::DNA_SIZE;

#[derive(Debug, Default)]
pub struct OutputVariables {
    pub per_case_ratio_1: u64, // a:b:c take a
    pub per_case_ratio_2: u64, // a:b:c take b
    pub per_case_ratio_3: u64, // a:b:c take c
}

// #[derive(Debug, Default)]
// pub struct Handlers {
//     pub rs_coop: RunningSum,
//     pub rs_lc: RunningSum,
//     pub rs_ld: RunningSum,
//     pub n: usize,
//     pub prev_ind: usize,
// }

// impl Handlers {
//     pub fn handle_sums(&mut self, coop: f32, lc: f32, ld: f32) {
//         self.rs_coop.sm += coop;
//         self.rs_coop.smsq += coop * coop;
//         self.rs_lc.sm += lc;
//         self.rs_lc.smsq += lc * lc;
//         self.rs_ld.sm += ld;
//         self.rs_ld.smsq += ld * ld;
//         self.n += 1;
//     }

//     pub fn reset(&mut self) {
//         self.n = 0;
//         self.rs_coop.reset();
//         self.rs_lc.reset();
//         self.rs_ld.reset();
//     }
// }

// #[derive(Debug, Default)]
// pub struct RunningSum {
//     pub sm: f32,
//     pub smsq: f32,
// }

// impl RunningSum {
//     fn reset(&mut self) {
//         self.sm = 0.;
//         self.smsq = 0.;
//     }
// }

pub struct InputSearch {
    pub lc: f32,
    pub ld: f32,
    pub asp_infl: f32,
    pub rat_infl: f32,
    pub reward_infl: f32,
    pub static_infl: f32,
    pub dynamc_infl: f32,
}

impl InputSearch {
    pub fn from_string_vec(v: Vec<String>) -> Self {
        assert!(v.len() == DNA_SIZE);
        let v: Vec<f32> = v
            .into_iter()
            .map(|s| s.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to parse string to float");
        Self {
            lc: v[0],
            ld: v[1],
            asp_infl: v[2],
            rat_infl: v[3],
            reward_infl: v[4],
            static_infl: v[5],
            dynamc_infl: v[6],
        }
    }
    pub fn generate_set_of_parameters(rng: &mut impl RngCore) -> Vec<f64> {
        let lc = rng.gen_range(0.005..1.0_f64);
        let ld = rng.gen_range(0.005..1.0_f64);
        let asp_infl = rng.gen_range(0. ..1.0_f64);
        let rat_infl = rng.gen_range(0.05..1.0_f64);
        let reward_infl = rng.gen_range(0.1..1.0_f64);
        let static_infl = rng.gen_range(1. ..2.0_f64);
        let dynamc_infl = rng.gen_range(0.5..1.5_f64);
        vec![
            lc,
            ld,
            asp_infl,
            rat_infl,
            reward_infl,
            static_infl,
            dynamc_infl,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use krabmaga::thread_rng;

    #[test]
    fn generating_test_of_params_length() {
        let mut rng = thread_rng();
        let arr = InputSearch::generate_set_of_parameters(&mut rng);
        assert_eq!(arr.len(), DNA_SIZE);
    }
}
