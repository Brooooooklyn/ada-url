use std::mem;

use thiserror::Error;

mod ffi {
    use std::ffi::c_char;

    #[repr(C)]
    pub struct ada_url {
        _unused: [u8; 0],
    }

    #[repr(C)]
    pub struct CppStringView {
        pub data: *const c_char,
        pub length: usize,
    }

    impl AsRef<str> for CppStringView {
        fn as_ref(&self) -> &str {
            unsafe {
                let slice = std::slice::from_raw_parts(self.data.cast(), self.length);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }

    #[repr(C)]
    pub struct AdaUrlObject {
        pub href: CppStringView,
        pub origin: CppStringView,
        pub protocol: CppStringView,
        pub username: CppStringView,
        pub password: CppStringView,
        pub hostname: CppStringView,
        pub port: CppStringView,
        pub pathname: CppStringView,
        pub search: CppStringView,
        pub hash: CppStringView,
        url: *mut ada_url,
    }

    impl Drop for AdaUrlObject {
        fn drop(&mut self) {
            unsafe {
                delete_url(self.url);
            }
        }
    }

    extern "C" {
        pub fn parse(url: *const c_char, length: usize, url_aggregator: *mut AdaUrlObject) -> bool;
        pub fn delete_url(url_aggregator: *mut ada_url);
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid url: \"{0}\"")]
    ParseUrl(String),
}

#[repr(transparent)]
pub struct Url(ffi::AdaUrlObject);

impl Url {
    pub fn href(&self) -> &str {
        self.0.href.as_ref()
    }

    pub fn origin(&self) -> &str {
        self.0.origin.as_ref()
    }

    pub fn protocol(&self) -> &str {
        self.0.protocol.as_ref()
    }

    pub fn username(&self) -> &str {
        self.0.username.as_ref()
    }

    pub fn password(&self) -> &str {
        self.0.password.as_ref()
    }

    pub fn hostname(&self) -> &str {
        self.0.hostname.as_ref()
    }

    pub fn port(&self) -> &str {
        self.0.port.as_ref()
    }

    pub fn pathname(&self) -> &str {
        self.0.pathname.as_ref()
    }

    pub fn search(&self) -> &str {
        self.0.search.as_ref()
    }

    pub fn hash(&self) -> &str {
        self.0.hash.as_ref()
    }
}

pub fn parse<U: AsRef<str>>(url: U) -> Result<Url, Error> {
    let mut url_aggregator = mem::MaybeUninit::<ffi::AdaUrlObject>::uninit();
    let length = url.as_ref().len();
    if unsafe {
        ffi::parse(
            url.as_ref().as_ptr().cast(),
            length,
            url_aggregator.as_mut_ptr(),
        )
    } {
        Ok(Url(unsafe { url_aggregator.assume_init() }))
    } else {
        Err(Error::ParseUrl(url.as_ref().to_owned()))
    }
}

mod test {
    #[test]
    fn should_parse_basic_url() {
        use super::parse;

        if let Ok(url) = parse("https://google.com") {
            assert_eq!(url.href(), "https://google.com/");
            assert_eq!(url.origin(), "https://google.com");
            assert_eq!(url.protocol(), "https:");
            assert_eq!(url.username(), "");
            assert_eq!(url.password(), "");
            assert_eq!(url.hostname(), "google.com");
            assert_eq!(url.port(), "");
            assert_eq!(url.pathname(), "/");
            assert_eq!(url.search(), "");
            assert_eq!(url.hash(), "");
        } else {
            panic!("parse failed");
        }
    }
}
