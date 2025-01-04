use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use chrono::NaiveDateTime;
use clap::ArgMatches;
use git2::{Repository, Commit};
use sha2::{Digest, Sha256};
use cmd_lib::run_fun;
use cmd_lib::run_cmd;
use serde::Deserialize;


#[derive(Deserialize)]
struct Config {
    #[serde(rename = "branch")]
    branch_name: String,

    #[serde(rename = "startCommit")]
    start_commit: String,

    pdf_output_dir: String,

    html_index_file: String,

    #[serde(rename = "githubRepoUrl")]
    github_repo_url: String,
}



pub fn run(sub_matches: &ArgMatches) -> Result<(), String> {
    let nixpkgs_version = crate::utils::nixpkgs_version();
    let htldoc_version = crate::utils::htldoc_version();
    let build_dir = crate::utils::get_build_dir();
    let src_dir = std::env::current_dir().unwrap();


    // write the config.json for the script.py
    let expr = format!(r#"
        let 
            config = (import ./htldoc.nix {{ }});
            default = {{
                branch = "master";
                pdf_output_dir = "{}/listing";
                html_index_file = "{}/listing/index.html";
                githubRepoUrl = "";
                startCommit = "";
            }};
        in default // (if builtins.hasAttr "genListing" config then config.genListing else {{ }})
    "#, build_dir.display(), build_dir.display());
    run_cmd!(nix eval --expr ${expr} --impure --json > ${build_dir}/gen_listing_config.json).unwrap();


    // create the listing dir in the build_dir
    run_cmd!(mkdir -p ${build_dir}/listing).unwrap();


    // create the listing_src dir
    run_cmd!(mkdir -p ${build_dir}/listing_src).unwrap();


    // run the ChatGPT generated code, that actually builds all pdfs
    let config_path = format!("{}/gen_listing_config.json", build_dir.display());
    let mut config: Config = serde_json::from_reader(File::open(config_path).unwrap()).unwrap();

    // Ensure output directory exists
    fs::create_dir_all(&config.pdf_output_dir).unwrap();

    // Clone the git repo at
    let src_repo_base_dir = Repository::discover_path(src_dir, Vec::<PathBuf>::new()).unwrap();
    run_cmd!(rm -rf ${build_dir}/listing_src).unwrap();
    run_cmd!(mkdir -p ${build_dir}/listing_src).unwrap();
    let repo = Repository::clone(src_repo_base_dir.to_str().unwrap(), format!("{}/listing_src", build_dir.display())).unwrap();

    // Get the target branch
    let branch = repo.find_branch(&config.branch_name, git2::BranchType::Local).unwrap();
    let branch_commit = branch.get().peel_to_commit().unwrap();

    // Start processing commits
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.set_sorting(git2::Sort::TIME.union(git2::Sort::REVERSE));
    revwalk.push(branch_commit.id()).unwrap();

    let mut found_start_commit = false;
    let mut last_pdf_hash: Option<String> = None;
    let mut commit_list = Vec::new();

    // if config.start_commit is empty, start at the first commit of the branch
    if config.start_commit.as_str() == "" {
        config.start_commit = format!("{}", branch_commit.id());
    }

    for oid in revwalk {
        let oid = oid.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        if !found_start_commit {
            if commit.id().to_string() == config.start_commit {
                found_start_commit = true;
            } else {
                continue;
            }
        }

        // Checkout the commit
        repo.checkout_tree(&commit.as_object(), None).unwrap();
        repo.set_head_detached(commit.id()).unwrap();

        // build the pdf for this commit
        println!("building pdf for commit: {}", commit.id());
        let pdf_filename = format!("{}.pdf", commit.id());
        let pdf_filepath = Path::new(&config.pdf_output_dir).join(&pdf_filename);

        let verbose_flag = match sub_matches.get_flag("verbose") {
            true => "-v",
            false => "",
        };

        // TODO, this runs the htldoc version of the newest version.... should run the one, that
        // was pinned on the commit we are building
        // a problem: if there was a local version pinned once, that is no longer available....
        // so i'd say: be able to override to always use the newest version
        run_cmd!( cd ${build_dir}/listing_src; nix run ${htldoc_version} -- build $verbose_flag).unwrap();

        // for Latex to generate the refs corectly, we need to build it twice.....
        run_cmd!( cd ${build_dir}/listing_src; nix run ${htldoc_version} -- build $verbose_flag).unwrap();

        // will break when the htldocBuildDir is not set to "build"
        // TODO: be able to pass a --config htldoc_version=build
        run_cmd!( cp ${build_dir}/listing_src/build/out.pdf ${pdf_filepath} ).unwrap(); 
                                                                              


        // Check if the PDF content has changed
        let current_pdf_hash = calculate_file_hash(&pdf_filepath).unwrap();
        if let Some(last_hash) = &last_pdf_hash {
            if *last_hash == current_pdf_hash {
                fs::remove_file(&pdf_filepath).unwrap();
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
    generate_html_index(&commit_list, &config.html_index_file, &config.github_repo_url, &config.pdf_output_dir).unwrap();
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



