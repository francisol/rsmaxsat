#[cfg(feature = "evalmaxsat")]
pub mod evalmaxsat;


pub enum Status {
    SATISFIABLE,UNSATISFIABLE,UNKNOWN
}

pub trait MaxSATSolver {
    fn add_clause(& mut self, clause:&Vec<i32>,w:Option<i64>);
    fn solve(& mut self)->Status;
    fn bool_model(& mut self)->Vec<bool>;
    fn value(& mut self,lit: i32)->bool;
}