extern crate examples;
extern crate spirv_cross;

use examples::words_from_bytes;
use spirv_cross::{msl, spirv};
use spirv_cross::spirv::SPIRType_BaseType;

fn main() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("../vertex.spv")));

    // Parse a SPIR-V module
    let mut ast = spirv::Ast::<msl::Target>::parse(&module).unwrap();

    let mut compiler_options = msl::CompilerOptions::default();

    // Set some overrides
    compiler_options.resource_binding_overrides.insert(
        msl::ResourceBindingLocation {
            stage: spirv::ExecutionModel::Vertex,
            desc_set: 0,
            binding: 0,
        },
        msl::ResourceBinding {
            base_type: SPIRType_BaseType::Image,
            buffer_id: 5,
            texture_id: 6,
            sampler_id: 7,
            count: 0,
        },
    );

    compiler_options.vertex_attribute_overrides.insert(
        msl::VertexAttributeLocation(1),
        msl::VertexAttribute {
            buffer_id: 1,
            format: msl::Format::Other,
            rate: msl::Rate::PerVertex,
            built_in: None,
            vecsize: 0,
        },
    );

    ast.set_compiler_options(&compiler_options).unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to MSL
    let shader = ast.compile().unwrap();

    let bufs = ast.get_shader_resources().unwrap();
    println!("{:?}", bufs.uniform_buffers);

    let res = ast.get_automatic_msl_resource_binding(bufs.uniform_buffers[0].id);

    println!("{}", shader);
}
