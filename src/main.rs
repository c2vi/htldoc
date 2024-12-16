use clap::{ArgAction, ArgMatches};
use clap::{Arg, crate_version, Command};

pub mod init;
pub mod utils;
pub mod build;

fn main() {

    let cli_matches = cli_matches();

    let result = match cli_matches.subcommand() {
        Some(("run-pinned-version", sub_matches)) => {
            println!("not implemented yet, use the htldoc script found in the root of the repo");
            Err("not implemented".to_string())
        },

        Some(("init", sub_matches)) => init::run(sub_matches),

        Some(("build", sub_matches)) => build::run(sub_matches),

        _ => Err("invalid command".to_owned())
    };

    if let Err(e) = result {
        println!("ERROR: {}", e);
    }

}

fn cli_matches() -> clap::ArgMatches {

    let main = Command::new("htldoc")
        .version(crate_version!())
        .author("c2vi (Sebastian Moser)")
        .about("a quick tool to write documents with markdown, typst and latex")
        .subcommand(
                Command::new("run-pinned-version")
                .aliases(["rp"])
                .about("Run the version of htldoc, that is pinned in the htldoc file")
            )
        .subcommand(Command::new("init")
                .about("create a new documentation inside a folder")
                .aliases(["i"])
                .arg(Arg::new("htldocVersion")
                    .long("htldocVersion")
                    .alias("ver")
                    .help("The htldocVersion put into the htldoc.nix file")
                )
                .arg(Arg::new("path"))
            )
        .subcommand(
                Command::new("build")
                .aliases(["b"])
            )
        .arg_required_else_help(true);

    return main.get_matches();
}


