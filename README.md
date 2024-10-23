# CREATEFILE

A command line tool to create a dummy file of an arbitrary size.

## Usage

    Usage: createfile.exe [OPTIONS] --name <NAME>

    Options:
    -n, --name <NAME>  Name or path to the file
    -s, --size <SIZE>  Size of the file [default: 5]
    -u, --unit <UNIT>  Unit of measurement [default: m] [possible values: b, k, m, g]
    -h, --help         Print help (see more with '--help')
    -V, --version      Print version


## Installation

Install via `cargo`:

     cargo install https://github.com/maciakl/createfile/ 
 
 On Windows, this tool is also distributed via `scoop` (see [scoop.sh](https://scoop.sh)).

 First, you need to add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket
    scoop update

 Next simply run:
 
    scoop install createfile
