use super::prime::PrimeFactorizer;

#[test]
fn prime_factorizer() {
    let p = PrimeFactorizer::prepare(1000);

    let mut factors = Vec::new();
    p.factorize(10, &mut factors);

    assert_eq!(factors, vec![2, 5]);
}
