pub mod solver;


#[cfg(test)]
mod tests {
    use crate::solver::MaxSATSolver;

    use super::*;

    #[test]
    fn evalmaxsat() {
        let mut solver = solver::evalmaxsat::CadicalEvalMaxSATSolver::new();
        solver.add_clause(&vec![1, -2], None);
        solver.add_clause(&vec![-1, 2], None);
        solver.add_clause(&vec![2], Some(1));
        match solver.solve() {
            solver::Status::SATISFIABLE => {
                assert_eq!(true, solver.value(2));
            }
            solver::Status::UNSATISFIABLE => assert_eq!(1, 0),
            solver::Status::UNKNOWN => assert_eq!(10, 0),
        }
    }
}
