use rand::prelude::*;
use rand::RngCore;

use crate::model::ga_search::ga_explore::DNA_SIZE;

use super::evacuee_mod::strategies::aspiration_strategy::AspirationStrategy;
use super::evacuee_mod::strategies::aspiration_strategy::LogAspManip;
use super::evacuee_mod::strategies::aspiration_strategy::RootAsp;
use super::lerp::equations::Equation;
use super::lerp::equations::LerpStruct;

#[derive(Debug, Default)]
pub struct OutputVariables {
    pub per_case_ratio_1: u64, // a:b:c take a
    pub per_case_ratio_2: u64, // a:b:c take b
    pub per_case_ratio_3: u64, // a:b:c take c
}

pub struct InputSearch {
    pub lc: f32,
    pub ld: f32,
    pub asp_infl: Box<dyn AspirationStrategy + Send>,
    pub rat_infl: LerpStruct,
    pub reward_infl: LerpStruct,
    pub dynamc_infl: f32,
    pub static_infl: f32,
}

impl InputSearch {
    pub fn from_string_vec(v: Vec<String>, d: f32) -> Self {
        assert!(v.len() == DNA_SIZE);
        v.into_iter()
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| InputSearch::from_float_vertor(&v, d))
            .expect("Failed to parse string to float")
    }

    pub fn from_float_vertor(v: &Vec<f64>, dist: f32) -> Self {
        Self {
            lc: v[0] as f32,
            ld: v[1] as f32,
            asp_infl: gen_sel_asp((v[2], v[3])),
            rat_infl: gen_self_ratio((v[4], v[5], v[10]), dist).expect("Invalid type"),
            reward_infl: gen_self_rew((v[6], v[7], v[10]), dist).expect("Invalid type"),
            static_infl: v[8] as f32,
            dynamc_infl: v[9] as f32,
        }
    }
    pub fn generate_set_of_parameters(rng: &mut impl RngCore) -> Vec<f64> {
        let lc = rng.gen_range(0.0..1.0_f64);
        let ld = rng.gen_range(0.0..1.0_f64);
        let asp_infl = rng.gen_range(0. ..1.0_f64);
        let rat_infl = rng.gen_range(0. ..1.0_f64);
        let reward_infl = rng.gen_range(0. ..1.0_f64);
        let static_infl = rng.gen_range(0. ..3.0_f64);
        let dynamc_infl = rng.gen_range(0. ..3.0_f64);
        let asp_eq = rng.gen_range(0..2u8) as f64;
        let reward_eq = rng.gen_range(0..4u8) as f64;
        let rat_eq = rng.gen_range(0..4u8) as f64;
        let reward_limit = rng.gen_range(25. ..=100f64);
        vec![
            lc,           // 0
            ld,           // 1
            asp_eq,       // 2 eq
            asp_infl,     // 3 infl
            rat_eq,       // 4 eq
            rat_infl,     // 5 infl
            reward_eq,    // 6 eq
            reward_infl,  // 7 infl
            static_infl,  // 8
            dynamc_infl,  // 9
            reward_limit, // 10
        ]
    }
}

fn gen_sel_asp((typ, prob): (f64, f64)) -> Box<dyn AspirationStrategy + Send> {
    let prob = prob as f32;
    match typ as u8 {
        0 => Box::new(LogAspManip(prob)),
        1 => Box::new(RootAsp(prob)),
        _ => unreachable!(),
    }
}

fn gen_self_ratio((typ, prob, reward): (f64, f64, f64), max_d: f32) -> Option<LerpStruct> {
    let prob = prob as f32;
    let eq = Equation::from_u8(typ as u8)?;
    Some(LerpStruct::new(0., max_d, 0., reward as f32, prob, eq))
}

fn gen_self_rew((typ, prob, reward): (f64, f64, f64), max_d: f32) -> Option<LerpStruct> {
    let prob = prob as f32;
    let eq = Equation::from_u8(typ as u8)?;
    Some(LerpStruct::new(0., max_d, reward as f32, 0., prob, eq))
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
