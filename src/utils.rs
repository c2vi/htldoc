use std::fs::read;
use std::process::Command;
use std::{io, fs};
use std::path::{Path, PathBuf};
use cmd_lib::run_fun;

pub fn nixpkgs_version() -> String {
    // get the nixpkgs version to use
    // use the nixpkgs version from the nixos-version command, if available by default
    // else use the version used by the htldoc flake
    let mut nixpkgs_version: Option<String> = None;
    let nixpkgs_version_output_result = Command::new("nixos-version")
        .arg("--hash")
        .output();
        ;
    if let Ok(nixpkgs_version_output) = nixpkgs_version_output_result {
        if !nixpkgs_version_output.status.success() {
            println!("{}", String::from_utf8(nixpkgs_version_output.stderr).unwrap());
            panic!("failed to get the nixpkgs_version with the nixos-version command");
        } else {
            let mut tmp = String::from_utf8(nixpkgs_version_output.stdout).expect("not utf8");
            tmp.pop(); // remove the \n at the  end
            nixpkgs_version = Some(tmp);
        }
    } else {
        let htldoc_version = htldoc_version();
        if let Ok(res) = run_fun!(nix eval ${htldoc_version}#self.inputs.nixpkgs.rev --raw) {
            nixpkgs_version = Some(res);
        } else {
            println!("using nixpkgs/master, because both previous methods (nixos-version and nixpkgs-used-by-htldoc-flake) failed");
            nixpkgs_version = Some("master".to_owned())
        }
    }

    return nixpkgs_version.unwrap();
}

pub fn htldoc_version() -> String {
    let output = Command::new("nix")
        .arg("eval")
        .arg("--impure")
        .arg("--expr")
        .arg(format!(r#"
            let 
                config = (import ./htldoc.nix {{ }});
            in if builtins.hasAttr "htldocVersion" config then config.htldocVersion else "github:c2vi/htldoc/master"
        "#))
        .output().expect("failed to get the htldocVersion from the htldoc.nix file")
        ;
        
    let mut htldoc_version = String::from_utf8(output.stdout).expect("not utf8");
    htldoc_version.pop(); // remove \n
    htldoc_version.pop(); // remove the " at the end
    htldoc_version.remove(0); // remove the " at the begining
    return htldoc_version;
}


// thanks to: https://stackoverflow.com/a/65192210
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn template_path(htldoc_version: &str) -> PathBuf {
    let template_path_string = run_fun!(nix eval --raw ${htldoc_version}#self.outPath).expect("failed to get #self.outPath of the htldocVersion in the htldoc.nix");

    let template_path = PathBuf::from(template_path_string);

    return template_path;

}


