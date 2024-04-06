use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "cornelius_tool",
    about = "Executes equality saturation on Java mutations to identify equivalent and redundant mutations"
)]
pub struct AppConfig {
    #[structopt(name = "JAVA_SOURCE_FILES")]
    /// Java source files to analyze
    pub source_files: Vec<String>,

    #[structopt(long, default_value = "30")]
    /// Max iterations. Zero allows infinite iterations.
    pub max_iterations: usize,

    #[structopt(long, default_value = "10000")]
    /// Max graph nodes. Zero allows infinite nodes.
    pub max_nodes: usize,

    #[structopt(long, default_value = "5")]
    /// Execution time limit in seconds
    pub execution_timeout: u64,

    #[structopt(long)]
    /// Stops execution upon encountering an error
    pub halt_on_error: bool,

    #[structopt(long = "output_dir", short = "o", default_value = "equivalence_results")]
    /// Directory for storing output equivalence files
    pub results_directory: String,
}

