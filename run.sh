#!/usr/bin/env bash

DIR_PATH=$"/change/to/absolute/path/"       # Mention the absolute path of the directory here
EXT_LIST=$".txt, .json"                     # Mention the comma separated file extensions (with . ) the program needs to look for. Ex. ".txt, .json"

cargo run -- \
--dir-path ${DIR_PATH} \
--file-extensions "${EXT_LIST}"
