mod reader;

pub fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args()
        .skip(1).collect();
    match args.len() {
        1 => {
            let file_reader = reader::Reader::new(args.get(0).unwrap()).unwrap();
            file_reader.parse();
        }
        _ => {
            panic!("Usage assembler <your_prog.asm>");
        }
    }
    Ok(())
}
