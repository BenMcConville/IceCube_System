use super::app::graph_data::Graph;
use super::App;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Axis, Block, Chart, Dataset, GraphType};

pub fn create_chart(data: &[(f64, f64)]) -> Chart {
    // let datasets = vec![
    //     // Scatter chart
    //     // Dataset::default()
    //     //     .name("data1")
    //     //     .graph_type(GraphType::Scatter)
    //     //     .style(Style::default())
    //     //     .data(&[(0.0, 2.0), (1.0, 1.0), (2.0, 2.0), (3.0, 1.0), (4.0, 2.0)]),
    //     // Line chart
    //     Dataset::default()
    //         .name("data2")
    //         .graph_type(GraphType::Line)
    //         .style(Style::default())
    //         .data(graph.create_data_set()),
    // ];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title("X Axis")
        .style(Style::default())
        .bounds([0.0, 10.0])
        .labels(vec!["0.0".into(), "5.0".into(), "10.0".into()]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("Y Axis")
        .style(Style::default())
        .bounds([0.0, 10.0])
        .labels(vec!["0.0".into(), "5.0".into(), "10.0".into()]);

    // Create the chart and link all the parts together

    Chart::new(vec![Dataset::default()
        .name("data2")
        .graph_type(GraphType::Scatter)
        .style(Style::default())
        .data(data)])
    .x_axis(x_axis)
    .y_axis(y_axis)
    .bg(ratatui::style::Color::Rgb(41, 44, 50))
}
