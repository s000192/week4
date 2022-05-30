use ark_bls12_381::Fq;
use rand::random;
use ndarray::{arr2, Array, Array1, Array2};
// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
struct Freivald {
    x: Array1<Fq>// Array/Vec of Fq,
}

impl Freivald {
    // TODO: Create constructor for object
    fn new(array_size: usize) -> Self {
        let mut arr: Array1<Fq> = Array1::zeros(array_size);
        // Populate vector with values r^i for i=0..matrix_size
        let mut i = 0;
        while i < array_size {
            let rand_num: i32 = random();
            let random_fq: Fq = Fq::from(rand_num);
            arr[i] = random_fq;
            i = i + 1;
        }
        // Return freivald value with this vector as its x value
        return Freivald{x: arr};
    }

    // TODO: Add proper types to input matrices. Remember matrices should hold Fq values
    fn verify(&self, matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        assert!(check_matrix_dimensions(&matrix_a, &matrix_b, &supposed_ab));
        // TODO: check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct
        let lhs = matrix_a.dot(matrix_b).dot(&self.x);
        let rhs = supposed_ab.dot(&self.x);
        lhs == rhs
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // TODO: Add types for arguments
    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)

// TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
    // TODO: Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents
    matrix_a.dim() == matrix_b.dim() && matrix_b.dim() == supposed_ab.dim()
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;


    lazy_static! {
        static ref MATRIX_A: Array2<Fq> = arr2(&[
            [Fq::from(1), Fq::from(2), Fq::from(3)],
            [Fq::from(4), Fq::from(5), Fq::from(6)],
            [Fq::from(7), Fq::from(8), Fq::from(9)]
        ]);
        static ref MATRIX_A_DOT_A: Array2<Fq> = arr2(&[
            [Fq::from(30), Fq::from(36), Fq::from(42)],
            [Fq::from(66), Fq::from(81), Fq::from(96)],
            [Fq::from(102), Fq::from(126), Fq::from(150)]
        ]);
        static ref MATRIX_B: Array2<Fq> = arr2(&[
            [Fq::from(4), Fq::from(5), Fq::from(6)],
            [Fq::from(1), Fq::from(2), Fq::from(3)],
            [Fq::from(7), Fq::from(8), Fq::from(9)]
        ]);
        static ref MATRIX_B_DOT_B: Array2<Fq> = arr2(&[
            [Fq::from(63), Fq::from(78), Fq::from(93)],
            [Fq::from(27), Fq::from(33), Fq::from(39)],
            [Fq::from(99), Fq::from(123), Fq::from(147)]
        ]);
        static ref MATRIX_C: Array2<Fq> = Array2::ones((3, 3));
        static ref MATRIX_C_DOT_C: Array2<Fq> = Array::from_elem((3, 3), Fq::from(3));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq>,
        #[case] matrix_b: &Array2<Fq>,
        #[case] supposed_ab: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq>,
        #[case] b: &Array2<Fq>,
        #[case] c: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
