use cmd_lib::run_fun;
use cmd_lib::run_cmd;


pub fn run() -> Result<(), String> {
    let nixpkgs_version = crate::utils::nixpkgs_version();
    let build_dir = crate::utils::get_build_dir();


    // write the config.json for the script.py
    let expr = format!(r#"
        let 
            config = (import ./htldoc.nix {{ }});
            default = {
                branch = "master";
                pdf_output_dir = {build_dir}/listings
                html_index_file = {build_dir}/listings/index.html
                githubRepoUrl = "";
            };
        in default // config.genListing
    "#);
    run_cmd!(nix eval --expr ${expr} --json > ${build_dir}/gen_log_config.json);


    // create the listing dir in the build_dir
    run_cmd!(mkdir -p ${build_dir}/listing);


    // create the listing_src dir
    run_cmd!(mkdir -p ${build_dir}/listing);


    // run the python script


use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::str;
use chrono::NaiveDateTime;
use git2::{Repository, Commit};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
struct Config {
    branch_name: String,
    start_commit: String,
    pdf_output_dir: String,
    html_index_file: String,
    github_repo_url: String,
    predefined_command: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "--config" {
        eprintln!("Usage: cargo run -- --config <path_to_config_json>");
        std::process::exit(1);
    }

    let config_path = &args[2];
    let config: Config = serde_json::from_reader(File::open(config_path)?)?;

    // Ensure output directory exists
    fs::create_dir_all(&config.pdf_output_dir)?;

    // Open the Git repository
    let repo = Repository::open(".")?;

    // Get the target branch
    let branch = repo.find_branch(&config.branch_name, git2::BranchType::Local)?;
    let branch_commit = branch.get().peel_to_commit()?;

    // Start processing commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push(branch_commit.id())?;

    let mut found_start_commit = false;
    let mut last_pdf_hash: Option<String> = None;
    let mut commit_list = Vec::new();

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        if !found_start_commit {
            if commit.id().to_string() == config.start_commit {
                found_start_commit = true;
            } else {
                continue;
            }
        }

        // Checkout the commit
        repo.checkout_tree(&commit.as_object(), None)?;
        repo.set_head_detached(commit.id())?;

        // Run the predefined command
        let pdf_filename = format!("{}.pdf", commit.id());
        let pdf_filepath = Path::new(&config.pdf_output_dir).join(&pdf_filename);

        let output = Command::new(&config.predefined_command[0])
            .args(&config.predefined_command[1..])
            .arg(&pdf_filepath)
            .output()?;

        if !output.status.success() {
            eprintln!("Error generating PDF for commit {}: {:?}", commit.id(), output);
            continue;
        }

        // Check if the PDF content has changed
        let current_pdf_hash = calculate_file_hash(&pdf_filepath)?;
        if let Some(last_hash) = &last_pdf_hash {
            if *last_hash == current_pdf_hash {
                fs::remove_file(&pdf_filepath)?;
                continue;
            }
        }
        last_pdf_hash = Some(current_pdf_hash);

        // Store commit details
        commit_list.push((
            commit.id().to_string(),
            commit.time().seconds(),
            pdf_filename,
        ));
    }

    // Generate the HTML index
    generate_html_index(&commit_list, &config.html_index_file, &config.github_repo_url, &config.pdf_output_dir)?;
    println!("Processing complete. HTML index generated.");

    Ok(())
}

fn calculate_file_hash(filepath: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(buffer);
    Ok(format!("{:x}", hasher.finalize()))
}

fn generate_html_index(
    commit_list: &Vec<(String, i64, String)>,
    html_index_file: &str,
    github_repo_url: &str,
    pdf_output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(html_index_file)?;
    file.write_all(b"<html><head><title>Commit PDFs</title></head><body>")?;
    file.write_all(b"<h1>Commit PDFs</h1><ul>")?;

    let mut sorted_commits = commit_list.clone();
    sorted_commits.sort_by_key(|(_, date, _)| *date);

    for (hash, date, pdf) in sorted_commits {
        let commit_url = format!("{}/commit/{}", github_repo_url, hash);
        let date_str = NaiveDateTime::from_timestamp_opt(date, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown date".to_string());

        file.write_all(
            format!(
                "<li><a href='{}/{}'>{}</a> - <a href='{}'>{}</a></li>",
                pdf_output_dir, pdf, hash, commit_url, date_str
            )
            .as_bytes(),
        )?;
    }

    file.write_all(b"</ul></body></html>")?;
    Ok(())
}
}



