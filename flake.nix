{
  description = "flake for htldoc";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

 	  flake-utils.url = "github:numtide/flake-utils";
  };


  outputs = { self, nixpkgs, flake-utils, ... }@inputs: flake-utils.lib.eachDefaultSystem (system: 
  let
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    packages.dipl.typst = pkgs.writeShellApplication {
      name = "build typst docs";
      runtimeInputs = with pkgs; [
        texlive.combined.scheme-full
        pandoc
      ];
      text = ''
        echo pwd is: $(pwd)
      '';
    };

    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [ texlive.combined.scheme-full ];
    };

  });
}
