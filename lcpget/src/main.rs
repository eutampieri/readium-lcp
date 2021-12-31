mod lib;
fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Provide a file")
        .replace(".lcpl", ".epub");

    let epub = lib::license_to_epub(
        std::fs::read_to_string(std::env::args().nth(1).expect("Provide a file"))
            .expect("Invalid file")
            .as_str(),
    );
    std::fs::write(&file_name, epub).expect("Could not create out file");
    println!("Added license to {}", file_name);
}
