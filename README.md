# rhead

A `head` clone written in Rust.

"head" is a command line utility which gives you the first n lines (default 10) of STDIN or a text file. It can also give you just the first `c` bytes of a file.

# Usage Examples

```
# Handle text from Stdin
cat /path/to/file | rhead -n 20

# Handle text in a file
rhead -n 1 /etc/passwd
```

```
Usage: rhead [OPTIONS] [FILE]

Arguments:
  [FILE]  The file to read from (optional)

Options:
  -n, --number <NUMBER>  The number of lines to print [default: 10]
  -c, --chars <CHARS>    The number of characters to print (optional) [default: 0]
  -h, --help             Print help
  -V, --version          Print version
```

# Limitations

Input data stream or input file MUST contain UTF-8 otherwise it will be rejected.