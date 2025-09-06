use bzip2::read::BzDecoder;
use log::info;
use std::io::Read;


pub struct FileHelper;


const BZ2_SUFFIX: &str = ".bz2";


impl FileHelper {
    pub fn get_url_reader(file_url: &str) -> Box<dyn Read> {
        info!("reading file {}", file_url);
        let response_result = ureq::get(file_url)
            .call()
            .expect("Failed to get file from URL");
        let reader = response_result.into_body().into_reader();

        // if the file is bz2 compressed, wrap the reader with BzDecoder
        if file_url.ends_with(BZ2_SUFFIX) {
            Box::new(BzDecoder::new(reader))
        } else {
            Box::new(reader)
        }
    }
}
