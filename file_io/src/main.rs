mod read_lines;

fn main() {
    let filename="./test.txt";
    let lines = read_lines::get_lines_iterator(filename);
    for line_result in lines {
        match line_result {
            Ok(good_line) => println!("{}", good_line),
            Err(err) => println!("{:?}", err)
        }
    }
}
