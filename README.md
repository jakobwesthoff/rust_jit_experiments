# 🦀 JIT Playground 🧪

**⚠️ WARNING: THIS REPOSITORY MAY CONTAIN HARMFUL CODE. DO NOT COMPILE AND EXECUTE IT UNLESS YOU ARE FAMILIAR WITH ITS CONTENTS**

This repository serves as a playground for experimental tests conducted in preparation for developing a Just-In-Time (JIT) compiler for my [rust_brain](http://github.com/jakobwesthoff/rust_brain) project. The main objective of this endeavor was to create an environment for exploring the execution of x86 code on Apple Silicon (ARM) based Macs, and to delve into the dynamic mapping of executable code regions from within Rust.

For a detailed walkthrough of the development process, feel free to watch the [video on my channel](https://www.youtube.com/@MrJakob).

## Build Instructions

To build the project, start by running the `./enter-x86-env.sh` script to access a shell within an x86 docker container (especially if you are on a non-x86 platform ;)).

After that, simply execute `cargo build` to compile the project.

### Building Assembly Files

The files located in the `asm` directory are not intended for direct execution; they are designed to generate the necessary byte codes for our project. You can inspect them using `ndisasm`.

```shell
cd asm
nasm -o foo.o return_value.s
ndisasm -b64 foo.o
```
