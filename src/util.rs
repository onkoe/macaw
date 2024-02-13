/// Information about Cargo.toml.
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn get_pkg_name() -> String {
    format!(
        "{}{}",
        (built_info::PKG_NAME[..1]).to_uppercase(),
        &built_info::PKG_NAME[1..]
    )
}