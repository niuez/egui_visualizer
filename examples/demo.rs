use visualizer_shapes::*;

fn main() {
    let frames = Frames::new()
        .add_frame(
            Frame::new(pos(-10.0, -10.0), pos(100.0, 100.0))
            .add_element(
                Path::from_vertices(vec![pos(10.0, 10.0), pos(90.0, 90.0)])
                    .stroke(Color::new(122, 0, 122), 2.0)
                    .element()
            )
            .add_element(
                Path::from_vertices(vec![pos(90.0, 10.0), pos(10.0, 90.0)])
                    .element()
                    .with_msg("cross")
            )
            .add_element(
                Path::from_vertices(vec![pos(40.0, 40.0), pos(40.0, 50.0), pos(50.0, 50.0), pos(50.0, 40.0)])
                    .close(Color::new(0, 122, 122))
                    .element()
                    .with_msg("box")
            )
            .add_element(
                Circle::new(pos(10.0, 40.0), 10.0)
                    .stroke(Color::new(0, 0, 0), 1.0)
                    .element()
                    .with_msg("circle")
            )
        );
    frames.encode_to_file("visualizer/demo.vis").unwrap();
}

