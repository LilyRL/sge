use ecs::*;
use sge::prelude::*;

#[derive(Component)]
struct Player;

#[main("ECS example")]
async fn main() -> anyhow::Result<()> {
    let mut world = Ecs::new();

    spawn_player(&mut world);
    world.add_systems((player_movement_system, log_player_position_system));

    loop {
        clear_screen(Color::NEUTRAL_100);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}

fn spawn_player(world: &mut Ecs) {
    world.spawn((
        Player,
        DrawWorld,
        ShapeComponent::new(Circle::new(Vec2::ZERO, Vec2::splat(50.0), Color::RED_500)),
        MovementBundle::default(),
    ));
}

fn player_movement_system(query: Query<(&Player, &mut Velocity2D)>) {
    for (_, mut vel) in query {
        dbg!(vel.0);
        let movement =
            pressed_movement_vector(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD);

        vel.0 = movement.normalize_or_zero() * 1000.0;
    }
}

fn log_player_position_system(query: Query<(&Player, &Position2D)>) {
    for (_, pos) in query {
        dbg!(pos.0);
    }
}
