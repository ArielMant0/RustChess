extern crate vulkano_shaders;

fn main() {
    // building the shaders used in the examples
    vulkano_shaders::build_glsl_shaders([
        ("src/bin/chess_vs.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/bin/chess_fs.glsl", vulkano_shaders::ShaderType::Fragment),
    ].iter().cloned());
}
