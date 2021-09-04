use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{
    GpuAdapter, GpuCanvasConfiguration, GpuCanvasContext, GpuDevice, GpuFragmentState,
    GpuPrimitiveState, GpuPrimitiveTopology, GpuRenderPassDescriptor, GpuRenderPipeline,
    GpuRenderPipelineDescriptor, GpuShaderModuleDescriptor, GpuVertexState,
};

#[wasm_bindgen]
pub struct Renderer {
    pipeline: GpuRenderPipeline,
    device: GpuDevice,
    context: GpuCanvasContext,
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
        let pipeline = device.create_render_pipeline(
            &GpuRenderPipelineDescriptor::new(&GpuVertexState::new(
                "main",
                &device.create_shader_module(&GpuShaderModuleDescriptor::new(include_str!(
                    "./triangle.vert.wgsl"
                ))),
            ))
            .fragment({
                let obj = Object::new();
                Reflect::set(&obj, &"format".into(), &presentation_format.into()).ok();
                let array = Array::new();
                array.push(&obj);
                &GpuFragmentState::new(
                    "main",
                    &device.create_shader_module(&GpuShaderModuleDescriptor::new(include_str!(
                        "./red.flag.wgsl"
                    ))),
                    &array,
                )
            })
            .primitive(&GpuPrimitiveState::new().topology(GpuPrimitiveTopology::TriangleList)),
        );
        Ok(Renderer {
            pipeline,
            device,
            context,
        })
    }

    pub fn frame(&self) -> Result<(), JsValue> {
        let command_encoder = self.device.create_command_encoder();
        let texture_view = self.context.get_current_texture().create_view();
        let pass_encoder = command_encoder.begin_render_pass({
            let obj = Object::new();
            Reflect::set(&obj, &"view".into(), &texture_view.into()).ok();
            Reflect::set(&obj, &"loadValue".into(), &{
                let obj = Object::new();
                Reflect::set(&obj, &"r".into(), &0.0.into()).ok();
                Reflect::set(&obj, &"g".into(), &0.0.into()).ok();
                Reflect::set(&obj, &"b".into(), &0.0.into()).ok();
                Reflect::set(&obj, &"a".into(), &1.0.into()).ok();
                obj.into()
            })
            .ok();
            Reflect::set(&obj, &"storeOp".into(), &"store".into()).ok();
            let color_attachments = Array::new();
            color_attachments.push(&obj);
            &GpuRenderPassDescriptor::new(&color_attachments)
        });
        pass_encoder.set_pipeline(&self.pipeline);
        pass_encoder.draw_with_instance_count_and_first_vertex_and_first_instance(3, 1, 0, 0);
        pass_encoder.end_pass();
        self.device.queue().submit(&{
            let command_buffers = Array::new();
            command_buffers.push(&command_encoder.finish());
            command_buffers.into()
        });
        Ok(())
    }
}
