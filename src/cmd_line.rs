pub struct CommandLineArgs {
    pub size: u32,
    pub file_path: String
}

const LEN_OPTION_FLAG: &str = "-n";


/**
 * This function parses the Command Line Arguments and either returns an error of an instance of CommandLineArg struct or an error code
 * If the parsing is unsuccessful use the print_message_from_error_code function to print appropriate error message 
 * If Success -> CommandLineArgs struct. If size == 0, that means process the whole file
 */
pub fn parse_cmd_line_args() -> Result<CommandLineArgs, i32> {
    let mut return_value = CommandLineArgs {
        size: 0,
        file_path: String::new()
    };

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        /*
            This condition is reached if there is no arguments passed
            Error Code of 0 is emitted. This is where we would display the usage string to the user
        */
        return Err(0);
    }

    let mut is_next_len = false;
    let mut is_file_name_parsed = false;

    for curr_arg in args {
        if curr_arg == LEN_OPTION_FLAG {
            is_next_len = true;
        } else if is_next_len {
            /* 
                This is parsing the len argument from the command-line. 
                Error Code -1 is emitted if the value supplied after the -n is not a positive integer which fits in the length of 32 bits
            */
            return_value.size = match curr_arg.parse::<u32>() {
                Ok(parsed_int) => parsed_int,
                Err(_err) => return Err(-1)
            };
            is_next_len = false;
        } else {
            if !is_file_name_parsed {
                return_value.file_path = curr_arg;
                is_file_name_parsed = true;
            } else {
                /*
                    This condition is reached if there is another string supplied after the file name. 
                    This is ambigious as we don't know if the first string or the next string is the file name
                    Error Code -2 is emitted if there is another string(s) supplied after the first one which corresponds to the file name
                */
                return Err(-2);
            }
        }

    }

    if !is_file_name_parsed {
        /*
            This condition is reached if there is no file name supplied
            Error Code -3 is emitted if there is no file name supplied
        */
        Err(-3)
    } else {
        Ok(return_value)
    }

}

/**
 * Prints the appropriate message to the user given the error code returned by the parse_cmd_line_args function
 */
pub fn print_message_from_error_code(error_code: i32) {
    let program_name = get_program_name();
    match error_code {
        0 => print_usage_string(&program_name),
        -1 => print_invalid_len_error(&program_name),
        -2 => print_ambigious_file_name_error(&program_name),
        -3 => print_no_file_name_error(&program_name),
        _ => {}
    }
}

/**
 * Returns the name of the executable
 */
pub fn get_program_name() -> String {
    std::env::args().collect::<Vec<String>>()[0].clone() 
}

fn print_usage_string(program_name: &String) {
    println!("Usage: {} [-n LEN] FILE", program_name);
}

fn print_invalid_len_error(program_name: &String) {
    println!("{}: Invalid parameter was specified for the len in the -n option", program_name);
}

fn print_ambigious_file_name_error(program_name: &String) {
    println!("{}: More than one file name was specified", program_name);
}

fn print_no_file_name_error(program_name: &String) {
    println!("{}: No file name was specified", program_name);
}

