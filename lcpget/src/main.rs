use lcp_types::LicenseDocument;
use std::io::{Read, Write};

pub fn license_to_epub(license: LicenseDocument) -> Vec<u8> {
    let epub_url = license
        .links
        .iter()
        .find(|x| x.get_rel() == lcp_types::link::LinkRelationship::Publication)
        .map(|x| x.href.clone())
        .unwrap();

    let mut file = vec![];
    ureq::get(&epub_url)
        .call()
        .expect("Failed to fetch book")
        .into_reader()
        .read_to_end(&mut file)
        .unwrap();

    let mut zip = zip::write::ZipWriter::new_append(std::io::Cursor::new(&mut file))
        .expect("Failed to open epub file");

    let options = zip::write::FileOptions::default();
    zip.start_file("META-INF/license.lcpl", options).unwrap();
    zip.write(serde_json::to_string(&license).unwrap().as_bytes())
        .unwrap();
    zip.finish().unwrap();
    drop(zip);
    file
}

fn main() {
    let license: LicenseDocument = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(std::env::args().nth(1).expect("Provide a file"))
            .expect("File not found"),
    ))
    .unwrap();
    let file_name = std::env::args()
        .nth(1)
        .expect("Provide a file")
        .replace(".lcpl", ".epub");

    let epub = license_to_epub(license);
    std::fs::write(&file_name, epub).expect("Could not create out file");
    println!("Added license to {}", file_name);
}
