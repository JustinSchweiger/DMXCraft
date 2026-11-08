//! GPU-Darstellung mit wgpu (Vulkan, Metal oder DirectX 12 – je nach System).
//!
//! Ablauf je Bild:
//! 1. Schattenkarten für die hellsten Scheinwerfer (nur Tiefe, je eine Ebene
//!    eines Textur-Arrays),
//! 2. Szene in ein HDR-Ziel (mit Mehrfachabtastung): undurchsichtige Objekte
//!    mit allen Scheinwerfern, durchsichtige, dann die Lichtstrahlen additiv,
//! 3. Bloom (helle Stellen verkleinern und weich wieder vergrößern),
//! 4. Tonemapping in eine Textur, die egui als Bild anzeigt.

use std::collections::HashMap;

use eframe::egui;
use eframe::egui_wgpu::{self, wgpu};
use glam::{Mat4, Vec3, Vec4};
use wgpu::util::DeviceExt;

use super::mesh::{self, Mesh, Vertex};

const HDR: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const OUT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
pub const MAX_LIGHTS: usize = 256;
pub const MAX_SHADOWS: usize = 32;
const BLOOM_LEVELS: usize = 5;

/// Netz-Schlüssel (Traversen je Länge/Profil eigenes Netz).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MeshKey {
    Cube,
    Plane,
    Cylinder,
    Cone,
    /// Länge und Breite in mm, Vierpunkt?
    Truss(u32, u32, bool),
    /// Teil eines eigenen Modells (Schlüssel-Hash, Teil)
    Model(u64, u16),
}

impl MeshKey {
    /// Schlüssel eines Modellteils.
    pub fn model(key: &str, part: usize) -> Self {
        let h = key
            .strip_prefix('m')
            .and_then(|h| u64::from_str_radix(h, 16).ok())
            .unwrap_or_else(|| {
                key.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
                    (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
                })
            });
        MeshKey::Model(h, part as u16)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub m: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub params: [f32; 4],
}

