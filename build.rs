

fn main(){
    cc::Build::new()
    .file("minicoro.c")
    .compile("minicoro")
}