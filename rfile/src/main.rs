use std::fs::File;

fn main() {
    let filename = "./src/main.rs";
    println!("In file {}", filename);

    let mut file = File::open(filename);
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer);

    println!("With text:\n{:?}", buffer);
}
