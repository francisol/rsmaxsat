#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::vec;

use super::MaxSATSolver;

pub mod bindings{
    include!(concat!(env!("OUT_DIR"), "/evalmaxsat_bindings.rs"));
}

pub struct CadicalEvalMaxSATSolver{
    inner: bindings::Eval_CadicalEvalMaxSAT
}

impl  CadicalEvalMaxSATSolver{
    pub fn new()->Self{
        unsafe {
            Self{
                inner:bindings::Eval_CadicalEvalMaxSAT::new()
            }
        }
    }
}

impl MaxSATSolver for  CadicalEvalMaxSATSolver{
    fn add_clause(& mut self, clause:&Vec<i32>,w:Option<i64>) {
        unsafe {
        self.inner.addClause(clause.as_ptr(),clause.len(),w.unwrap_or(0));
        }
    }

    fn solve(& mut self)->super::Status {
        unsafe {
            if self.inner.solve() {
                return  super::Status::SATISFIABLE;
            }
            return  super::Status::UNSATISFIABLE;
        }
    }
    
    fn bool_model(& mut self)->Vec<bool> {
        let mut model=vec![true];
        unsafe {
        for i in 1..self.inner.nVars()+1  {
           model.push( self.inner.getValue(i as i32));
        }
    }
        model
    }
}