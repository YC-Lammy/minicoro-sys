

fn main(){
    cc::Build::new()
    .file("minicoro.h")
    .opt_level(3)
    .compile("minicoro")
}