use visualizer_shapes::*;

fn main() {
    let mut frames = Frames::new();
    for i in 0..100 {
        let frame = Frame::new(pos(0.0, 0.0), pos(100.0, 100.0))
            .add_element(
                Circle::new(pos(i as f32, i as f32), 5.0)
                    .stroke(Color::new(0, 0, 0), 1.0)
                    .fill(Color::turbo(i as f32 / 100.0))
                    .element()
            )
            .add_element(
                Text::new(format!("{}", i), 5.0, pos(i as f32, i as f32))
                .color(Color::new(255, 255, 255))
                .element()
            )
            ;
        frames = frames.add_frame(frame);
    }
    frames.encode_to_file("visualizer/multi.vis").unwrap();
}

