use crate::npcs::agent::Agent;
use crate::npcs::npc_components::genes::Genes;
use crate::npcs::npc_components::target::Target;
use crate::npcs::npc_components::npc_status::Status;

static mut SA_COUNTER: u32 = 0;

#[allow(dead_code)]
#[derive(Clone)]
pub struct SimpleAgent {
    genes: Genes,
    id: i32,
    position: (u32, u32),
    reward: u32,
    energy: u8,
    max_energy: u8,
    status: Status,
    target: Target,
    monster_target_id: u32,
    agent_target_id: u32,
    treasure_target_id: u32,
    tile_target: Option<(u32, u32)>,
    path: Option<Vec<(i32, i32)>>,
    leader: bool,
    follower: bool,
    leader_id: u32,
    followers: Vec<u32>,
}

impl From<&Agent> for SimpleAgent {
    fn from(agent: &Agent) -> Self {
        SimpleAgent {
            genes: agent.get_genes().clone(),
            id: agent.get_id(),
            position: agent.get_position(),
            reward: agent.get_reward(),
            energy: agent.get_energy(),
            max_energy: agent.get_max_energy(),
            status: agent.get_status().clone(),
            target: agent.get_target().clone(),
            monster_target_id: agent.get_monster_target_id(),
            agent_target_id: agent.get_agent_target_id(),
            treasure_target_id: agent.get_treasure_target_id(),
            tile_target: agent.get_tile_target(),
            path: agent.get_path(),
            leader: agent.is_leader(),
            follower: agent.is_follower(),
            leader_id: agent.get_leader_id(),
            followers: agent.get_followers(),
        }
    }
}

impl SimpleAgent {
    pub fn simple_agent_convert(agent: &Agent) -> Self {
        SimpleAgent::from(agent)
    }

    /// Creates a new [`SimpleAgent`].
    pub fn new(x: u32, y: u32) -> Self {
        // Increment the static counter variable after creating a new instance
        unsafe {
            SA_COUNTER += 1;
        }

        // Create and return a new instance of the Agent struct
        SimpleAgent {
            genes: Genes::generate(),
            id: unsafe { SA_COUNTER },
            position: (x, y),
            energy: 100,
            max_energy: 100,
            reward: 0,
            status: Status::Idle,
            target: Target::None,
            monster_target_id: u32::MAX,
            agent_target_id: u32::MAX,
            treasure_target_id: u32::MAX,
            tile_target: None::<(u32, u32)>,
            path: None::<Vec<(i32, i32)>>,
            leader: false,
            follower: false,
            leader_id: 0,
            followers: Vec::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_position(&self) -> (u32, u32) {
        self.position
    }
}
