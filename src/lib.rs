
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
        line_x,
        axis_x,
        line_y,
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

        let padding_x = 20.0;
        let padding_y = 20.0;

        let graph_left = rect.left()+padding_x;
        let graph_bottom = rect.bottom()+padding_y;
        let graph_right = rect.right();

        let top_left = [graph_left, rect.top()];
        let bottom_left = [graph_left, graph_bottom];
        let bottom_right = [rect.right(), graph_bottom];

        let xy_trans = [rect.x()+padding_x, rect.y()+padding_y];
        let dim_trans = [rect.w()-padding_x, rect.h()-padding_y];

        let thickness = style.thickness(ui.theme());
        let color = style.color(ui.theme());

        widget::PlotPath::new(min_x, max_x, min_y, max_y, f)
            .wh(dim_trans)
            .xy(xy_trans)
            .color(color)
            .parent(id)
            .crop_kids()
            .set(state.ids.plot_path, ui);
        // X
        widget::Line::abs(bottom_left, bottom_right)
            .color(color)
            .thickness(thickness)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.line_x, ui);
        let axis_area_x = Rect::from_corners([graph_left, rect.bottom()+5.0], [rect.right(), graph_bottom]);
        axis::Axis::new(min_x, max_x)
            .orientation(axis::Orientation::Horizontal)
            .xy(axis_area_x.xy())
            .wh(axis_area_x.dim())
            .set(state.ids.axis_x, ui);
        // Y
        widget::Line::abs(bottom_left, top_left)
            .color(color)
            .thickness(thickness)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.line_y, ui);
        let axis_area_y = Rect::from_corners([rect.left()+5.0, graph_bottom], [graph_left, rect.top()]);
        axis::Axis::new(min_y, max_y)
            .orientation(axis::Orientation::Vertical)
            .xy(axis_area_y.xy())
            .wh(axis_area_y.dim())
            .set(state.ids.axis_y, ui);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
