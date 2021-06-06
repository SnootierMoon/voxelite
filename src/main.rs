mod render;
mod window;

const DEBUG_MODE: bool = cfg!(debug_assertions);

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .init();

    let window = window::Window::new();

    let instance = render::Instance::new(window.window());
    let mut surface = render::Surface::new(instance.clone(), window.window());
    let mut renderer = render::Renderer::new(&surface);
    let mut voxel_renderer = render::VoxelRenderer::new(&surface);

    let mut camera = render::Camera::new(ultraviolet::Vec3::new(-5., 0., 0.), 0., 0.);

    window.run(move |state, window| {
        if state.quit() {
            instance.wait_idle();
            return
        }

        camera.pos_mut().x -= state.frame_elapsed().as_secs_f32();

        let matrix = camera.matrix(45., surface.aspect_ratio());

        if !renderer.render(&mut surface, |command_buffer| {
            voxel_renderer.render(command_buffer, &matrix)
        }) {
            instance.wait_idle();
            surface.rebuild(window.window());
            voxel_renderer.rebuild(&surface)
        }
    });
}
