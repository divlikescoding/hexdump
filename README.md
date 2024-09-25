# Compiling Hexdump

It has been developed using Cargo, so you need to ensure that you have both the Rust Compiler and the Cargo Package Manager installed.

```
git@github.com:divlikescoding/hexdump.git //Clone repo

cd hexdump //Navigaet to hexdump folder

cargo build //Compile the Project
```

# Running Hexdump

After following these steps you will see that there is an executable in the target/debug directory.

```
./target/debug/hexdump [-n LEN] FILE //To run the utility
```

# Description

This hexdump utility is implemented in alignment with the Linux hexdump command. It outputs file contents in lines of 16 bytes, with each line composed of eight 2-byte words. A 32-bit hexadecimal offset is printed at the beginning of each line, indicating the byte offset of the first byte in that line from the start of the file. 

The byte order is determined by the system's endianness. On a little-endian machine, each 2-byte word is displayed with the least significant byte (LSB) first, followed by the most significant byte (MSB). On a big-endian machine, the order is reversed.

This implementation also optimizes output by omitting repetitive lines. If multiple consecutive lines contain identical byte sequences, only the first is displayed, and subsequent matching lines are replaced with an asterisk (*), followed by the next unique line with its corresponding offset.
