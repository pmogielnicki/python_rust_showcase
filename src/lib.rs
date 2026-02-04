use pyo3::prelude::*;
use rayon::prelude::*;
use rand::prelude::*;
use rand::distr::Uniform;

// Define the class at top level so PyO3 can generate its bindings
#[pyclass]

// Struct holding the data about the array for pyo3 to store it
struct PArray {
    data: Vec<f64>,
    size: usize,
}

#[pymethods]
impl PArray {

    // What to do on creation
    #[new]
    fn new(insize: usize) -> Self {Self { data: Vec::with_capacity(insize), size: insize }}
    // Push value onto PArray
    fn push(&mut self, value: f64) {self.data.push(value);}
    // Clear array
    fn clear(&mut self) {self.data.clear();}
    // Sort array in paralell (Thank you rayon)
    fn sort(&mut self) {self.data.par_sort_unstable_by(|a, b| a.total_cmp(b));}
    // Get data from PArray as array
    fn randomize(&mut self) {
        self.data.clear();
        let mut rng = rand::rng();

        let r_dist = Uniform::new(0., 10.).unwrap();

        let rand_vec: Vec<f64> = (0..self.size)
            .into_par_iter()
            .map_init(rand::rng, |rng, _| rng.sample(r_dist))
            .collect();
        self.data = rand_vec;

    }
    fn retrieve(&self) -> Vec<f64> {self.data.clone()}
}

//Declare python module
#[pymodule]
//Black magic, basically tell interpreter to do something.
fn python_rust_showcase(py: Python, m: &PyModule) -> PyResult<()> {
    //Add class to module
    m.add_class::<PArray>()?;
    //Return Ok (This module is not error safe but is fine for a small example)
    Ok(())
}