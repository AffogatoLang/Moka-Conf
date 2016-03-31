# Moka-Conf
A small command line app for creating a module.toml file for a Moka Module

## Installation
Download the mokaconf [binary](https://github.com/AffogatoLang/Moka-Conf/releases/tag/v0.1.0) and place it in 
a folder linked to your path.

e.g. placing mokaconf in `~/.lbin` and adding `EXPORT PATH="$HOME/.lbin":"$PATH"` to the your .bashrc

## Using Moka-Conf
From a terminal, navigate to the folder in which you want a module.toml file and type `mokaconf new` to start the program 
(Future versions of Moka-Conf will have more subcommands). Proceed to enter in the information that is prompted for; 
Moka-Conf will then ask you to confirm the contents of the file and if you accept, it will be written to a `module.toml` 
file in the current directory.

## Building From Source
Moka-Conf requires the latest [Rust Nightly](https://www.rust-lang.org/downloads.html#nightly) for the regex_macros plugin
to compile. Simply download this repo and run "cargo build" in the downloaded folder.
