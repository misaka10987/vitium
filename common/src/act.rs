use crate::{feature::FeatureAction, player::Token, skill::SkillAction, Target, DEBUG_DESCR, UID};
use serde::{Deserialize, Serialize};

/// Used for in-game chat.
#[derive(Clone, Serialize, Deserialize)]
pub struct Bubble {
    /// Content of the chat message.
    pub text: String,
    /// People(s) that you speak to, empty for towards all current people in the scenario.
    pub towards: Vec<u64>,
}

impl Bubble {
    pub fn example() -> Self {
        Self {
            text: DEBUG_DESCR.to_string(),
            towards: vec![114514, 1919810],
        }
    }
}

/// All in-game actions are defined here.
#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
    /// To send in-game chat.
    Say(Bubble),
    /// To move to a scenario with given uid.
    Move(u64),
    /// To wield an item with given uid.
    Wield(u64),
    /// Activate a feature.
    Feature(FeatureAction),
    /// Apply a skill.
    Skill(SkillAction),
    /// Launch melee attack.
    Attack(Target),
    /// Shoot enemy.
    Shoot(Target),
    /// Cast spell on enemy.
    Cast(Target),
    /// Set fallback targets for occasions attack failed.
    FallbackTarget(Vec<Target>),
    /// Drop item(s) from your inventory.
    DropItem(Vec<u64>),
    /// Pick up item(s) from the scenario.
    PickItem(Vec<u64>),
    /// To wear an armor.
    Wear(u64),
    /// To take off an armor.
    TakeOff(u64),
    /// In seconds.
    Relax(u16),
    /// In seconds.
    Sleep(u16),
    /// Say hello to the world.
    Hello,
}

impl Action {
    pub fn examples() -> Vec<Self> {
        vec![
            Self::Say(Bubble::example()),
            Self::Move(1145141919810),
            Self::Wield(1145141919810),
            Self::Feature(FeatureAction::example()),
            Self::Skill(SkillAction::example()),
            Self::Attack(Target::Chara(1145141919810)),
            Self::Shoot(Target::Mob(1145141919810)),
            Self::Cast(Target::NPC(1145141919810)),
            Self::FallbackTarget(vec![Target::Chara(114514), Target::Chara(1919810)]),
            Self::DropItem(vec![114514, 1919810]),
            Self::PickItem(vec![114514, 1919810]),
            Self::Relax(3600),
            Self::Sleep(21600),
            Self::Hello,
        ]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Act {
    /// Action serial number.
    pub uid: u64,
    /// Character that does the action.
    pub chara: u64,
    /// In-game turn number.
    pub turn: u64,
    /// Detailed action.
    pub action: Action,
    /// Authentication.
    pub token: Token,
}

impl Act {
    pub fn examples() -> Vec<Self> {
        Action::examples()
            .into_iter()
            .map(|a| Act {
                uid: 1145141919810,
                chara: 114514,
                turn: 12345,
                action: a,
                token: Token::new(),
            })
            .collect()
    }
}

impl UID for Act {
    /// UID getter.
    fn uid(&self) -> u64 {
        self.uid
    }
    /// UID setter.
    fn set_uid(&mut self, uid: u64) -> &mut Self {
        self.uid = uid;
        self
    }
}

#[test]
fn see_json() {
    use serde_json::to_string as json;
    for i in Act::examples() {
        println!("{}", json(&i).unwrap())
    }
}
