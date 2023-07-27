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

        let pos = 0.08;
        let v1 = Vertex3d {
            position: [-pos, -pos, -pos],
            color: magenta,
        };
        let v2 = Vertex3d {
            position: [ pos, -pos, -pos],
            color: magenta,
        };
        let v3 = Vertex3d {
            position: [-pos, -pos,  pos],
            color: magenta,
        };
        let v4 = Vertex3d {
            position: [ pos, -pos,  pos],
            color: magenta,
        };
        let v5 = Vertex3d {
            position: [-pos, -pos,  pos],
            color: magenta,
        };
        let v6 = Vertex3d {
            position: [ pos, -pos, -pos],
            color: magenta,
        };

        let v7 = Vertex3d {
            position: [-pos,  pos, -pos],
            color: green,
        };
        let v8 = Vertex3d {
            position: [ pos,  pos, -pos],
            color: green,
        };
        let v9 = Vertex3d {
            position: [-pos,  pos,  pos],
            color: green,
        };
        let v10 = Vertex3d {
            position: [ pos,  pos,  pos],
            color: green,
        };
        let v11 = Vertex3d {
            position: [-pos,  pos,  pos],
            color: green,
        };
        let v12 = Vertex3d {
            position: [ pos,  pos, -pos],
            color: green,
        };


        let v13 = Vertex3d {
            position: [-pos, -pos, -pos],
            color: yellow,
        };
        let v14 = Vertex3d {
            position: [ pos, -pos, -pos],
            color: yellow,
        };
        let v15 = Vertex3d {
            position: [-pos,  pos, -pos],
            color: yellow,
        };
        let v16 = Vertex3d {
            position: [ pos,  pos, -pos],
            color: yellow,
        };
        let v17 = Vertex3d {
            position: [-pos,  pos, -pos],
            color: yellow,
        };
        let v18 = Vertex3d {
            position: [ pos, -pos, -pos],
            color: yellow,
        };

        let v19 = Vertex3d {
            position: [-pos, -pos,  pos],
            color: blue,
        };
        let v20 = Vertex3d {
            position: [ pos, -pos,  pos],
            color: blue,
        };
        let v21 = Vertex3d {
            position: [-pos,  pos,  pos],
            color: blue,
        };
        let v22 = Vertex3d {
            position: [ pos,  pos,  pos],
            color: blue,
        };
        let v23 = Vertex3d {
            position: [-pos,  pos,  pos],
            color: blue,
        };
        let v24 = Vertex3d {
            position: [ pos, -pos,  pos],
            color: blue,
        };

        let v25 = Vertex3d {
            position: [-pos, -pos, -pos],
            color: cyan,
        };
        let v26 = Vertex3d {
            position: [-pos, -pos,  pos],
            color: cyan,
        };
        let v27 = Vertex3d {
            position: [-pos,  pos, -pos],
            color: cyan,
        };
        let v28 = Vertex3d {
            position: [-pos,  pos,  pos],
            color: cyan,
        };
        let v29 = Vertex3d {
            position: [-pos,  pos, -pos],
            color: cyan,
        };
        let v30 = Vertex3d {
            position: [-pos, -pos,  pos],
            color: cyan,
        };

        let v31 = Vertex3d {
            position: [ pos, -pos, -pos],
            color: red,
        };
        let v32 = Vertex3d {
            position: [ pos, -pos,  pos],
            color: red,
        };
        let v33 = Vertex3d {
            position: [ pos,  pos, -pos],
            color: red,
        };
        let v34 = Vertex3d {
            position: [ pos,  pos,  pos],
            color: red,
        };
        let v35 = Vertex3d {
            position: [ pos,  pos, -pos],
            color: red,
        };
        let v36 = Vertex3d {
            position: [ pos, -pos,  pos],
            color: red,
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
            vec![
            v1, v2, v3, v4, v5, v6, // magenta
            v7, v8, v9, v10, v11, v12, // green
            v13, v14, v15, v16, v17, v18, // yellow
            v19, v20, v21, v22, v23, v24, // blue
            v25, v26, v27, v28, v29, v30, // cyan
            v31, v32, v33, v34, v35, v36, // red
            ],
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
