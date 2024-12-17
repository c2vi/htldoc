use std::io::stderr;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;
use clap::ArgMatches;
use cmd_lib::run_cmd;
use cmd_lib::run_fun;

use crate::utils;


pub fn run(sub_matches: &ArgMatches) -> Result<(), String> {
    let mut src_dir = std::env::current_dir().map_err(|e| format!("{}", e))?;
    let build_dir = src_dir.as_path().join("build");
    let htldoc_version = crate::utils::htldoc_version();
    let nixpkgs_rev = crate::utils::nixpkgs_version();
    let template_dir = crate::utils::template_dir(htldoc_version.as_str());

    /////// create the build dir
    std::fs::create_dir_all(build_dir.as_path());



    /////// print path infos
    println!("htldoc_ersion: {}", htldoc_version);
    println!("nixpkgs_rev: {}", nixpkgs_rev);
    println!("src_dir: {}", src_dir.display());
    println!("template_dir: {}", template_dir.display());
    println!("build_dir: {}", build_dir.display());


    //////// sync the source_dir to build_dir
    run_cmd!(PWD=$src_dir nix run nixpkgs/${nixpkgs_rev}#rsync -- -r . ./build --exclude build);


    /////// find out what template shall be built
    let expr = format!("let userConfig = import {}/htldoc.nix {{ }}; in userConfig.template", src_dir.display());
    let res = run_fun!(nix eval --expr $expr --raw --impure).unwrap();

    if res.as_str() == "dipl" {
        return build_dipl(template_dir, build_dir, src_dir, htldoc_version, nixpkgs_rev);
    } else {
        panic!("template: '{}' not known", res);
    }
    


    Ok(())
}

pub fn build_dipl(template_dir: PathBuf, build_dir: PathBuf, src_dir: PathBuf, htldoc_version: String, nixpkgs_rev: String) -> Result<(), String> {

    /////// copy template files to build_dir
    utils::copy_dir_all(template_dir.as_path().join("diplomarbeit").join("latex_template_htlinn"), build_dir.as_path()).expect("error while copying template files into the build folder");

    let chmod_output = Command::new("chmod")
        .arg("+w")
        .arg("-R")
        .arg(format!("{}", build_dir.as_path().display()))
        .output().expect("failed to run chmod +w")
        ;



    ////// generate settings.tex from htldoc.nix
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
        "#, template_dir.display(), src_dir.display(), template_dir.display()))
        .output().expect("failed to eval the $template/diplomarbeit/latex_template_htlinn/template/settings-tex.nix")
        ;
    if !settings_tex_output.status.success() {
        println!("{}", String::from_utf8(settings_tex_output.stderr).unwrap());
        return  Err("failed to eval the $template/diplomarbeit/latex_template_htlinn/template/settings-tex.nix".to_owned());
    }
    let settings_tex = String::from_utf8(settings_tex_output.stdout).expect("not utf8");

    std::fs::write(build_dir.as_path().join("template").join("settings.tex"), settings_tex).map_err(|e| format!("from IO err: {}", e))?;


    ////// generate abstract.tex
    run_cmd!(nix run nixpkgs/${nixpkgs_rev}#pandoc -- --from markdown --to latex ${src_dir}/src/abstract.md -o ${build_dir}/abstract.tex).unwrap();


    ////// run latex build commands
    println!("################### RUNNING PDFLATEX ###################");
    let build_output = Command::new("nix")
        .current_dir(build_dir.as_path())
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



