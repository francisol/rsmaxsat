pub mod slover;


#[cfg(test)]
mod tests {
    use crate::slover::MaxSATSolver;

    use super::*;

    #[test]
    fn evalmaxsat() {
        let mut solver = slover::evalmaxsat::CadicalEvalMaxSATSolver::new();
        solver.add_clause(&vec![1, -2], None);
        solver.add_clause(&vec![-1, 2], None);
        solver.add_clause(&vec![2], Some(1));
        match solver.solve() {
            slover::Status::SATISFIABLE => {
                assert_eq!(vec![true, true, true], solver.bool_model());
            }
            slover::Status::UNSATISFIABLE => assert_eq!(1, 0),
            slover::Status::UNKNOWN => assert_eq!(10, 0),
        }
    }
}
