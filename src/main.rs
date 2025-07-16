mod cpu;
pub mod gpu;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::NamedKey,
    window::WindowBuilder,
};

use std::{thread, time::Duration};

use std::fs::File;
use std::io::Read;

fn load_boot_rom(path: &str) -> std::io::Result<Vec<u8>>
{
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() -> Result<(), pixels::Error>
{
    let boot_rom = load_boot_rom("dmg_boot.bin").expect("Failed to load boot ROM");
    println!("Boot ROM loaded, {} bytes", boot_rom.len());

    let mut cpu = cpu::CPU::new(boot_rom);

    let event_loop = EventLoop::new().unwrap();

    let window_size = PhysicalSize::new(240, 180);
    let window = WindowBuilder::new()
        .with_title("Checkerboard")
        .with_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels: Pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

    let _ = event_loop.run(move |event, event_loop_target| {
        event_loop_target.set_control_flow(ControlFlow::Poll);

        match event
        {
            Event::WindowEvent { event, .. } => match event
            {
                WindowEvent::CloseRequested =>
                {
                    event_loop_target.exit();
                }

                WindowEvent::KeyboardInput { event, .. } =>
                {
                    if event.state == ElementState::Pressed
                    {
                        if let winit::keyboard::Key::Named(NamedKey::Escape) = event.logical_key
                        {
                            event_loop_target.exit();
                        }
                        /*if let winit::keyboard::Key::Named(NamedKey::Shift) = event.logical_key
                        {
                            thread::sleep(Duration::from_millis(500));
                        }*/
                    }
                }

                WindowEvent::RedrawRequested =>
                {
                    let vram = cpu.bus.gpu.vram;
                    render_tileset(&vram, pixels.frame_mut(), window_size.width as usize);
                    //thread::sleep(Duration::from_millis(10));

                    if pixels.render().is_err()
                    {
                        event_loop_target.exit();
                    }
                }

                _ =>
                {}
            },

            Event::AboutToWait =>
            {
                for _ in 0..100
                {
                    cpu.step(); // or whatever your stepping function is
                }
                window.request_redraw();
            }

            _ =>
            {}
        }
    });
    Ok(())
}

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;
const TILE_BYTES: usize = 16;
const TILE_COUNT: usize = 384;
const TILES_PER_ROW: usize = 24;
const TILESET_ROWS: usize = TILE_COUNT / TILES_PER_ROW; // 16

/// Decode a single tile
fn decode_tile(tile_data: &[u8]) -> [[u8; 8]; 8]
{
    let mut tile = [[0u8; 8]; 8];

    for y in 0..8
    {
        let low = tile_data[y * 2];
        let high = tile_data[y * 2 + 1];

        for x in 0..8
        {
            let bit = 1 << (7 - x);
            let lo_bit = (low & bit) >> (7 - x);
            let hi_bit = (high & bit) >> (7 - x);
            tile[y][x] = (hi_bit << 1) | lo_bit;
        }
    }

    tile
}

/// Convert pixel value to RGBA
fn pixel_value_to_rgba(value: u8) -> [u8; 4]
{
    match value
    {
        0 => [255, 255, 255, 255],
        1 => [192, 192, 192, 255],
        2 => [96, 96, 96, 255],
        3 => [0, 0, 0, 255],
        _ => [255, 0, 255, 255], // Error color
    }
}

/// Render all tiles to the frame buffer
fn render_tileset(vram: &[u8], frame: &mut [u8], frame_width: usize)
{
    for tile_index in 0..TILE_COUNT
    {
        let tile_x = tile_index % TILES_PER_ROW;
        let tile_y = tile_index / TILES_PER_ROW;

        let tile_base = tile_index * TILE_BYTES;
        let tile_data = &vram[tile_base..tile_base + TILE_BYTES];
        let tile = decode_tile(tile_data);

        for y in 0..TILE_HEIGHT
        {
            for x in 0..TILE_WIDTH
            {
                let color = pixel_value_to_rgba(tile[y][x]);

                let screen_x = tile_x * TILE_WIDTH + x;
                let screen_y = tile_y * TILE_HEIGHT + y;

                let i = (screen_y * frame_width + screen_x) * 4;

                frame[i..i + 4].copy_from_slice(&color);
            }
        }
    }
}
