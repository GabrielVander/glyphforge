use lib_ascii::domain::entities::GlyphEngineAsciiImpl;
use lib_core::domain::entities::Glyph;
use lib_image::domain::entities::{LumaImage, LumaPixel};
use std::{
    env,
    io::{self, Write, stdout},
};

fn receive_and_process_decoded_frames(
    decoder: &mut ffmpeg_next::decoder::Video,
    input_context: &mut ffmpeg_next::format::context::Input,
    video_stream_index: &usize,
    stdout_handle: &mut io::Stdout,
    duration_per_frame: &std::time::Duration,
) -> Result<(), ffmpeg_next::Error> {
    let mut frame = ffmpeg_next::frame::Video::empty();
    let mut scaler = None;

    // Get initial terminal size
    let (mut term_width, mut term_height): (u32, u32) = match crossterm::terminal::size() {
        Ok((w, h)) => (w.max(1) as u32, h.max(1) as u32), // ensure at least 1x1
        Err(_) => (80, 24),                               // fallback
    };
    for (stream, packet) in input_context.packets() {
        if stream.index() != *video_stream_index {
            continue;
        }

        decoder.send_packet(&packet)?;
        while decoder.receive_frame(&mut frame).is_ok() {
            let width = frame.width();
            let height = frame.height();
            // Re-check terminal size on every frame (to handle resize)
            let (new_term_width, new_term_height) = match crossterm::terminal::size() {
                Ok((w, h)) => (w.max(1) as u32, h.max(1) as u32),
                Err(_) => (term_width, term_height),
            };

            // Update terminal size if changed
            if new_term_width != term_width || new_term_height != term_height {
                term_width = new_term_width;
                term_height = new_term_height;
                // Invalidate scaler so it recreates with new size
                scaler = None;
            }

            // Compute target size while preserving aspect ratio
            let aspect_ratio = width as f64 / height as f64;
            let terminal_ratio = term_width as f64 / term_height as f64;

            let (target_width, target_height) = if aspect_ratio > terminal_ratio {
                // Video is wider than terminal → constrain by width
                (term_width, (term_width as f64 / aspect_ratio) as u32)
            } else {
                // Video is taller than terminal → constrain by height
                ((term_height as f64 * aspect_ratio) as u32, term_height)
            };

            // Adjust for character aspect ratio (ASCII chars are taller than wide)
            // Most terminals have ~2:1 char height:width ratio (e.g., 16x8 pixels per char)
            // let target_height = target_height / 2; // Compensate for character shape

            // Clamp to reasonable bounds
            // let target_width = target_width.min(200); // Avoid huge output
            // let target_height = target_height.clamp(1, 100);
            // Initialize scaler on first frame
            if scaler.is_none() {
                scaler = Some(ffmpeg_next::software::scaling::context::Context::get(
                    frame.format(),
                    width,
                    height,
                    ffmpeg_next::format::Pixel::GRAY8,
                    target_width,
                    target_height,
                    ffmpeg_next::software::scaling::flag::Flags::BILINEAR,
                )?);
            }

            let scaler = scaler.as_mut().unwrap();

            // Scale to smaller frame
            let mut scaled_frame = ffmpeg_next::frame::Video::empty();
            scaler.run(&frame, &mut scaled_frame)?;

            // Convert to ASCII and print
            render_ascii_frame(&scaled_frame, stdout_handle).unwrap();

            // Wait to maintain frame rate
            std::thread::sleep(*duration_per_frame);
        }
    }

    // Flush decoder
    decoder.send_eof()?;
    while decoder.receive_frame(&mut frame).is_ok() {
        if let Some(scaler) = &mut scaler {
            let mut scaled_frame = ffmpeg_next::frame::Video::empty();
            scaler.run(&frame, &mut scaled_frame)?;
            render_ascii_frame(&scaled_frame, stdout_handle).unwrap();
            std::thread::sleep(*duration_per_frame);
        }
    }

    Ok(())
}

