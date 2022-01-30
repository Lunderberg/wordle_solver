use wordle::Word;

use itertools::Itertools;

use plotly::{common::Marker, layout, Bar, Layout, Plot};

pub struct WordlePlotter {
    plot: Plot,
}

impl WordlePlotter {
    pub fn new() -> Self {
        Self { plot: Plot::new() }
    }

    pub fn add_results<const N: usize>(
        &mut self,
        name: &str,
        paths: &Vec<Vec<Word<N>>>,
    ) {
        let (bar_x, bar_y): (Vec<usize>, Vec<usize>) =
            paths.iter().map(|p| p.len()).counts().into_iter().unzip();
        let trace = Bar::new(bar_x, bar_y)
            .name(name)
            .opacity(0.5)
            .marker(Marker::new().size(1500));
        self.plot.add_trace(trace);
    }

    pub fn plot(mut self) {
        let xaxis = layout::Axis::new().dtick(1.0);

        let layout = Layout::new()
            .width(800)
            .height(600)
            .x_axis(xaxis)
            .bar_mode(layout::BarMode::Overlay)
            .bar_gap(0.0);

        self.plot.set_layout(layout);
        self.plot.show();
    }
}
