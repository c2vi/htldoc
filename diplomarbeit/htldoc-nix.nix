{ htldocVersion, ... }: ''

# this is a config file for a diploma thesis, that is built with the htldoc tool
# this tool can be found on https://github.com/c2vi/htldoc
# a list of all options, that can be set here can be found on https://github.com/c2vi/htldoc/blob/main/diplomarbeit/options.md

{ ... }: {
  template = "dipl";

  # the options you HAVE to change
  ###############################################

  title = "TITLE";

  subtitle = "SUBTITLE";

  institute = "INSTITUTE";

  submissionDate = "2024-12-24";

  authors = [ "AUTHOR 1" "AUTHOR 2" ];

  supervisors = [ "SUPERVISOR 1" "SUPERVISOR 2" ];

  # here you define all the chapters of the document
  # this is, because typst and markdown don't have chapters so tha \chapter{} is added by htldoc
  # you therefore must not add a \chapter in your tex src files
  # the order of chapters and files in a chapter is kept like how it's defined here
  # filenames are relative to the ./src folder, subfolders work
  chapters = [
    [ "ChapterName" "file.tex" "reference_shortname" ]
    [ "ChapterName" [ "file1.md" "file2.typ" "file3.tex" ] "reference_shortname" ]
  ];


  # some other usefull options
  ###############################################

  partner = "";

  subject = "";

  keywords = [];

  lang = "ngerman";

  draftMode = false;

  twoSidePrinting = true;

  htldocVersion = "${htldocVersion}"; # can be used to pin a specific version of htldoc

  htldocBuildDir = "./build"; # where to put build artefacts into
}

''
