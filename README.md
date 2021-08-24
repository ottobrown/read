# read
### A super simple program to read the contents of a file or directory

## Installation
Simply cargo build from the source

    cargo build --release

## Usage
    read.exe <Options> <path>

## Config

### read style
- default: lines are printed all at once
- line-by-line: lines are printed 1-by-1, requiring the user to press enter to print the next line
- top: lines are printed all at once, and the cursor is moved to the top