use std::io::stderr;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::utils;


pub fn run(sub_matches: &ArgMatches) -> Result<(), String> {
    let mut src_path = std::env::current_dir().map_err(|e| format!("{}", e))?;
    let build_path = src_path.as_path().join("build");
    let htldoc_version = crate::utils::htldoc_version();
    
    // create the build dir
    std::fs::create_dir_all(build_path.as_path());


    // copy template files to there
    let template_path_output = Command::new("nix")
        .arg("eval")
        .arg(format!("{}#self.outPath", htldoc_version))
        .output().expect("failed to get #self.outPath of the htldocVersion in the htldoc.nix")
        ;
    let mut template_path_string = String::from_utf8(template_path_output.stdout).expect("not utf8");
    if !template_path_output.status.success() {
        println!("{}", String::from_utf8(template_path_output.stderr).unwrap());
        return  Err("failed to get the template_path_output".to_owned());
    }
    template_path_string.pop(); // remove \n
    template_path_string.pop(); // remove the " at the end
    template_path_string.remove(0); // remove the " at the begining
    let template_path = PathBuf::from(template_path_string);

    utils::copy_dir_all(template_path.as_path().join("diplomarbeit").join("latex_template_htlinn"), build_path.as_path()).expect("error while copying template files into the build folder");

    // print path infos
    println!("htldocVersion: {}", htldoc_version);
    println!("src_path: {}", src_path.display());
    println!("template_path: {}", template_path.display());
    println!("build_path: {}", build_path.display());


    let chmod_output = Command::new("chmod")
        .arg("+w")
        .arg("-R")
        .arg(format!("{}", build_path.as_path().display()))
        .output().expect("failed to run chmod +w")
        ;


    // generate settings.tex from config.nix
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
    // github:nixos/nixpkgs/$(nixos-version --hash)
    // use the nixpkgs version from the nixos-version command, if available
    let mut nixpkgs_version: Option<String> = None;
    let nixpkgs_version_output_result = Command::new("nixos-version")
        .arg("--hash")
        .output();
        ;
    if let Ok(nixpkgs_version_output) = nixpkgs_version_output_result {
        if !nixpkgs_version_output.status.success() {
            println!("{}", String::from_utf8(nixpkgs_version_output.stderr).unwrap());
            return  Err("failed to get the nixpkgs_version with the nixos-version command".to_owned());
        } else {
            let mut tmp = String::from_utf8(nixpkgs_version_output.stdout).expect("not utf8");
            tmp.pop(); // remove the \n at the  end
            nixpkgs_version = Some(tmp);
        }
    } else {
        nixpkgs_version = Some("master".to_owned())
    }

    let build_output = Command::new("nix")
        .current_dir(build_path.as_path())
        .arg("shell")
        .arg(format!("nixpkgs/{}#pandoc", nixpkgs_version.as_ref().unwrap()))
        .arg(format!("nixpkgs/{}#texlive.combined.scheme-full", nixpkgs_version.as_ref().unwrap()))
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



