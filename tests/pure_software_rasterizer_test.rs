use std::f32::consts::PI;
use widya::game3d::math3d::{Mat4, Vec3};
use widya::game3d::pure_rasterizer::{
    SoftwareMesh, SoftwareRasterizer, SoftwareTriangle, SoftwareVertex,
};
use widya::multimedia::image::{ImageBuffer, RgbaPixel};

#[test]
fn test_software_3d_rasterizer_cube_rendering() {
    let width = 64;
    let height = 64;
    let mut rasterizer = SoftwareRasterizer::new(width, height);
    let mut framebuffer = ImageBuffer::new(width, height, RgbaPixel::BLACK);

    // 1. Create a 3D red cube
    let cube = SoftwareMesh::create_cube(1.5, RgbaPixel::new(255, 0, 0, 255));
    assert_eq!(cube.triangles.len(), 12); // 6 faces * 2 triangles

    // 2. Setup Camera & Matrices
    let eye = Vec3::new(0.0, 0.0, 4.0);
    let target = Vec3::ZERO;
    let up = Vec3::UP;

    let view = Mat4::look_at(eye, target, up);
    let proj = Mat4::perspective_fov(PI / 3.0, 1.0, 0.1, 100.0);
    let view_proj = view.multiply(&proj);
    let model = Mat4::IDENTITY;

    // 3. Render mesh
    rasterizer.render_mesh(&cube, &model, &view_proj, &mut framebuffer);

    // 4. Assert center pixel is rendered with red (shaded)
    let center_idx = (32 * width + 32) as usize;
    let center_px = framebuffer.pixels[center_idx];
    assert!(center_px.r > 0, "Center pixel should be shaded red, got {:?}", center_px);
    assert_eq!(center_px.g, 0);
    assert_eq!(center_px.b, 0);

    // Corner pixel should remain background black
    let corner_px = framebuffer.pixels[0];
    assert_eq!(corner_px, RgbaPixel::BLACK);

    // Z-buffer at center should be finite (< infinity)
    assert!(rasterizer.z_buffer[center_idx].is_finite());

    // 5. Full-stack verification: export software rendered 3D scene directly to pure QOI and BMP!
    let qoi_data = framebuffer.to_qoi(false).expect("Export to QOI failed");
    assert!(qoi_data.starts_with(b"qoif"));

    let bmp_data = framebuffer.to_bmp(24).expect("Export to BMP failed");
    assert!(bmp_data.starts_with(b"BM"));
}

#[test]
fn test_z_buffer_hidden_surface_removal() {
    let width = 32;
    let height = 32;
    let mut rasterizer = SoftwareRasterizer::new(width, height);
    let mut framebuffer = ImageBuffer::new(width, height, RgbaPixel::BLACK);

    // Red triangle further away (Z = 1.0)
    let red_color = RgbaPixel::new(255, 0, 0, 255);
    let t_red = SoftwareTriangle::new(
        SoftwareVertex::new(Vec3::new(-1.0, -1.0, 1.0), red_color),
        SoftwareVertex::new(Vec3::new( 1.0, -1.0, 1.0), red_color),
        SoftwareVertex::new(Vec3::new( 0.0,  1.0, 1.0), red_color),
    );

    // Green triangle closer to camera (Z = -0.5)
    let green_color = RgbaPixel::new(0, 255, 0, 255);
    let t_green = SoftwareTriangle::new(
        SoftwareVertex::new(Vec3::new(-1.0, -1.0, -0.5), green_color),
        SoftwareVertex::new(Vec3::new( 1.0, -1.0, -0.5), green_color),
        SoftwareVertex::new(Vec3::new( 0.0,  1.0, -0.5), green_color),
    );

    let mut mesh_both = SoftwareMesh::new();
    // Intentionally add red FIRST, green SECOND
    mesh_both.triangles.push(t_red);
    mesh_both.triangles.push(t_green);

    let identity = Mat4::IDENTITY;
    rasterizer.render_mesh(&mesh_both, &identity, &identity, &mut framebuffer);

    // The center pixel MUST be green because the green triangle occludes the red triangle!
    let center_px = framebuffer.pixels[(16 * width + 16) as usize];
    assert!(center_px.g > 0, "Closer green triangle should occlude further red triangle, got {:?}", center_px);
}
