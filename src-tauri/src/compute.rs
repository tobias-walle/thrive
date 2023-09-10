use std::str::FromStr;

use indoc::formatdoc;
use pyo3::{
    exceptions::PyException,
    pyclass, pymethods,
    types::{IntoPyDict, PyDict},
    PyCell, PyResult, Python,
};
use shared::{Coordinate, State, TableCell, TableCellWithCoordinates};

#[pyclass]
struct PythonApi {
    state: State,
}

#[pymethods]
impl PythonApi {
    fn cell(&self, coord: &str) -> PyResult<Option<i32>> {
        let coord =
            Coordinate::from_str(coord).map_err(|e| PyException::new_err(format!("{e}")))?;
        let value = self.state.cell(&coord);
        Ok(value.computed.parse::<i32>().ok())
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn compute(state: State, coord: Coordinate) -> Vec<TableCellWithCoordinates> {
    let cell = state.cell(&coord);
    let computed = match cell.get_formula() {
        Some(formula) => {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| {
                let api = PyCell::new(
                    py,
                    PythonApi {
                        state: state.clone(),
                    },
                )
                .unwrap();
                let locals = PyDict::new(py);
                let globals = [("api", api)].into_py_dict(py);

                #[rustfmt::skip]
                let formula = formatdoc!(r#"
                    global c;
                    c = api.cell

                    def main():
                        return {formula}

                    result = main()
                "#);

                match py.run(formula.trim(), Some(globals), Some(locals)) {
                    Ok(()) => {
                        let result = locals
                            .get_item("result")
                            .expect("should have 'result' variable")
                            .to_string();
                        result.to_string()
                    }
                    Err(e) => format!("{e}"),
                }
            })
        }
        None => cell.text.clone(),
    };
    vec![TableCell {
        computed,
        ..cell.clone()
    }
    .with_coord(coord)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions as pretty;

    #[test]
    fn compute_should_copy_text_if_no_formula_is_provided() {
        let coord = Coordinate::new(0, 0);
        let state = State::from([TableCellWithCoordinates {
            coord,
            cell: TableCell {
                text: "Hello World".into(),
                ..Default::default()
            },
        }]);

        let computed = compute(state.clone(), coord);

        pretty::assert_eq!(
            computed,
            [TableCellWithCoordinates {
                coord,
                cell: TableCell {
                    computed: "Hello World".into(),
                    ..state.cell(&coord).clone()
                },
            }]
        );
    }

    #[test]
    fn compute_should_solve_simple_formulas() {
        let coord = Coordinate::new(0, 0);
        let state = State::from([TableCellWithCoordinates {
            coord,
            cell: TableCell {
                text: "=(1+3)*3".into(),
                ..Default::default()
            },
        }]);

        let computed = compute(state.clone(), coord);

        pretty::assert_eq!(
            computed,
            [TableCellWithCoordinates {
                coord,
                cell: TableCell {
                    computed: "12".into(),
                    ..state.cell(&coord).clone()
                },
            }]
        );
    }

    #[test]
    fn compute_should_solve_formulas_with_references_to_other_cells() {
        let coord_a1 = Coordinate::new(0, 0);
        let coord_b1 = Coordinate::new(1, 0);
        let coord_a2 = Coordinate::new(0, 1);
        let state = State::from([
            TableCellWithCoordinates {
                coord: coord_a1,
                cell: TableCell {
                    text: "1".into(),
                    computed: "1".into(),
                    ..Default::default()
                },
            },
            TableCellWithCoordinates {
                coord: coord_b1,
                cell: TableCell {
                    text: "2".into(),
                    computed: "2".into(),
                    ..Default::default()
                },
            },
            TableCellWithCoordinates {
                coord: coord_a2,
                cell: TableCell {
                    text: r#"=c("A1") + c("B1") + 3"#.into(),
                    ..Default::default()
                },
            },
        ]);

        let computed = compute(state.clone(), coord_a2);

        pretty::assert_eq!(
            computed,
            [TableCellWithCoordinates {
                coord: coord_a2,
                cell: TableCell {
                    computed: "6".into(),
                    ..state.cell(&coord_a2).clone()
                },
            }]
        );
    }
}