impl Instance {
    pub fn new(m: Mat4, color: Vec3, opacity: f32, gloss: f32, emissive: f32) -> Self {
        Self {
            m: m.to_cols_array_2d(),
            color: color.extend(opacity).into(),
            params: [gloss, emissive, 0.0, 0.0],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub pos: [f32; 4],
    pub dir: [f32; 4],
    pub color: [f32; 4],
    /// x = Schattenebene (−1 = ohne)
    pub extra: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    view_proj: [[f32; 4]; 4],
    camera: [f32; 4],
    env: [f32; 4],
    misc: [f32; 4],
}

/// Was in einem Bild gezeichnet wird.
#[derive(Default)]
pub struct Frame3d {
    /// Netz, Bild (0 = keins) und Lage
    pub opaque: Vec<(MeshKey, u32, Instance)>,
    pub transparent: Vec<(MeshKey, u32, Instance)>,
    /// Werfen Schatten (Bauten des Veranstaltungsorts).
    pub casters: Vec<(MeshKey, u32, Instance)>,
    pub beams: Vec<Instance>,
    pub lights: Vec<Light>,
    /// Blickmatrix je Schattenebene (Index = `Light::extra.x`).
    pub shadow_mats: Vec<Mat4>,
    /// Kantenlänge der Schattenkarten (0 = keine Schatten).
    pub shadow_size: u32,
    /// Bloom-Stärke (0 = aus).
    pub bloom: f32,
    pub view_proj: Mat4,
    pub eye: Vec3,
    /// Umgebungslicht, Strahlen, Nebel
    pub env: Vec3,
    pub time: f32,
    pub clear: Vec3,
}

struct GpuMesh {
    vb: wgpu::Buffer,
    ib: wgpu::Buffer,
    count: u32,
}

struct Bloom {
    /// Ebenen 1/2 … 1/32
    views: Vec<wgpu::TextureView>,
    /// Abwärts: Quelle → Ebene k; aufwärts: Ebene k+1 → k
    down: Vec<wgpu::BindGroup>,
    up: Vec<wgpu::BindGroup>,
    _params: Vec<wgpu::Buffer>,
}

struct Targets {
    size: [u32; 2],
    msaa: u32,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    resolve: wgpu::TextureView,
    out: wgpu::TextureView,
    tone_bind: wgpu::BindGroup,
    bloom: Bloom,
}

struct Shadows {
    size: u32,
    layers: u32,
    views: Vec<wgpu::TextureView>,
    passes: Vec<(wgpu::Buffer, wgpu::BindGroup)>,
}

pub struct Gpu {
    opaque: [wgpu::RenderPipeline; 2],
    transparent: [wgpu::RenderPipeline; 2],
    beam: [wgpu::RenderPipeline; 2],
    shadow_pipe: wgpu::RenderPipeline,
    tonemap: wgpu::RenderPipeline,
    bloom_down: wgpu::RenderPipeline,
    bloom_up: wgpu::RenderPipeline,
    globals: wgpu::Buffer,
    lights: wgpu::Buffer,
    shadow_mats: wgpu::Buffer,
    post: wgpu::Buffer,
    scene_layout: wgpu::BindGroupLayout,
    scene_bind: wgpu::BindGroup,
    shadow_layout: wgpu::BindGroupLayout,
    shadow_sampler: wgpu::Sampler,
    shadows: Shadows,
    tone_layout: wgpu::BindGroupLayout,
    bloom_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    tex_layout: wgpu::BindGroupLayout,
    tex_sampler: wgpu::Sampler,
    /// Bild 1 × 1 weiß für Flächen ohne Bild
    white: wgpu::BindGroup,
    textures: HashMap<u32, wgpu::BindGroup>,
    meshes: HashMap<MeshKey, GpuMesh>,
    instances: wgpu::Buffer,
    instance_cap: u64,
    targets: Option<Targets>,
    pub texture: Option<egui::TextureId>,
    /// Name der Grafikkarte und Schnittstelle (Vulkan …).
    pub adapter: String,
}

/// Index der Pipeline je Mehrfachabtastung (1 bzw. 4).
fn msaa_index(msaa: u32) -> usize {
    usize::from(msaa > 1)
}

fn uniform_entry(binding: u32, vis: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: vis,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn texture_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}

fn sampler_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    }
}

fn buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn fullscreen_pipeline(
    device: &wgpu::Device,
    label: &str,
    module: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    fs: &str,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module,
            entry_point: Some("vs_full"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module,
            entry_point: Some(fs),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    })
}

const ADDITIVE: wgpu::BlendState = wgpu::BlendState {
    color: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    },
    alpha: wgpu::BlendComponent::OVER,
};

