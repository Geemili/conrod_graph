
#[macro_use]
extern crate conrod;
extern crate num;

pub mod ruler;

use conrod::{Color, Colorable, Positionable, Scalar, Sizeable, Widget, widget};

pub enum Orientation {
	Horizontal,
	Vertical,
}

pub struct LineGraph<'a, F> {
    common: widget::CommonBuilder,
    style: Style,
    // Stuff that plot path has
    min_x: f64,
    max_x: f64,
    label_x: Option<&'a str>,
    min_y: f64,
    max_y: f64,
    label_y: Option<&'a str>,
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
        label_x,
        axis_x,
        label_y,
        axis_y,
    }
}

pub struct State {
    ids: Ids,
}

impl<'a, F> LineGraph<'a, F> {

    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64, f: F) -> Self {
        LineGraph {
            common: widget::CommonBuilder::new(),
            style: Style::new(),
            min_x: min_x,
            max_x: max_x,
            label_x: None,
            min_y: min_y,
            max_y: max_y,
            label_y: None,
            f: f,
        }
    }

    pub fn label_x(mut self, text: Option<&'a str>) -> Self {
        self.label_x = text;
        self
    }

    pub fn label_y(mut self, text: Option<&'a str>) -> Self {
        self.label_y = text;
        self
    }

}

impl<'a, F> Widget for LineGraph<'a, F>
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

        let color = style.color(ui.theme());

        // Create x axis
        let x_axis = ruler::Ruler::new(min_x, max_x)
            .orientation(Orientation::Horizontal);

        // Calculate it's height
        let x_axis_height = match x_axis.default_y_dimension(ui) {
            conrod::Dimension::Absolute(number) => number,
            _ => 0.0,
        };

        // Create y axis
        let y_axis = ruler::Ruler::new(min_y, max_y)
            .orientation(Orientation::Vertical);

        // Calculate it's width
        let y_axis_width = match y_axis.default_x_dimension(ui) {
            conrod::Dimension::Absolute(number) => number,
            _ => 0.0,
        };

        let (label_x_widget, label_x_height) = if let Some(text) = self.label_x {
            let label_x_widget = widget::Text::new(text);
            let height = match label_x_widget.default_y_dimension(ui) {
                conrod::Dimension::Absolute(constant) => constant,
                _ => 0.0,
            };
            (Some(label_x_widget), height)
        } else {
            (None, 0.0)
        };

        let (label_y_widget, label_y_width) = if let Some(text) = self.label_y {
            let label_y_widget = widget::Text::new(text);
            let width = match label_y_widget.default_x_dimension(ui) {
                conrod::Dimension::Absolute(constant) => constant,
                _ => 0.0,
            };
            (Some(label_y_widget), width)
        } else {
            (None, 0.0)
        };

        // Create rectangles for the axis'
        let x_axis_rect = Rect::from_corners(
            [rect.left() + y_axis_width + label_y_width, rect.bottom() + label_x_height],
            [rect.right(),                               rect.bottom() + x_axis_height + label_x_height]);
        let y_axis_rect = Rect::from_corners(
            [rect.left() + label_y_width,                rect.bottom() + x_axis_height + label_x_height],
            [rect.left() + y_axis_width + label_y_width, rect.top()]);

        // Place the labels (if they exist
        if let Some(label_x_widget) = label_x_widget {
            let label_x_rect = Rect::from_corners(
                [rect.left() + y_axis_width + label_y_width, rect.bottom()],
                [rect.right(),                               rect.bottom() + label_x_height]);
            label_x_widget
                .color(color)
                .xy(label_x_rect.xy())
                .set(state.ids.label_x, ui);
        }
        if let Some(label_y_widget) = label_y_widget {
            let label_y_rect = Rect::from_corners(
                [rect.left(),                 rect.bottom() + x_axis_height + label_x_height],
                [rect.left() + label_y_width, rect.top()]);
            label_y_widget
                .color(color)
                .xy(label_y_rect.xy())
                .set(state.ids.label_y, ui);
        }

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
            [x_axis_rect.left(), y_axis_rect.bottom()],
            [rect.right(),       rect.top()]);
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
