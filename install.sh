#!/bin/bash

# exit when any command fails
set -e

# Cargo.toml file
cargo_toml_filepath=$(cargo locate-project --message-format plain)

# Rust project dir as an absolute path
project_dir=$(dirname "$cargo_toml_filepath")
project_dir=$(cd "$project_dir" && pwd)

# build the Rust project
cargo build

# extract the project name from the command `cargo tree`
project_name=$(cargo tree -q --depth 0 | grep -o "^\w*\b")

# get the debug dir created by `cargo build`
target_dir=$(realpath "$project_dir")/target
debug_dir=$(realpath "$target_dir")/debug

# path to the executable binary file
bin_src="$debug_dir/$project_name"

# dir holding the executable binary files
BIN_DIR=$HOME/.local/bin

# make the bin dir if it does not exist
if [ ! -d "$BIN_DIR" ]
then
    mkdir -p "$BIN_DIR"
    echo created dir "$BIN_DIR"
fi

# copy the executable file to the bin dir
bin_dest="$BIN_DIR"/"$project_name"
cp "$bin_src" "$bin_dest"
echo "$bin_src" is moved to "$bin_dest"

# bin dir is already in the PATH variable
if [[ $PATH =~ $BIN_DIR ]]; then 
    exit 1
fi

# get the shell rc file
if [[ $SHELL =~ .*zsh ]]; then 
    shell_rc_file=$HOME/.zshrc
elif [[ $SHELL =~ .*bash ]]; then 
    shell_rc_file=$HOME/.bashrc
else
    echo Your shell "$SHELL" is unknown
fi

# add the bin dir to PATH variable

export_command="
# <<< hackpng install.sh <<<
# !! this dir contains self-developed executable files
export PATH=\"\$PATH:$BIN_DIR\"
# >>> hackpng install.sh >>>"

echo "$export_command" >> "$shell_rc_file"
echo Added "$BIN_DIR" to PATH