impl Gpu {
    pub fn new(rs: &egui_wgpu::RenderState) -> Self {
        let device = &rs.device;
        let info = rs.adapter.get_info();
        let adapter = format!("{} · {:?}", info.name, info.backend);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viz scene"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let tone = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viz tonemap"),
            source: wgpu::ShaderSource::Wgsl(include_str!("tonemap.wgsl").into()),
        });
        let vf = wgpu::ShaderStages::VERTEX_FRAGMENT;
        let scene_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viz scene layout"),
            entries: &[
                uniform_entry(0, vf),
                storage_entry(1),
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
                storage_entry(4),
            ],
        });
        let shadow_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viz shadow layout"),
            entries: &[uniform_entry(0, wgpu::ShaderStages::VERTEX)],
        });
        let tone_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viz tone layout"),
            entries: &[
                texture_entry(0),
                sampler_entry(1),
                texture_entry(2),
                uniform_entry(3, wgpu::ShaderStages::FRAGMENT),
            ],
        });
        let bloom_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viz bloom layout"),
            entries: &[
                texture_entry(0),
                sampler_entry(1),
                uniform_entry(2, wgpu::ShaderStages::FRAGMENT),
            ],
        });
        let tex_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viz tex layout"),
            entries: &[texture_entry(0), sampler_entry(1)],
        });
        let tex_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("viz tex sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            anisotropy_clamp: 8,
            ..Default::default()
        });
        let white = Self::make_texture(
            device,
            &rs.queue,
            &tex_layout,
            &tex_sampler,
            &image::RgbaImage::from_pixel(1, 1, image::Rgba([255; 4])),
        );
        let globals = buffer(
            device,
            "viz globals",
            std::mem::size_of::<Globals>() as u64,
            wgpu::BufferUsages::UNIFORM,
        );
        let lights = buffer(
            device,
            "viz lights",
            (MAX_LIGHTS * std::mem::size_of::<Light>()) as u64,
            wgpu::BufferUsages::STORAGE,
        );
        let shadow_mats = buffer(
            device,
            "viz shadow mats",
            (MAX_SHADOWS * 64) as u64,
            wgpu::BufferUsages::STORAGE,
        );
        let post = buffer(device, "viz post", 16, wgpu::BufferUsages::UNIFORM);
        let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("viz shadow sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let shadows = Self::make_shadows(device, &shadow_layout, 1, 1);
        let scene_bind = Self::scene_bind(
            device,
            &scene_layout,
            &globals,
            &lights,
            &shadows,
            &shadow_sampler,
            &shadow_mats,
        );

        let scene_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viz scene pl"),
            bind_group_layouts: &[Some(&scene_layout), Some(&tex_layout)],
            ..Default::default()
        });
        let shadow_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viz shadow pl"),
            bind_group_layouts: &[None, Some(&shadow_layout)],
            ..Default::default()
        });
        let tone_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viz tone pl"),
            bind_group_layouts: &[Some(&tone_layout)],
            ..Default::default()
        });
        let bloom_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viz bloom pl"),
            bind_group_layouts: &[None, Some(&bloom_layout)],
            ..Default::default()
        });
        let vertex_layouts = [
            Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 8 => Float32x2],
            }),
            Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Instance>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![
                    2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4,
                    6 => Float32x4, 7 => Float32x4
                ],
            }),
        ];
        let pipe = |label: &str,
                    fs: &str,
                    cull: Option<wgpu::Face>,
                    depth_write: bool,
                    blend: Option<wgpu::BlendState>,
                    msaa: u32| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&scene_pl),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &vertex_layouts,
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: cull,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH,
                    depth_write_enabled: Some(depth_write),
                    depth_compare: Some(wgpu::CompareFunction::LessEqual),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: msaa,
                    ..Default::default()
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(fs),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: HDR,
                        blend,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        let both = |f: &dyn Fn(u32) -> wgpu::RenderPipeline| [f(1), f(4)];
        let opaque = both(&|m| {
            pipe(
                "viz opaque",
                "fs_scene",
                Some(wgpu::Face::Back),
                true,
                None,
                m,
            )
        });
        let transparent = both(&|m| {
            pipe(
                "viz transparent",
                "fs_scene",
                None,
                false,
                Some(wgpu::BlendState::ALPHA_BLENDING),
                m,
            )
        });
        let beam = both(&|m| pipe("viz beam", "fs_beam", None, false, Some(ADDITIVE), m));
        let shadow_pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viz shadow"),
            layout: Some(&shadow_pl),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_shadow"),
                compilation_options: Default::default(),
                buffers: &vertex_layouts,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: Default::default(),
            fragment: None,
            multiview_mask: None,
            cache: None,
        });
        let tonemap = fullscreen_pipeline(
            device,
            "viz tonemap",
            &tone,
            &tone_pl,
            "fs_tonemap",
            OUT,
            None,
        );
        let bloom_down = fullscreen_pipeline(
            device,
            "viz bloom down",
            &tone,
            &bloom_pl,
            "fs_down",
            HDR,
            None,
        );
        let bloom_up = fullscreen_pipeline(
            device,
            "viz bloom up",
            &tone,
            &bloom_pl,
            "fs_up",
            HDR,
            Some(ADDITIVE),
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("viz sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let instance_cap = 4096;
        let instances = buffer(
            device,
            "viz instances",
            instance_cap * std::mem::size_of::<Instance>() as u64,
            wgpu::BufferUsages::VERTEX,
        );
        Self {
            opaque,
            transparent,
            beam,
            shadow_pipe,
            tonemap,
            bloom_down,
            bloom_up,
            globals,
            lights,
            shadow_mats,
            post,
            scene_layout,
            scene_bind,
            shadow_layout,
            shadow_sampler,
            shadows,
            tone_layout,
            bloom_layout,
            sampler,
            tex_layout,
            tex_sampler,
            white,
            textures: HashMap::new(),
            meshes: HashMap::new(),
            instances,
            instance_cap,
            targets: None,
            texture: None,
            adapter,
        }
    }

    fn make_shadows(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        size: u32,
        layers: u32,
    ) -> Shadows {
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("viz shadow maps"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let mut views = vec![tex.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        })];
        let mut passes = Vec::new();
        for l in 0..layers {
            views.push(tex.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: l,
                array_layer_count: Some(1),
                ..Default::default()
            }));
            let b = buffer(device, "viz shadow pass", 64, wgpu::BufferUsages::UNIFORM);
            let g = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("viz shadow pass"),
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: b.as_entire_binding(),
                }],
            });
            passes.push((b, g));
        }
        Shadows {
            size,
            layers,
            views,
            passes,
        }
    }

    fn scene_bind(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        globals: &wgpu::Buffer,
        lights: &wgpu::Buffer,
        shadows: &Shadows,
        sampler: &wgpu::Sampler,
        mats: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viz scene bind"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: globals.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: lights.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&shadows.views[0]),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: mats.as_entire_binding(),
                },
            ],
        })
    }

    fn ensure_shadows(&mut self, device: &wgpu::Device, size: u32, layers: u32) {
        let (size, layers) = (size.max(1), layers.clamp(1, MAX_SHADOWS as u32));
        if self.shadows.size == size && self.shadows.layers == layers {
            return;
        }
        self.shadows = Self::make_shadows(device, &self.shadow_layout, size, layers);
        self.scene_bind = Self::scene_bind(
            device,
            &self.scene_layout,
            &self.globals,
            &self.lights,
            &self.shadows,
            &self.shadow_sampler,
            &self.shadow_mats,
        );
    }

    /// Bild mit Mipmaps hochladen und binden.
    fn make_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        img: &image::RgbaImage,
    ) -> wgpu::BindGroup {
        let (w, h) = img.dimensions();
        let levels = 32 - w.max(h).max(1).leading_zeros();
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("viz texture"),
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let mut level = img.clone();
        for l in 0..levels {
            let (lw, lh) = level.dimensions();
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex,
                    mip_level: l,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &level,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * lw),
                    rows_per_image: Some(lh),
                },
                wgpu::Extent3d {
                    width: lw,
                    height: lh,
                    depth_or_array_layers: 1,
                },
            );
            if l + 1 < levels {
                level = image::imageops::resize(
                    &level,
                    (lw / 2).max(1),
                    (lh / 2).max(1),
                    image::imageops::FilterType::Triangle,
                );
            }
        }
        let view = tex.create_view(&Default::default());
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viz texture bind"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    pub fn has_texture(&self, id: u32) -> bool {
        self.textures.contains_key(&id)
    }

    /// Bild hochladen (`None`: nicht lesbar – weiß verwenden, nicht erneut versuchen).
    pub fn add_texture(
        &mut self,
        rs: &egui_wgpu::RenderState,
        id: u32,
        img: Option<&image::RgbaImage>,
    ) {
        let g = match img {
            Some(img) => Self::make_texture(
                &rs.device,
                &rs.queue,
                &self.tex_layout,
                &self.tex_sampler,
                img,
            ),
            None => self.white.clone(),
        };
        self.textures.insert(id, g);
    }

    pub fn has_mesh(&self, key: MeshKey) -> bool {
        self.meshes.contains_key(&key)
    }

    /// Netz hochladen (eigene Modelle; eingebaute Formen entstehen von selbst).
    pub fn add_mesh(&mut self, device: &wgpu::Device, key: MeshKey, m: &Mesh) {
        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viz vb"),
            contents: bytemuck::cast_slice(&m.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viz ib"),
            contents: bytemuck::cast_slice(&m.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.meshes.insert(
            key,
            GpuMesh {
                vb,
                ib,
                count: m.indices.len() as u32,
            },
        );
    }

    fn mesh(&mut self, device: &wgpu::Device, key: MeshKey) {
        if self.meshes.contains_key(&key) {
            return;
        }
        let m: Mesh = match key {
            // werden vorher mit `add_mesh` geladen; sonst Platzhalter
            MeshKey::Model(..) => mesh::cube(),
            MeshKey::Cube => mesh::cube(),
            MeshKey::Plane => mesh::plane(),
            MeshKey::Cylinder => mesh::cylinder(24),
            MeshKey::Cone => mesh::cone(40),
            MeshKey::Truss(len, w, sq) => mesh::truss(len as f32 / 1000.0, w as f32 / 1000.0, sq),
        };
        self.add_mesh(device, key, &m);
    }

    fn ensure_targets(&mut self, rs: &egui_wgpu::RenderState, size: [u32; 2], msaa: u32) {
        if self
            .targets
            .as_ref()
            .is_some_and(|t| t.size == size && t.msaa == msaa)
        {
            return;
        }
        let device = &rs.device;
        let tex = |label, format, samples, usage, w: u32, h: u32| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: w.max(1),
                        height: h.max(1),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        let att = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let bind = wgpu::TextureUsages::TEXTURE_BINDING;
        let [w, h] = size;
        let color = tex("viz color", HDR, msaa, att, w, h);
        let depth = tex("viz depth", DEPTH, msaa, att, w, h);
        let resolve = tex("viz resolve", HDR, 1, att | bind, w, h);
        // auslesbar für Bilder des KI-Assistenten (MCP)
        let out = tex(
            "viz out",
            OUT,
            1,
            att | bind | wgpu::TextureUsages::COPY_SRC,
            w,
            h,
        );
        // Bloom-Ebenen und ihre Bindungen
        let views: Vec<wgpu::TextureView> = (0..BLOOM_LEVELS)
            .map(|k| tex("viz bloom", HDR, 1, att | bind, w >> (k + 1), h >> (k + 1)))
            .collect();
        let dims = |k: usize| [((w >> k).max(1)) as f32, ((h >> k).max(1)) as f32];
        let mut params = Vec::new();
        let mut group = |src: &wgpu::TextureView, src_dims: [f32; 2], threshold: f32| {
            let b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("viz bloom pass"),
                contents: bytemuck::cast_slice(&[
                    0.5 / src_dims[0],
                    0.5 / src_dims[1],
                    threshold,
                    0.0f32,
                ]),
                usage: wgpu::BufferUsages::UNIFORM,
            });
            let g = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("viz bloom pass"),
                layout: &self.bloom_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(src),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: b.as_entire_binding(),
                    },
                ],
            });
            params.push(b);
            g
        };
        let mut down = vec![group(&resolve, dims(0), 1.0)];
        for k in 1..BLOOM_LEVELS {
            down.push(group(&views[k - 1], dims(k), 0.0));
        }
        let up: Vec<wgpu::BindGroup> = (1..BLOOM_LEVELS)
            .map(|k| group(&views[k], dims(k + 1), 0.0))
            .collect();
        let bloom = Bloom {
            views,
            down,
            up,
            _params: params,
        };
        let tone_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viz tone bind"),
            layout: &self.tone_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&resolve),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&bloom.views[0]),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: self.post.as_entire_binding(),
                },
            ],
        });
        // Textur für egui anmelden bzw. austauschen
        let mut r = rs.renderer.write();
        match self.texture {
            Some(id) => {
                r.update_egui_texture_from_wgpu_texture(device, &out, wgpu::FilterMode::Linear, id)
            }
            None => {
                self.texture =
                    Some(r.register_native_texture(device, &out, wgpu::FilterMode::Linear))
            }
        }
        drop(r);
        self.targets = Some(Targets {
            size,
            msaa,
            color,
            depth,
            resolve,
            out,
            tone_bind,
            bloom,
        });
    }

    /// Kann die Grafikkarte den Visualisierer zeichnen? (Ältere bzw.
    /// OpenGL-Treiber haben keine Speicherpuffer im Fragment-Shader.)
    pub fn supported(rs: &egui_wgpu::RenderState) -> bool {
        rs.device.limits().max_storage_buffers_per_shader_stage >= 2
    }

    /// Das zuletzt gezeichnete Bild auslesen (RGBA, sRGB).
    pub fn read_image(&self, rs: &egui_wgpu::RenderState) -> Option<image::RgbaImage> {
        let t = self.targets.as_ref()?;
        let [w, h] = t.size;
        let row = w * 4;
        let padded =
            row.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let device = &rs.device;
        let buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("viz readback"),
            size: u64::from(padded * h),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut enc = device.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: t.out.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buf,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
        rs.queue.submit([enc.finish()]);
        let slice = buf.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        device.poll(wgpu::PollType::wait_indefinitely()).ok()?;
        rx.recv().ok()?.ok()?;
        let data = slice.get_mapped_range().ok()?;
        let mut px = Vec::with_capacity((row * h) as usize);
        for y in 0..h {
            let start = (y * padded) as usize;
            px.extend_from_slice(&data[start..start + row as usize]);
        }
        drop(data);
        buf.unmap();
        image::RgbaImage::from_raw(w, h, px)
    }

    /// Zeichnet ein Bild in die egui-Textur.
    pub fn render(&mut self, rs: &egui_wgpu::RenderState, size: [u32; 2], msaa: u32, f: &Frame3d) {
        let size = [size[0].clamp(1, 8192), size[1].clamp(1, 8192)];
        self.ensure_targets(rs, size, msaa);
        let device = &rs.device;
        let queue = &rs.queue;
        let n_shadows = f.shadow_mats.len().min(MAX_SHADOWS);
        if f.shadow_size > 0 && n_shadows > 0 {
            self.ensure_shadows(device, f.shadow_size, n_shadows as u32);
        }
        // Instanzen nach Netz, Durchgang und Bild gruppieren (3 = Schattenwerfer)
        let mut groups: Vec<(MeshKey, u8, u32, Vec<Instance>)> = Vec::new();
        let mut push = |key: MeshKey, pass: u8, tex: u32, inst: Instance| {
            if let Some(g) = groups
                .iter_mut()
                .find(|g| g.0 == key && g.1 == pass && g.2 == tex)
            {
                g.3.push(inst);
            } else {
                groups.push((key, pass, tex, vec![inst]));
            }
        };
        for (k, t, i) in &f.opaque {
            push(*k, 0, *t, *i);
        }
        for (k, t, i) in &f.transparent {
            push(*k, 1, *t, *i);
        }
        for i in &f.beams {
            push(MeshKey::Cone, 2, 0, *i);
        }
        if n_shadows > 0 {
            for (k, _, i) in &f.casters {
                push(*k, 3, 0, *i);
            }
        }
        for g in &groups {
            self.mesh(device, g.0);
        }
        let all: Vec<Instance> = groups.iter().flat_map(|g| g.3.iter().copied()).collect();
        if all.len() as u64 > self.instance_cap {
            self.instance_cap = (all.len() as u64).next_power_of_two();
            self.instances = buffer(
                device,
                "viz instances",
                self.instance_cap * std::mem::size_of::<Instance>() as u64,
                wgpu::BufferUsages::VERTEX,
            );
        }
        if !all.is_empty() {
            queue.write_buffer(&self.instances, 0, bytemuck::cast_slice(&all));
        }
        let n = f.lights.len().min(MAX_LIGHTS);
        if n > 0 {
            let mut lights = f.lights[..n].to_vec();
            if n_shadows == 0 {
                for l in &mut lights {
                    l.extra[0] = -1.0;
                }
            }
            queue.write_buffer(&self.lights, 0, bytemuck::cast_slice(&lights));
        }
        if n_shadows > 0 {
            let mats: Vec<[[f32; 4]; 4]> = f.shadow_mats[..n_shadows]
                .iter()
                .map(|m| m.to_cols_array_2d())
                .collect();
            queue.write_buffer(&self.shadow_mats, 0, bytemuck::cast_slice(&mats));
            for (i, m) in mats.iter().enumerate() {
                queue.write_buffer(&self.shadows.passes[i].0, 0, bytemuck::cast_slice(m));
            }
        }
        let globals = Globals {
            view_proj: f.view_proj.to_cols_array_2d(),
            camera: f.eye.extend(1.0).into(),
            env: Vec4::new(f.env.x, f.env.y, f.env.z, n as f32).into(),
            misc: [f.time, 1.0, 0.0, 0.0],
        };
        queue.write_buffer(&self.globals, 0, bytemuck::bytes_of(&globals));
        queue.write_buffer(
            &self.post,
            0,
            bytemuck::cast_slice(&[f.bloom, 1.0f32, 0.0, 0.0]),
        );

        let t = self.targets.as_ref().expect("Ziele angelegt");
        let mi = msaa_index(msaa);
        let stride = std::mem::size_of::<Instance>() as u64;
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("viz") });
        // 1. Schatten
        for layer in 0..n_shadows {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viz shadow"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadows.views[layer + 1],
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.shadow_pipe);
            pass.set_bind_group(1, &self.shadows.passes[layer].1, &[]);
            let mut off = 0u64;
            for g in &groups {
                let count = g.3.len() as u64;
                if g.1 == 3 {
                    let m = &self.meshes[&g.0];
                    pass.set_vertex_buffer(0, m.vb.slice(..));
                    pass.set_vertex_buffer(
                        1,
                        self.instances.slice(off * stride..(off + count) * stride),
                    );
                    pass.set_index_buffer(m.ib.slice(..), wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..m.count, 0, 0..count as u32);
                }
                off += count;
            }
        }
        // 2. Szene
        {
            let (view, resolve) = if msaa > 1 {
                (&t.color, Some(&t.resolve))
            } else {
                (&t.resolve, None)
            };
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viz scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: resolve,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: f.clear.x as f64,
                            g: f.clear.y as f64,
                            b: f.clear.z as f64,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &t.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_bind_group(0, &self.scene_bind, &[]);
            for pass_kind in 0..3u8 {
                pass.set_pipeline(match pass_kind {
                    0 => &self.opaque[mi],
                    1 => &self.transparent[mi],
                    _ => &self.beam[mi],
                });
                let mut off = 0u64;
                for g in &groups {
                    let count = g.3.len() as u64;
                    if g.1 == pass_kind {
                        let m = &self.meshes[&g.0];
                        let tex = if g.2 == 0 {
                            &self.white
                        } else {
                            self.textures.get(&g.2).unwrap_or(&self.white)
                        };
                        pass.set_bind_group(1, tex, &[]);
                        pass.set_vertex_buffer(0, m.vb.slice(..));
                        pass.set_vertex_buffer(
                            1,
                            self.instances.slice(off * stride..(off + count) * stride),
                        );
                        pass.set_index_buffer(m.ib.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..m.count, 0, 0..count as u32);
                    }
                    off += count;
                }
            }
        }
        // 3. Bloom
        let fullscreen = |enc: &mut wgpu::CommandEncoder,
                          pipe: &wgpu::RenderPipeline,
                          target: &wgpu::TextureView,
                          group: &wgpu::BindGroup,
                          load: wgpu::LoadOp<wgpu::Color>| {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viz post"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(pipe);
            pass.set_bind_group(1, group, &[]);
            pass.draw(0..3, 0..1);
        };
        if f.bloom > 0.0 {
            let clear = wgpu::LoadOp::Clear(wgpu::Color::BLACK);
            for k in 0..BLOOM_LEVELS {
                fullscreen(
                    &mut enc,
                    &self.bloom_down,
                    &t.bloom.views[k],
                    &t.bloom.down[k],
                    clear,
                );
            }
            for k in (1..BLOOM_LEVELS).rev() {
                fullscreen(
                    &mut enc,
                    &self.bloom_up,
                    &t.bloom.views[k - 1],
                    &t.bloom.up[k - 1],
                    wgpu::LoadOp::Load,
                );
            }
        }
        // 4. Tonemapping
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viz tonemap"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &t.out,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.tonemap);
            pass.set_bind_group(0, &t.tone_bind, &[]);
            pass.draw(0..3, 0..1);
        }
        queue.submit(Some(enc.finish()));
    }
}
