use macroquad::{ prelude::*, window };

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

#[macroquad::main("Scarengine")]
async fn main() {
    //* pre loop vars and logic go here */
    window::request_new_screen_size(640.0, 360.0);

    let map = bmp::open("assets/images/map.bmp").unwrap();

    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(yaw.cos() * pitch.cos(), pitch.sin(), yaw.sin() * pitch.cos()).normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let tex_floor = load_texture("assets/textures/floor.png").await.unwrap();
    tex_floor.set_filter(FilterMode::Nearest);

    let tex_ceiling = load_texture("assets/textures/ceiling.png").await.unwrap();
    tex_ceiling.set_filter(FilterMode::Nearest);

    let tex_wall = load_texture("assets/textures/wall.png").await.unwrap();
    tex_wall.set_filter(FilterMode::Nearest);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(yaw.cos() * pitch.cos(), pitch.sin(), yaw.sin() * pitch.cos()).normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(Color {
            r: 100.0 / 255.0,
            g: 149.0 / 255.0,
            b: 237.0 / 255.0,
            a: 1.0,
        });
        set_camera(
            &(Camera3D {
                position,
                up,
                target: position + front,
                ..Default::default()
            })
        );

        //* main loop logic goes here */

        draw_grid(16, 1.0, RED, WHITE);
        let map_offset: Vec3<> = Vec3 { x: 7.5, y: 0.0, z: 7.5 };
        for x in 0..map.get_width() {
            for y in 0..map.get_height() {
                if map.get_pixel(x, y).r == 0 {
                    //draw walls
                    draw_cube(
                        Vec3 { x: x as f32, y: 1.2, z: y as f32 } - map_offset,
                        Vec3 { x: 1.0, y: 2.4, z: 1.0 },
                        Some(&tex_wall),
                        WHITE
                    );
                } else {
                    //draw floor
                    draw_plane(
                        Vec3 { x: x as f32, y: 0.5, z: y as f32 } - map_offset,
                        Vec2 { x: 0.5, y: 0.5 },
                        Some(&tex_floor),
                        WHITE
                    );

                    //draw ceiling
                    draw_plane(
                        Vec3 { x: x as f32, y: 2.4, z: y as f32 } - map_offset,
                        Vec2 { x: 0.5, y: 0.5 },
                        Some(&tex_ceiling),
                        WHITE
                    );
                }
            }
        }

        draw_line_3d(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 10.0,
                z: 0.0,
            },
            GREEN
        );
        draw_line_3d(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 { x: 0.0, y: 0.0, z: -10.0 },
            BLUE
        );
        draw_line_3d(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 { x: 10.0, y: 0.0, z: 0.0 },
            RED
        );
        //* main loop logic ends here */

        next_frame().await;
    }
}
