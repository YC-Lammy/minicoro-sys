

fn main(){
    cc::Build::new()
    .define("MCO_NO_DEBUG", None)
    .define("MINICORO_IMPL", None)
    .file("minicoro.h")
    .opt_level(3)
    .compile("minicoro")
}