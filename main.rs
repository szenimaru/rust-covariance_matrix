extern crate ndarray;
use ndarray::prelude::*;
//[dependencies]
//ndarray = { version = "0.13.0", features = ["blas"] }

fn set_sample_data() -> Vec<ndarray::Array1<f64>>{
    let mut sample_data = Vec::new();

    sample_data.push(ndarray::arr1( & [ 1.0,2.0,3.0 ] ));
    sample_data.push(ndarray::arr1( & [ 2.0,4.0,6.0 ] ));
    sample_data.push(ndarray::arr1( & [ 3.0,6.0,9.0 ] ));

    return sample_data;
}

fn cov_mat(x: Vec<ndarray::Array1<f64>>) -> Array2<f64> {

    if x.is_empty() {
        eprintln!("error : cov_mat");
        eprintln!("Nothing data");
        std::process::exit(1);
    }

    for i in 1..x.len() {
        if x[0].len() != x[i].len() {
            eprintln!("error : cov_mat");
            eprintln!("Not all data dimension's are the same");
            std::process::exit(1);
        }
    }

    let mut mu: Array1<f64> = Array::zeros(x[0].len());

    for i in 0..x[0].len() {
        for j in 0..x.len() {
            mu[i] += x[j][i];
        }
    }

    for i in 0..x[0].len() {
        mu[i] /= x.len() as f64;
    }

    let mut x_mu = Vec::new();// x - mu

    for i in 0..x.len() {
        x_mu.push( &x[i] - &mu );
    }

    let mut sig: Array2<f64> = Array::zeros((x[0].len(),x[0].len()));

    for i in 0..x_mu[0].len() {
        for j in 0..x_mu[0].len() {
            for k in 0..x_mu.len() {
                sig[[i,j]] += x_mu[k][i]*x_mu[k][j];
            }
        }
    }

    for i in 0..x_mu[0].len() {
        for j in 0..x_mu[0].len() {
            sig[[i,j]] /= x_mu.len() as f64;
        }
    }

    return sig;
}

fn main() {
    //println!("Hello, world!");
    
    let x = set_sample_data();

    let sig = cov_mat(x);

    println!("{}",sig);

}
