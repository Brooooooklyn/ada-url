use napi_derive::napi;

#[napi]
pub fn validate(url: String) -> bool {
    ada_url::parse(url).is_ok()
}
