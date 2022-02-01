# number_of_lines <br />

## Pre-requisites <br />

- **Installing Curl** <br />
Type the following command on your shell and follow the on-scren instructions <br />
```sudo apt update``` <br />
```sudo apt install curl``` <br />

- **Installing Rust** <br />
Type the following command on your shell and follow the on-scren instructions <br />
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` <br />

## Program Functionality <br />
### This program takes a directory path and list of file extensions as arguments and prints of the total number of lines present in all the files having the supplied extensions in all sub-directories present in the supplied path <br /> 

### Running the program : <br />
- give chmod 777 permission to the run script: <br />
```chmod 777 run.sh``` <br /><br />

- Provide the absolute path of the required directory to this shell valiable: <br />
```DIR_PATH=$"/change/to/absolute/path/"```

- Provide the comma separated file extensions to this shell valiable, pass empty string ```""``` if all files need to be checked : <br />
```EXT_LIST=$".txt, .json"```

- Run the program using the following command : <br />
```./run.sh```
