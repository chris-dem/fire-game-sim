

#[derive(Debug,Default)]
pub struct OutputVariables {
    pub per_case_ratio_1 : u64, // a:b:c take a
    pub per_case_ratio_2 : u64, // a:b:c take b
    pub per_case_ratio_3 : u64, // a:b:c take c
}

#[derive(Debug,Default)]
pub struct Handlers {
    pub rs_coop : RunningSum,
    pub rs_lc   : RunningSum,
    pub rs_ld   : RunningSum,
    pub n : usize,
    pub prev_ind : usize,
}


impl Handlers {
    pub fn handle_sums(&mut self, coop : f32, lc : f32, ld : f32) {
        self.rs_coop.sm   += coop;
        self.rs_coop.smsq += coop *  coop;
        self.rs_lc.sm     += lc;
        self.rs_lc.smsq   += lc * lc;
        self.rs_ld.sm     += ld;
        self.rs_ld.smsq   += ld * ld; 
        self.n += 1;
    }

    pub fn reset(&mut self) {
        self.n = 0;
        self.rs_coop.reset();
        self.rs_lc.reset();
        self.rs_ld.reset();
    }
}


#[derive(Debug,Default)]
pub struct RunningSum {
    pub sm : f32,
    pub smsq : f32,
}

impl RunningSum {
    fn reset(&mut self) {
        self.sm   = 0.;
        self.smsq = 0.;
    } 
}

pub struct InputSearch {
   pub lc          : f32,
   pub ld          : f32,
   pub asp_infl    : f32,
   pub rat_infl    : f32,
   pub reward_infl : f32,
   pub static_infl : f32,
   pub dynamc_infl : f32,
}