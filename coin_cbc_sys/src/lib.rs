#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knapsack() {
        // Simple knapsack problem
        // Maximize  5x[1] + 3x[2] + 2x[3] + 7x[4] + 4x[5]
        // s.t.      2x[1] + 8x[2] + 4x[3] + 2x[4] + 5x[5] <= 10
        // All x binary
        let start = [0, 1, 2, 3, 4, 5];
        let rowindex = [0, 0, 0, 0, 0];
        let value = [2., 8., 4., 2., 5.];
        let collb = [0., 0., 0., 0., 0.];
        let colub = [1., 1., 1., 1., 1.];
        let obj = [5., 3., 2., 7., 4.];
        let feasible = [1., 1., 0., 0., 0.];
        let rowlb = [-std::f64::INFINITY];
        let rowub = [10.];

        unsafe {
            let model = Cbc_newModel();
            Cbc_loadProblem(
                model,
                5,
                1,
                start.as_ptr(),
                rowindex.as_ptr(),
                value.as_ptr(),
                collb.as_ptr(),
                colub.as_ptr(),
                obj.as_ptr(),
                rowlb.as_ptr(),
                rowub.as_ptr(),
            );
            assert_eq!(5, Cbc_getNumCols(model));
            assert_eq!(1, Cbc_getNumRows(model));
            for i in 0..5 {
                Cbc_setInteger(model, i);
                assert!(Cbc_isInteger(model, i) != 0);
            }
            Cbc_setObjSense(model, -1.);
            assert_eq!(-1., Cbc_getObjSense(model));
            Cbc_setInitialSolution(model, feasible.as_ptr());

            Cbc_solve(model);

            assert!(Cbc_isProvenOptimal(model) != 0);
            assert!((Cbc_getObjValue(model) - 16.).abs() < 1e-6);
            assert!((Cbc_getBestPossibleObjValue(model) - 16.).abs() < 1e-6);

            let sol = std::slice::from_raw_parts(Cbc_getColSolution(model), 5);
            assert!((sol[0] - 1.).abs() < 1e-6);
            assert!((sol[1] - 0.).abs() < 1e-6);
            assert!((sol[2] - 0.).abs() < 1e-6);
            assert!((sol[3] - 1.).abs() < 1e-6);
            assert!((sol[4] - 1.).abs() < 1e-6);

            Cbc_deleteModel(model);
        }
    }
}
