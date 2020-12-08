use crate::polynomial::Polynomial;

#[test]
fn polynomial_evaluation() {
  let mut polynomial: Polynomial<f64> = Polynomial::<f64>::new();
  polynomial.set_coefficient(0, 5.0);
  polynomial.set_coefficient(1, -3.0);
  polynomial.set_coefficient(2, 4.0);

  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    assert!(approx_eq!(f64, 4.0*i.powi(2) - 3.0*i + 5.0, polynomial.evaluate(i), epsilon=0.00001));
  }

  polynomial.purge_coefficient(2);
  polynomial.purge_coefficient(1);
  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    assert!(approx_eq!(f64, 5.0, polynomial.evaluate(i), epsilon=0.00001));
  }

  polynomial.set_coefficient(10, 127.0);
  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    assert!(approx_eq!(f64, 127.0 * i.powi(10) + 5.0, polynomial.evaluate(i), epsilon=0.00001));
  }
}

#[test]
fn polynomial_derivative_evaluation() {
  let mut polynomial: Polynomial<f64> = Polynomial::<f64>::new();
  polynomial.set_coefficient(0, 5.0);
  polynomial.set_coefficient(1, -3.0);
  polynomial.set_coefficient(2, 4.0);

  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    let (eval, deriv) = polynomial.evaluate_derivative(i);
    assert!(approx_eq!(f64, 4.0*i.powi(2) - 3.0*i + 5.0, eval, epsilon=0.00001));
    assert!(approx_eq!(f64, 8.0*i - 3.0, deriv));
  }

  polynomial.purge_coefficient(2);
  polynomial.purge_coefficient(1);
  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    let (eval, deriv) = polynomial.evaluate_derivative(i);
    assert!(approx_eq!(f64, 5.0, eval, epsilon=0.00001));
    assert!(approx_eq!(f64, 0.0, deriv, epsilon=0.00001));
  }

  polynomial.set_coefficient(10, 127.0);
  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    let (eval, deriv) = polynomial.evaluate_derivative(i);
    assert!(approx_eq!(f64, 127.0 * i.powi(10) + 5.0, eval, epsilon=0.00001));
    assert!(approx_eq!(f64, 1270.0 * i.powi(9), deriv, epsilon=0.00001));
  }
}

#[test]
fn polynomial_derivative() {
  let mut polynomial: Polynomial<f64> = Polynomial::<f64>::new();
  polynomial.set_coefficient(0, 5.0);
  polynomial.set_coefficient(1, -3.0);
  polynomial.set_coefficient(2, 4.0);
  polynomial = polynomial.derivative();

  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    println!("{} {}", 8.0*i - 3.0, polynomial.evaluate(i));
    assert!(approx_eq!(f64, 8.0*i - 3.0, polynomial.evaluate(i)));
  }

  polynomial.purge_coefficient(2);
  polynomial.purge_coefficient(1);
  polynomial.set_coefficient(0, 5.0);
  polynomial = polynomial.derivative();
  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    assert!(approx_eq!(f64, 0.0, polynomial.evaluate(i), epsilon=0.00001));
  }

  polynomial.set_coefficient(10, 127.0);
  polynomial.set_coefficient(0, 5.0);
  polynomial = polynomial.derivative();
  for i in (-1000..1000).step_by(1) {
    let i = i as f64 * 0.001;
    assert!(approx_eq!(f64, 1270.0 * i.powi(9), polynomial.evaluate(i), epsilon=0.00001));
  }
}