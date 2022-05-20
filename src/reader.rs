use std::cmp::min;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const LF_BYTE: u8 = '\n' as u8;
const CR_BYTE: u8 = '\r' as u8;

const BUFFER_SIZE: usize = 4;

#[allow(dead_code)]
fn show_lines (lines: &Vec<String>) {
    println!("Lines ==== {}", lines.len());
    for (index, line) in lines.iter().enumerate() {
        println!("[{}] line: {}", index, line)
    }
}

#[allow(dead_code)]
fn show_hex_buf (buffer: &[u8]) {
    print!("[");
    for c in buffer {
        print!(" {:#01x} ", c)
    }
    println!("]");
}

pub async fn read_lines(file_name: &str, num_lines: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut buffer = [0u8; BUFFER_SIZE as usize]; // буфер для чтения из файла, байты
    let mut str_buffer: Vec<u8> = Vec::new();     // вектор для формирования строки

    let mut my_file = File::open(file_name).unwrap();

    let file_size = my_file.seek(SeekFrom::End(0)).unwrap();
    // println!("file size: {}", file_size);

    if file_size < 1 {
        return lines;
    }
    let mut file_pos = file_size as usize;

    while lines.len() < num_lines {
        let mut offset = min(BUFFER_SIZE, file_pos) as i64;

        let cur_pos = my_file.seek(SeekFrom::Current(-offset)).unwrap();
        // println!("offset {} cur_pos: {}", offset, cur_pos);

        my_file.read_exact(&mut buffer[0..(offset as usize)]).unwrap();

        my_file.seek(SeekFrom::Current(-offset)).unwrap();
        file_pos -= offset as usize;

        // show_hex_buf(&buffer);

        while offset > 0  {

            let ch = buffer[offset as usize-1];
            // println!(" {:#01x} ", ch);

            offset -=1;

            if ch == LF_BYTE {
                str_buffer.reverse();
                let str = String::from_utf8(str_buffer).unwrap();
                // println!("str is '{}' ", str);
                str_buffer = Vec::new();
                lines.push(str);
                // println!("lines.len is '{}' ", lines.len());
                if lines.len() == num_lines {
                    // println!("NUM_LINES end");
                    break
                }
            }
            else if ch == CR_BYTE {}
            else {
                str_buffer.push(ch)
            }
        }

        // if file_pos == 0 {
        //     break
        // }

        if cur_pos == 0 {
        //     str_buffer.reverse();
            // println!("cur_pos=0");
            // show_hex_buf(&str_buffer);
            // let str = String::from_utf8(str_buffer).unwrap();
            // lines.push(str);
            break
        }
    }

    // show_lines(&lines);

    lines.reverse();
    return lines;
}
