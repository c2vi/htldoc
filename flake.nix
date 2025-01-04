{
  description = "flake for htldoc";

  inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/release-24.05";

 	  flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
    };

  };


  outputs = { self, nixpkgs, flake-utils, crane, ... }@inputs: flake-utils.lib.eachDefaultSystem (system: 
  let
    pkgs = nixpkgs.legacyPackages.${system};
    craneLib = crane.mkLib pkgs;
  in
  {

    packages.default = craneLib.buildPackage {
      src = craneLib.cleanCargoSource ./.;

      buildInputs = with pkgs; [ nix coreutils rsync ];
    };



    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [ texlive.combined.scheme-full ];
    };


  }) // {
    inherit self;
  };

}
