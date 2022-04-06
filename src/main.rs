//====================================================================

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod core;
mod render;
mod voxels;

//====================================================================

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = core::state::State::new(&window).await;


    let mut last_update_inst = std::time::Instant::now();
    let mut last_frame_inst = std::time::Instant::now();

    let (mut frame_time_frame_count, mut frame_time_accum_time) = (0, 0.0);
    let (mut fps_frame_count, mut fps_accum_time) = (0, 0.0);


    let mut average_frame_time = std::collections::VecDeque::from([0., 0., 0., 0.]);
    let mut average_fps = std::collections::VecDeque::from([0., 0., 0., 0.]);

    let mut debug_accum_time = 0.0;

    const TARGET_FPS: f64 = 75.;



    event_loop.run(move |event, _, control_flow| match event {

        //--------------------------------------------------

        //Control Events go here

        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !state.input(event) {
            match event {

                //________________________________________

                //Window Events go Here
                
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                //________________________________________

                WindowEvent::Resized(new_inner_size) => {
                    state.resize(*new_inner_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size)
                }

                //________________________________________

                _ => {}

                //________________________________________
            }
        }

        //--------------------------------------------------

        Event::RedrawRequested(window_id) => if window_id == window.id() {
            state.update();

            let last_elapsed = last_frame_inst.elapsed().as_secs_f32();

            frame_time_accum_time += last_elapsed;
            fps_accum_time += last_elapsed;
            debug_accum_time += last_elapsed;

            last_frame_inst = std::time::Instant::now();

            frame_time_frame_count += 1;
            fps_frame_count += 1;

            if frame_time_frame_count == 100 {

                let frame_time = frame_time_accum_time * 1000.0 / frame_time_frame_count as f32;
                average_frame_time.push_front(frame_time);
                average_frame_time.remove(3);

                frame_time_accum_time = 0.0;
                frame_time_frame_count = 0;
            }

            if fps_accum_time >= 1.0 {
                average_fps.push_front(fps_frame_count as f32);
                average_fps.remove(3);

                fps_accum_time = 0.0;
                fps_frame_count = 0;
            }


            if debug_accum_time >= 2.0 {
                //let mut average_fps_count = 0;
                //for count in &average_fps {
                //    average_fps_count += count;
                //}

                let average_fps_count: f32 = average_fps.iter().sum();
                let average_frame_time_count: f32 = average_frame_time.iter().sum();

                println!("Avg fps: {}, Avg frame time: {}", average_fps_count, average_frame_time_count);

                debug_accum_time = 0.;
            }



            match state.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => state.resize(state.get_size()),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }

        //--------------------------------------------------

        Event::MainEventsCleared => {
            window.request_redraw();
        }
        
        /*
        Event::RedrawEventsCleared => {
            
            let target_frametime = std::time::Duration::from_secs_f64(1.0 / TARGET_FPS);
            let time_since_last_frame = last_update_inst.elapsed();
            if time_since_last_frame >= target_frametime {
                window.request_redraw();
                last_update_inst = std::time::Instant::now();
            } else {
                *control_flow = ControlFlow::WaitUntil(
                    std::time::Instant::now() + target_frametime - time_since_last_frame,
                );
            }
        }*/

        //--------------------------------------------------

        _ => {}

        //--------------------------------------------------
    });
}

//====================================================================

fn main() {
    pollster::block_on(run());
}

//====================================================================