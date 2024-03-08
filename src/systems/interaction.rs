use crate::prelude::*;
//menu navigation moi processing system
pub fn process_interactions(state: &mut State, commands: &mut CommandBuffer) {

    //grab MOI that has whatever option in the interaction menu
    //grab the interaction menu active rn (will be tagged in future rn there's only one)
    //then get the specific one chosen and check if c_and_c is some, if it is run that function
    //this will handle all the "action" of the choice and make whatever changes it needs to to the world and return a result type to be turned into an moi
    //take the ChoiceResult it gives you and push it into the ECS as its own MOI
    //then another tick process the choiceResult and display the result
}
