#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
        }
    }
}
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material_idx: usize,
}
#[derive(Clone, Debug)]
pub struct Material {
    pub base_col: [f32; 4],
}

impl Default for Material {
    fn default() -> Self {
        Material {
            base_col: [0.0, 0.0, 0.0, 0.0],
        }
    }
}
#[derive(Clone, Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub mats: Vec<Material>,
}

pub fn load_model(path: &str) -> Model {
    let (doc, buf, _img) = gltf::import(path).expect("failed to load model");
    let mut meshes: Vec<Mesh> = Vec::new();
    let mut mats: Vec<Material> = vec![Material::default(); doc.materials().len()];
    if mats.is_empty() {
        mats.push(Material::default());
    }

    if doc.nodes().len() > 0 {
        process_node(
            doc.nodes().next().as_ref().unwrap(),
            &buf,
            &mut meshes,
            &mut mats,
        );
    }

    Model { meshes, mats }
}

fn process_node(
    node: &gltf::Node,
    buffers: &[gltf::buffer::Data],
    meshes: &mut Vec<Mesh>,
    mats: &mut [Material],
) {
    if let Some(mesh) = node.mesh() {
        for prim in mesh.primitives() {
            if prim.mode() == gltf::mesh::Mode::Triangles {
                let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));
                let positions = {
                    let iter = reader
                        .read_positions()
                        .expect("Failed to process mesh node. (Vertices must have position)");
                    iter.collect::<Vec<[f32; 3]>>()
                };

                let mut vertices: Vec<Vertex> = positions
                    .into_iter()
                    .map(|position| Vertex {
                        position,
                        ..Default::default()
                    })
                    .collect();

                if let Some(normals) = reader.read_normals() {
                    for (i, normal) in normals.enumerate() {
                        vertices[i].normal = normal;
                    }
                }

                let indices = reader
                    .read_indices()
                    .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
                    .expect("Failed to read indices");

                let prim_mat = prim.material();
                let pbr = prim_mat.pbr_metallic_roughness();
                let material_idx = prim_mat.index().unwrap_or(0);

                let mat = &mut mats[material_idx];
                mat.base_col = pbr.base_color_factor();

                meshes.push(Mesh {
                    vertices,
                    indices,
                    material_idx,
                });
            }
        }
    }
}
