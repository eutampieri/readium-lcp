use lcp_types::LicenseDocument;
use std::io::{Read, Write};

pub extern "C" fn license_download_epub() {
    todo!()
}

pub fn license_to_epub(license_file: &str) -> Vec<u8> {
    use lcp_types::crypto::Key;
    let license: LicenseDocument = serde_json::from_str(license_file).unwrap();
    dbg!(license
        .encryption
        .content_key
        .decrypt("538be5918e9c77f19370a25f02c0a1b4c186f606ef516386b20eb1b3decbff9d"));
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
