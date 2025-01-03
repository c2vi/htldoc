
# A script to generate html listing
With this script you can generate a listing in the form of a folder, that can be uploaded to any static webserver, of every commit that changed the resulting pdf (including the pdf).

To generate the listing
```bash
htldoc gen_listing
# or htldoc gl
```


## The Options
You can just add the following to the htldoc.nix file in your repo and fill in what you need.
```nix
genListing = {

    rsyncDestinationArg = "";

    branch = "";

    startingCommit = "";

    uploadCommand = "";

}
```

The default options with documentation:
```nix
genListing = {

    # with this option set rsync is used to copy the listing to the webserver
    # you can specify the destination arg passed to the rsync invocation
    rsyncDestinationArg = "";

    # the branch to be used for generating the listing
    branch = "master";

    # the earliest commit to include in the listing
    startingCommit = "<first one>";

    # the command to run after generating the listing to upload it to a webserver
    # in case you need something more custom than an rsync command
    # this command will be interpreted by bash having LISTING_OUT_DIR set to the directory of the generated listing
    uploadCommand = "";

}
```






