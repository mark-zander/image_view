use image_view::run;
use image_view::cli;

fn main() {
    pollster::block_on(run(cli::Args::new()));
}
