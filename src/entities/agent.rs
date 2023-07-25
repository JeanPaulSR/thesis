use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use crate::mcst::NpcAction;

static mut A_COUNTER: u32 = 0;

#[derive(Clone)]
pub struct Agent {
    pub entity: Entity,
    pub greed: f32,
    pub aggression: f32,
    pub social: f32,
    pub self_preservation: f32,
    pub vision: f32,
    pub transform: Transform,
    pub sprite_bundle: SpriteBundle,
    pub action: Option<NpcAction>,
    pub id: u32,
}

//Test
impl Agent {
    pub fn new_agent(
        mut x: f32,
        mut y: f32,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        x = x * 32.0;
        y = y * 32.0;
        let mut rng = rand::thread_rng();
        let greed_distribution = Uniform::new(0.5, 1.0);
        let aggression_distribution = Uniform::new(0.3, 0.8);
        let common_distribution = Uniform::new(0.0, 1.0);
        let vision_distribution = Uniform::new(3.0, 8.0);

        let sprite_size = Vec2::new(32.0, 32.0); // Adjust to your sprite size

        let texture_handle = asset_server.load("textures/agent.png");

        let entity = commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(sprite_size),
            transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
            ..Default::default()
        }).id();

        // Increment the static counter variable after creating a new instance
        unsafe {
            A_COUNTER += 1;
        }

        Agent {
            greed: greed_distribution.sample(&mut rng),
            aggression: aggression_distribution.sample(&mut rng),
            social: common_distribution.sample(&mut rng),
            self_preservation: common_distribution.sample(&mut rng),
            vision: vision_distribution.sample(&mut rng),
            id: unsafe { A_COUNTER },
            entity,
            transform: Transform::from_translation(Vec3::new(x , y , 0.0)),
            sprite_bundle: SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(sprite_size),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Adjust position in relation to the agent transform
                ..Default::default()
            },
            action: None::<NpcAction>,
        }
    }

    pub fn travel(
        &mut self,
        x: f32,
        y: f32,
        commands: &mut Commands,
    ) {
        let new_transform = Transform::from_translation(Vec3::new(x * 32.0, y * 32.0, 1.0));
        self.transform = new_transform;
        commands.entity(self.entity).insert(self.transform.clone());
    }

    pub fn print(&self) {
        println!("Agent ID: {}", self.id);
        println!("Greed: {}", self.greed);
        println!("Aggression: {}", self.aggression);
        println!("Social: {}", self.social);
        println!("Self Preservation: {}", self.self_preservation);
        println!("Vision: {}", self.vision);
        println!("Position: x={}, y={}", self.transform.translation.x/32.0, self.transform.translation.y/32.0);
        // Add more fields as needed
    }

    fn set_action(&mut self, action: NpcAction) {
        self.action = Some(action);
    }

    fn perform_action(&mut self) {
        if let Some(ref action) = self.action {
            match action {
                NpcAction::MoveToVillage => {
                    // Logic for moving to a village
                }
                NpcAction::MoveToTreasure => {
                    // Logic for moving to a treasure
                }
                NpcAction::MoveToMonster => {
                    // Logic for moving to a monster
                }
                NpcAction::MoveToSteal => {
                    // Logic for moving to steal
                }
            }
            // Clear the action after performing it
            self.action = None;
        }
    }

}