const HEADER_DIR: &str = "/usr/local/include/";
const LIB_DIR: &str = "/usr/local/lib/";

fn main(){
    println!("cargo::rerun-if-changed=src/c/");
    println!("cargo:rustc-link-lib=mojitos");


    bindgen::builder()
        .header(HEADER_DIR.to_owned() + "/mojitos.h")
        .blocklist_file("/usr/lib/.*")
        .blocklist_file("/usr/include.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate().expect("Coulnd't generate bindings").write_to_file("src/mojitos.rs").expect("Couldn't write bindings to file");
}