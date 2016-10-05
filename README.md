# Conrod Graph

`conrod_graph` is a collection of widgets for conrod that ease the creation of graphs. It was inspired by the Qt Custom Plot library.

## How to use

At the moment, `conrod_graph` only supports drawing line graphs. The graph accepts a `Fn(f64) -> f64`.

```Rust
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
```
