#[macro_use]
extern crate conrod;
extern crate num;
extern crate conrod_graph;
extern crate piston_window;
extern crate find_folder;

use piston_window::{EventLoop, PistonWindow, UpdateEvent, WindowSettings};

widget_ids! {
    struct Ids {
        canvas,
        graph,
    }
}

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let opengl = piston_window::OpenGL::V3_1;

    let mut window: PistonWindow = WindowSettings::new("Graph Demo", [WIDTH, HEIGHT])
        .opengl(opengl)
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(60);

    let mut ui = conrod::UiBuilder::new().build();

    let ids = Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets")
            .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // Create an image. Empty, since we have no images.
    let image_map = conrod::image::Map::new();

    while let Some(event) = window.next() {

        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            set_ui(ui.set_widgets(), &ids)
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }
}

fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids) {
    use conrod::{widget, Positionable, Sizeable, Widget};
    use conrod_graph::LineGraph;

    // Set up canvas
    widget::Canvas::new()
        .set(ids.canvas, ui);

    // Creature a graph from x: [-1.0, 1.0] and y: [-1.0, 1.0]
    LineGraph::new(-1.0, 1.0, -1.0, 1.0, |x| f64::sin((x + 1.0)*std::f64::consts::PI))
		.parent(ids.canvas)
        .middle()
        .wh_of(ids.canvas)
		.set(ids.graph, ui);
}
