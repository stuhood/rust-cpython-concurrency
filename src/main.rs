use cpython::{PyResult, Python};
use futures::future;

fn a_python_function() -> PyResult<()> {
    let gil = Python::acquire_gil();
    gil.python().run("2 + 2", None, None)
}

#[tokio::main]
async fn main() {
    let num_threads: u8 = std::env::args()
        .nth(1)
        .expect("Expected one integer argument for the number of threads to use.")
        .parse()
        .unwrap();
    let handles = (0..num_threads)
        .map(|_| {
            tokio::task::spawn_blocking(move || {
                for _ in 0..1_000_000 {
                    a_python_function().unwrap()
                }
            })
        })
        .collect::<Vec<_>>();
    let _ = future::join_all(handles).await;
}
