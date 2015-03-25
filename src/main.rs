extern crate glium;
extern crate glutin;

fn main() {
    use glium::DisplayBuild;

    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello, world!"))
        .build_glium()
        .unwrap();

    loop {
        for event in display.poll_events() {
            match event {
                glutin::Event::Moved(x, y) => println!("Moved: ({}, {})", x, y),
                glutin::Event::Closed => println!("Closed!"),
                _ => ()
            };
            println!("Got an event");
        }
    }
}
