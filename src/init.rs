use std::fs::copy;
use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::ArgMatches;

use cmd_lib::run_fun;
use cmd_lib::run_cmd;


pub fn run(sub_matches: &ArgMatches) -> Result<(), String> {

    /////// get the template to init
    let template = match sub_matches.get_one::<String>("template") {
        Some(tempalte) => tempalte.to_owned(),
        None => {
            return Err("no tempalte name specified".to_owned());
        },
    };


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


    //////// get template_dir
    let template_dir = crate::utils::template_dir(htldoc_version.as_str());


    if template.as_str() == "dipl" {
        run_dipl(dest_dir, htldoc_version, template_dir)?;
    } else {
        return Err(format!("template named {} not implemented", template));
    }


    Ok(())
}

pub fn run_dipl(dest_dir: PathBuf, htldoc_version: String, template_dir: PathBuf) -> Result<(), String> {

    ////////// eval the htldoc.nix into there
    let expr = format!("import {}/diplomarbeit/htldoc-nix.nix {{ htldocVersion = \"{}\"; }}", template_dir.display(), htldoc_version);
    run_cmd!(nix eval --raw --impure --expr $expr > ${dest_dir}/htldoc.nix).expect("failed to create htldoc.nix");


    ////////// create the src dir
    create_dir_all(dest_dir.as_path().join("src")).unwrap();


    ////////// create the img dir
    create_dir_all(dest_dir.as_path().join("img")).unwrap();

    //// copy abstract.md
    run_cmd!(cp ${template_dir}/diplomarbeit/abstract.md ${dest_dir}/src/).unwrap();
    run_cmd!(chmod +w ${dest_dir}/src/abstract.md).unwrap();


    /////// write .gitignore file
    std::fs::write(dest_dir.as_path().join(".gitignore"), "/build\n").unwrap();


    ////////// copy abstract file


    Ok(())
}
