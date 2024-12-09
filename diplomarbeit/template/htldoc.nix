
# this is a config file for a diploma thesis, that is built with the htldoc tool
# this tool can be found on https://github.com/c2vi/htldoc
# a list of all options, that can be set here can be found on https://github.com/c2vi/htldoc/blob/main/diplomarbeit/options.md

{ ... }: {
  template = "diploma";

  # the options you HAVE to change
  ###############################################

  title = "TITLE";

  subtitle = "SUBTITLE";

  institute = "INSTITUTE";

  submissionDate = "2024-12-24";

  authors = [ "AUTHOR 1" "AUTHOR 2" ];

  supervisors = [ "SUPERVISOR 1" "SUPERVISOR 2" ];


  # some other usefull options
  ###############################################

  partner = "";

  subject = "";

  keywords = [];

  lang = "ngerman";

  draftMode = false;

  twoSidePrinting = true;

  htldocVersion = "github:c2vi/htldoc/master"; # can be used to pin a specific version of htldoc

  htldocBuildDir = "./build"; # where to put build artefacts into
}
