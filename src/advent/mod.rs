trait AdventSolver {
    fn solve(&mut self, input_path: &str) -> Result<(), anyhow::Error>;
}

// Generated by build.rs
include!("_all_days.rs");
