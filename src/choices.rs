/*
choices.rs: This module contains a Data-structure that is essential for the non-deterministic
            pattern of RLisp. The Structure keeps track of all choices that can be made and serves
            methods to work with them. More details can be found in the ReadMe.
 */

// Import needed functionality from Rust stdlib
use std::rc::Rc;
use std::cell::RefCell;

// load functionality from sibling modules
use crate::types::{RlReturn, RlType, choice_error};
use crate::env::RlEnv;
use crate::printer;

/// The RlChoices type wraps the Choices in a reference Cell for easy access on heap
pub type RlChoicesManager = Rc<ChoicesManager>;

#[derive(Clone, Debug)]
pub struct ChoicesManager {
    total_depth: RefCell<NumberBox>,
    current_depth: RefCell<NumberBox>,
    choice_points: RefCell<Vec<Choices>>,
    expression: RefCell<ExpressionWrapper>,
}
/**
Static Method to create a new RlChoicesManager Instance
 */
pub fn new_choices_manager(expression: RlType) -> RlChoicesManager {
    return Rc::new(ChoicesManager {
        choice_points: RefCell::new(Vec::new()),
        expression: RefCell::new(ExpressionWrapper::new(expression)),
        total_depth: RefCell::new(NumberBox::new(0)),
        current_depth: RefCell::new(NumberBox::new(0)),
    });
}


pub fn reset_choices_manager(manager: &RlChoicesManager) {
    manager.choice_points.borrow_mut().clear();
    manager.total_depth.borrow_mut().set(0);
    manager.current_depth.borrow_mut().set(0);
}


/**
This Method updates the choice points. It is to be called before every eval
returns true when there are choices left and you can try again and returns false if you cannot try again
 */
pub fn update_choice_points(manager: &RlChoicesManager) -> bool {
    // total_depth is only 0 if there are no choice points
    if manager.total_depth.borrow_mut().get() != 0 {
        // check if last failed choice point went out of bounds
        if manager.choice_points.borrow()[(manager.current_depth.borrow_mut().get() - 1) % manager.total_depth.borrow_mut().get()].out_of_bounds() {
            //if the choices in the first choice point are depleted there are no more choices left to try
            if manager.current_depth.borrow_mut().get() == 1 {
                println!("cleared top choice point");
                manager.choice_points.borrow_mut().clear();
                update_depth_fields(manager);
                // signals that all choices have been tried
                return false;
            }
            // remove the choice points coming after the last failed choice point
            let mut i = 0;
            while i < (manager.choice_points.borrow().len() - (manager.current_depth.borrow_mut().get() - 1)) {
                manager.choice_points.borrow_mut().pop();
                i += 1;
            }
        }
        // reset depth fields
        update_depth_fields(manager);
        return true;
    }
    return false;
}

/**
This method creates a new choice point and returns the first choice of the new choice point
 */
fn append_choice_point(manager: &RlChoicesManager, choices: Vec<RlType>) -> RlReturn {
    // borrow current choice points as mutable and add new point to it
    let mut mut_ref = manager.choice_points.borrow_mut();
    mut_ref.push(Choices::new_choices(choices));
    // increase total depth counter
    manager.total_depth.borrow_mut().inc();
    // return the new added choice point
    mut_ref.last_mut().unwrap().current_choice()
}

/**
This method resets the current depth counter. To be called every time the expression gets
evaluated again
 */
fn update_depth_fields(manager: &RlChoicesManager) {
    manager.total_depth.borrow_mut().set(manager.choice_points.borrow().len());
    manager.current_depth.borrow_mut().set(0);
}

/**
This method checks whether we reached the last choice point. Used to see if a new choice point
needs to be created or the next choice should be tried.
 */
fn last_choice_point(manager: &RlChoicesManager) -> bool {
    return manager.current_depth.borrow_mut().get() == manager.total_depth.borrow_mut().get()
}


/**
This method returns the the next choice of the current choice point
 */
pub fn get_choice(manager: &RlChoicesManager, choices: Vec<RlType>) -> RlReturn {
    if last_choice_point(manager) {
        manager.current_depth.borrow_mut().inc();
        append_choice_point(manager, choices)
    } else {
        manager.current_depth.borrow_mut().inc();
        if !last_choice_point(manager) {
            //keep current choice
            manager.choice_points.borrow_mut()[manager.current_depth.borrow_mut().get() - 1].current_choice()
        } else {
            //try next choice
            manager.choice_points.borrow_mut()[manager.current_depth.borrow_mut().get() - 1].next_choice()
        }
    }
}

/**
This method gets the currently saved expression in the choices manager
*/
pub fn get_expression(manager: &RlChoicesManager) -> RlType {
    return manager.expression.borrow().get();
}


/**
This method sets the an expression in the choices manager
*/
pub fn set_expression(manager: &mut RlChoicesManager, exp: RlType) {
    manager.expression.borrow_mut().set(exp);
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
    pub fn new_choices(new_choices: Vec<RlType>) -> Choices {
        return Choices{
            choices: new_choices,
            index: 0,
        };
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

    /**
    This method returns the current choice for current choice point
    */
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
        return self.index >= self.choices.len();
    }

    /**
    resets the index of the choicepoint. This enables nested search.
    */
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

#[derive(Clone, Debug)]
struct NumberBox {
    number: usize,
}

impl NumberBox {
    pub fn new(num: usize) -> NumberBox {
        return NumberBox {
            number: num
        }
    }

    pub fn get(&self) -> usize {
        return self.number;
    }

    pub fn inc(&mut self) {
        self.number += 1;
    }

    pub fn set(&mut self, value: usize) {
        self.number = value;
    }
}


#[derive(Clone, Debug)]
struct ExpressionWrapper {
    expression: RlType,
}

impl ExpressionWrapper {
    pub fn new(expression: RlType) -> ExpressionWrapper {
        return ExpressionWrapper {
            expression
        }
    }

    pub fn get(&self) -> RlType {
        return self.expression.clone();
    }

    pub fn set(&mut self, expression: RlType) {
        self.expression = expression;
    }
}
