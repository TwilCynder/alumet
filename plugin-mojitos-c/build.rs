fn main(){
    bindgen::builder()
        .header("src/c/libmojitos.h")
        .blocklist_file("/usr/.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate().expect("Coulnd't generate bindings").write_to_file("src/mojitos.rs").expect("Couldn't write bindings to file");
}