#[path = "./classes/pipe_builder.rs"]
mod pipe_builder;

#[path = "./classes/bird.rs"]
mod bird;
use bird::Bird;


use nannou::prelude::*;

enum GameState {
    Running,
    GameOver,
}


struct Model {
    game_state:GameState,
    window_height: u32,
    window_width: u32,
    background: wgpu::Texture,
    gravity: f32,
    impulse_rate: f32,
    bird: Bird,
    delta_time: f32,
    delta_time_span: i64,
    pipe_delta_time: f32,
    pipe_delta_time_span: i64,
    pipes: Vec<pipe_builder::Pipe>,
    time_to_new_retry: f32,
}

fn main() {
    nannou::app(model).event(event).view(view).run();
}

fn handle_game_play_events(_app:&App, _model: &mut Model, _event:Event){
    let time = (_app.time - _model.time_to_new_retry) as f64;
    let delta_time = _model.delta_time as f64;
    let delta_time_span = _model.delta_time_span as f64;
    handle_space_press_on_gameplay(_model, _event);

    let pipe_delta_time = _model.pipe_delta_time as f64;
    let pipe_delta_time_span = _model.pipe_delta_time_span as f64;

    if time >= (pipe_delta_time * pipe_delta_time_span) {
        _model.pipe_delta_time_span += 1;
        let pipe = pipe_builder::build(_model.window_height as f32, _model.window_width as f32);
        _model.pipes.push(pipe);
    }

    if time >= (delta_time * delta_time_span) {
        _model.delta_time_span += 1;

        _model.bird.update(_model.delta_time);
        for pipe in &mut _model.pipes {
            pipe.update(_model.delta_time);
        }
        if _model.pipes.len() > 0 {
            if _model.pipes[0].is_out_of_bounds() {
                _model.pipes.remove(0);
            }
        }

        if _model.bird.detect_collision(&_model.pipes) == true {
            _model.game_state = GameState::GameOver;
            _model.delta_time_span = 0;
            _model.pipe_delta_time_span = 0;
        }

        _model.bird.set_force(-_model.gravity);
        
    }
}

fn handle_game_over_events(_app:&App, _model: &mut Model, _event:Event){
    handle_space_press_on_gameover(_app,_model, _event);
}

fn handle_space_press_on_gameover(_app:&App, _model: &mut Model, _event: Event) {
    match _event {
        Event::DeviceEvent(_id, device_event) => match device_event {
            nannou::winit::event::DeviceEvent::Key(keyboard_input) => {
                if keyboard_input.state == nannou::winit::event::ElementState::Pressed {
                    if keyboard_input.virtual_keycode
                        == Some(nannou::winit::event::VirtualKeyCode::Space)
                    {
                        _model.game_state = GameState::Running;
                        _model.bird.reset();
                        _model.pipes.clear();
                        _model.time_to_new_retry = _app.time;
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn handle_space_press_on_gameplay(_model: &mut Model, _event: Event) {
    match _event {
        Event::DeviceEvent(_id, device_event) => match device_event {
            nannou::winit::event::DeviceEvent::Key(keyboard_input) => {
                if keyboard_input.state == nannou::winit::event::ElementState::Pressed {
                    if keyboard_input.virtual_keycode
                        == Some(nannou::winit::event::VirtualKeyCode::Space)
                    {
                        _model.bird.reset_velocity();
                        _model.bird.apply_force(_model.impulse_rate);
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn model(_app: &App) -> Model {
    let window_height = 504;
    let window_width = 900;

    _app.new_window()
        .size(window_width, window_height)
        .view(view)
        .build()
        .unwrap();
    let assets = _app.assets_path().unwrap();
    let img_path = assets.join("bg.png");
    let background = wgpu::Texture::from_path(_app, img_path).unwrap();

    let bird_img_path = assets.join("flappyBird.png");
    let bird_img = wgpu::Texture::from_path(_app, bird_img_path).unwrap();
    Model {
        game_state: GameState::Running,
        window_height,
        window_width,
        background,
        gravity: 300.0,
        impulse_rate: 15000.0,
        bird: Bird::new(0.0, 0.0, 1.0, bird_img, window_height as f32, window_width as f32),
        delta_time: 0.0125,
        delta_time_span: 0,
        pipe_delta_time: 5.0,
        pipe_delta_time_span: 0,
        pipes: Vec::new(),
        time_to_new_retry:0.0
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    match _model.game_state {
        GameState::Running => {
            handle_game_play_events(_app, _model, _event);
        }
        GameState::GameOver => {
            handle_game_over_events(_app, _model, _event);
        }
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();

    // Clear the background to purple.
    draw.texture(&_model.background);

    // Write to the window frame.
  
    _model.bird.draw(&draw);
    for pipe in &_model.pipes {
        pipe.draw(&draw);
    }
    match _model.game_state {
        GameState::GameOver => {
        draw.text("Game Over")
            .x_y(0.0, 0.0)
            .color(BLACK)
            .font_size(50);
        }
        _ => {}
    }   
    
    draw.to_frame(_app, &_frame).unwrap();
}
