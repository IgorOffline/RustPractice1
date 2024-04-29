use macroquad::prelude::*;

#[macroquad::main("Tic Tac Toe")]
async fn main() {
    let texture: Texture2D = load_texture("assets/openmoji1F449yellow_p.png")
        .await
        .unwrap();

    let gray_background: Color = Color::from_hex(0x4C4C4C);
    let _gray1: Color = Color::from_hex(0x6C6C6C);
    let _gray2: Color = Color::from_hex(0x8F8F8F);
    let _blue1: Color = Color::from_hex(0x41535C);
    let _blue2: Color = Color::from_hex(0x5A737F);

    macroquad::window::request_new_screen_size(500.0, 500.0);
    loop {
        clear_background(gray_background);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
