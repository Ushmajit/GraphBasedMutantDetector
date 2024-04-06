use crate::execution_config::ExecutionConfig;
use crate::runtime_metrics::RuntimeMetrics;
use crate::peg::{Peg, PegAnalysis};
use crate::rewrites::RewriteSystem;
use egg::*;
use serde::Deserialize;
use serde_aux::prelude::*;
use serde_xml_rs::from_reader;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
#[serde(rename = "node_equivalence")]
pub struct NodeEquivalence {
    pub first: String,
    pub second: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename = "node_equivalences")]
pub struct NodeEquivalences {
    #[serde(rename = "node_equivalence", default)]
    pub equivalences: Vec<NodeEquivalence>,
}

#[derive(Debug, Deserialize)]
pub struct Subjects {
    #[serde(rename = "subject", default)]
    pub subjects: Vec<Subject>,
    pub identifier_table: IdTable,
    #[serde(default)]
    pub equivalences: NodeEquivalences,
    #[serde(skip)]
    pub id_mapping: HashMap<Id, Id>,
}

#[derive(Debug, Deserialize)]
pub struct IdTable {
    #[serde(rename = "dedup_entry")]
    pub entries: Vec<IdEntry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AnalysisResult {
    pub score: u32,
    pub equivalence_classes: Vec<HashSet<u32>>,
}

#[derive(Debug, Deserialize)]
pub struct IdEntry {
    pub identifier: String,
    pub peg_representation: String,
}


impl Subjects {

    pub fn compute_expression(&mut self) -> Result<RecExpr<Peg>, String> {
        info!("Starting computation of RecExpr");
        let mut rec_expression = RecExpr::default();
        let mut highest_raw_id = -1;

        for entry in self.identifier_table.entries.iter() {
            let raw_id_int = entry.identifier.parse::<i32>()
                    .unwrap_or_else(|_| panic!("Failed to parse identifier '{}' as u32", &entry.identifier));
            if raw_id_int <= highest_raw_id {
                return Err(format!("Error: Identifier {} <= previous identifier {}", raw_id_int, highest_raw_id));
            }
            if raw_id_int < 0 {
                return Err(format!("Error: Identifier {} is less than 0", raw_id_int));
            }
            highest_raw_id = raw_id_int;
            let internal_id = Id::from(raw_id_int as usize);
            
            let mut peg_instance: Peg = parse_peg_from_string(entry.peg_representation.clone())?;
            let children = peg_instance.children_mut();
            let children_ids: Vec<_> = peg_instance.children().iter().cloned().collect();

            for (i, child_id) in children_ids.iter().enumerate() {
                if let Some(&mapped_id) = self.id_mapping.get(child_id) {
                    peg_instance.children_mut()[i] = mapped_id;
                }
            }            
            let egg_internal_id = rec_expression.add(peg_instance);
            self.id_mapping.insert(internal_id, egg_internal_id);
        }
        info!("RecExpr computation finished");
        Ok(rec_expression)
    }

    pub fn total_mutants(&self) -> usize {
        self.subjects.iter().map(|subject| subject.mutants.len()).sum()
    }
}

fn parse_peg_from_string(peg_str: String) -> Result<Peg, String> {
    let formatted_peg_str = match peg_str.chars().next() {
        Some('(') if peg_str.ends_with(')') => &peg_str[1..peg_str.len() - 1],
        Some('"') if peg_str.ends_with('"') && peg_str.len() > 1 => {
            return Ok(Peg::Symbol(Symbol::from(&peg_str[1..peg_str.len() - 1])));
        },
        _ if !peg_str.starts_with('(') && !peg_str.starts_with('"') => &peg_str,
        _ => return Err(format!("Invalid PEG string format: {}", peg_str)),
    };

    let parts: Vec<&str> = formatted_peg_str.split_whitespace().collect();
    let operator = parts.get(0).ok_or_else(|| "PEG string is empty".to_string())?;
    let children = parts[1..]
        .iter()
        .map(|&s| s.parse::<usize>()
             .map_err(|_| format!("Failed to parse identifier '{}' in '{}'", s, formatted_peg_str))
             .map(Id::from))
        .collect::<Result<Vec<_>, _>>()?;

    Peg::from_op_str(operator, children)
}

#[derive(Debug, Deserialize)]
pub struct Subject {
    #[serde(rename = "sourcefile")]
    pub source_file: String,
    pub method: String,
    pub pid: String,
    #[serde(rename = "mutant")]
    pub mutants: Vec<Mutant>,
    #[serde(default)]
    pub analysis_result: AnalysisResult,
}

impl Subject {
    pub fn new(source_file: String, method: String, inputs: &[(u32, &str)]) -> Self {
        let pid = inputs.get(0).expect("Inputs must contain at least one entry").1.parse()
            .expect("Failed to parse pid from inputs");

        let mutants = inputs[1..]
            .iter()
            .map(|&(id, src)| Mutant {
                mid: id,
                pid: src.parse().expect("Failed to parse mutant pid"),
            })
            .collect();

        Subject {
            source_file,
            method,
            pid,
            mutants,
            analysis_result: AnalysisResult::default(),
        }
    }

