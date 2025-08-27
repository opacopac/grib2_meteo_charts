use bzip2::read::BzDecoder;
use log::info;
use std::io::Read;

const BZ2_SUFFIX: &str = ".bz2";


pub struct FileHelper;


impl FileHelper {
    pub fn get_file_reader(filename: &str) -> Box<dyn Read> {
        info!("reading file {}", filename);
        let response_result = ureq::get(filename)
            .call()
            .expect("Failed to get file from URL");
        let reader = response_result.into_body().into_reader();

        // if the file is bz2 compressed, wrap the reader with BzDecoder
        if filename.ends_with(BZ2_SUFFIX) {
            Box::new(BzDecoder::new(reader))
        } else {
            Box::new(reader)
        }
    }
}
