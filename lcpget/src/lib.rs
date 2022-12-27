use lcp_types::{encryption_manifest, LicenseDocument};
use std::io::{Read, Write};

pub extern "C" fn license_download_epub() {
    todo!()
}

pub fn license_to_epub(license_file: &str) -> Vec<u8> {
    use lcp_types::crypto::Key;
    let license: LicenseDocument = serde_json::from_str(license_file).unwrap();
    let content_key_plus_iv = &license
        .encryption
        .content_key
        .decrypt(&hex_literal::hex!(""));
    let iv = &content_key_plus_iv[32..];
    let content_key = &content_key_plus_iv[0..32];

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

    let encryption_map = zip
        .by_name("META-INF/encryption.xml")
        .map(|mut x| {
            let mut buffer = String::new();
            x.read_to_string(&mut buffer).unwrap();
            buffer
        })
        .unwrap();

    let encryption_map = encryption_manifest::parse_manifest(&encryption_map);
    let options = zip::write::FileOptions::default();
    let mut out_file = Vec::new();
    let mut zip_writer = zip::write::ZipWriter::new(std::io::Cursor::new(&mut out_file));

    let mut file_buffer = Vec::new();
    for filename in zip.file_names().map(|x| x.to_string()).collect::<Vec<_>>() {
        file_buffer.clear();
        zip_writer.start_file(&filename, options).unwrap();
        let mut file = zip.by_name(&filename).unwrap();
        file_buffer.reserve(file.size() as usize);

        file.read_to_end(&mut file_buffer).unwrap();
        assert_eq!(file.size() as usize, file_buffer.len());
        if let Some((algo, comp)) = encryption_map.get(&filename) {
            let mut decryptable = Vec::from(iv);
            decryptable.append(&mut file_buffer);
            let decrypted = algo.decrypt(&content_key, &decryptable);
            if let Some(l) = comp {
                let inflated = inflate::inflate_bytes(&decrypted).unwrap();
                assert_eq!(*l, inflated.len());
                zip_writer.write_all(&inflated).unwrap();
            } else {
                zip_writer.write_all(&decrypted).unwrap();
            }
        } else {
            zip_writer.write_all(file_buffer.as_ref()).unwrap();
        }
    }

    zip_writer
        .start_file("META-INF/license.lcpl", options)
        .unwrap();
    zip_writer.write_all(license_file.as_bytes()).unwrap();
    zip_writer.finish().unwrap();
    drop(zip_writer);
    out_file
}
