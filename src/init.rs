use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::ArgMatches;

use cmd_lib::run_fun;
use cmd_lib::run_cmd;


pub fn run(sub_matches: &ArgMatches) -> Result<(), String> {

    ////////// get the dest_dir path
    let dest_dir = match sub_matches.get_one::<String>("path") {
        Some(path) => PathBuf::from(path),
        None => {
            PathBuf::from(std::env::current_dir().map_err(|e| format!("{}", e))?)
        },
    };


    ////////// make sure dest_dir exists
    std::fs::create_dir_all(dest_dir.as_path());


    ////////// get the htldocVersion to be used
    // from arg or the latest from master
    let htldoc_version = match sub_matches.get_one::<String>("htldocVersion") {
        Some(val) => val.to_owned(),
        None => {
            let htldoc_rev = run_fun!(nix eval "github:c2vi/htldoc#self.rev" --raw).expect("getting rev of htldoc github repo failed");
            format!("github:c2vi/htldoc/{}", htldoc_rev)
        },
    };


    //////// get template_path
    let template_path = crate::utils::template_path(htldoc_version.as_str());


    run_dipl(dest_dir, htldoc_version, template_path)?;


    Ok(())
}

pub fn run_dipl(dest_dir: PathBuf, htldoc_version: String, template_path: PathBuf) -> Result<(), String> {

    ////////// eval the htldoc.nix into there
    let expr = format!("import {}/diplomarbeit/htldoc-nix.nix {{ htldocVersion = \"{}\"; }}", template_path.display(), htldoc_version);
    run_cmd!(nix eval --raw --impure --expr $expr > ${dest_dir}/htldoc.nix).expect("failed to create htldoc.nix");


    ////////// create the src dir
    create_dir_all(dest_dir.as_path().join("src")).unwrap();


    ////////// create the img dir
    create_dir_all(dest_dir.as_path().join("img")).unwrap();


    /////// write .gitignore file
    std::fs::write(dest_dir.as_path().join(".gitignore"), "/build\n").unwrap();


    ////////// copy abstract file


    Ok(())
}
