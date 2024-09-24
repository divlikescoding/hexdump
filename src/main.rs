mod cmd_line;
mod hexdump;

fn main() {
    let cmd_line_args_or_error = cmd_line::parse_cmd_line_args();
    match cmd_line_args_or_error {
        Ok(cmd_line_args) => hexdump::hexdump(cmd_line_args),
        Err(error_code) => cmd_line::print_message_from_error_code(error_code),
    }
}
