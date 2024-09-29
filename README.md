# rhead

A `head` clone written in Rust.

"head" is a command line utility which gives you the first n lines (default 10) of STDIN or a text file.

# Usage Examples

```
cat /path/to/file | rhead -n 20
rhead -n 1 /etc/passwd
```