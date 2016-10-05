
use conrod::{widget, utils, self};
use conrod::{Color, Colorable, Positionable, Scalar, Sizeable, Widget};

pub enum Orientation {
	Horizontal,
	Vertical,
}


// TODO: Make the Axis type generic over scale type (log, linear, etc.)

pub struct Axis<X> {
    common: widget::CommonBuilder,
    style: Style,
	orientation: Orientation,
	tick_count: usize,

	min: X,
	max: X,
	origin: X,
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

use num::Zero;

impl<X> Axis<X>
	where X: Clone + Zero + Into<f64>
	{

    pub fn new(min: X, max: X) -> Self {
        Axis {
            common: widget::CommonBuilder::new(),
            style: Style::new(),
			orientation: Orientation::Horizontal,
			tick_count: 8,
            min: min,
            max: max,
			origin: X::zero(),
        }
    }

	pub fn orientation(mut self, orientation: Orientation) -> Self {
		self.orientation = orientation;
		self
	}

	pub fn origin(mut self, origin: X) -> Self {
		self.origin = origin;
		self
	}

	pub fn tick_count(mut self, tick_count: usize) -> Self {
		self.tick_count = tick_count;
		self
	}

	fn generate_ticks(&self) -> Vec<f64> {
		let origin: f64 = self.origin.clone().into();
		let min: f64 = self.min.clone().into();
		let max: f64 = self.max.clone().into();

		// Get tick step
		let exact_step = (max - min)/(self.tick_count as f64 + 1e-10); //
		let magnitude = 10.0f64.powf(exact_step.log(10.0).floor());
		let mantissa = exact_step / magnitude;
		let tick_step = pick_closest(&[1.0, 2.0, 2.5, 5.0, 10.0], mantissa)*magnitude;

		// Generate positions for tick marks
		let mut marks = vec![];
		let first_step = ((min - origin) / tick_step).floor();
		let last_step = ((max - origin) / tick_step).floor();
		let mut tick_count = (last_step - first_step) + 1.0;
		if tick_count < 0.0 {
			tick_count = 0.0;
		}
		let mut i = 0.0;
		while i < tick_count {
			let mark = origin + (first_step+i) * tick_step;
			if mark >= min && mark <= max  {
				marks.push(mark);
			}
			i += 1.0;
		}

		marks
	}

}

fn pick_closest(elements: &[f64], target: f64) -> f64 {
	match get_lower_bound_index(elements, target) {
		None => elements[elements.len()-1],
		Some(0) => elements[0],
		Some(index) if target-elements[index-1] < target-elements[index] => elements[index-1],
		Some(index) => elements[index],
	}
}

/// Gets the first element of the slice that is not lower than or equal to the target
fn get_lower_bound_index(elements: &[f64], target: f64) -> Option<usize> {
	let mut index = 0;
	while index < elements.len() {
		if elements[index] > target {
			return Some(index);
		}
		index += 1;
	}
	None
}

impl<X> Widget for Axis<X>
    where X: Clone + Zero + Into<f64>,
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
        let Axis { ref min, ref max, ref orientation, ..} = self;

		// The code that actually figures out where to put lines. E.G., the logic specific to this
		// widget

		// Allows us to use a generic axis for both horizontal and vertical orientations
		// TODO: Look into allowing arbitrary angles for graph axis
		let draw_rect = [
			[rect.left(), rect.right()],
			[rect.bottom(), rect.top()],
		];
		// coord_ord[0] == one we are drawing on
		let coord_ord = match *orientation {
			Orientation::Horizontal => [0, 1],
			Orientation::Vertical => [1, 0],
		};

		let min: f64 = min.clone().into();
		let max: f64 = max.clone().into();

        let thickness = style.thickness(ui.theme());
        let color = style.color(ui.theme());

        // Generate tick mark ids
        let visible_tick_marks = self.generate_ticks();

        // Generate tick mark ids
        let num_tick_marks = visible_tick_marks.len();
        if state.ids.ticks.len() < num_tick_marks {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.ticks.resize(num_tick_marks, id_gen));
        }

		// Convience lambda
        let point_to_plot =
            |x| utils::map_range(x, min, max, draw_rect[coord_ord[0]][0], draw_rect[coord_ord[0]][1]);

		// Iterate through the tick mark positions and place them on UI
        let mut id_iter = state.ids.ticks.iter();
		for mark_position in visible_tick_marks {
            let &id_tick = id_iter.next().expect("Axis ran out of widget ids");
            let line_plot_coord = point_to_plot(mark_position.into());

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
        }
    }
}
