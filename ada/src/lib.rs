use std::ptr;

mod ffi {
    use std::ffi::c_char;

    #[repr(C)]
    pub struct ada_url {
        _unused: [u8; 0],
    }

    extern "C" {
        pub fn parse(url: *const c_char, length: usize, url_aggregator: *mut *mut ada_url) -> bool;
    }
}

#[repr(transparent)]
pub struct Url(*mut ffi::ada_url);

#[derive(Debug)]
pub enum Error {
    ParseUrl,
}

pub fn parse<U: AsRef<str>>(url: U) -> Result<Url, Error> {
    let mut url_aggregator = ptr::null_mut();
    let length = url.as_ref().len();
    if unsafe {
        ffi::parse(
            url.as_ref().as_ptr().cast(),
            length,
            &mut url_aggregator as *mut _,
        )
    } {
        Ok(Url(url_aggregator))
    } else {
        Err(Error::ParseUrl)
    }
}

mod test {
    #[test]
    fn should_parse_basic_url() {
        use super::parse;
        assert!(parse("https://google.com").is_ok());
    }
}
