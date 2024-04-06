use mutant_detector::app_config::AppConfig;
use mutant_detector::execution_config::ExecutionConfig;
use mutant_detector::runtime_metrics::RuntimeMetrics;
use mutant_detector::driver::parse_and_run;
use mutant_detector::driver::*;

use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use itertools::Itertools;

fn main() -> Result<(), String> {
    env_logger::builder().try_init().unwrap_or_else(|_| {
        eprintln!("Warning: Failed to initialize logger.");
    });
    let args = AppConfig::from_args();
    let config = ExecutionConfig::from(args.clone());
    let mut global_data = RuntimeMetrics::default();
    let output_directory = &config.results_directory;
    prepare_output_directory(output_directory)?;

    args.source_files.iter().enumerate().try_for_each(|(index, subj_file)| {
        process_subject_file(index, subj_file, &args, &config, &mut global_data)
    })?;

    if args.source_files.len() >= 1 {
        print_summary(global_data);
    }

    Ok(())
}

fn prepare_output_directory(output_directory: &str) -> Result<(), String> {
    if Path::new(output_directory).exists() {
        remove_dir_all(output_directory).map_err(|e| e.to_string())?;
    }
    create_dir(output_directory).map_err(|e| e.to_string())
}

fn process_subject_file(
    index: usize,
    subj_file: &str,
    args: &AppConfig,
    config: &ExecutionConfig,
    global_data: &mut RuntimeMetrics,
) -> Result<(), String> {
    
    match parse_and_run(subj_file, config, global_data) {
        Ok(subjects) => {
            let found = subjects.subjects.iter().map(|subj| subj.analysis_result.score).sum::<u32>();
            global_data.record_discovered_equivalences(found);
            if found > 0 {
                println!("    [+] Found {} equivalences", found);
            }

            write_results(&subjects, subj_file, &config.results_directory)?
        }
        Err(msg) => {
            eprintln!("Error processing subject file '{}': {}", subj_file, msg);
            if config.halt_on_error {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn write_results(subjects: &Subjects, subj_file: &str, output_directory: &str) -> Result<(), String> {
    let file_name = format!("{}/{}.equiv-class", output_directory, subj_file.split('/').last().unwrap());
    write_subjects_to_single_file(subjects, &file_name)
        .map_err(|e| format!("Failed to write results to '{}': {}", file_name, e))
}

fn print_summary(global_data: RuntimeMetrics) {
    println!("        SUMMARY");
    println!("        =======");
    println!("Mutants found: {}", global_data.total_mutants);
    println!("Equivs found: {}", global_data.total_discovered_equivalences);
}

pub fn write_subjects_to_single_file(subjects: &Subjects, file: &str) -> Result<(), Error> {
    let mut file = File::create(file)?;
    let mut equiv_file_contents = vec![];
    for subject in &subjects.subjects {
        equiv_file_contents.push(get_equiv_file_contents_for_subject(subject));
    }

    let file_contents = equiv_file_contents.join("\n");
    file.write_all(file_contents.as_bytes())
}

fn get_equiv_file_contents_for_subject(subject: &Subject) -> String {
    let ar = &subject.analysis_result;
    let mut equiv_classes_as_strings = vec![];
    for equiv_class in &ar.equivalence_classes {
        if equiv_class.len() == 1 && equiv_class.iter().next().unwrap() == &0 {
            continue;
        }
        let equiv_class_as_string: String = itertools::sorted(equiv_class)
            .iter()
            .map(|id| (**id).to_string())
            .intersperse(" ".to_string())
            .collect();
        equiv_classes_as_strings.push(equiv_class_as_string);
    }
    format!("{}\n", equiv_classes_as_strings.join("\n"))
}

