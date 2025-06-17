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

    let window_size = PhysicalSize::new(320, 240);
    let window = WindowBuilder::new()
        .with_title("Checkerboard")
        .with_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

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
                        if let winit::keyboard::Key::Named(NamedKey::Shift) = event.logical_key
                        {
                            draw_checkerboard(
                                pixels.frame_mut(),
                                window_size.width,
                                window_size.height,
                                true,
                            );
                            if pixels.render().is_err()
                            {
                                event_loop_target.exit();
                            }
                            thread::sleep(Duration::from_millis(500));
                        }
                    }
                }

                WindowEvent::RedrawRequested =>
                {
                    draw_checkerboard(
                        pixels.frame_mut(),
                        window_size.width,
                        window_size.height,
                        false,
                    );
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

fn draw_checkerboard(frame: &mut [u8], width: u32, height: u32, flip: bool)
{
    let square_size = 20;

    for y in 0..height
    {
        for x in 0..width
        {
            let mut is_dark = (x / square_size + y / square_size) % 2 == 0;
            if flip
            {
                is_dark = !is_dark;
            }

            let i = ((y * width + x) * 4) as usize;

            if is_dark
            {
                frame[i..i + 4].copy_from_slice(&[0x40, 0x40, 0x40, 0xff]); // dark square
            }
            else
            {
                frame[i..i + 4].copy_from_slice(&[0xc0, 0xc0, 0xc0, 0xff]); // light square
            }
        }
    }
}
