use crate::app_config::AppConfig;
use instant::Duration;

pub struct ExecutionConfig {
    pub max_iterations: usize,
    pub max_nodes: usize,
    pub execution_timeout: Duration,
    pub halt_on_error: bool,
    pub results_directory: String,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        ExecutionConfig {
            max_iterations: 30,
            max_nodes: 10_000,
            execution_timeout: Duration::from_secs(5),
            halt_on_error: false,
            results_directory: "equivalence_results".to_string(), // Updated default output directory
        }
    }
}

impl ExecutionConfig {
    pub fn set_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = if max_iterations == 0 { usize::MAX } else { max_iterations };
        self
    }

    pub fn set_execution_timeout(mut self, execution_timeout: Duration) -> Self {
        self.execution_timeout = execution_timeout;
        self
    }

    pub fn set_max_nodes(mut self, max_nodes: usize) -> Self {
        self.max_nodes = if max_nodes == 0 { usize::MAX } else { max_nodes };
        self
    }

    pub fn set_halt_on_error(mut self, halt_on_error: bool) -> Self {
        self.halt_on_error = halt_on_error;
        self
    }

    pub fn set_results_directory(mut self, results_directory: String) -> Self {
        self.results_directory = results_directory;
        self
    }
}

impl ToString for ExecutionConfig {
    fn to_string(&self) -> String {
        format!(
            "Iteration Limit: {}
Node Limit: {}
Timeout: {}
Halt on Error: {}
Results Directory: {}",
            self.max_iterations,
            self.max_nodes,
            self.execution_timeout.as_secs(),
            self.halt_on_error,
            self.results_directory
        )
    }
}

impl From<AppConfig> for ExecutionConfig {
    fn from(args: AppConfig) -> Self {
        ExecutionConfig::default()
            .set_max_iterations(args.max_iterations)
            .set_max_nodes(args.max_nodes)
            .set_execution_timeout(Duration::from_secs(args.execution_timeout))
            .set_halt_on_error(args.halt_on_error)
            .set_results_directory(args.results_directory)
    }
}