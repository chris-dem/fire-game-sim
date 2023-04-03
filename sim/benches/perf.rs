use criterion::profiler::Profiler;
use pprof::{flamegraph, ProfilerGuard};
use std::{fs::OpenOptions, os::raw::c_int, path::Path};

pub struct FlamegraphProfiler<'a> {
    frequency: c_int,
    active_profiler: Option<ProfilerGuard<'a>>,
}

impl<'a> FlamegraphProfiler<'a> {
    #[allow(dead_code)]
    pub fn new(frequency: c_int) -> Self {
        Self {
            frequency,
            active_profiler: None,
        }
    }
}

impl<'a> Profiler for FlamegraphProfiler<'a> {
    fn start_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        self.active_profiler =
            Some(ProfilerGuard::new(self.frequency).expect("Profiler failed to start"));
    }

    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        if !benchmark_dir.exists() {
            std::fs::create_dir_all(benchmark_dir).expect("Profiler failed to start")
        }
        let flamegraph_path = benchmark_dir.join("flamegraph.svg");
        let flamegraph_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(flamegraph_path)
            .expect("File system error while creating the flamegraph.svg");
        if let Some(profiler) = self.active_profiler.take() {
            profiler
                .report()
                .build()
                .expect("Failed to build report")
                .flamegraph(flamegraph_file)
                .expect("Error writing to flamegraph file")
        }
    }
}
