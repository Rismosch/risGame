use std::sync::Arc;

use vulkano::buffer::Buffer;
use vulkano::buffer::BufferCreateInfo;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::Subbuffer;
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::memory::allocator::AllocationCreateInfo;
use vulkano::memory::allocator::MemoryUsage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::Pipeline;

use crate::gpu_objects::UniformBufferObject;
use crate::gpu_objects::Vertex3d;

pub type Uniform<U> = (Subbuffer<U>, Arc<PersistentDescriptorSet>);

pub struct Buffers {
    pub vertex: Subbuffer<[Vertex3d]>,
    //pub index: Subbuffer<[u16]>,
    pub uniforms: Vec<Uniform<UniformBufferObject>>,
}

impl Buffers {
    pub fn new(
        allocators: &crate::allocators::Allocators,
        uniform_buffer_count: usize,
        pipeline: &Arc<GraphicsPipeline>,
    ) -> Result<Self, String> {
        // vertex
        let red = [1.0, 0.0, 0.0];
        let green = [0.0, 1.0, 0.0];
        let blue = [0.0, 0.0, 1.0];
        let cyan = [0.0, 1.0, 1.0];
        let magenta = [1.0, 0.0, 1.0];
        let yellow = [1.0, 1.0, 0.0];

        let vertex1 = Vertex3d {
            position: [-0.5, -0.5, -0.5],
            color: magenta,
        };
        let vertex2 = Vertex3d {
            position: [ 0.5, -0.5, -0.5],
            color: magenta,
        };
        let vertex3 = Vertex3d {
            position: [-0.5, -0.5,  0.5],
            color: magenta,
        };
        let vertex4 = Vertex3d {
            position: [ 0.5, -0.5,  0.5],
            color: magenta,
        };
        let vertex5 = Vertex3d {
            position: [-0.5, -0.5,  0.5],
            color: magenta,
        };
        let vertex6 = Vertex3d {
            position: [ 0.5, -0.5, -0.5],
            color: magenta,
        };

        let vertex = Buffer::from_iter(
            &allocators.memory,
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6],
        )
        .map_err(|e| format!("failed to create vertex buffer: {}", e))?;

        // uniform
        let mut uniforms = Vec::new();
        for _ in 0..uniform_buffer_count {
            let ubo = UniformBufferObject::default();

            let uniform_buffer = Buffer::from_data(
                &allocators.memory,
                BufferCreateInfo {
                    usage: BufferUsage::UNIFORM_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                ubo,
            )
            .map_err(|e| format!("failed to create uniform buffer: {}", e))?;

            let descriptor_set = PersistentDescriptorSet::new(
                &allocators.descriptor_set,
                pipeline
                    .layout()
                    .set_layouts()
                    .get(0)
                    .ok_or("failed to get descriptor set layout")?
                    .clone(),
                [WriteDescriptorSet::buffer(0, uniform_buffer.clone())],
            )
            .map_err(|e| format!("failed to create persistent descriptor set: {}", e))?;

            uniforms.push((uniform_buffer, descriptor_set));
        }

        Ok(Self { vertex, uniforms })
    }
}
