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
use crate::types::{error, RlErr, RlReturn, RlType};

/// The RlChoices type wraps the Choices in a reference Cell for easy access on heap
pub type RlChoices = Rc<Choices>;

#[derive(Clone, Debug)]
pub struct Choices {
    choices: Vec<RlType>,
    index: usize,
    outer: Option<RlChoices>,
}


impl Choices {
    /**
    Static Method to create a new RlChoices Instance
     */
    pub fn new_choices(new_choices: Vec<RlType>, outer_choices: Option<RlChoices>) -> RlChoices {
        return Rc::new(Choices{
            choices: new_choices,
            index: 0,
            outer: outer_choices
        });
    }

    /**
    This method updates the index for the next choice and returns it.
    */
    pub fn next_choice(&mut self) -> RlResult {
        self.index += 1;
        return match self.choices.get(self.index) {
            Some(choice) => Ok(choice.clone()),
            None => error("No choices left for Choices object!"),
        }
    }

    /**
    This Method returns whether there are choices left in this choices instance
    */
    pub fn choices_left(&self) -> bool {
        return self.index + 1 < self.choices.len();
    }
}


