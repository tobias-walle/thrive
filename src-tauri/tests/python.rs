use pyo3::prelude::*;

#[test]
fn test_if_python_can_be_used() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let result = py
            .eval("[i * 10 for i in range(5)]", None, None)
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
            })
            .unwrap();
        let res: Vec<i64> = result.extract().unwrap();
        assert_eq!(res, vec![0, 10, 20, 30, 40]);
    });
}
