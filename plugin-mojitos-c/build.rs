fn main(){
    cc::Build::new()
        .file("src/c/test.c")
        .include("src/c/")
        .compile("test");
}