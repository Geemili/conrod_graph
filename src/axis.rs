
use conrod::{Ui, Dimension, FontSize, widget, text, utils, self};
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
        /// The color of the line
        - tick_mark_size: Scalar { 10.0 }
        /// The font of the numbers
        - font_id: Option<text::font::Id> { theme.font_id }
        /// The size of the font
        - font_size: FontSize { theme.font_size_small }
    }
}

widget_ids! {
    struct Ids {
        line,
        ticks[],
        labels[],
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

	fn default_x_dimension(&self, ui: &Ui) -> Dimension {
		let tick_mark_width = match self.orientation {
			Orientation::Horizontal => self.style.tick_mark_size(&ui.theme),
			Orientation::Vertical => 0.0,
		};
		let font = match self.style.font_id(&ui.theme)
			.or(ui.fonts.ids().next())
			.and_then(|id| ui.fonts.get(id))
			{
				Some(font) => font,
				None => return Dimension::Absolute(tick_mark_width),
			};

		let font_size = self.style.font_size(&ui.theme);
		let mut max_width = 0.0;
        let visible_tick_marks = self.generate_ticks();
		for tick_mark in self.generate_ticks() {
			let width = text::line::width(&format!("{}", tick_mark), font, font_size);
			max_width = utils::partial_max(max_width, width);
		}
		Dimension::Absolute(max_width + tick_mark_width)
	}

	fn default_y_dimension(&self, ui: &Ui) -> Dimension {
		let tick_mark_height = match self.orientation {
			Orientation::Horizontal => 0.0,
			Orientation::Vertical => self.style.tick_mark_size(&ui.theme),
		};
		let font = match self.style.font_id(&ui.theme)
			.or(ui.fonts.ids().next())
			.and_then(|id| ui.fonts.get(id))
			{
				Some(font) => font,
				None => return Dimension::Absolute(tick_mark_height),
			};

		let font_size = self.style.font_size(&ui.theme);
		let height = text::height(1, font_size, 0.0);
		Dimension::Absolute(height + tick_mark_height)
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
		let font_size = style.font_size(&ui.theme);

        // Generate tick mark ids
        let visible_tick_marks = self.generate_ticks();

        // Generate tick mark ids
        let num_tick_marks = visible_tick_marks.len();
        if state.ids.ticks.len() < num_tick_marks {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.ticks.resize(num_tick_marks, id_gen));
        }
        if state.ids.labels.len() < num_tick_marks {
            let id_gen = &mut ui.widget_id_generator();
            state.update(|state| state.ids.labels.resize(num_tick_marks, id_gen));
        }

		// Convience lambda
        let point_to_plot =
            |x| utils::map_range(x, min, max, draw_rect[coord_ord[0]][0], draw_rect[coord_ord[0]][1]);

		// Get the size of the tick marks
		let tick_mark_size = style.tick_mark_size(&ui.theme);

		// Iterate through the tick mark positions and place them on UI
        let mut id_iter = state.ids.ticks.iter();
        let mut label_id_iter = state.ids.labels.iter();
		for mark_position in visible_tick_marks {
			// Get the next id
            let &id_tick = id_iter.next().expect("Axis ran out of widget ids for tick marks");
            let &id_label = label_id_iter.next().expect("Axis ran out of widget ids for labels");

			// Create label
			let text = format!("{}", mark_position);
			let label = widget::Text::new(&text)
				.font_size(font_size);

			// Get the size of the label
			let mut label_size = [
				match label.default_x_dimension(ui) {Dimension::Absolute(scalar)=>scalar,_=>0.0},
				match label.default_y_dimension(ui) {Dimension::Absolute(scalar)=>scalar,_=>0.0},
			];

			// Calculate where the mark is placed in UI coordinates
			let line_plot_coord = point_to_plot(mark_position.into());

			let mut label_rect = [[0.0; 2]; 2];
			label_rect[0][coord_ord[0]] = line_plot_coord - label_size[coord_ord[0]]/2.0; // Lower major axis
			label_rect[0][coord_ord[1]] = draw_rect[coord_ord[1]][0] - label_size[coord_ord[1]]/2.0;
			label_rect[1][coord_ord[0]] = line_plot_coord + label_size[coord_ord[0]]/2.0; // Lower major axis
			label_rect[1][coord_ord[1]] = draw_rect[coord_ord[1]][0] + label_size[coord_ord[1]]/2.0;

			let label_rect = conrod::Rect::from_corners(label_rect[0], label_rect[1]);

			let mut start_coord = [0.0; 2];
			start_coord[coord_ord[0]] = line_plot_coord;
			start_coord[coord_ord[1]] = draw_rect[coord_ord[1]][1];

			let mut end_coord = [0.0; 2];
			end_coord[coord_ord[0]] = line_plot_coord;
			end_coord[coord_ord[1]] = draw_rect[coord_ord[1]][1] + tick_mark_size;

			// Add widgets
            widget::Line::abs(start_coord, end_coord)
                .color(color)
                .thickness(thickness)
                .parent(id)
                .graphics_for(id)
                .set(id_tick, ui);

			// Set text so that it matches up with the divider line
			let label = match *orientation {
				Orientation::Horizontal => label.x(label_rect.x()).y(label_rect.top()),
				Orientation::Vertical => label.x(label_rect.right()).y(label_rect.y()),
			};
			label
				.color(color)
				.parent(id)
				.graphics_for(id)
				.wh(label_rect.dim())
				.set(id_label, ui);
        }

		let mut divider_points = [[0.0, 0.0], [0.0, 0.0]];
		divider_points[0][coord_ord[0]] = draw_rect[coord_ord[0]][0]; // Start point
		divider_points[0][coord_ord[1]] = draw_rect[coord_ord[1]][1]; // Start point
		divider_points[1][coord_ord[0]] = draw_rect[coord_ord[0]][1]; // End point
		divider_points[1][coord_ord[1]] = draw_rect[coord_ord[1]][1]; // End point

		// Place line between numbers and tick marks
		widget::Line::abs(divider_points[0], divider_points[1])
			.color(color)
			.thickness(thickness)
			.parent(id)
			.graphics_for(id)
			.set(state.ids.line, ui);
    }
}
