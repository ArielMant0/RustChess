extern crate vulkano_shaders;

fn main() {
    // building the shaders used in the examples
    vulkano_shaders::build_glsl_shaders([
        ("src/bin/teapot_vs.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/bin/teapot_fs.glsl", vulkano_shaders::ShaderType::Fragment),
    ].iter().cloned());
}
