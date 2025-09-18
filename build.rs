fn main()
{
    println!("cargo:rerun-if-changed=.metadata/Microsoft.Internal.winmd");
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--in",
        "default",
        ".metadata/Microsoft.Internal.winmd",
        "--out",
        "src/bindings.rs",
        "--flat",
        "--filter",
        "Microsoft.Internal",
        "--reference",
        "windows,skip-root,Windows",
    ])
    .unwrap();
}
