use std::{fs::File, io::Error, io::BufReader, io::Read};

use crate::cmd_line::{get_program_name, CommandLineArgs};

/**
 * Given the CommandLineArgs struct parsed by the parse_cmd_line_args function, it prints the hex dump of the given file with the given arguments
 */
pub fn hexdump(cmd_line_args: CommandLineArgs) {
    let file_obj = match File::open(cmd_line_args.file_path) {
        Ok(file_obj) => file_obj,
        Err(err) => {
            print_message_from_error_code(err);
            std::process::exit(0);
        }
    };

    let file_buffer = BufReader::new(file_obj);

    let mut byte_count: u32 = 0;
    let mut offset: u32 = 0;

    let mut curr_bytes: Vec<u8> = Vec::new();
    let mut prev_bytes: Vec<u8> = Vec::new();
    let mut is_star_printed: bool = false;

    let is_little_endian = check_is_little_endian();

    for byte_or_error in file_buffer.bytes() {
        if cmd_line_args.size > 0 && byte_count == cmd_line_args.size {
            break;
        }
        let byte = byte_or_error.unwrap();
        if curr_bytes.len() < 16 {
            curr_bytes.push(byte);
        } else {
            print_byte_line(&curr_bytes, &prev_bytes, &mut is_star_printed, 
                &mut offset, is_little_endian);
            prev_bytes = curr_bytes;
            curr_bytes = vec![byte];
        }
        byte_count = byte_count + 1;
    };

    print_byte_line(&curr_bytes, &prev_bytes, &mut is_star_printed, &mut offset, 
        is_little_endian);

    println!("{}", get_hex_value_for_byte_vec(&byte_count.to_le_bytes()));
}

fn get_byte_line(bytes: &Vec<u8>, offset: u32, is_little_endian: bool) -> String {
    //let mut return_value: String = get_hex_value_for_four_byte(offset);
    let mut return_value: String = get_hex_value_for_byte_vec(&offset.to_le_bytes());
    let mut curr_word: [u8; 2] = [0, 0];
    let mut byte_index: u8 = 0;
    for curr_byte_ref in bytes {
        let curr_byte = *curr_byte_ref;
        if is_little_endian {
            if byte_index == 1 {
                curr_word[1] = curr_byte;
            } else {
                curr_word[0] = curr_byte;
            }
        } else {
            if byte_index == 0 {
                curr_word[1] = curr_byte;
            } else {
                curr_word[0] = curr_byte;
            }
        }
        if byte_index == 0 {
            byte_index = byte_index + 1;
        } else {
            return_value = format!("{} {}", return_value, 
                get_hex_value_for_byte_vec(&curr_word));
            byte_index = 0;
            curr_word = [0, 0];
        }
    }

    /* If there are an odd number of bytes in the sequence then print the last byte in the MSB position of the next word  */
    if byte_index == 1 {
        return_value = format!("{} {}", return_value, 
            get_hex_value_for_byte_vec(&curr_word));
    }

    return_value
}

fn print_byte_line(curr_bytes: &Vec<u8>, prev_bytes: &Vec<u8>, 
    is_star_printed: &mut bool, offset: &mut u32, is_little_endian: bool) {
    let is_bytes_equal_to_prev = is_byte_vec_equal(&curr_bytes, 
        &prev_bytes);
    if is_bytes_equal_to_prev {
        if !*is_star_printed {
            println!("*");
            *is_star_printed = true;
        }
    } else {
        let curr_line = get_byte_line(&curr_bytes, *offset, 
            is_little_endian);
        println!("{}", curr_line);
        *is_star_printed = false;
    }

    *offset = *offset + (curr_bytes.len() as u32);
}

fn is_byte_vec_equal(left_bytes: &Vec<u8>, right_bytes: &Vec<u8>) -> bool {
    if left_bytes.len() != right_bytes.len() {
        return false;
    } else {
        for index in 0..left_bytes.len() {
            if left_bytes[index] != right_bytes[index] {
                return false;
            }
        }
        return true;
    }
}

fn get_hex_value_for_byte_vec(byte_vec: &[u8]) -> String {
    let mut return_value: String = String::new();
    for curr_byte_ref in byte_vec {
        let curr_byte: u8 = *curr_byte_ref;
        let lower_nibble: u8 = curr_byte & 0x0f;
        let upper_nibble: u8 = (curr_byte >> 4) & 0x0f;

        let lower_nibble_char: char = get_hex_value_for_nibble(lower_nibble);
        let upper_nibble_char: char = get_hex_value_for_nibble(upper_nibble);

        return_value.insert(0, lower_nibble_char);
        return_value.insert(0, upper_nibble_char);
    }

    return_value
}

fn get_hex_value_for_nibble(nibble: u8) -> char {
    match nibble {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => '\0'
    }
}

fn check_is_little_endian() -> bool {
    let value = u32::from_le_bytes([1, 0, 0, 0]);
    value == 1
}

fn print_message_from_error_code(err: Error) {
    let program_name = get_program_name();
    println!("{}: {}", program_name, err);
}