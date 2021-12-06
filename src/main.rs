use rand::Rng;

use quicksilver::{
    blinds::MouseButton,
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
    input::Event,
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Square Example",
            size: Vector { x: 500.0, y: 500.0 },
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Creates the board. Is false if no bomb but true for bomb.
    const BOARD_WIDTH: usize = 10;
    const BOARD_HEIGHT: usize = 10;
    let mut board_bombs: [bool; BOARD_HEIGHT * BOARD_WIDTH] = [false; BOARD_HEIGHT * BOARD_WIDTH];
    let mut board_uncoverd: [bool; BOARD_HEIGHT * BOARD_WIDTH] =
        [false; BOARD_HEIGHT * BOARD_WIDTH];
    let mut board_numbers: [i32; BOARD_HEIGHT * BOARD_WIDTH] = [0; BOARD_HEIGHT * BOARD_WIDTH];
    // let mut bombs_left = 0;

    // Generates bombs.
    for i in 0..(BOARD_HEIGHT * BOARD_WIDTH) {
        let r: u8 = rand::thread_rng().gen();
        if r < 50 {
            board_bombs[i] = true;
            // bombs_left += 1;
        }
    }

    // Clears all 0's.
    fn clear_zero(
        x: usize,
        y: usize,
        board_numbers: &mut [i32; BOARD_HEIGHT * BOARD_WIDTH],
        board_uncoverd: &mut [bool; BOARD_HEIGHT * BOARD_WIDTH],
        board_bombs: &mut [bool; BOARD_HEIGHT * BOARD_WIDTH],
    ) {
        if (x > 0)
            && (board_numbers[y * BOARD_WIDTH + (x - 1)] == 0)
            && (!board_bombs[y * BOARD_WIDTH + (x - 1)])
        {
            board_uncoverd[y * BOARD_WIDTH + (x - 1)] = true;
        }

        if (x > 0)
            && (y > 0)
            && (board_numbers[(y - 1) * BOARD_WIDTH + (x - 1)] == 0)
            && (!board_bombs[(y - 1) * BOARD_WIDTH + (x - 1)])
        {
            board_uncoverd[(y - 1) * BOARD_WIDTH + (x - 1)] = true;
        }

        if (y > 0)
            && (board_numbers[(y - 1) * BOARD_WIDTH + x] == 0)
            && (!board_bombs[(y - 1) * BOARD_WIDTH + x])
        {
            board_uncoverd[(y - 1) * BOARD_WIDTH + x] = true;
        }

        if (x < BOARD_WIDTH - 1)
            && (y > 0)
            && (board_numbers[(y - 1) * BOARD_WIDTH + x + 1] == 0)
            && (!board_bombs[(y - 1) * BOARD_WIDTH + x + 1])
        {
            board_uncoverd[(y - 1) * BOARD_WIDTH + x + 1] = true;
        }

        if (x < BOARD_WIDTH - 1)
            && (board_numbers[y * BOARD_WIDTH + x + 1] == 0)
            && (!board_bombs[y * BOARD_WIDTH + x + 1])
        {
            board_uncoverd[y * BOARD_WIDTH + x + 1] = true;
        }

        if (x < BOARD_WIDTH - 1)
            && (y < BOARD_HEIGHT - 1)
            && (board_numbers[(y + 1) * BOARD_WIDTH + x + 1] == 0)
            && (!board_bombs[(y + 1) * BOARD_WIDTH + x + 1])
        {
            board_uncoverd[(y + 1) * BOARD_WIDTH + x + 1] = true;
        }

        if (y < BOARD_HEIGHT - 1)
            && (board_numbers[(y + 1) * BOARD_WIDTH + x] == 0)
            && (!board_bombs[(y + 1) * BOARD_WIDTH + x])
        {
            board_uncoverd[(y + 1) * BOARD_WIDTH + x] = true;
        }

        if (x > 0)
            && (y < BOARD_HEIGHT - 1)
            && (board_numbers[(y + 1) * BOARD_WIDTH + (x - 1)] == 0)
            && (!board_bombs[(y + 1) * BOARD_WIDTH + (x - 1)])
        {
            board_uncoverd[(y + 1) * BOARD_WIDTH + (x - 1)] = true;
        }
    }

    // Generate numbers.
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let mut bombs_close = 0;

            if (x > 0) && (board_bombs[y * BOARD_WIDTH + (x - 1)]) {
                bombs_close += 1;
            }

            if (x > 0) && (y > 0) && (board_bombs[(y - 1) * BOARD_WIDTH + (x - 1)]) {
                bombs_close += 1;
            }

            if (y > 0) && (board_bombs[(y - 1) * BOARD_WIDTH + x]) {
                bombs_close += 1;
            }

            if (x < BOARD_WIDTH - 1) && (y > 0) && (board_bombs[(y - 1) * BOARD_WIDTH + x + 1]) {
                bombs_close += 1;
            }

            if (x < BOARD_WIDTH - 1) && (board_bombs[y * BOARD_WIDTH + x + 1]) {
                bombs_close += 1;
            }

            if (x < BOARD_WIDTH - 1)
                && (y < BOARD_HEIGHT - 1)
                && (board_bombs[(y + 1) * BOARD_WIDTH + x + 1])
            {
                bombs_close += 1;
            }

            if (y < BOARD_HEIGHT - 1) && (board_bombs[(y + 1) * BOARD_WIDTH + x]) {
                bombs_close += 1;
            }

            if (x > 0) && (y < BOARD_HEIGHT - 1) && (board_bombs[(y + 1) * BOARD_WIDTH + (x - 1)]) {
                bombs_close += 1;
            }

            board_numbers[y * BOARD_WIDTH + x] = bombs_close;
        }
    }

    let mut last_mouse_pos_corrected = Vector::new(0.0, 0.0);
    let block_size = Vector::new(
        window.size().x / BOARD_WIDTH as f32,
        window.size().y / BOARD_HEIGHT as f32,
    );

    loop {
        while let Some(e) = input.next_event().await {
            match e {
                Event::PointerInput(m_e) => {
                    if m_e.button().eq(&MouseButton::Left) {
                        let pos = last_mouse_pos_corrected / 50.0;
                        if (0.0 <= pos.x && pos.x < BOARD_WIDTH as f32)
                            && (0.0 <= pos.y && pos.y < BOARD_HEIGHT as f32)
                        {
                            board_uncoverd[(pos.y as usize) * BOARD_WIDTH + (pos.x as usize)] =
                                true;
                            clear_zero(
                                pos.x as usize,
                                pos.y as usize,
                                &mut board_numbers,
                                &mut board_uncoverd,
                                &mut &mut board_bombs,
                            );
                        }
                    }
                }
                Event::PointerMoved(m_e) => {
                    let pos = Vector::new(
                        m_e.location().x
                            - (m_e.location().x % (window.size().x / BOARD_WIDTH as f32)),
                        m_e.location().y
                            - (m_e.location().y % (window.size().x / BOARD_WIDTH as f32)),
                    );
                    last_mouse_pos_corrected = pos;
                }
                _ => {}
            }
        }
        // Clear the screen to a blank, white color.
        gfx.clear(Color::WHITE);

        // Draws squares.
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let pos = Vector::new(
                    x as f32 * (window.size().x / BOARD_WIDTH as f32),
                    y as f32 * (window.size().y / BOARD_HEIGHT as f32),
                );

                let rect = Rectangle::new(pos, block_size);
                gfx.fill_rect(
                    &rect,
                    if board_uncoverd[y * BOARD_WIDTH + x] {
                        if board_bombs[y * BOARD_WIDTH + x] {
                            Color::RED
                        } else {
                            Color::WHITE
                        }
                    } else {
                        Color::BLACK
                    },
                );
                gfx.stroke_rect(&rect, Color::WHITE);
            }
        }

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if board_uncoverd[y * BOARD_WIDTH + x] && !board_bombs[y * BOARD_WIDTH + x] {
                    let pos = Vector::new(
                        x as f32 * (window.size().x / BOARD_WIDTH as f32),
                        y as f32 * (window.size().y / BOARD_HEIGHT as f32),
                    );

                    let rect = Rectangle::new(pos, block_size);

                    if board_numbers[y * BOARD_WIDTH + x] != 0 {
                        match Image::load(
                            &gfx,
                            format!("{}.png", board_numbers[y * BOARD_WIDTH + x]),
                        )
                        .await
                        {
                            Err(e) => println!("{}", e),
                            Ok(v) => gfx.draw_image(&v, rect),
                        };
                    }
                }
            }
        }

        let rect = Rectangle::new(last_mouse_pos_corrected, block_size);
        gfx.fill_rect(&rect, Color::from_rgba(255, 255, 255, 0.4));
        gfx.stroke_rect(&rect, Color::WHITE);

        // Send the data to be drawn.
        gfx.present(&window)?;
    }
}
