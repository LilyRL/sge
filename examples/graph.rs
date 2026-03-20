use graph_networks::*;
use sge::prelude::*;

fn init_network(network: &mut Network) {
    network.insert_nodes_with_links(&[
        &[1, 3],
        &[],
        &[2],
        &[1],
        &[2, 3, 0],
        &[0],
        &[5],
        &[2],
        &[2],
        &[2],
        &[2],
        &[2, 4, 5],
        &[8, 11],
        &[12],
    ]);
}

fn main() -> AResult<()> {
    init("Graphs")?;

    let mut pan = PanningCameraController::new();

    let mut network = Network::new();
    init_network(&mut network);

    loop {
        pan.update();
        network.update(true);

        network.calc_positions_by_force(100.0, 20);

        if key_pressed(KeyCode::KeyR) {
            network.clear();
            init_network(&mut network);
        }

        if key_pressed(KeyCode::KeyA) {
            let len = network.len();
            let n = rand_range(0..len);
            let node = network.nth_node(n).unwrap();
            let c = rand_range(0..len);
            let c = network.nth_node(c).unwrap();
            node.add_connections(&mut network, &[c]);
        }

        for line in network.iter_connection_lines() {
            let color = if line.is_hovered {
                Color::WHITE
            } else {
                Color::NEUTRAL_700
            };
            let dir = (line.end - line.start).normalize();
            draw_arrow_world(line.start, line.end - dir * 20.0, 2.0, color);
        }

        for node in network.iter_node_positions() {
            let color = if node.is_hovered {
                Color::WHITE
            } else {
                Color::NEUTRAL_400
            };
            draw_circle_world(node.pos, 20.0, color);
            let dim = measure_text(node.id.to_string());
            draw_colored_text_world(node.id.to_string(), node.pos - dim.size / 2.0, Color::BLACK);
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
