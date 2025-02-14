use std::error::Error;

use glob::glob;

fn main() -> Result<(), Box<dyn Error>>{
    let files = glob("./src/c/*.c")?.filter_map(|path| match path {
        Ok(p) => Some(p),
        Err(_) => None,
    });

    cc::Build::new()
        .files(files)
        .include("src/c/")
        .compile("libmojitos")
    ;

    println!("cargo::rerun-if-changed=src/c/");

    bindgen::builder()
        .header("src/c/libmojitos.h")
        .blocklist_file("/usr/.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate().expect("Coulnd't generate bindings").write_to_file("src/mojitos.rs").expect("Couldn't write bindings to file");

    Ok(())
}