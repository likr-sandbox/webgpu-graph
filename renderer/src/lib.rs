use js_sys::{Array, Float32Array, Object, Promise, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{
    GpuAdapter, GpuBindGroup, GpuBindGroupDescriptor, GpuBindGroupEntry, GpuBuffer,
    GpuBufferDescriptor, GpuBufferUsage, GpuCanvasConfiguration, GpuCanvasContext, GpuColorDict,
    GpuColorTargetState, GpuCommandEncoder, GpuComputePipeline, GpuComputePipelineDescriptor,
    GpuDevice, GpuFragmentState, GpuMapMode, GpuPrimitiveState, GpuPrimitiveTopology,
    GpuProgrammableStage, GpuRenderPassColorAttachment, GpuRenderPassDescriptor, GpuRenderPipeline,
    GpuRenderPipelineDescriptor, GpuShaderModule, GpuShaderModuleDescriptor, GpuStoreOp,
    GpuVertexState,
};

#[wasm_bindgen]
pub struct Renderer {
    map_read_buffer: GpuBuffer,
    map_write_buffer: GpuBuffer,
    compute_in_buffer: GpuBuffer,
    compute_out_buffer: GpuBuffer,
    render_pipeline: GpuRenderPipeline,
    compute_pipeline: GpuComputePipeline,
    compute_bind_group: GpuBindGroup,
    device: GpuDevice,
    context: GpuCanvasContext,
}

fn create_compute_pipeline(
    device: &GpuDevice,
    entry_point: &str,
    shader_module: &GpuShaderModule,
) -> GpuComputePipeline {
    device.create_compute_pipeline(&GpuComputePipelineDescriptor::new(
        &GpuProgrammableStage::new(entry_point, shader_module),
    ))
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        context: &GpuCanvasContext,
        adapter: &GpuAdapter,
        device: &GpuDevice,
    ) -> Result<Renderer, JsValue> {
        let context = context.clone();
        let device = device.clone();
        let presentation_format = context.get_preferred_format(&adapter);
        context.configure(
            &GpuCanvasConfiguration::new(&device, presentation_format).size(&{
                let value = Array::new();
                value.push(&300.into());
                value.push(&150.into());
                value.into()
            }),
        );
        let shader_module = device.create_shader_module(&GpuShaderModuleDescriptor::new(
            include_str!("./shader.wgsl"),
        ));
        let render_pipeline = device.create_render_pipeline(
            &GpuRenderPipelineDescriptor::new(&GpuVertexState::new("vertex_main", &shader_module))
                .fragment({
                    let array = Array::new();
                    array.push(&GpuColorTargetState::new(presentation_format));
                    &GpuFragmentState::new("fragment_main", &shader_module, &array)
                })
                .primitive(&GpuPrimitiveState::new().topology(GpuPrimitiveTopology::TriangleList)),
        );
        let map_read_buffer = device.create_buffer(&GpuBufferDescriptor::new(
            400.,
            GpuBufferUsage::MAP_READ | GpuBufferUsage::COPY_DST,
        ));
        let map_write_buffer = device.create_buffer(&GpuBufferDescriptor::new(
            400.,
            GpuBufferUsage::MAP_WRITE | GpuBufferUsage::COPY_SRC,
        ));
        let compute_in_buffer = device.create_buffer(&GpuBufferDescriptor::new(
            400.,
            GpuBufferUsage::STORAGE | GpuBufferUsage::COPY_DST,
        ));
        let compute_out_buffer = device.create_buffer(&GpuBufferDescriptor::new(
            400.,
            GpuBufferUsage::STORAGE | GpuBufferUsage::COPY_SRC,
        ));
        let compute_pipeline = create_compute_pipeline(&device, &"compute_main", &shader_module);
        let compute_bind_group = device.create_bind_group(&{
            let entries = Array::new();
            entries.push(&GpuBindGroupEntry::new(0, &{
                let resource = Object::new();
                Reflect::set(&resource, &"buffer".into(), &compute_in_buffer)?;
                Reflect::set(&resource, &"offset".into(), &0.into())?;
                Reflect::set(&resource, &"size".into(), &400.into())?;
                resource.into()
            }));
            entries.push(&GpuBindGroupEntry::new(1, &{
                let resource = Object::new();
                Reflect::set(&resource, &"buffer".into(), &compute_out_buffer)?;
                Reflect::set(&resource, &"offset".into(), &0.into())?;
                Reflect::set(&resource, &"size".into(), &400.into())?;
                resource.into()
            }));
            GpuBindGroupDescriptor::new(&entries, &compute_pipeline.get_bind_group_layout(0))
        });
        Ok(Renderer {
            map_read_buffer,
            map_write_buffer,
            compute_in_buffer,
            compute_out_buffer,
            render_pipeline,
            compute_pipeline,
            compute_bind_group,
            device,
            context,
        })
    }

    pub fn frame(&self) -> Result<(), JsValue> {
        self.run(|command_encoder| {
            let texture_view = self.context.get_current_texture().create_view();
            let pass_encoder = command_encoder.begin_render_pass({
                let color_attachments = Array::new();
                color_attachments.push(&GpuRenderPassColorAttachment::new(
                    &GpuColorDict::new(1., 0., 0., 0.),
                    GpuStoreOp::Store,
                    &texture_view,
                ));
                &GpuRenderPassDescriptor::new(&color_attachments)
            });
            pass_encoder.set_pipeline(&self.render_pipeline);
            pass_encoder.draw_with_instance_count(3, 1);
            pass_encoder.end_pass();
            Ok(())
        })
    }

    pub fn compute(&self) {
        let command_encoder = self.device.create_command_encoder();
        command_encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
            &self.map_write_buffer,
            0,
            &self.compute_in_buffer,
            0,
            400,
        );
        let pass_encoder = command_encoder.begin_compute_pass();
        pass_encoder.set_pipeline(&self.compute_pipeline);
        pass_encoder.set_bind_group(0, &self.compute_bind_group);
        pass_encoder.dispatch(100);
        pass_encoder.end_pass();
        command_encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
            &self.compute_out_buffer,
            0,
            &self.map_read_buffer,
            0,
            400,
        );
        self.device.queue().submit(&{
            let command_buffers = Array::new();
            command_buffers.push(&command_encoder.finish());
            command_buffers.into()
        });
    }

    pub fn map_write(&self) -> Promise {
        self.map_write_buffer.map_async(GpuMapMode::WRITE)
    }

    pub fn unmap_write(&self, src: &Float32Array) {
        let src_buffer = Float32Array::new(&self.map_write_buffer.get_mapped_range());
        src_buffer.set(&src, 0);
        self.map_write_buffer.unmap();
    }

    pub fn map_read(&self) -> Promise {
        self.map_read_buffer.map_async(GpuMapMode::READ)
    }

    pub fn unmap_read(&self, dst: &Float32Array) {
        let dst_buffer = Float32Array::new(&self.map_read_buffer.get_mapped_range());
        dst.set(&dst_buffer, 0);
        self.map_read_buffer.unmap();
    }

    fn run<F: FnOnce(&GpuCommandEncoder) -> Result<(), JsValue>>(
        &self,
        f: F,
    ) -> Result<(), JsValue> {
        let command_encoder = self.device.create_command_encoder();
        f(&command_encoder)?;
        self.device.queue().submit(&{
            let command_buffers = Array::new();
            command_buffers.push(&command_encoder.finish());
            command_buffers.into()
        });
        Ok(())
    }
}
