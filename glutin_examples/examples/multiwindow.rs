mod support;

use femtovg::{Canvas, Color, Path, Paint};
use femtovg::renderer::OpenGl;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use support::{ContextCurrentWrapper, ContextTracker, ContextWrapper};

fn main() {
    let el = EventLoop::new();
    let mut ct = ContextTracker::default();

    let mut windows = std::collections::HashMap::new();
    for index in 0..3 {
        let title = format!("Charming Window #{}", index + 1);
        let wb = WindowBuilder::new().with_title(title);
        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let renderer = OpenGl::new(|s| windowed_context.context().get_proc_address(s) as *const _)
        .expect("Cannot create renderer");
        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

        let size = windowed_context.window().inner_size();
        canvas.set_size(size.width, size.height, 1.0);

        //let gl = support::load(&windowed_context.context());
        let window_id = windowed_context.window().id();
        let context_id = ct.insert(ContextCurrentWrapper::PossiblyCurrent(
            ContextWrapper::Windowed(windowed_context),
        ));
        windows.insert(window_id, (context_id, canvas, index));
    }

    el.run(move |event, _, control_flow| {
        //println!("{:?}", event);
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::Resized(physical_size) => {
                    let windowed_context = ct.get_current(windows[&window_id].0).unwrap();
                    let windowed_context = windowed_context.windowed();
                    windowed_context.resize(physical_size);
                }
                WindowEvent::CloseRequested => {
                    if let Some((cid, _, _)) = windows.remove(&window_id) {
                        ct.remove(cid);
                        println!("Window with ID {:?} has been closed", window_id);
                    }
                }
                _ => (),
            },

            Event::MainEventsCleared => {
                for (_, window) in &windows {
                    let windowed_context = ct.get_current(window.0).unwrap();
                    windowed_context.windowed().window().request_redraw();
                }
            }

            Event::RedrawRequested(window_id) => {
                if let Some(window) = &mut windows.get_mut(&window_id) {
    
                    let windowed_context = ct.get_current(window.0).unwrap();

                    let size = windowed_context.windowed().window().inner_size();

                    //window.1.set_size(size.width, size.height, 1.0);
                    window.1.clear_rect(0, 0, size.width, size.height, Color::rgb(255, 255, 255));


                    let mut path = Path::new();
                    path.rect(0.0, 0.0, 50.0, 50.0);
                    window.1.fill_path(&mut path, Paint::color(Color::rgb(200, 50, 50)));
    
                    window.1.flush();
                    windowed_context.windowed().swap_buffers().unwrap();

                }

            }
            _ => (),
        }

        if windows.is_empty() {
            *control_flow = ControlFlow::Exit
        } else {
            *control_flow = ControlFlow::Wait
        }
    });
}
