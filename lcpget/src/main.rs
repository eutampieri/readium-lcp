use lcp_types::LicenseDocument;
use std::io::{Read, Write};

pub fn license_to_epub(license_file: &str) -> Vec<u8> {
    let license: LicenseDocument = serde_json::from_str(license_file).unwrap();
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

    let mut zip =
        zip::read::ZipArchive::new(std::io::Cursor::new(file)).expect("Failed to open epub file");

    let options = zip::write::FileOptions::default();
    let mut out_file = Vec::new();
    let mut zip_writer = zip::write::ZipWriter::new(std::io::Cursor::new(&mut out_file));

    for file in zip.file_names().map(|x| x.to_string()).collect::<Vec<_>>() {
        let mut file_buffer = Vec::new();
        zip_writer.start_file(&file, options).unwrap();
        let mut file = zip.by_name(dbg!(&file)).unwrap();
        file_buffer.reserve(dbg!(file.size()) as usize);

        file.read_to_end(&mut file_buffer).unwrap();
        assert_eq!(file.size() as usize, file_buffer.len());
        zip_writer.write_all(file_buffer.as_ref()).unwrap();
    }

    zip_writer
        .start_file("META-INF/license.lcpl", options)
        .unwrap();
    zip_writer.write_all(license_file.as_bytes()).unwrap();
    zip_writer.finish().unwrap();
    drop(zip_writer);
    out_file
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Provide a file")
        .replace(".lcpl", ".epub");

    let epub = license_to_epub(
        std::fs::read_to_string(std::env::args().nth(1).expect("Provide a file"))
            .expect("Invalid file")
            .as_str(),
    );
    std::fs::write(&file_name, epub).expect("Could not create out file");
    println!("Added license to {}", file_name);
}