    pub fn from_file(path: String) -> Result<Subjects, String> {
        use std::fs;
        info!("Reading subject file from path: {}", path);

        let contents = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file at '{}': {}", path, e))?;
        info!("Subject file contents loaded successfully.");

        let subjects: Subjects = from_reader(contents.as_bytes())
            .map_err(|e| format!("Failed to parse subjects from file '{}': {}", path, e))?;
        info!("Subjects parsed successfully.");

        Ok(subjects)
    }
}


#[derive(Debug, Deserialize)]
pub struct Mutant {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub mid: u32,
    pub pid: String,
}

pub fn parse_and_run(
    subj_file: &str,
    run_config: &ExecutionConfig,
    global_data: &mut RuntimeMetrics,
) -> Result<Subjects, String> {
    let trimmed_file_path = subj_file.trim();
    let rewrite_rules = crate::rewrites::rw_rules();
    info!("Processing subject file: {}", trimmed_file_path);
    global_data.increment_subject_files();

    let subjects = Subject::from_file(trimmed_file_path.to_string())
        .map_err(|e| format!("Failed to process subjects from file '{}': {}", trimmed_file_path, e))?;
    info!("Successfully loaded subjects.");

    run_on_subjects(subjects, &rewrite_rules, run_config, global_data)
}

pub fn run_on_subjects(
    mut subjects: Subjects,
    rules: &RewriteSystem,
    run_config: &ExecutionConfig,
    global_data: &mut RuntimeMetrics,
) -> Result<Subjects, String> {
    global_data.record_new_subjects(subjects.subjects.len() as u32);
    global_data.record_new_mutants(subjects.total_mutants() as u32);

    let rec_expr = subjects.compute_expression()
        .map_err(|e| format!("Failed to compute RecExpr: {}", e))?;

    let mut egraph = EGraph::<Peg, PegAnalysis>::default();
    let mut id_offset_map = HashMap::<Id, Id>::new();

    rec_expr.as_ref().iter().enumerate().try_for_each(|(idx, node)| {
        let mut node = node.clone();
        node.for_each_mut(|id: &mut Id| *id = *id_offset_map.get(id)
            .expect(&format!("Node at idx {} has unbound child {}", idx, id)));

        let id = egraph.add(node);
        let canonical_id = egraph.find(id);
        id_offset_map.insert(Id::from(idx), canonical_id);
        Ok(())
    }).map_err(|e: Box<dyn std::error::Error>| e.to_string())?;

    subjects.equivalences.equivalences.iter().try_for_each(|equivalence| {
        let fst_id = Id::from(equivalence.first.parse::<usize>().map_err(|_| format!("Couldn't parse ID {}", equivalence.first))?);
        let snd_id = Id::from(equivalence.second.parse::<usize>().map_err(|_| format!("Couldn't parse ID {}", equivalence.second))?);

        let (id1, id2) = (
            id_offset_map.get(&fst_id).ok_or_else(|| format!("ID {} not found in id_offset_map", fst_id)),
            id_offset_map.get(&snd_id).ok_or_else(|| format!("ID {} not found in id_offset_map", snd_id)),
        );

        match (id1, id2) {
            (Ok(id1), Ok(id2)) => {
                egraph.union(*id1, *id2);
                Ok(())
            },
            _ => Err("Failed to find node IDs in the offset map.".to_string()),
        }
    })?;

    let runner = Runner::default()
        .with_egraph(egraph)
        .with_iter_limit(run_config.max_iterations)
        .with_node_limit(run_config.max_nodes)
        .with_time_limit(run_config.execution_timeout)
        .with_scheduler(egg::SimpleScheduler)
        .run(rules);

    global_data.record_stop_reason(&runner.stop_reason);
    
    let id_mapping_ref = &subjects.id_mapping;
    subjects.subjects.iter_mut().enumerate().for_each(|(_i, subj)| {
        analyze_subject(subj, &runner.egraph, &rec_expr, id_mapping_ref, &id_offset_map);
    });

    Ok(subjects)
}


fn analyze_subject(
    subj: &mut Subject,
    egraph: &EGraph<Peg, PegAnalysis>,
    _expr: &RecExpr<Peg>,
    raw_id_to_egg_id: &HashMap<Id, Id>,
    id_update: &HashMap<Id, Id>,
) {
    let mut rev_can_id_lookup = HashMap::<Id, HashSet<u32>>::new();
    let primary_id = Id::from(subj.pid.parse::<usize>().expect("Failed to parse subject pid"));
    let primary_id = *raw_id_to_egg_id.get(&primary_id)
        .expect("Subject pid not found in raw_id_to_egg_id mapping");
    let canonical_primary_id = *id_update.get(&primary_id)
        .expect("Subject pid not found in id_update mapping");
    let canonical_primary_id = egraph.find(canonical_primary_id);

    rev_can_id_lookup.entry(canonical_primary_id).or_insert_with(HashSet::new).insert(0);

    subj.mutants.iter().for_each(|mutant| {
        let mutant_id = Id::from(mutant.pid.parse::<usize>().expect("Failed to parse mutant pid"));
        let mutant_id = *raw_id_to_egg_id.get(&mutant_id)
            .expect("Mutant pid not found in raw_id_to_egg_id mapping");
        let canonical_mutant_id = *id_update.get(&mutant_id)
            .expect("Mutant pid not found in id_update mapping");
        let canonical_mutant_id = egraph.find(canonical_mutant_id);

        rev_can_id_lookup.entry(canonical_mutant_id).or_insert_with(HashSet::new).insert(mutant.mid);
    });

    let mut num_equivalences = 0;
    let equiv_classes: Vec<HashSet<u32>> = rev_can_id_lookup.values().cloned().collect();
    num_equivalences = equiv_classes.iter().map(|class| class.len() as u32 - 1).sum();

    subj.analysis_result = AnalysisResult {
        score: num_equivalences,
        equivalence_classes: equiv_classes, 
    };
}



