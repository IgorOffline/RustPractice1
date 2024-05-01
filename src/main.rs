use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RowCol {
    pub row: i32,
    pub col: i32,
}

#[macroquad::main("Tic Tac Toe")]
async fn main() {
    let texture_hand: Texture2D = load_texture("assets/openmoji1F449yellow_p.png")
        .await
        .unwrap();
    let texture_x: Texture2D = load_texture("assets/openmoji_274C_p.png").await.unwrap();
    let texture_circle: Texture2D = load_texture("assets/openmoji1F535_p.png").await.unwrap();

    let gray_background: Color = Color::from_hex(0x4C4C4C);
    let gray1: Color = Color::from_hex(0x6C6C6C);
    let _gray2: Color = Color::from_hex(0x8F8F8F);
    let _blue1: Color = Color::from_hex(0x41535C);
    let _blue2: Color = Color::from_hex(0x5A737F);

    let mut x_to_move = true;
    let mut selected_square = RowCol { row: 0, col: 0 };
    let mut grid: [[i32; 3]; 3] = [[0; 3]; 3];

    macroquad::window::request_new_screen_size(500.0, 500.0);
    loop {
        if is_key_pressed(KeyCode::H) && selected_square.col > 0 {
            selected_square.col -= 1;
        }
        if is_key_pressed(KeyCode::J) && selected_square.row < 2 {
            selected_square.row += 1;
        }
        if is_key_pressed(KeyCode::K) && selected_square.row > 0 {
            selected_square.row -= 1;
        }
        if is_key_pressed(KeyCode::L) && selected_square.col < 2 {
            selected_square.col += 1;
        }
        if is_key_pressed(KeyCode::F) {
            let new_square_value;
            if x_to_move {
                new_square_value = 1;
            } else {
                new_square_value = 2;
            }
            grid[selected_square.row as usize][selected_square.col as usize] = new_square_value;

            x_to_move = !x_to_move;

            println!(
                "selected_square: {:?}, x_to_move: {}",
                selected_square, x_to_move
            );
        }

        clear_background(gray_background);
        let row_col_start = 100.0;
        let row_col_offset = 100.0;
        for i in 0..3 {
            for j in 0..3 {
                if grid[i][j] == 1 {
                    draw_texture_ex(
                        &texture_x,
                        row_col_start + j as f32 * row_col_offset,
                        row_col_start + i as f32 * row_col_offset,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(75.0, 75.0)),
                            ..Default::default()
                        },
                    );
                } else if grid[i][j] == 2 {
                    draw_texture_ex(
                        &texture_circle,
                        row_col_start + j as f32 * row_col_offset,
                        row_col_start + i as f32 * row_col_offset,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(75.0, 75.0)),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_rectangle(
                        row_col_start + j as f32 * row_col_offset,
                        row_col_start + i as f32 * row_col_offset,
                        75.0,
                        75.0,
                        gray1,
                    );
                }
            }
        }
        draw_texture_ex(
            &texture_hand,
            row_col_start + (selected_square.col as f32 * row_col_offset) - 15.0,
            row_col_start + (selected_square.row as f32 * row_col_offset) + 15.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(50.0, 50.0)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
