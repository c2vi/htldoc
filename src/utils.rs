use std::process::Command;

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
