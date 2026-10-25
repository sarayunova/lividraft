//! Windows only: embed the app icon and version info into `lividraft.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/lividraft.ico");
    println!("cargo:rerun-if-env-changed=CADCRAFT_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=CADCRAFT_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/lividraft.ico").exists() {
        res.set_icon("../../assets/app-icon/lividraft.ico");
    }
    res.set("ProductName", "LiVi Draft")
        .set("FileDescription", "LiVi Draft computer-aided design")
        .set("CompanyName", "LiVi")
        .set("LegalCopyright", "Copyright (c) 2026 LiVi and the LiVi Draft contributors, based on CADCraft by the ArtCraft Team. MIT OR Apache-2.0.")
        .set("OriginalFilename", "lividraft.exe")
        .set("InternalName", "lividraft");
    if let Err(e) = res.compile() {
        if std::env::var_os("CADCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=lividraft.exe built without icon/version resources: {e}");
    }
}
