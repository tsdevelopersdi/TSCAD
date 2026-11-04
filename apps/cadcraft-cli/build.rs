//! Windows only: embed version info (VERSIONINFO) into `tscad-cli.exe`.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CADCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "TSCAD")
        .set("CompanyName", "Solusi Daya Indonesia")
        .set("FileDescription", "TSCAD command-line tool")
        .set("LegalCopyright", "Copyright (c) Solusi Daya Indonesia. MIT OR Apache-2.0.")
        .set("OriginalFilename", "tscad-cli.exe")
        .set("InternalName", "tscad-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("CADCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=tscad-cli.exe built without version resources: {e}");
    }
}
