use std::process::Command;
use std::{io, fs};
use std::path::Path;

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
