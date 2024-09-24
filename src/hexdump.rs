use crate::cmd_line::CommandLineArgs;

pub fn hexdump(cmd_line_args: CommandLineArgs) {
    println!("Success. Size: {} File Name: {}", cmd_line_args.size, cmd_line_args.file_path);
}