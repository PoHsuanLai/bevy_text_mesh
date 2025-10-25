use bevy::prelude::*;

use crate::{
    font_loader::TextMeshFont,
    mesh_cache::{CacheKey, MeshCache},
    text_mesh::{FontStyle, TextMesh},
};

pub(crate) struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub uvs: Vec<[f32; 2]>,
}

// FIXME: add validator, that validates all .unwrap's() at addition time
// now crashes might occur
//
// TODO: optimization possibility - take char mesh up to modified char
// from the existing mesh
pub(crate) fn generate_text_mesh(
    text_mesh: &TextMesh,
    text_mesh_font: &TextMeshFont,
    cache: Option<&mut MeshCache>,
) -> MeshData {
    trace!("Generate text mesh: {:?}", text_mesh.text);

    let mut internal_cache;

    let cache = match cache {
        Some(cache) => cache,
        None => {
            internal_cache = Some(MeshCache::default());
            internal_cache.as_mut().unwrap()
        }
    };

    // TODO performance: pre-allocate capacity
    let mut vertices = Vec::new(); //with_capacity(4308); // TODO: allocate opportunistically
    let mut normals = Vec::new(); //with_capacity(4308); // TODO: allocate opportunistically
    let mut indices = Vec::new(); //with_capacity(8520);

    let mut vertices_offset: usize = 0;

    let depth = 0.08;

    let text = if text_mesh.style.font_style.contains(FontStyle::UPPERCASE) {
        text_mesh.text.to_uppercase()
    } else if text_mesh.style.font_style.contains(FontStyle::LOWERCASE) {
        text_mesh.text.to_lowercase()
    } else {
        text_mesh.text.clone() // TODO performance - extra allocation
    };

    let scalar = match text_mesh.style.font_size.as_scalar() {
        Some(scalar) => scalar,
        None => todo!("Font automatic sizing has not been implemented yet"),
    };

    let spacing = Vec2::new(0.08, 0.1) * scalar;

    let mut scaled_offset = Vec2::ZERO;
    let mut scaled_row_y_max_height = 0.;

    //println!("scalar={}, spacing={}", scalar, spacing);
    for char in text.chars() {
        //println!("{} offset={}", char, scaled_offset);
        if char == ' ' {
            scaled_offset.x += 0.2 * scalar + spacing.x;
            continue;
        } else if char == '\n' {
            scaled_offset.x = 0.;
            scaled_offset.y -= scaled_row_y_max_height + spacing.y;
            continue;
        }

        let key = CacheKey::new_3d(char, depth);

        let mesh = match cache.meshes.get(&key) {
            Some(mesh) => mesh,
            None => {
                // Get font from TextMeshFont
                let font = text_mesh_font.get_font().expect("Failed to load font");

                let mesh = match &text_mesh.size.depth {
                    Some(unit) => {
                        let depth = unit.as_scalar().unwrap();
                        match font.glyph_to_mesh_3d(char, text_mesh.style.mesh_quality, depth) {
                            Ok(mesh) => mesh,
                            Err(e) => {
                                // Try to generate '?' as fallback
                                eprintln!("WARNING: Failed to convert glyph '{}' to 3D mesh: {:?}", char, e);
                                eprintln!("  Font: {:?}", text_mesh.style.font);
                                eprintln!("  Quality: {:?}", text_mesh.style.mesh_quality);
                                eprintln!("  Depth: {}", depth);
                                eprintln!("  Trying fallback to '?' character...");

                                match font.glyph_to_mesh_3d('?', text_mesh.style.mesh_quality, depth) {
                                    Ok(mesh) => mesh,
                                    Err(_) => panic!("Failed to generate 3D mesh for character '{}' and fallback '?'", char),
                                }
                            }
                        }
                    },
                    None => todo!("2d glyphs are not implemented yet. Define depth"),
                };

                cache.meshes.insert(key.clone(), mesh);
                cache.meshes.get(&key).unwrap()
            }
        };

        let (mut xmin, mut xmax) = (f32::MAX, f32::MIN);
        let (mut ymin, mut ymax) = (f32::MAX, f32::MIN);
        for vertex in mesh.iter_vertices() {
            let (x, y, _z) = vertex.val();
            // optimization possibility: calculate per-glyph min/max when caching
            if x < xmin {
                xmin = x;
            }
            if x > xmax {
                xmax = x;
            }

            if y < ymin {
                ymin = y;
            }
            if y > ymax {
                ymax = y;
            }
        }

        let y_diff = (ymax - ymin) * scalar;
        if scaled_row_y_max_height < y_diff {
            scaled_row_y_max_height = y_diff;
        }

        for vertex in mesh.iter_vertices() {
            let (x, y, z) = vertex.val();
            vertices.push([
                x * scalar + scaled_offset.x - xmin * scalar,
                y * scalar + scaled_offset.y,
                z * scalar,
            ]);
        }

        /*
        println!(
            " - x({:.3} - {:.3})={:.3}, y({:.3} - {:.3})={:.3}",
            xmin * scalar,
            xmax * scalar,
            (xmax - xmin) * scalar,
            ymin * scalar,
            ymax * scalar,
            (ymax - ymin) * scalar
        );
        */
        // 13 microsecs

        for normal in mesh.iter_normals().unwrap() {
            let (x, y, z) = normal.val();
            normals.push([x, y, z]);
        }
        // total = 24ms

        for face in mesh.iter_faces() {
            let val = face.val();
            indices.extend_from_slice(&[
                (val.0) as u32 + vertices_offset as u32,
                (val.1) as u32 + vertices_offset as u32,
                (val.2) as u32 + vertices_offset as u32,
            ]);
        }
        // 30 microsecs

        vertices_offset += mesh.vertex_count();

        scaled_offset.x += (xmax - xmin) * scalar + spacing.x;

        if text_mesh.size.wrapping
            && scaled_offset.x + scalar + spacing.x > text_mesh.size.width.as_scalar().unwrap()
        {
            scaled_offset.x = 0.;
            scaled_offset.y -= scaled_row_y_max_height + spacing.y;
        }

        //println!("");
    }

    let uvs = vertices.iter().map(|_vert| [0., 1.]).collect::<Vec<_>>();

    MeshData {
        vertices,
        normals,
        indices,
        uvs,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mesh_data_generator::generate_text_mesh, text_mesh::TextMesh, SizeUnit, TextMeshSize,
        TextMeshStyle, font_loader::TextMeshFont,
    };

    use super::*;

    pub(crate) fn get_font_bytes() -> Vec<u8> {
        std::fs::read("./assets/fonts/FiraMono-Medium.ttf").unwrap()
    }

    #[test]
    fn test_generate_mesh() {
        let mut mesh_cache = MeshCache::default();
        let font = TextMeshFont {
            bytes: get_font_bytes(),
        };

        let text_mesh = TextMesh {
            text: "hello world!".to_string(),
            size: TextMeshSize {
                width: SizeUnit::NonStandard(36. * 2.),
                height: SizeUnit::NonStandard(36. * 5.),
                ..Default::default()
            },
            style: TextMeshStyle {
                font_size: SizeUnit::NonStandard(18.),
                ..Default::default()
            },
            ..Default::default()
        };

        let _ = generate_text_mesh(&text_mesh, &font, Some(&mut mesh_cache));
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_get_glyph_cached(b: &mut Bencher) {
        let mut mesh_cache = MeshCache::default();
        let font = TextMeshFont {
            bytes: tests::get_font_bytes(),
        };

        let text_mesh = TextMesh::new_no_font("hello world!".to_string());
        let _ = generate_text_mesh(&text_mesh, &font, Some(&mut mesh_cache));

        b.iter(|| {
            let _ = generate_text_mesh(&text_mesh, &font, Some(&mut mesh_cache));
        });
    }

    #[bench]
    fn bench_get_glyph_no_cache(b: &mut Bencher) {
        let font = TextMeshFont {
            bytes: tests::get_font_bytes(),
        };
        let text_mesh = TextMesh::new_no_font("hello world!".to_string());

        b.iter(|| {
            let _ = generate_text_mesh(&text_mesh, &font, None);
        });
    }
}
