# Compiling Hexdump

It has been developed using Cargo, so you need to ensure that you have both the Rust Compiler and the Cargo Package Manager installed.

```
git clone git@github.com:divlikescoding/hexdump.git //Clone repo

cd hexdump //Navigate to hexdump folder

cargo build //Compile the Project
```

# Running Hexdump

After following these steps you will see that there is an executable in the target/debug directory.

```
./target/debug/hexdump [-n LEN] FILE //To run the utility
```

# Description

This hexdump utility is designed to mirror the functionality of the Linux hexdump command. It formats file content into lines of 16 bits, with each line consisting of eight 2-byte words. At the beginning of each line, a 32-bit hexadecimal offset is displayed, representing the byte offset of the first byte on that line relative to the start of the file. The byte order adhres to the system's endianness. Additionally, the total number of bytes processed is printed at the end of the output.

To enhance output efficiency, the utility suppresses consecutive duplicate lines. When identical byte sequences appear across multiple consecutive lines, only the first line is printed, and subsequent matching lines are replaced by an asterisk (*). The output then resumes with the next unique line and its corresponding offset.
