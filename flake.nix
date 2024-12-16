{
  description = "flake for htldoc";

  inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/release-24.05";

 	  flake-utils.url = "github:numtide/flake-utils";
  };


  outputs = { self, nixpkgs, flake-utils, ... }@inputs: flake-utils.lib.eachDefaultSystem (system: 
  let
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    packages.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "htldoc";
      version = "0.1.0";

      nativeBuildInputs = with pkgs; [ nix coreutils rsync ];

      cargoSha256 = "sha256-qNAIexEx9WhxPa0qIdaaBnjZJiENaiaIML0/aXpbJp0=";

      src = ./.;
    };

    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [ texlive.combined.scheme-full ];
    };


  }) // {
    inherit self;
  };

}
