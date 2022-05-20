mod reader;

use std::env;
use futures::executor::block_on;
use crate::reader::{read_lines};

async fn read_once(file_name: &str, num_lines: usize) {
    println!("===== file name: {}", file_name);
    let lines = read_lines(file_name, num_lines).await;
    println!(" lines: {}", lines.len() );
    for (index, line) in lines.iter().enumerate() {
        println!("[{}] line: {}", index, line)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let row_nums: usize = args[1].parse().unwrap();
    let file_path = &args[2];

    let read_file_future = read_once(file_path, row_nums);
    block_on(read_file_future);
}

#[cfg(test)]
mod tests {
    use std::env;
    use futures::executor::block_on;
    use crate::read_once;

    const FILE_NAME1: &str = "fixtures/monostr.txt";
    const FILE_NAME2: &str = "fixtures/logfile.txt";
    const FILE_NAME3: &str = "fixtures/biser.txt_utf8.txt";
    const FILE_NAME4: &str = "fixtures/empty.txt";
    const FILE_NAME5: &str = "fixtures/one.txt";
    const NUM_LINES: usize = 10;

    #[test]
    fn it_works_1() {
        let path = env::current_dir().unwrap();
        println!("path: {}", path.display());
        block_on(read_once(FILE_NAME1, NUM_LINES));
    }

    #[test]
    fn it_works_2() {
        block_on(read_once(FILE_NAME2, NUM_LINES));
    }

    #[test]
    fn it_works_3() {
        block_on(read_once(FILE_NAME3, NUM_LINES));
    }

    #[test]
    fn it_works_4() {
        block_on(read_once(FILE_NAME4, NUM_LINES));
    }

    #[test]
    fn it_works_5() {
        block_on(read_once(FILE_NAME5, NUM_LINES));
    }
}
