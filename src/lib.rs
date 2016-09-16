
#[macro_use]
extern crate conrod;
extern crate num;

use conrod::widget;
use conrod::{Color, Colorable, Positionable, Scalar, Sizeable, Widget};

pub struct LineGraph<X, Y, F> {
    common: widget::CommonBuilder,
    style: Style,
    // Stuff that plot path has
    min_x: X,
    max_x: X,
    min_y: Y,
    max_y: Y,
    f: F,
    // Graph details
    tick_increment_x: X,
    tick_increment_y: Y,
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
        line_y,
    }
}

pub struct State {
    ids: Ids,
}

impl<X, Y, F> LineGraph<X, Y, F> {

    pub fn new(min_x: X, max_x: X, tick_increment_x: X, min_y: Y, max_y: Y, tick_increment_y: Y, f: F) -> Self {
        LineGraph {
            common: widget::CommonBuilder::new(),
            style: Style::new(),
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
            f: f,
            tick_increment_x: tick_increment_x,
            tick_increment_y: tick_increment_y,
        }
    }

}

impl<X, Y, F> Widget for LineGraph<X, Y, F>
    where X: num::NumCast + Clone,
          Y: num::NumCast + Clone,
          F: Fn(X) -> Y,
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
        let widget::UpdateArgs { id, state, style, rect, ui, ..} = args;
        let LineGraph { min_x, max_x, min_y, max_y, f, tick_increment_x, tick_increment_y, ..} = self;

        let top_left = [rect.left()+10.0, rect.top()];
        let bottom_left = [rect.left()+10.0, rect.bottom()+10.0];
        let bottom_right = [rect.right(), rect.bottom()+10.0];

        let xy_trans = [rect.x()+20.0, rect.y()+20.0];
        let dim_trans = [rect.w()-20.0, rect.h()-20.0];

        let thickness = style.thickness(ui.theme());
        let color = style.color(ui.theme());

        widget::PlotPath::new(min_x, max_x, min_y, max_y, f)
            .wh(dim_trans)
            .xy(xy_trans)
            .color(color)
            .parent(id)
            .set(state.ids.plot_path, ui);
        widget::Line::abs(bottom_left, bottom_right)
            .color(color)
            .thickness(thickness)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.line_x, ui);
        widget::Line::abs(bottom_left, top_left)
            .color(color)
            .thickness(thickness)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.line_y, ui);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
