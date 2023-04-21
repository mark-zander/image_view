use image_view::run;
use image_view::cli::Cli;

fn main() {
    let args = Cli::new();
    println!("{:?}", args);

    pollster::block_on(run(&args));
}
