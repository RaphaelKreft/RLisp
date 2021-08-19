/*
choices.rs: This module contains a Data-structure that is essential for the non-deterministic
            pattern of RLisp. The Structure keeps track of all choices that can be made and serves
            methods to work with them. More details can be found in the ReadMe.
 */

// Import needed functionality from Rust stdlib
use std::cell::RefCell;
use std::rc::Rc;
// load functionality from sibling modules
use crate::stdlib::core;
use crate::types::{error, RlErr, RlReturn, RlType, choice_error};
use crate::env::RlEnv;

/// The RlChoices type wraps the Choices in a reference Cell for easy access on heap
pub type RlChoices = Rc<Choices>;
pub type RlChoicesManager = Rc<ChoicesManager>;

#[derive(Clone, Debug)]
pub struct ChoicesManager {
    total_depth: usize,
    current_depth: usize,
    choice_points: Vec<RlChoices>,
    expression: RlType,
    environment: RlEnv,
}

impl ChoicesManager {

    /**
    Static Method to create a new RlChoicesManager Instance
     */
    pub fn new_choices_manager(expression:RlType, environment: RlEnv) -> RlChoicesManager {
        return Rc::new(ChoicesManager {
            total_depth: 0,
            current_depth: 0,
            choice_points: Vec::new(),
            expression,
            environment,
        });
    }
    /**
    This Method updates the choice points. It is to be called before every eval
    returns true when there are choices left and you can try again and returns false if you cannot try again
    */
    pub fn update_choice_points(&mut self) -> bool {
        // total_depth is only 0 if there are no choice points
        if self.total_depth != 0 {
            if self.choice_points[self.current_depth].out_of_bounds() {
                //if the choices in the first choice point are depleted there are no more choices left to try
                if self.current_depth == 0 {
                    self.choice_points.clear();
                    self.update_depth_fields();
                    return false;
                }
                let mut i = 0;
                while i < (self.choice_points.len() - self.current_depth) {
                    self.choice_points.pop();
                    i += 1;
                }
            }
            self.update_depth_fields();
            return true;
        }
        return false;
    }

    /**
    This method creates a new choice point and returns the first choice of the new choice point
     */
    fn append_choice_point(&mut self, choices: Vec<RlType>) -> RlReturn {
        self.choice_points.push(Choices::new_choices(choices));
        self.total_depth += 1;
        self.choice_points[self.choice_points.len() - 1].current_choice()
    }

    /**
    This method resets the current depth counter. To be called every time the expression gets
    evaluated again
     */
    fn update_depth_fields(&mut self) {
        self.total_depth = self.choice_points.len();
        self.current_depth = 0;
    }

    /**
    This method checks whether we reached the last choice point. Used to see if a new choice point
    needs to be created
     */
    fn new_choice_point(&self) -> bool {
        return self.current_depth == self.total_depth
    }

    fn last_choice_point(&self) -> bool {
        return self.current_depth + 1 == self.total_depth
    }


    /**
    This method returns the the next choice of the current choice point
     */
    pub fn get_choice(&mut self, choices: Vec<RlType>) -> RlReturn {
        if self.new_choice_point() {
            self.current_depth += 1;
            self.append_choice_point(choices)
        } else {
            self.current_depth += 1;
            if !self.last_choice_point() {
                self.choice_points[self.current_depth - 1].current_choice()
            } else {
                self.choice_points[self.current_depth].next_choice()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Choices {
    choices: Vec<RlType>,
    index: usize,
}


impl Choices {
    /**
    Static Method to create a new RlChoices Instance
     */
    pub fn new_choices(new_choices: Vec<RlType>) -> RlChoices {
        return Rc::new(Choices{
            choices: new_choices,
            index: 0,
        });
    }

    /**
    This method updates the index for the next choice and returns it.
    */
    pub fn next_choice(&mut self) -> RlReturn {
        self.index += 1;
        return match self.choices.get(self.index) {
            Some(choice) => Ok(choice.clone()),
            None => Err(choice_error("No choices left for Choices object!")),
        }
    }

    pub fn current_choice(&mut self) -> RlReturn {
        return match self.choices.get(self.index) {
            Some(choice) => Ok(choice.clone()),
            None => Err(choice_error("No choices left for Choices object!"))
        }
    }

    /**
    This Method returns whether there are choices left in this choices instance
    */
    pub fn choices_left(&self) -> bool {
        return self.index + 1 < self.choices.len();
    }

    /**
    This method check whether the index of a Choice is already out of bounds
    */
    pub fn out_of_bounds(&self) -> bool {
        return self.index < self.choices.len();
    }

    /**
    resets the index of the choicepoint. This enables nested search.
    */
    pub fn reset(&mut self) {
        self.index = 0;
    }
}


