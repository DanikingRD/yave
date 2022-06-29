use pollster::block_on;
use thunderdome::Arena;
use wgpu::{Backends, Features, Instance, Limits, PowerPreference, PresentMode, SurfaceConfiguration, TextureUsages};
use winit::window::Window;
use serde_derive::Deserialize;

/// Game renderer (wgpu)
pub struct Renderer {
    /// The surface where the game is rendered on
    pub surface: wgpu::Surface,
    /// The bridge between the GPU and the renderer
    pub device: wgpu::Device,
    /// A queue used to submit commands to the device
    pub queue: wgpu::Queue,
    /// An arena which holds buffers
    pub arena: Arena<wgpu::Buffer>,
    /// The surface configuration
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        })).unwrap();

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: PresentMode::Fifo,
        };

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            features: Features::empty(),
            limits: Limits::default(),
        }, None)).unwrap();

        surface.configure(&device, &surface_config);

        let arena = Arena::new();

        Self {
            surface,
            device,
            queue,
            arena,
            surface_config
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RenderPipelineDescription {
    pub vertex_module: String,
    pub fragment_module: String,
    pub vertex_entry: String,
    pub fragment_entry: String,
    pub primitive: RenderPipelinePrimitive,
    pub samples: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RenderPipelinePrimitive {
    pub topology: String,
    pub front_face: String,
    pub cull_mode: Option<String>,
    pub polygon_mode: String,
    pub unclipped_depth: bool,
    pub conservative: bool
}

#[derive(Debug)]
pub struct PipelineBundle {
    pub pipeline_layout: wgpu::PipelineLayout,
    pub render_pipeline: wgpu::RenderPipeline,
}