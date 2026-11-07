//! GPU-Darstellung mit wgpu (Vulkan, Metal oder DirectX 12 – je nach System).
//!
//! Ablauf je Bild: Szene in ein HDR-Ziel (mit Mehrfachabtastung) zeichnen –
//! erst undurchsichtige Objekte mit allen Scheinwerfern, dann durchsichtige,
//! dann die Lichtstrahlen additiv – und per Tonemapping in eine Textur
//! übertragen, die egui als Bild anzeigt.

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

/// Netz-Schlüssel (Traversen je Länge/Profil eigenes Netz).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MeshKey {
    Cube,
    Plane,
    Cylinder,
    Cone,
    /// Länge und Breite in mm, Vierpunkt?
    Truss(u32, u32, bool),
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
    pub opaque: Vec<(MeshKey, Instance)>,
    pub transparent: Vec<(MeshKey, Instance)>,
    pub beams: Vec<Instance>,
    pub lights: Vec<Light>,
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

struct Targets {
    size: [u32; 2],
    msaa: u32,
    color: wgpu::TextureView,
    depth: wgpu::TextureView,
    resolve: wgpu::TextureView,
    out: wgpu::TextureView,
    tone_bind: wgpu::BindGroup,
}

pub struct Gpu {
    opaque: [wgpu::RenderPipeline; 2],
    transparent: [wgpu::RenderPipeline; 2],
    beam: [wgpu::RenderPipeline; 2],
    tonemap: wgpu::RenderPipeline,
    globals: wgpu::Buffer,
    lights: wgpu::Buffer,
    scene_bind: wgpu::BindGroup,
    tone_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
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
        let scene_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viz scene layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let tone_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viz tone layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let globals = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("viz globals"),
            size: std::mem::size_of::<Globals>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let lights = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("viz lights"),
            size: (MAX_LIGHTS * std::mem::size_of::<Light>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let scene_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viz scene bind"),
            layout: &scene_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: globals.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: lights.as_entire_binding(),
                },
            ],
        });
        let scene_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viz scene pl"),
            bind_group_layouts: &[Some(&scene_layout)],
            ..Default::default()
        });
        let tone_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viz tone pl"),
            bind_group_layouts: &[Some(&tone_layout)],
            ..Default::default()
        });
        let vertex_layouts = [
            Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
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
        let additive = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
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
        let beam = both(&|m| pipe("viz beam", "fs_beam", None, false, Some(additive), m));
        let tonemap = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viz tonemap"),
            layout: Some(&tone_pl),
            vertex: wgpu::VertexState {
                module: &tone,
                entry_point: Some("vs_full"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &tone,
                entry_point: Some("fs_tonemap"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: OUT,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("viz sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let instance_cap = 4096;
        let instances = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("viz instances"),
            size: instance_cap * std::mem::size_of::<Instance>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            opaque,
            transparent,
            beam,
            tonemap,
            globals,
            lights,
            scene_bind,
            tone_layout,
            sampler,
            meshes: HashMap::new(),
            instances,
            instance_cap,
            targets: None,
            texture: None,
            adapter,
        }
    }

    fn mesh(&mut self, device: &wgpu::Device, key: MeshKey) {
        if self.meshes.contains_key(&key) {
            return;
        }
        let m: Mesh = match key {
            MeshKey::Cube => mesh::cube(),
            MeshKey::Plane => mesh::plane(),
            MeshKey::Cylinder => mesh::cylinder(24),
            MeshKey::Cone => mesh::cone(40),
            MeshKey::Truss(len, w, sq) => mesh::truss(len as f32 / 1000.0, w as f32 / 1000.0, sq),
        };
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

    fn ensure_targets(&mut self, rs: &egui_wgpu::RenderState, size: [u32; 2], msaa: u32) {
        if self
            .targets
            .as_ref()
            .is_some_and(|t| t.size == size && t.msaa == msaa)
        {
            return;
        }
        let device = &rs.device;
        let tex = |label, format, samples, usage| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: size[0],
                        height: size[1],
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
        let color = tex("viz color", HDR, msaa, att);
        let depth = tex("viz depth", DEPTH, msaa, att);
        let resolve = tex(
            "viz resolve",
            HDR,
            1,
            att | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        let out = tex(
            "viz out",
            OUT,
            1,
            att | wgpu::TextureUsages::TEXTURE_BINDING,
        );
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
        });
    }

    /// Zeichnet ein Bild in die egui-Textur.
    pub fn render(&mut self, rs: &egui_wgpu::RenderState, size: [u32; 2], msaa: u32, f: &Frame3d) {
        let size = [size[0].clamp(1, 8192), size[1].clamp(1, 8192)];
        self.ensure_targets(rs, size, msaa);
        let device = &rs.device;
        let queue = &rs.queue;
        // Netze vorbereiten, Instanzen nach Netz gruppieren
        let mut groups: Vec<(MeshKey, u8, Vec<Instance>)> = Vec::new();
        let mut push = |key: MeshKey, pass: u8, inst: Instance| {
            if let Some(g) = groups.iter_mut().find(|g| g.0 == key && g.1 == pass) {
                g.2.push(inst);
            } else {
                groups.push((key, pass, vec![inst]));
            }
        };
        for (k, i) in &f.opaque {
            push(*k, 0, *i);
        }
        for (k, i) in &f.transparent {
            push(*k, 1, *i);
        }
        for i in &f.beams {
            push(MeshKey::Cone, 2, *i);
        }
        for g in &groups {
            self.mesh(device, g.0);
        }
        let all: Vec<Instance> = groups.iter().flat_map(|g| g.2.iter().copied()).collect();
        if all.len() as u64 > self.instance_cap {
            self.instance_cap = (all.len() as u64).next_power_of_two();
            self.instances = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("viz instances"),
                size: self.instance_cap * std::mem::size_of::<Instance>() as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        if !all.is_empty() {
            queue.write_buffer(&self.instances, 0, bytemuck::cast_slice(&all));
        }
        let n = f.lights.len().min(MAX_LIGHTS);
        if n > 0 {
            queue.write_buffer(&self.lights, 0, bytemuck::cast_slice(&f.lights[..n]));
        }
        let globals = Globals {
            view_proj: f.view_proj.to_cols_array_2d(),
            camera: f.eye.extend(1.0).into(),
            env: Vec4::new(f.env.x, f.env.y, f.env.z, n as f32).into(),
            misc: [f.time, 1.0, 0.0, 0.0],
        };
        queue.write_buffer(&self.globals, 0, bytemuck::bytes_of(&globals));

        let t = self.targets.as_ref().expect("Ziele angelegt");
        let mi = msaa_index(msaa);
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("viz") });
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
            let stride = std::mem::size_of::<Instance>() as u64;
            let mut offset = 0u64;
            for pass_kind in 0..3u8 {
                pass.set_pipeline(match pass_kind {
                    0 => &self.opaque[mi],
                    1 => &self.transparent[mi],
                    _ => &self.beam[mi],
                });
                let mut off = 0u64;
                for g in &groups {
                    let count = g.2.len() as u64;
                    if g.1 == pass_kind {
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
                offset = off;
            }
            let _ = offset;
        }
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
