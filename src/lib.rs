
#[macro_use]
extern crate conrod;
extern crate num;

pub mod axis;

use conrod::{widget, utils};
use conrod::{Color, Colorable, Positionable, Scalar, Sizeable, Widget};

pub struct LineGraph<F> {
    common: widget::CommonBuilder,
    style: Style,
    // Stuff that plot path has
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    f: F,
}

widget_style! {
    style Style {
        /// The thickness of the plotted line
        - thickness: Scalar { 1.0 }
        /// The color of the line
        - color: Color { theme.shape_color }
    }
}

widget_ids! {
    struct Ids {
        plot_path,
        axis_x,
        axis_y,
    }
}

pub struct State {
    ids: Ids,
}

impl<F> LineGraph<F> {

    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64, f: F) -> Self {
        LineGraph {
            common: widget::CommonBuilder::new(),
            style: Style::new(),
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
            f: f,
        }
    }

}

impl<F> Widget for LineGraph<F>
    where F: Fn(f64) -> f64,
{
    type State = State;
    type Style = Style;
    type Event = ();

    fn common(&self) -> &widget::CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut widget::CommonBuilder {
        &mut self.common
    }

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        use conrod::Rect;
        let widget::UpdateArgs { id, state, style, rect, ui, ..} = args;
        let LineGraph { min_x, max_x, min_y, max_y, f, ..} = self;

        let thickness = style.thickness(ui.theme());
        let color = style.color(ui.theme());

        // Create x axis
        let x_axis = axis::Axis::new(min_x, max_x)
            .orientation(axis::Orientation::Horizontal);

        // Calculate it's height
        let x_axis_height = match x_axis.default_y_dimension(ui) {
            conrod::Dimension::Absolute(number) => number,
            _ => 0.0,
        };

        // Create y axis
        let y_axis = axis::Axis::new(min_y, max_y)
            .orientation(axis::Orientation::Vertical);

        // Calculate it's width
        let y_axis_width = match y_axis.default_x_dimension(ui) {
            conrod::Dimension::Absolute(number) => number,
            _ => 0.0,
        };

        // Create rectangles for the axis'
        let x_axis_rect = Rect::from_corners(
            [rect.left() + y_axis_width, rect.bottom()],
            [rect.right(),               rect.bottom() + x_axis_height]);
        let y_axis_rect = Rect::from_corners(
            [rect.left(),                rect.bottom() + x_axis_height],
            [rect.left() + y_axis_width, rect.top()]);

        // Place the axis'
        x_axis
            .xy(x_axis_rect.xy())
            .wh(x_axis_rect.dim())
            .set(state.ids.axis_x, ui);
        y_axis
            .xy(y_axis_rect.xy())
            .wh(y_axis_rect.dim())
            .set(state.ids.axis_y, ui);

        // Place PlotPath
        let plot_path_rect = Rect::from_corners(
            [rect.left() + y_axis_width, rect.bottom() + x_axis_height],
            [rect.right(),               rect.top()]);
        widget::PlotPath::new(min_x, max_x, min_y, max_y, f)
            .wh(plot_path_rect.dim())
            .xy(plot_path_rect.xy())
            .color(color)
            .parent(id)
            .crop_kids()
            .set(state.ids.plot_path, ui);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
