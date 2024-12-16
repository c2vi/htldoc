use std::io::stderr;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;
use clap::ArgMatches;
use cmd_lib::run_cmd;

use crate::utils;


pub fn run(sub_matches: &ArgMatches) -> Result<(), String> {
    let mut src_path = std::env::current_dir().map_err(|e| format!("{}", e))?;
    let build_path = src_path.as_path().join("build");
    let htldoc_version = crate::utils::htldoc_version();
    let nixpkgs_rev = crate::utils::nixpkgs_version();
    let template_path = crate::utils::template_path(htldoc_version.as_str());

    // create the build dir
    std::fs::create_dir_all(build_path.as_path());


    // copy template files to build_path
    utils::copy_dir_all(template_path.as_path().join("diplomarbeit").join("latex_template_htlinn"), build_path.as_path()).expect("error while copying template files into the build folder");

    // print path infos
    println!("htldoc_ersion: {}", htldoc_version);
    println!("nixpkgs_rev: {}", nixpkgs_rev);
    println!("src_path: {}", src_path.display());
    println!("template_path: {}", template_path.display());
    println!("build_path: {}", build_path.display());


    let chmod_output = Command::new("chmod")
        .arg("+w")
        .arg("-R")
        .arg(format!("{}", build_path.as_path().display()))
        .output().expect("failed to run chmod +w")
        ;

    //////// copy the source_dir to build_dir
    run_cmd!(PWD=$src_path nix run nixpkgs/${nixpkgs_rev}#rsync -- -r . ./build --exclude build);

    // generate settings.tex from htldoc.nix
    let settings_tex_output = Command::new("nix")
        .arg("eval")
        .arg("--impure")
        .arg("--raw")
        .arg("--expr")
        .arg(format!(r#"
            let 
                htldocFlake = builtins.getFlake "{htldoc_version}";
                pkgs = import htldocFlake.inputs.nixpkgs {{}};
                lib = pkgs.lib;
                defaultConfig = import {}/diplomarbeit/default-config.nix {{ }};
                userConfig = import {}/htldoc.nix {{ }};
                config = userConfig // defaultConfig;
            in import {}/diplomarbeit/latex_template_htlinn/template/settings-tex.nix {{ inherit config lib; }}
        "#, template_path.display(), src_path.display(), template_path.display()))
        .output().expect("failed to eval the $template/diplomarbeit/latex_template_htlinn/template/settings-tex.nix")
        ;
    if !settings_tex_output.status.success() {
        println!("{}", String::from_utf8(settings_tex_output.stderr).unwrap());
        return  Err("failed to eval the $template/diplomarbeit/latex_template_htlinn/template/settings-tex.nix".to_owned());
    }
    let settings_tex = String::from_utf8(settings_tex_output.stdout).expect("not utf8");

    std::fs::write(build_path.as_path().join("template").join("settings.tex"), settings_tex).map_err(|e| format!("from IO err: {}", e))?;


    // run latex build commands

    println!("################### RUNNING PDFLATEX ###################");
    let build_output = Command::new("nix")
        .current_dir(build_path.as_path())
        .arg("shell")
        .arg(format!("nixpkgs/{}#pandoc", nixpkgs_rev))
        .arg(format!("nixpkgs/{}#texlive.combined.scheme-full", nixpkgs_rev))
        .arg("-c")
        .arg("pdflatex")
        .arg("main.tex")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output().expect("failed to run the pdflatex build command")
        ;

    Ok(())
}



