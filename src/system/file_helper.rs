use crate::system::system_error::SystemError;
use bzip2::read::BzDecoder;
use log::info;
use std::io::Read;


pub struct FileHelper;


const BZ2_SUFFIX: &str = ".bz2";


impl FileHelper {
    pub fn get_url_reader(file_url: &str) -> Result<Box<dyn Read>, SystemError> {
        info!("reading file {}", file_url);
        let response = ureq::get(file_url)
            .header("Accept-Encoding", "gzip, br")
            .call()?;

        // TOODO: temp
        /*if let Some(enc) = response.headers().get("Content-Encoding") {
            info!("Content-Encoding: {}", enc.to_str().unwrap_or("unknown"));
        } else {
            info!("Content-Encoding header not found");
        }*/

        let reader = response.into_body().into_reader();

        // if the file is bz2 compressed, wrap the reader with BzDecoder
        let boxed: Box<dyn Read> = if file_url.ends_with(BZ2_SUFFIX) {
            Box::new(BzDecoder::new(reader))
        } else {
            Box::new(reader)
        };

        Ok(boxed)
    }
}
