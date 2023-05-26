// Rather than pass a mesh to the gpu using a vertex buffer this program
// utilizes the gpu to build the mesh from the image texture. Displays
// the image as a surface where the z value is computed from the image
// intensity.
use image_view::run;
use image_view::cli::Cli;

fn main() {
    let cli = Cli::new();
    println!("{:?}", cli);

    pollster::block_on(run(&cli));
}
