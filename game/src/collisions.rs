use crate::player::respawn;
use crate::*;
use bevy::prelude::*;

pub const COLLISION_MARGIN: f32 = 10.; //margin for collisions

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_with_platform)
            .add_system(collision_attack);
    }
}

fn collision_with_platform(
    query_platforms: Query<&Platform>,
    mut query: Query<
        (&mut Grounded, &mut Transform, &SpriteSize, &mut Velocity),
        Or<(With<Player>, With<Skeleton>, With<TemporalGhost>)>,
    >,
) {
    for (mut grounded, mut transform, sprite_size, mut velocity) in query.iter_mut() {
        for platform in query_platforms.iter() {
            let entity_position = &mut transform.translation;
            let entity_size = sprite_size.0;
            let platform_position = platform.position;
            let platform_size = platform.size;
            if let Some(collision) = collide(
                *entity_position,
                entity_size + COLLISION_MARGIN, //add margin to make sure that when player is on ground, there is collision
                platform_position,
                platform_size + COLLISION_MARGIN,
            ) {
                match collision {
                    Collision::Top => {
                        if velocity.vy < 1. {
                            grounded.0 = true;
                            velocity.vy = 0.;
                            entity_position.y =
                                platform_position.y + platform_size.y / 2. + entity_size.y / 2.;
                            //set player on the platform to avoid float precision issues
                            break;
                        }
                    }
                    Collision::Left => {
                        if velocity.vx > 0. {
                            velocity.vx = 0.;
                            entity_position.x =
                                platform_position.x - platform_size.x / 2. - entity_size.x / 2.;
                        }
                    }
                    Collision::Right => {
                        if velocity.vx < 0. {
                            velocity.vx = 0.;
                            entity_position.x =
                                platform_position.x + platform_size.x / 2. + entity_size.x / 2.;
                        }
                    }
                    _ => (),
                };
            } else {
                grounded.0 = false;
            }
        }
    }
}

fn collision_attack(
    mut commands: Commands,
    mut query_player: Query<
        (
            &mut Transform,
            &SpriteSize,
            &SpriteSizeAttack,
            &Attack,
            &mut Velocity,
            &mut Acceleration,
        ),
        With<Player>,
    >,
    query_monster: Query<
        (Entity, &Transform, &SpriteSize, &SpriteSizeAttack, &Attack),
        (With<Enemy>, Without<Player>),
    >,
) {
    for (
        mut tf_player,
        sprite_size_player,
        sprite_size_attack_player,
        attack_player,
        mut velocity_player,
        mut acceleration_player,
    ) in query_player.iter_mut()
    {
        for (entity, tf_enemy, sprite_size_enemy, sprite_size_attack_enemy, attack_enemy) in
            query_monster.iter()
        {
            // Si le player attacking entre en collision avec le squelette normal
            let player_position = sprite_size_attack_player.position;
            let player_size = sprite_size_attack_player.size;

            let enemy_position = tf_enemy.translation;
            let enemy_size = sprite_size_enemy.0;

            if let Some(collision) =
                collide(player_position, player_size, enemy_position, enemy_size)
            {
                match collision {
                    Collision::Top => {
                        break;
                    }
                    Collision::Left => {
                        if attack_player.is_attacking {
                            commands.entity(entity).despawn();
                        }
                        break;
                    }
                    Collision::Right => {
                        if attack_player.is_attacking {
                            commands.entity(entity).despawn();
                        }
                        break;
                    }
                    Collision::Bottom => {
                        break;
                    }
                    Collision::Inside => {
                        if attack_player.is_attacking {
                            commands.entity(entity).despawn();
                        }
                        break;
                    }
                }
            }

            let player_position = tf_player.translation;
            let player_size = sprite_size_player.0;

            let enemy_position = sprite_size_attack_enemy.position;
            let enemy_size = sprite_size_attack_enemy.size;

            if let Some(collision) =
                collide(enemy_position, enemy_size, player_position, player_size)
            {
                match collision {
                    Collision::Top => {
                        break;
                    }
                    Collision::Left => {
                        if attack_enemy.is_attacking {
                            respawn(velocity_player, tf_player, acceleration_player);
                        }
                        break;
                    }
                    Collision::Right => {
                        if attack_enemy.is_attacking {
                            respawn(velocity_player, tf_player, acceleration_player);
                        }
                        break;
                    }
                    Collision::Bottom => {
                        break;
                    }
                    Collision::Inside => {
                        if attack_enemy.is_attacking {
                            respawn(velocity_player, tf_player, acceleration_player);
                        }
                        break;
                    }
                }
            }
        }
    }
}
