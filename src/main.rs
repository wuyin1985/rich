use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

use bevy_app::{App, AppBuilder, Plugin};
use bevy_ecs::world;
use serde::Deserialize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use assets::AssetPlugin;
use game::GamePlugin;
use render::render_manager::RenderState;

use wgpu;


#[derive(Debug, Deserialize)]
struct Property {
    atk: i32,
    def: i32,
}

#[derive(Debug, Deserialize)]
struct LandData
{
    size: i32,
    color: f32,
}

#[derive(Debug, Deserialize)]
enum ActorTypeData
{
    Rain { water: i32, weight: i32 },
    Thunder { light: String },
    Land { data: LandData },
}

#[derive(Debug, Deserialize)]
struct Enemy {
    name: String,
    property: Property,
    speed: i32,
    type_data: ActorTypeData,
}

fn main() {
    env_logger::init();
    let mut builder = App::build();
    builder.add_plugin(AssetPlugin).add_plugin(GamePlugin);
    let mut app = builder.app;

    let event_loop = EventLoop::new();
    let title = env!("CARGO_PKG_NAME");
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    use futures::executor::block_on;
    let mut state = block_on(RenderState::new(&window, env::current_dir().unwrap().to_str().unwrap()));
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                app.update();
                state.update();

                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    });
}