fn render_ascii_frame(
    frame: &ffmpeg_next::frame::Video,
    stdout_handle: &mut std::io::Stdout,
) -> Result<(), Box<dyn std::error::Error>> {
    crossterm::execute!(
        stdout_handle,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;

    let data: &[u8] = frame.data(0);
    let linesize = frame.stride(0);
    let width = frame.width() as usize;
    let height = frame.height() as usize;
    let mut image = LumaImage::new(width, height);

    for y in 0..height {
        let row_start = y * linesize;
        for x in 0..width {
            let offset = row_start + x;
            let byte: u8 = data[offset];
            image.add_child(Box::new(LumaPixel::new(byte)));
        }
    }
    stdout_handle.flush()?;
    write!(
        stdout_handle,
        "{}",
        image.as_text(&GlyphEngineAsciiImpl::new())
    )?;

    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("GlyphForge CLI - ASCII Media Renderer");
    println!("--------------------------------------");
    println!("Arguments: {:?}", &args[1..]);

    if args.len() != 1 {
        eprintln!("Usage: <path>");
        std::process::exit(1);
    }

    let path: &str = args[0].as_str();
    ffmpeg_next::init().unwrap();

    let mut input: ffmpeg_next::format::context::Input = ffmpeg_next::format::input(path).unwrap();
    let stream: ffmpeg_next::Stream<'_> = input
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .unwrap();

    let video_stream_index = stream.index();
    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(stream.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    // Terminal setup
    let mut stdout_handle = stdout();
    crossterm::execute!(stdout_handle, crossterm::cursor::Hide).unwrap();
    let _guard = scopeguard::guard((), |_| {
        let _ = crossterm::execute!(stdout(), crossterm::cursor::Show);
    });

    let frame_rate: ffmpeg_next::Rational = stream.avg_frame_rate();

    let duration_per_frame: std::time::Duration =
        if frame_rate.denominator() != 0 && frame_rate.numerator() != 0 {
            std::time::Duration::from_secs_f64(
                frame_rate.denominator() as f64 / frame_rate.numerator() as f64,
            )
        } else {
            std::time::Duration::from_millis(40) // fallback ~25fps
        };

    println!("Playing... Press Ctrl+C to quit.");

    // Decode and render frames

    receive_and_process_decoded_frames(
        &mut decoder,
        &mut input,
        &video_stream_index,
        &mut stdout_handle,
        &duration_per_frame,
    )
    .unwrap();
    // let size_parts: Vec<&str> = args[0].split('x').collect();
    //
    // println!("Received size argument: {}", args[0]);
    // let target_width: usize = size_parts[0].parse::<usize>().unwrap();
    // let target_height: usize = size_parts[1].parse::<usize>().unwrap();
    //
    // println!(
    //     "Target dimensions: width={} height={}",
    //     target_width, target_height
    // );
    //
    // // Get a handle to the standard input stream.
    // let stdin_handle: io::StdinLock<'static> = io::stdin().lock();
    // let mut bytes: io::Bytes<io::StdinLock<'static>> = stdin_handle.bytes();
    // let mut is_done: bool = false;
    // let mut byte_num: u16 = 1;
    // let mut image: LumaImage = LumaImage::new(target_width, target_height);
    // let engine: GlyphEngineAsciiImpl = GlyphEngineAsciiImpl::new();
    //
    // while !is_done {
    //     println!("Reading {:?} bytes from stdin...", bytes.size_hint());
    //     match bytes.take_while() {
    //         Some(byte) => {
    //             // println!("Reading byte #{}: {:?}", byte_num, byte);
    //             image.add_raw_byte(byte.unwrap());
    //             byte_num = byte_num.wrapping_add(1);
    //         }
    //         _ => {
    //             println!("No more bytes to read from stdin.");
    //             is_done = true;
    //         }
    //     }
    //
    //     println!("{}", image.as_text(&engine));
    //     image = LumaImage::new(target_width, target_height);
    // }
}
