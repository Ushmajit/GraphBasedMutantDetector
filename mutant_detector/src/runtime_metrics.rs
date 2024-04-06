use egg::StopReason;

#[derive(Default, Debug)]
pub struct RuntimeMetrics {
    pub max_iterations_count: u32,
    pub execution_time_limit: u32,
    pub max_nodes_count: u32,
    pub saturation_events: u32,
    pub other_events: u32,
    pub total_subjects: u32,
    pub total_subject_files: u32,
    pub total_mutants: u32,
    pub total_discovered_equivalences: u32,
}

impl RuntimeMetrics {
    pub fn record_stop_reason(&mut self, stop_reason: &Option<StopReason>) {
        match stop_reason {
            Some(reason) => match reason {
                StopReason::Saturated => self.saturation_events += 1,
                StopReason::IterationLimit(_) => self.max_iterations_count += 1,
                StopReason::TimeLimit(_) => self.execution_time_limit += 1,
                StopReason::NodeLimit(_) => self.max_nodes_count += 1,
                StopReason::Other(_) => self.other_events += 1,
            },
            None => (),
        }
    }

    pub fn record_discovered_equivalences(&mut self, new_equivalences: u32) {
        self.total_discovered_equivalences += new_equivalences;
    }

    pub fn record_new_subjects(&mut self, new_subjects: u32) {
        self.total_subjects += new_subjects;
    }

    pub fn increment_subject_files(&mut self) {
        self.total_subject_files += 1;
    }

    pub fn record_new_mutants(&mut self, new_mutants: u32) {
        self.total_mutants += new_mutants;
    }
}

impl ToString for RuntimeMetrics {
    fn to_string(&self) -> String {
        format!(
            "Iteration Limit Stops: {}
Time Limit Stops: {}
Node Limit Stops: {}
Saturation Events: {}
Other Stop Reasons: {}
Total Subject Files: {}
Total Subjects: {}
Total Mutants: {}
Total Discovered Equivalences: {}
",
            self.max_iterations_count,
            self.execution_time_limit,
            self.max_nodes_count,
            self.saturation_events,
            self.other_events,
            self.total_subject_files,
            self.total_subjects,
            self.total_mutants,
            self.total_discovered_equivalences
        )
    }
}
