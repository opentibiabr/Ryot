//! Sprite animations module.
use crate::appearances::Animation;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::Rng;
use std::time::Duration;

use self::sprites::{
    sprite_material_from_params, ChangingAppearanceFilter, FrameGroupComponent, LoadedAppearances,
    LoadedSprite, SpriteMaterial,
};

/// A resource to enable/disable sprite animation globally.
#[derive(Resource, PartialEq, Debug, Clone)]
pub struct SpriteAnimationEnabled(pub bool);

impl Default for SpriteAnimationEnabled {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub(crate) struct SynchronizedAnimationTimers(HashMap<AnimationKey, AnimationState>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationStartPhase {
    Random,
    Fixed(usize),
}

impl AnimationStartPhase {
    fn get(&self, total_phases: usize) -> usize {
        match self {
            AnimationStartPhase::Random => rand::thread_rng().gen_range(0..total_phases),
            AnimationStartPhase::Fixed(phase) => *phase,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationKey {
    pub phase_durations: Vec<Duration>,
    pub start_phase: AnimationStartPhase,
    pub total_phases: usize,
}

impl AnimationKey {
    fn create_timer(&self, phase: usize) -> Timer {
        Timer::new(self.duration(phase), TimerMode::Once)
    }

    fn duration(&self, phase: usize) -> Duration {
        self.phase_durations
            .get(phase)
            .cloned()
            .unwrap_or(Duration::from_millis(300))
    }

    pub(crate) fn default_state(&self) -> AnimationState {
        let current_phase = self.start_phase.get(self.total_phases);
        AnimationState::new(current_phase, self.create_timer(current_phase))
    }
}

pub(crate) trait SpriteAnimationExt {
    fn get_animation_key(&self) -> AnimationKey;
}

impl SpriteAnimationExt for Animation {
    fn get_animation_key(&self) -> AnimationKey {
        let phase_durations = self
            .phases
            .iter()
            .map(|phase| -> Duration {
                let range = phase.min()..phase.max();
                if range.start == range.end {
                    return Duration::from_millis(range.start.into());
                }
                Duration::from_millis(rand::thread_rng().gen_range(range).into())
            })
            .collect();

        AnimationKey {
            phase_durations,
            start_phase: match self.is_start_random() {
                true => AnimationStartPhase::Random,
                false => AnimationStartPhase::Fixed(self.start_phase() as usize),
            },
            total_phases: self.phases.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AnimationState {
    pub timer: Timer,
    pub current_phase: usize,
    just_finished: bool,
}

impl AnimationState {
    fn new(current_phase: usize, timer: Timer) -> Self {
        Self {
            timer,
            current_phase,
            just_finished: false,
        }
    }

    fn tick(&mut self, key: &AnimationKey, delta: Duration) {
        self.timer.tick(delta);
        self.just_finished = false;
        if self.timer.just_finished() {
            self.current_phase += 1;
            if self.current_phase >= key.total_phases {
                self.current_phase = 0;
            }
            self.timer.set_duration(key.duration(self.current_phase));
            self.timer.reset();
            self.just_finished = true;
        }
    }

    fn just_finished(&self) -> bool {
        self.just_finished
    }
}

#[derive(Debug, Clone)]
pub struct AnimationDescriptor {
    pub sprites: Vec<LoadedSprite>,
    pub layers: usize,
    pub skip: usize,
    pub synchronized: bool,
}

#[derive(Component, Clone, Debug)]
pub(crate) enum AnimationSprite {
    Independent {
        key: AnimationKey,
        descriptor: AnimationDescriptor,
        state: AnimationState,
    },
    Synchronized {
        key: AnimationKey,
        descriptor: AnimationDescriptor,
    },
}

impl AnimationSprite {
    pub(crate) fn from_key_and_descriptor(
        key: &AnimationKey,
        descriptor: &AnimationDescriptor,
    ) -> Self {
        if descriptor.synchronized {
            AnimationSprite::Synchronized {
                key: key.clone(),
                descriptor: descriptor.clone(),
            }
        } else {
            AnimationSprite::Independent {
                key: key.clone(),
                descriptor: descriptor.clone(),
                state: key.default_state(),
            }
        }
    }
}

/// An optional component to override animation timers.
#[derive(Component, Default)]
pub struct AnimationDuration(pub Duration);

pub fn toggle_sprite_animation(mut enabled: ResMut<SpriteAnimationEnabled>) {
    enabled.0 = !enabled.0;
}

pub(crate) fn initialize_animation_sprite_system(
    mut commands: Commands,
    q_maybe_animated: Query<
        (Entity, &GameObjectId, Option<&FrameGroupComponent>),
        ChangingAppearanceFilter,
    >,
    loaded_appereances: Res<LoadedAppearances>,
    mut synced_timers: ResMut<SynchronizedAnimationTimers>,
) {
    q_maybe_animated
        .iter()
        .for_each(|(entity, object_id, frame_group)| {
            let Some(loaded_appearance) =
                loaded_appereances.get(&(*object_id, frame_group.cloned().unwrap_or_default()))
            else {
                warn!("BUG: Loaded appearance for {:?} not found.", object_id);
                return;
            };
            let Some((ref key, ref descriptor)) = loaded_appearance.animation else {
                commands.entity(entity).remove::<AnimationSprite>();
                return;
            };
            commands
                .entity(entity)
                .insert(AnimationSprite::from_key_and_descriptor(key, descriptor));

            if descriptor.synchronized {
                synced_timers
                    .try_insert(key.clone(), key.default_state())
                    .ok();
            }
        });
}

/// A system that animates the sprites based on the `AnimationSprite` component.
/// It's meant to run every frame to update the animation of the entities.
/// It will only run if the entity has a `TextureAtlas` and an `AnimationSprite` component.
pub(crate) fn tick_animation_system(
    time: Res<Time>,
    mut synced_timers: ResMut<SynchronizedAnimationTimers>,
    mut q_sprites: Query<(
        &mut Handle<SpriteMaterial>,
        &mut AnimationSprite,
        Option<&Directional>,
        Option<&SpriteParams>,
        Option<&AnimationDuration>,
    )>,
    mut materials: ResMut<Assets<SpriteMaterial>>,
) {
    let delta = time.delta();
    synced_timers
        .iter_mut()
        .for_each(|(key, state)| state.tick(key, delta));

    q_sprites.iter_mut().for_each(
        |(mut material, mut anim, direction, sprite_params, duration)| {
            if let AnimationSprite::Independent { key, state, .. } = &mut *anim {
                if let Some(duration) = duration {
                    let frame_duration = duration.0 / key.total_phases as u32;
                    if state.timer.duration() != frame_duration {
                        state.timer.set_duration(frame_duration)
                    }
                }
                state.tick(key, delta);
            }

            let (state, descriptor) = match anim.as_ref() {
                AnimationSprite::Independent {
                    state, descriptor, ..
                } => (state, descriptor),
                AnimationSprite::Synchronized { key, descriptor } => {
                    let Some(state) = synced_timers.get(key) else {
                        return;
                    };
                    (state, descriptor)
                }
            };

            if state.just_finished() {
                let direction_index = match direction {
                    Some(dir) => dir.index(),
                    None => 0,
                } * descriptor.layers;
                let Some(sprite) = descriptor
                    .sprites
                    .get(direction_index + state.current_phase * descriptor.skip)
                else {
                    return;
                };
                *material = sprite_material_from_params(sprite_params, &mut materials, sprite);
            }
        },
    );
}
