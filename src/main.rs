use image_view::run;
use image_view::cli;

fn main() {
    let cli = cli::parse();
    let args = cli::Args::new(&cli);
    // println!("{:?}", cli);

    pollster::block_on(run(&cli, args));
}
