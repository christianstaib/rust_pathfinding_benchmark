use std::{
    fs,
    thread::sleep,
    time::{Duration, Instant},
};

use fast_paths::InputGraph;
use faster_paths::{
    ch::{
        contraction_adaptive_simulated::contract_adaptive_simulated_with_witness,
        pathfinding::dijkstra_with_state::ChDijkstraState,
    },
    graphs::{
        graph_factory::GraphFactory,
        graph_functions::{all_edges, generate_random_pair_test_cases},
        path::PathFindingWithInternalState,
    },
};
use indicatif::ProgressIterator;
use prettytable::{row, Table};

fn main() {
    let paths = fs::read_dir("./data").unwrap();

    let mut table = Table::new();
    table.add_row(row![
        "graph",
        "faster_paths generation",
        "faster_paths query",
        "fast_paths generation",
        "fast_paths query"
    ]);

    for path in paths.flatten() {
        let graph = GraphFactory::from_file(&path.path());
        let test_cases = generate_random_pair_test_cases(&graph, 1_000);

        //
        // faster_paths
        //
        let start = Instant::now();
        let contracted_graph = contract_adaptive_simulated_with_witness(&graph);
        let faster_paths_generation = start.elapsed();

        let mut path_finder = ChDijkstraState::new(&contracted_graph);
        let mut times = Vec::new();

        sleep(Duration::from_secs(3)); // cooldown and stuff
        for test_case in test_cases.iter().progress() {
            let start = Instant::now();
            let weight = path_finder.shortest_path_weight(&test_case.request);
            times.push(start.elapsed());

            assert_eq!(test_case.weight, weight);
        }

        let faster_paths_query = times.iter().sum::<Duration>() / times.len() as u32;

        //
        // fast_paths
        //
        let mut input_graph = InputGraph::new();

        for edge in all_edges(&graph) {
            input_graph.add_edge(
                edge.tail() as usize,
                edge.head() as usize,
                edge.weight() as usize,
            );
        }
        input_graph.freeze();

        let start = Instant::now();
        let fast_graph = fast_paths::prepare(&input_graph);
        let fast_paths_generation = start.elapsed();

        let mut path_calculator = fast_paths::create_calculator(&fast_graph);
        let mut times = Vec::new();

        sleep(Duration::from_secs(3)); // cooldown and stuff
        for test_case in test_cases.iter().progress() {
            let start = Instant::now();
            let shortest_path = path_calculator.calc_path(
                &fast_graph,
                test_case.request.source() as usize,
                test_case.request.target() as usize,
            );
            let weight = shortest_path.map(|path| path.get_weight() as u32);
            times.push(start.elapsed());

            assert_eq!(test_case.weight, weight);
        }
        let fast_paths_query = times.iter().sum::<Duration>() / times.len() as u32;

        //
        // table generation
        //

        table.add_row(row![
            path.file_name().into_string().unwrap(),
            faster_paths_generation.as_secs_f64(),
            faster_paths_query.as_secs_f64(),
            fast_paths_generation.as_secs_f64(),
            fast_paths_query.as_secs_f64(),
        ]);
    }

    table.printstd();
}
