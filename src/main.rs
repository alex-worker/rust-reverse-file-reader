mod reader;
use futures::executor::block_on;
use crate::reader::{read_lines};

const FILE_NAME1: &str = "monostr.txt";
const FILE_NAME2: &str = "logfile.txt";
const FILE_NAME3: &str = "biser.txt_Ascii.txt";
const FILE_NAME4: &str = "empty.txt";
const FILE_NAME5: &str = "one.txt";
const NUM_LINES: usize = 10;

async fn read_once(file_name: &str, num_lines: usize) {
    println!("===== file name: {}", file_name);
    let lines = read_lines(file_name, num_lines).await;
    println!(" lines: {}", lines.len() );
    for (index, line) in lines.iter().enumerate() {
        println!("[{}] line: {}", index, line)
    }
}

fn main() {
    block_on(read_once(FILE_NAME1, NUM_LINES));
    block_on(read_once(FILE_NAME2, NUM_LINES));
    block_on(read_once(FILE_NAME3, NUM_LINES));
    block_on(read_once(FILE_NAME4, NUM_LINES));
    block_on(read_once(FILE_NAME5, NUM_LINES));
}
