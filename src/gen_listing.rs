use cmd_lib::run_fun;
use cmd_lib::run_cmd;


pub fn run() -> Result<(), String> {
    let nixpkgs_version = crate::utils::nixpkgs_version();
    let build_dir = crate::utils::get_build_dir();


    // write the config.json for the script.py
    let expr = format!(r#"
        let 
            config = (import ./htldoc.nix {{ }});
            default = {
                branch = "master";
                pdf_output_dir = {build_dir}/listings
                html_index_file = {build_dir}/listings/index.html
                githubRepoUrl = "";
            };
        in default // config.genListing
    "#);
    run_cmd!(nix eval --expr ${expr} --json > ${build_dir}/gen_log_config.json);


    // create the listing dir in the build_dir
    run_cmd!(mkdir -p ${build_dir}/listing);


    // create the listing_src dir
    run_cmd!(mkdir -p ${build_dir}/listing);


    // run the python script
}



