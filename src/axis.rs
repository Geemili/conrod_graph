
use conrod::{widget, utils, self};
use conrod::{Color, Colorable, Positionable, Scalar, Sizeable, Widget};

pub enum Orientation {
	Horizontal,
	Vertical,
}

pub struct Axis<X> {
    common: widget::CommonBuilder,
    style: Style,
    // Stuff that plot path has
    min: X,
    max: X,
    tick_increment: X,
    orientation: Orientation,
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
        ticks[],
    }
}

pub struct State {
    ids: Ids,
}

impl<X> Axis<X> {

    pub fn new(orientation: Orientation, min: X, max: X, tick_increment: X) -> Self {
        Axis {
            common: widget::CommonBuilder::new(),
            style: Style::new(),
            min: min,
            max: max,
            tick_increment: tick_increment,
			orientation: orientation,
        }
    }

}

impl<X> Widget for Axis<X>
    where X: Into<f64>,
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
        let Axis { min, max, tick_increment, orientation, ..} = self;

		// The code that actually figures out where to put lines. E.G., the logic specific to this
		// widget

		// Allows us to use a generic axis for both horizontal and vertical orientations
		// TODO: Look into allowing arbitrary angles for graph axis
		let draw_rect = [
			[rect.left(), rect.right()],
			[rect.bottom(), rect.top()],
		];
		// coord_ord[0] == one we are drawing on
		let coord_ord = match orientation {
			Orientation::Horizontal => [0, 1],
			Orientation::Vertical => [1, 0],
		};

		let min: f64 = min.into();
		let max: f64 = max.into();
		let tick_increment: f64 = tick_increment.into();

        let thickness = style.thickness(ui.theme());
        let color = style.color(ui.theme());

        // Generate tick mark ids
        let visible_tick_marks = ((max - min)/tick_increment).ceil() as usize;
        if state.ids.ticks.len() < visible_tick_marks {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.ticks.resize(visible_tick_marks, id_gen));
        }

		// Convience lambda
        let point_to_plot =
            |x| utils::map_range(x, min, max, draw_rect[coord_ord[0]][0], draw_rect[coord_ord[0]][1]);

		// Get the first visible tick mark
        let mut current_tick = (min/tick_increment).ceil() * tick_increment;
        let mut id_iter = state.ids.ticks.iter();

		while current_tick < max {
            let &id_tick = id_iter.next().unwrap();
            let line_plot_coord = point_to_plot(current_tick);

			let mut start_coord = [0.0; 2];
			start_coord[coord_ord[0]] = line_plot_coord;
			start_coord[coord_ord[1]] = draw_rect[coord_ord[1]][0];

			let mut end_coord = [0.0; 2];
			end_coord[coord_ord[0]] = line_plot_coord;
			end_coord[coord_ord[1]] = draw_rect[coord_ord[1]][1];

            widget::Line::abs(start_coord, end_coord)
                .color(color)
                .thickness(thickness)
                .parent(id)
                .graphics_for(id)
                .set(id_tick, ui);

			// Go to the next visible tick mark
            current_tick += tick_increment;
        }
    }
}
