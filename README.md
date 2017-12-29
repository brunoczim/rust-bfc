# rust-bfc
A Brainfuck compiler written in Rust. Avaible for x86-64 and x86 GNU/Linux.

# How to use
```
bfc [options] file
options:
    -a X                      Sets the architecture to X, where X can be `x86` or `amd64`.Instead of `amd64`, `x86_64`, `x86-64` or `x64` could also be written. Must be defined only once.
    -f X                      Sets the format to X, where X can be `asm` or `elf`. Must be defined only once
    -h, --help                Shows this help message and exits. File argument is not necessary in this case.
    -o X                      Sets output file to X. Must be defined only once.
```

# Goals
To show basic compiler fundamentals.

# Extra info
* Cell size: 16 bits.
* GetChar operation return value on eof: -1.
* Increment or decrement overflow: wrapped.
* Segmentation Fault possibility: A tape that is too big had been created,
  or the tape address is too low.
