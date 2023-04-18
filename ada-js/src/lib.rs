use napi_derive::napi;

#[cfg(not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[napi]
pub fn validate(url: String) -> bool {
    ada_url::parse(url).is_ok()
}
