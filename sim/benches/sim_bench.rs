use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
};
use krabmaga::engine::{
    schedule::{self, Schedule},
    state::State,
};
use sim::model::{search::InputSearch, state::CellGrid};

pub mod perf;

fn sim_bench(c: &mut Criterion) {
    let n = 51;
    let mut state = CellGrid::new_training(
        InputSearch {
            lc: 0.3 as f32,
            ld: 0.3 as f32,
            asp_infl: 1e-7 as f32,
            rat_infl: 0.6634011009326777 as f32,
            reward_infl: 0.5 as f32,
            static_infl: 1.48319 as f32,
            dynamc_infl: 0.65 as f32,
        },
        n,
        n,
    );
    let mut schedule = Schedule::new();
    state.initial_config.fire_spread = Some(0.12);
    state.init(&mut schedule);

    c.bench_function("simulation_whole", |b| {
        // let state = state;
        // let schedule = schedule;
        b.iter(|| simulation(black_box((&mut state, &mut schedule))))
    });
}

fn custom_config() -> Criterion {
    Criterion::default()
        .with_profiler(perf::FlamegraphProfiler::new(100))
        .warm_up_time(std::time::Duration::from_secs(10))
        .measurement_time(std::time::Duration::from_secs(10))
}

fn simulation((state, schedule): (&mut CellGrid, &mut Schedule)) {
    let n_step = 750;
    for _ in 0..n_step {
        schedule.step(state);
        if state.end_condition(schedule) {
            break;
        }
    }
}

criterion_group! {
    name = benches;
    config = custom_config() ;
    targets= sim_bench
}
// criterion_group!(benches, sim_bench);
criterion_main!(benches);
