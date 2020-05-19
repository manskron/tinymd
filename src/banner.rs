pub fn print_short_banner() {
    println!("{}", get_title());
}

pub fn print_long_banner() {
    let pkg_authors = String::from(env!("CARGO_PKG_AUTHORS"));
    let pkg_homepage = String::from(env!("CARGO_PKG_HOMEPAGE"));

    print_short_banner();
    println!("Written by: {}", pkg_authors);
    println!("Homepage: {}\n", pkg_homepage);
}

pub fn get_title() -> String {
    let pkg_name = String::from(env!("CARGO_PKG_NAME"));
    let pkg_version = String::from(env!("CARGO_PKG_VERSION"));
    let pkg_description = String::from(env!("CARGO_PKG_DESCRIPTION"));

    format!("{} (v{}), {}", pkg_name, pkg_version, pkg_description)
}
