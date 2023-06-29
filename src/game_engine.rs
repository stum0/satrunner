use crate::{
    components::{Enemies, Particle, Player, Target},
    resources::{DotPos, EnemiesPool, EnemiesPos, LocalPlayerPos, ParticlePool, PlayerPos},
    setup::PLAYER_SPEED,
    WORLD_BOUNDS,
};
use bevy::prelude::*;

pub fn move_dot(
    mut particle_pool: ResMut<ParticlePool>,
    mut particles: Query<(&mut Particle, &mut Visibility, &mut Transform)>,
    dots: ResMut<DotPos>,
) {
    for dot in dots.0.iter() {
        if let Some(pool) = particle_pool.0.pop_front() {
            match particles.get_mut(pool) {
                Ok((_particle, mut visibility, mut transform)) => {
                    transform.translation = *dot;
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
            particle_pool.0.push_back(pool);
        }
    }
}

pub fn move_enemies(
    mut enemies_pool: ResMut<EnemiesPool>,
    mut enemies: Query<(&mut Enemies, &mut Visibility, &mut Transform)>,
    enemies_pos: ResMut<EnemiesPos>,
) {
    let mut pool_iter = enemies_pool.0.iter_mut();

    for enemy in enemies_pos.0.iter() {
        if let Some(pool) = pool_iter.next() {
            match enemies.get_mut(*pool) {
                Ok((_enemies, mut visibility, mut transform)) => {
                    transform.translation = Vec3::new(*enemy, -50., 0.1);
                    *visibility = Visibility::Visible;
                }
                Err(err) => {
                    info!("Error: {:?}", err);
                }
            }
        }
    }

    for pool in pool_iter {
        if let Ok((_particle, mut visibility, _transform)) = enemies.get_mut(*pool) {
            *visibility = Visibility::Hidden;
        }
    }
}

//todo: add Server reconciliation
pub fn move_local(
    mut query: Query<(&mut Transform, &mut Target, &mut Player)>,
    mut pp: ResMut<PlayerPos>,
    //pos: ResMut<LocalPlayerPos>,
) {
    for (mut t, tg, mut p) in query.iter_mut() {
        if p.moving {
            let current_position = Vec2::new(t.translation.x, t.translation.y);
            let direction = Vec2::new(tg.x, tg.y) - current_position;
            let distance_to_target = direction.length();

            if distance_to_target > 0.0 {
                let normalized_direction = direction / distance_to_target;
                let movement = normalized_direction * PLAYER_SPEED;

                let new_position = current_position + movement;

                if new_position.x.abs() <= WORLD_BOUNDS && new_position.y.abs() <= WORLD_BOUNDS {
                    if movement.length() < distance_to_target {
                        t.translation += Vec3::new(movement.x, 0.0, 0.0);
                        pp.0 += Vec3::new(movement.x, 0.0, 0.0);
                        info!("CLIENT SAYS: {:?}", t.translation.x);
                    } else {
                        t.translation = Vec3::new(tg.x, -50.0, 0.1);
                        pp.0 = Vec3::new(tg.x, -50.0, 0.1);
                        p.moving = false;
                    }
                } else {
                    p.moving = false;
                }
            } else {
                p.moving = false;
            }
        }
    }
}

//todo add client side collision
// pub fn collision(mut pp: ResMut<PlayerPos>) {}
