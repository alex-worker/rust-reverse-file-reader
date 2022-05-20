
#[allow(dead_code)]
pub fn show_lines (lines: &Vec<String>) {
    println!("----- lines: {} ------", lines.len() );
    for (index, line) in lines.iter().enumerate() {
        println!("[{}] line: {}", index+1, line)
    }
}

#[allow(dead_code)]
pub fn show_hex_buf (buffer: &[u8]) {
    print!("[");
    for c in buffer {
        print!(" {:#01x} ", c)
    }
    println!("]");
}
