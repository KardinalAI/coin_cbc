//! Coin CBC Rust bindings
//!
//! This crate exposes safe and efficient bindings to the Coin CBC C
//! API.
//!
//! For more information on how to install the `Cbc` library dependencies,
//! see [the respective README section](https://github.com/KardinalAI/coin_cbc/README.md#prerequisites-installing-cbc-library-files).
//!
//! This project is distributed under the MIT License by
//! [Kardinal](https://kardinal.ai).

#![deny(missing_docs)]

pub mod raw;

pub use raw::Sense;

use std::collections::BTreeMap;
use std::ffi::CString;
use std::os::raw::c_int;

/// A column identifier.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Col(u32);
impl Col {
    fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// A row identifier.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Row(u32);
impl Row {
    fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// A MILP model.
#[derive(Default, Clone)]
pub struct Model {
    num_cols: u32,
    num_rows: u32,
    col_lower: Vec<f64>,
    col_upper: Vec<f64>,
    row_lower: Vec<f64>,
    row_upper: Vec<f64>,
    obj_coefficients: Vec<f64>,
    weights: Vec<BTreeMap<Row, f64>>,
    is_integer: Vec<bool>,
    sense: Sense,
    initial_solution: Option<Vec<f64>>,
    parameters: BTreeMap<CString, CString>,
}

impl Model {
    /// Gets the current number of rows of the model.
    pub fn num_rows(&self) -> u32 {
        self.num_rows
    }
    /// Gets the current number of columns of the model.
    pub fn num_cols(&self) -> u32 {
        self.num_cols
    }
    /// Removes the initial solution.
    pub fn remove_initial_solution(&mut self) {
        self.initial_solution = None;
    }
    /// Sets the column value to the initial solution.
    ///
    /// If the solution is not present, it will be initialized with 0
    /// for all coefficients.
    pub fn set_col_initial_solution(&mut self, col: Col, value: f64) {
        if self.initial_solution.is_none() {
            self.initial_solution = Some(vec![0.; self.num_cols as usize]);
        }
        let sol = self.initial_solution.as_mut().unwrap();
        sol[col.as_usize()] = value;
    }
    /// Gets the column value to the initial solution.
    pub fn get_col_initial_solution(&self, col: Col) -> Option<f64> {
        self.initial_solution.as_ref().map(|s| s[col.as_usize()])
    }
    /// Sets the initial solution from a `Solution`.
    pub fn set_initial_solution(&mut self, solution: &Solution) {
        for col in self.cols() {
            self.set_col_initial_solution(col, solution.col(col));
        }
    }
    /// Sets a parameter.
    ///
    /// For documentation, launch the `cbc` binary and type `?`.
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        let key = match CString::new(key) {
            Ok(s) => s,
            Err(_) => return,
        };
        let value = match CString::new(value) {
            Ok(s) => s,
            Err(_) => return,
        };
        self.parameters.insert(key, value);
    }
    /// Sets parameters for an iterator.
    pub fn set_parameters(
        &mut self,
        iter: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) {
        for (k, v) in iter.into_iter() {
            self.set_parameter(k.as_ref(), v.as_ref());
        }
    }
    /// Gets an iterator on the row identifiers.
    pub fn rows(&self) -> impl Iterator<Item = Row> {
        (0..self.num_rows).map(Row)
    }
    /// Gets an iterator on the column identifiers.
    pub fn cols(&self) -> impl Iterator<Item = Col> {
        (0..self.num_cols).map(Col)
    }
    /// Adds a column to the model. Returns the corresponding column
    /// identifier.
    ///
    /// At creation, the bounds of the column are setted to [0, +∞].
    pub fn add_col(&mut self) -> Col {
        let col = Col(self.num_cols);
        self.num_cols += 1;
        self.obj_coefficients.push(0.);
        self.weights.push(Default::default());
        self.is_integer.push(false);
        self.col_lower.push(0.);
        self.col_upper.push(std::f64::INFINITY);
        self.initial_solution.as_mut().map(|sol| sol.push(0.));
        col
    }
    /// Adds an integer variable to the model.
    ///
    /// Equivalent to adding a column and setting it to integer.
    pub fn add_integer(&mut self) -> Col {
        let col = self.add_col();
        self.set_integer(col);
        col
    }
    /// Adds a binary variable to the model.
    ///
    /// Equivalent to adding a column and setting it to binary.
    pub fn add_binary(&mut self) -> Col {
        let col = self.add_col();
        self.set_binary(col);
        col
    }
    /// Adds a row to the model. Returns the corresponding row
    /// identifier.
    ///
    /// At creation, the bounds of the row are setted to [-∞, +∞].
    pub fn add_row(&mut self) -> Row {
        let row = Row(self.num_rows);
        self.num_rows += 1;
        self.row_lower.push(std::f64::NEG_INFINITY);
        self.row_upper.push(std::f64::INFINITY);
        row
    }
    /// Sets the weight corresponding to the given row and column in
    /// the constraint matrix.
    pub fn set_weight(&mut self, row: Row, col: Col, weight: f64) {
        if weight == 0. {
            self.weights[col.as_usize()].remove(&row);
        } else {
            self.weights[col.as_usize()].insert(row, weight);
        }
    }
    /// Changes the given column to integer variable.
    pub fn set_integer(&mut self, col: Col) {
        self.is_integer[col.as_usize()] = true;
    }
    /// Changes the given column to continuous variable.
    pub fn set_continuous(&mut self, col: Col) {
        self.is_integer[col.as_usize()] = false;
    }
    /// Changes the given column to binary variable.
    ///
    /// Equivalent to setting the column as integer and restricting it
    /// to [0, 1].
    pub fn set_binary(&mut self, col: Col) {
        self.set_integer(col);
        self.set_col_lower(col, 0.);
        self.set_col_upper(col, 1.);
    }
    /// Sets the upper bound of the given column.
    pub fn set_col_upper(&mut self, col: Col, value: f64) {
        self.col_upper[col.as_usize()] = value;
    }
    /// Sets the lower bound of the given column.
    pub fn set_col_lower(&mut self, col: Col, value: f64) {
        self.col_lower[col.as_usize()] = value;
    }
    /// Sets the objective coefficient of the given variable.
    pub fn set_obj_coeff(&mut self, col: Col, value: f64) {
        self.obj_coefficients[col.as_usize()] = value;
    }
    /// Sets the upper bound of the given row.
    pub fn set_row_upper(&mut self, row: Row, value: f64) {
        self.row_upper[row.as_usize()] = value;
    }
    /// Sets the lower bound of the given row.
    pub fn set_row_lower(&mut self, row: Row, value: f64) {
        self.row_lower[row.as_usize()] = value;
    }
    /// Force the given row to be equal to the given value.
    ///
    /// Equivalent to setting the upper bound and the lower bound.
    pub fn set_row_equal(&mut self, row: Row, value: f64) {
        self.set_row_upper(row, value);
        self.set_row_lower(row, value);
    }
    /// Sets the objective sense.
    pub fn set_obj_sense(&mut self, sense: Sense) {
        self.sense = sense;
    }
    /// Construct a `raw::Model` corresponding to the current state.
    pub fn to_raw(&self) -> raw::Model {
        let mut start = Vec::with_capacity(self.num_cols as usize + 1);
        let mut index = Vec::with_capacity(self.num_cols.max(self.num_rows) as usize);
        let mut value = Vec::with_capacity(self.num_cols.max(self.num_rows) as usize);
        start.push(0);
        for col_weights in &self.weights {
            for (r, w) in col_weights {
                index.push(r.0 as c_int);
                value.push(*w);
            }
            start.push(index.len() as c_int);
        }
        let mut raw = raw::Model::new();
        raw.load_problem(
            self.num_cols as usize,
            self.num_rows as usize,
            &start,
            &index,
            &value,
            Some(&self.col_lower),
            Some(&self.col_upper),
            Some(&self.obj_coefficients),
            Some(&self.row_lower),
            Some(&self.row_upper),
        );
        for (col, &is_int) in self.is_integer.iter().enumerate() {
            if is_int {
                raw.set_integer(col);
            } else {
                raw.set_continuous(col);
            }
        }
        raw.set_obj_sense(self.sense);
        for (k, v) in &self.parameters {
            raw.set_parameter(k, v);
        }
        if let Some(sol) = &self.initial_solution {
            raw.set_initial_solution(sol);
        }
        raw
    }
    /// Solves the model. Returns the solution.
    pub fn solve(&self) -> Solution {
        let mut raw = self.to_raw();
        raw.solve();
        Solution { raw }
    }
}

/// A solution to a MILP problem.
///
/// This is a thin wrapper over a `raw::Model` with accessors using
/// the typed identifiers.
pub struct Solution {
    raw: raw::Model,
}
impl Solution {
    /// Gets a shared reference to the internal `raw::Model`.
    pub fn raw(&self) -> &raw::Model {
        &self.raw
    }
    /// Gets the internal `raw::Model`
    pub fn into_raw(self) -> raw::Model {
        self.raw
    }
    /// Gets the value of the given column in the solution.
    pub fn col(&self, col: Col) -> f64 {
        self.raw.col_solution()[col.as_usize()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knapsack() {
        let mut m = Model::default();
        m.set_parameter("log", "0");
        let row = m.add_row();
        m.set_row_upper(row, 10.);
        let cols = vec![
            m.add_binary(),
            m.add_binary(),
            m.add_binary(),
            m.add_binary(),
            m.add_binary(),
        ];
        m.set_weight(row, cols[0], 2.);
        m.set_weight(row, cols[1], 8.);
        m.set_weight(row, cols[2], 4.);
        m.set_weight(row, cols[3], 2.);
        m.set_weight(row, cols[4], 5.);
        m.set_obj_coeff(cols[0], 5.);
        m.set_obj_coeff(cols[1], 3.);
        m.set_obj_coeff(cols[2], 2.);
        m.set_obj_coeff(cols[3], 7.);
        m.set_obj_coeff(cols[4], 4.);
        m.set_obj_sense(Sense::Maximize);

        let sol = m.solve();
        assert_eq!(raw::Status::Finished, sol.raw().status());
        assert_eq!(16., sol.raw().obj_value());
        assert_eq!(1., sol.col(cols[0]));
        assert_eq!(0., sol.col(cols[1]));
        assert_eq!(0., sol.col(cols[2]));
        assert_eq!(1., sol.col(cols[3]));
        assert_eq!(1., sol.col(cols[4]));
    }
}
