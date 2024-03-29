use crate::prelude::*;

///Component that tags an entity as the player
#[derive(Clone, Copy, Debug)]
pub struct Player;

#[derive(Clone, Debug, PartialEq)]
///Simple wrapper component that holds the key to accessing the entity's related interaction menu.
pub struct InteractionMenuKey(pub String);

#[derive(Copy, Clone, Debug)]
///Tagging component used to let the game know which interaction menu is active!
pub struct ActiveInteractionMenu;

///MOI to communicate what choice in an interaction menu was selected by the player
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InteractionMenuChoiceMOI {
    pub index: usize,
}
#[derive(Clone, Copy, Debug, PartialEq)]
///MOI that carries the result of an interaction menu
pub struct InteractionMenuResult {
    pub choice_result: Option<ChoiceResult>,
    pub current_option_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DegreeOfSuccess {
    Failure,
    PartialSuccess,
    FullSuccess,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChoiceResult {
    BinaryResult(bool),
    DegOfSuccess(DegreeOfSuccess),
}

//SKILLS IMPLEMENTATION

///Skills component
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SkillType {
    RangedWeapons,
    MeleeWeapons,
    Athletics,
    Reflexes,
    Perception,
    Stealth,
    Lockpick,
    Technology,
    Medicine,
    PlanetSense,
    Threaten,
    Manipulate,
    Charm,
    SocialCues,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Skills([i32; 14]);

impl Skills {
    pub fn new(
        ranged_weapons: i32,
        melee_weapons: i32,
        athletics: i32,
        reflexes: i32,
        perception: i32,
        stealth: i32,
        lockpick: i32,
        technology: i32,
        medicine: i32,
        planetsense: i32,
        threaten: i32,
        manipulate: i32,
        charm: i32,
        social_cues: i32,
    ) -> Self {
        Self([
            ranged_weapons,
            melee_weapons,
            athletics,
            reflexes,
            perception,
            stealth,
            lockpick,
            technology,
            medicine,
            planetsense,
            threaten,
            manipulate,
            charm,
            social_cues,
        ])
    }
    pub fn new_blank() -> Self {
        Self([0; 14])
    }
    pub fn get_skill(&self, skill: SkillType) -> i32 {
        self.0[skill as usize]
    }
    pub fn change_skill(&mut self, skill: SkillType, modifier: i32) {
        if self.0[skill as usize] + modifier < 6 && self.0[skill as usize] > -6 {
            self.0[skill as usize] += modifier;
        }
    }
    ///does a standard skillcheck when given the skilltype and any outside modifiers
    pub fn skillcheck(self, skill: SkillType, modifier: i32) -> RollResult {
        //rng struct needed to do dicerolls
        let mut rng = RandomNumberGenerator::new();
        //base skill roll is a simple 2d6 and we need an unmodified roll to check for special results
        let roll = rng.roll_dice(2, 6);
        if roll == 2 {
            //if it's "snake eyes" (two 1's) it's always a failure
            return RollResult::Failure;
        } else if roll == 12 {
            //if it's "boxcars" (two 6's) it's always a success
            return RollResult::FullSuccess;
        }
        //now that we've checked for special conditions we can move on to the normal diceroll
        let roll_with_mods = roll + modifier + self.get_skill(skill); //apply the skill modifier and any contextual mods
        if roll_with_mods < 7 {
            //anything less than a 7 is a failure
            return RollResult::Failure;
        } else if roll_with_mods > 6 && roll_with_mods < 10 {
            //7, 8, and 9 is a partial success
            return RollResult::PartialSuccess;
        } else if roll_with_mods > 9 {
            //10 or above is a full success
            return RollResult::FullSuccess;
        } else {
            //this should never happen but the compiler was getting mad at me if I didn't include it.
            return RollResult::FullSuccess;
        }
    }
}

pub enum RollResult {
    Failure,
    PartialSuccess,
    FullSuccess,
}
