use std::process::Command;
use std::path::PathBuf;

use clap::ArgMatches;


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
    println!("src_path: {}", src_path.display());
    println!("template_path: {}", template_path.display());
    println!("build_path: {}", build_path.display());


    // generate settings.tex from config.nix
    let settings_tex_output = Command::new("nix")
        .arg("eval")
        .arg("--impure")
        .arg("--raw")
        .arg("--expr")
        .arg(format!(r#"
            let 
                htldocFlake = builtins.getFlake {htldoc_version};
                pkgs = import htldocFlake.inputs.nixpkgs {{}};
                lib = pkgs.lib;
                defaultConfig = import {}/diplomarbeit/default-config.nix {{ }};
                userConfig = import {}/htldoc.nix {{ }};
                config = defaultConfig // userConfig;
            in import {}/diplomarbeit/latex_template_htlinn/template/settings-tex.nix {{ inherit config lib; }}
        "#, template_path.display(), src_path.display(), template_path.display()))
        .output().expect("failed to eval the $template/diplomarbeit/latex_template_htlinn/template/settings-tex.nix")
        ;
    if !settings_tex_output.status.success() {
        println!("{}", String::from_utf8(settings_tex_output.stderr).unwrap());
        return  Err("failed to eval the $template/diplomarbeit/latex_template_htlinn/template/settings-tex.nix".to_owned());
    }
    let settings_tex = String::from_utf8(settings_tex_output.stdout).expect("not utf8");
    println!("settings_tex: {}", settings_tex);
    println!("template_path: {}", template_path.display());
    println!("htldocVersion: {}", htldoc_version);
    println!(r#"
            let 
                defaultConfig = import {}/diplomarbeit/default-config.nix {{ }};
                userConfig = import {}/htldoc.nix {{ }};
                config = defaultConfig // userConfig;
            in import {}/diplomarbeit/latex_template_htlinn/template/settings-tex.nix config
        "#, template_path.display(), src_path.display(), template_path.display());
    std::fs::write(build_path.as_path().join("template").join("settings.tex"), settings_tex).map_err(|e| format!("from IO err: {}", e))?;


    // run latex build commands
    /*
    let settings_tex = Command::new("nix")
        .arg("eval")
        .arg("--impure")
        .arg("--expr")
        .arg(format!(r#"
            let 
                defaultConfig = import {template_path}/diplomarbeit/default-config.nix {};
                userConfig = import {src_path}/htldoc.nix {};
                config = defaultConfig // userConfig;
            in import {template_path}/diplomarbeit/latex_template_htlinn/template/settings-tex.nix config
        "#,))
        .output().expect("failed to eval the $template/diplomarbeit/latex_template_htlinn/template/settings-tex.nix")
    */



    Ok(())
}



