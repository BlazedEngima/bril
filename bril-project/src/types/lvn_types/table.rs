use crate::types::lvn_types::{ConstantKey, Expression, ValueNumber};
use ahash::{HashMap, HashSet};

pub struct LVNTable {
    next_value_number: ValueNumber,

    // Variable Name -> Value Number
    variable_numbers_map: HashMap<String, ValueNumber>,

    // Expression -> Value Number produced by that expression
    expression_numbers_map: HashMap<Expression, ValueNumber>,

    // Constant -> Value Number
    constant_numbers_map: HashMap<ConstantKey, ValueNumber>,

    // Value Number -> Variables currently holding that value
    value_numbers_map: HashMap<ValueNumber, HashSet<String>>,

    // Value Number -> Constant Key being represented by that number
    value_constants_map: HashMap<ValueNumber, ConstantKey>,
}

impl LVNTable {
    pub fn new() -> Self {
        Self {
            next_value_number: 1,
            variable_numbers_map: HashMap::default(),
            expression_numbers_map: HashMap::default(),
            constant_numbers_map: HashMap::default(),
            value_numbers_map: HashMap::default(),
            value_constants_map: HashMap::default(),
        }
    }

    pub fn fresh_value_number(&mut self) -> ValueNumber {
        let vn = self.next_value_number;
        self.next_value_number += 1;
        vn
    }

    /// Returns the current value number of a variable.
    pub fn get_vn_from_variable(&self, variable: &String) -> Option<ValueNumber> {
        self.variable_numbers_map.get(variable).copied()
    }

    /// Returns the current value number of a variable.
    /// Assigns a fresh value number if the variable hasn't been seen.
    pub fn get_or_create_vn_from_variable(&mut self, variable: &str) -> ValueNumber {
        if let Some(&vn) = self.variable_numbers_map.get(variable) {
            return vn;
        }

        let vn = self.fresh_value_number();
        self.assign_vn_to_variable(variable.to_string(), vn);

        vn
    }

    /// Assigns a variable to a value number, updating the reverse mapping.
    pub fn assign_vn_to_variable(&mut self, variable: String, vn: ValueNumber) {
        // Remove the variable from its previous value number.
        if let Some(old_vn) = self.get_vn_from_variable(&variable) {
            if old_vn == vn {
                return;
            }

            if let Some(variables) = self.value_numbers_map.get_mut(&old_vn) {
                variables.remove(&variable);

                // Remove empty sets to avoid accumulating stale entries.
                if variables.is_empty() {
                    self.value_numbers_map.remove(&old_vn);
                }
            }
        }

        // Update the forward mapping.
        self.variable_numbers_map.insert(variable.clone(), vn);

        // Update the reverse mapping.
        self.value_numbers_map
            .entry(vn)
            .or_default()
            .insert(variable);
    }

    /// Finds any variable currently holding the given value number.
    pub fn get_variable_from_vn(&self, vn: ValueNumber) -> Option<&str> {
        self.value_numbers_map
            .get(&vn)
            .and_then(|variables| variables.iter().next())
            .map(String::as_str)
    }

    /// Finds a variable holding the given value number, excluding `excluded`.
    pub fn get_variable_from_vn_except(&self, vn: ValueNumber, excluded: &str) -> Option<&str> {
        self.value_numbers_map
            .get(&vn)?
            .iter()
            .find(|variable| variable.as_str() != excluded)
            .map(String::as_str)
    }

    pub fn get_vn_from_expression(&self, expression: &Expression) -> Option<ValueNumber> {
        self.expression_numbers_map.get(expression).copied()
    }

    pub fn insert_expression(&mut self, expression: Expression, vn: ValueNumber) {
        self.expression_numbers_map.insert(expression, vn);
    }

    pub fn get_vn_from_constant(&self, constant: &ConstantKey) -> Option<ValueNumber> {
        self.constant_numbers_map.get(constant).copied()
    }

    pub fn get_constant_from_vn(&self, vn: &ValueNumber) -> Option<&ConstantKey> {
        self.value_constants_map.get(vn)
    }

    pub fn insert_constant(&mut self, constant: ConstantKey, vn: ValueNumber) {
        self.constant_numbers_map.insert(constant.clone(), vn);
        self.value_constants_map.insert(vn, constant);
    }

    /// Returns an existing constant's value number, or creates one.
    pub fn get_or_create_vn_from_constant(&mut self, constant: ConstantKey) -> ValueNumber {
        if let Some(&vn) = self.constant_numbers_map.get(&constant) {
            return vn;
        }

        let vn = self.fresh_value_number();
        self.insert_constant(constant, vn);
        vn
    }

    /// Returns an existing expression's value number, or creates one.
    pub fn get_or_create_expression_number(&mut self, expression: Expression) -> ValueNumber {
        if let Some(&vn) = self.expression_numbers_map.get(&expression) {
            return vn;
        }

        let vn = self.fresh_value_number();
        self.insert_expression(expression, vn);
        vn
    }
}

impl Default for LVNTable {
    fn default() -> Self {
        Self::new()
    }
}
