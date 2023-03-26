// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;

// fn main() -> PyResult<()> {
//     Python::with_gil(|py| {
//         let sys = py.import("sys")?;
//         let version: String = sys.getattr("version")?.extract()?;

//         let locals = [("os", py.import("os")?)].into_py_dict(py);
//         let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
//         let user: String = py.eval(code, None, Some(&locals))?.extract()?;

//         println!("Hello {}, I'm Python {}", user, version);
//         Ok(())
//     })
// }

// #![feature(stdsimd)]
// use std::arch::aarch64::ST;

use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::PyDict;

const PYTHON_CODE: &str = include_str!("./python/tut.py");
// const CODE: &str = r#"
// def function(*args, **kwargs):
//     assert args == ("hello",)
//     assert kwargs == {}
//     return "called with args"
// "#;

fn main() -> PyResult<()> {
    // pyo3::append_to_inittab!(jax);

    // pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        py.run(PYTHON_CODE, None, None)
        // Ok(())
    })
    // Ok(())
    //     Python::with_gil(|py| {
    //         let sys = py.import("sys")?;
    //         let os = py.import("os")?;
    //         let jax = py.import("jax")?;
    //         let np = py.import("jax.numpy")?;
    //         let rand = py.import("random")?;
    //         // let version: String = sys.getattr("version")?.extract()?;
    //         let version: String = sys.getattr("version")?.extract()?;
    //         // let pyv: i64 =
    //         //     os.call_method1("system", ("which python",))?.extract()?;
    //         // import jax.numpy as jnp
    //         // from jax import grad, jit, vmap
    //         // from jax import random

    //         // println!("Python version: {}", pyv);
    //         let username: String =
    //             os.call_method1("getenv", ("USER",))?.extract()?;
    //         // let path: String =
    //         //     sys.("path")?.extract()?;
    //         let path = sys.getattr(intern!(sys.py(), "path"))?;
    //         let prefix = sys.getattr(intern!(sys.py(), "prefix"))?;
    //         let base_prefix = sys.getattr(intern!(sys.py(), "base_prefix"))?;
    //         let key = jax.getattr("random")?.getattr("PRNGKey")?.call1((0,))?;
    //         println!("key: {}", key);
    //         py.run(
    //             "
    // import jax
    // key = jax.random.PRNGKey(0)
    // print(key)
    // x = jax.random.normal(key, (10,))
    // print(x)
    //             ",
    //             None,
    //             None,
    //         )?;
    //         println!("Prefix: {}", prefix);
    //         println!("Base Prefix: {}", base_prefix);
    //         println!("Hello, {username}");
    //         println!("Python Version: {version}");
    //         println!("path: {path}");
    //         println!("jax: {jax}");
    //         println!("np: {np}");
    //         Ok(())
    //     })
}
