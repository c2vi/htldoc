# htldoc

Templates and a small Tool to write documentation and project reports for the HTL (= Technical High School here in Austria)

# Template credits
The Latex template found in ./diplomarbeit/original_latex_template_htlinn is maintained by the [HTL Anichstraße](htlinn.ac.at) and based on the VWA template from the TU Graz, which originally was [here](https://latex.tugraz.at/vorlagen/allgemein), but the project Latex@Tu-Graz is now archived and the template found on the [webarchive](http://web.archive.org/web/20230401165622/https://latex.tugraz.at/vorlagen/allgemein)

# Usage
For now this tool needs the [Nix Package Manager](https://nixos.org/) installed. ([How to install](https://nixos.org/download/))

To build a htldoc document, run `htldoc build` in a folder, with a `htldoc.nix` file.

## Diploma thesis
To create a new htldoc diploma thesis in PATH. If no PATH is specified, the working directory will be used.
```
htldoc init diploma [PATH]
```

## HTL Documentation
TODO

## Application Documents
TODO
