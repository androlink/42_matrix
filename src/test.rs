#[cfg(test)]
mod matrix {
    use crate::matrix::Matrix;

    #[test]
    fn test_add() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[8.0, 6.0], [1.0, 6.0]].into();
        m1.add(&m2);
        assert_eq!(m1, res, "add {:?} and {:?}", "lol", "lol");
    }
    #[test]
    fn test_sub() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[-6.0, -2.0], [5.0, 2.0]].into();
        m1.sub(&m2);
        assert_eq!(m1, res, "sub {:?} and {:?}", m1, m2);
    }

    #[test]
    fn test_scale() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let res = [[2.0, 4.0], [6.0, 8.0]].into();
        m1.scl(2.);
        assert_eq!(m1, res, "scale {:?} by {:?}", m1, 2.);
    }
    #[test]
    fn add_assign_test() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[8.0, 6.0], [1.0, 6.0]].into();
        m1 += m2;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn add_test() {
        let m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[8.0, 6.0], [1.0, 6.0]].into();
        let result = m1 + m2;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }

    #[test]
    fn sub_assign_test() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[-6.0, -2.0], [5.0, 2.0]].into();
        m1 -= m2;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn sub_test() {
        let m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[-6.0, -2.0], [5.0, 2.0]].into();
        let result = m1 - m2;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }

    #[test]
    fn mul_assign_test() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let scalar = 2.;
        let res = [[2., 4.], [6., 8.]].into();
        m1 *= scalar;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn mul_test() {
        let m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let scalar = 2.;
        let res = [[2., 4.], [6., 8.]].into();
        let result = m1 * scalar;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }

    #[test]
    fn div_assign_test() {
        let mut m1 = Matrix::from([[2., 4.], [6., 8.]]);
        let scalar = 2.;
        let res = [[1., 2.], [3., 4.]].into();
        m1 /= scalar;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn div_test() {
        let m1 = Matrix::from([[2., 4.], [6., 8.]]);
        let scalar = 2.;
        let res = [[1., 2.], [3., 4.]].into();
        let result = m1 / scalar;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }
}

#[cfg(test)]
mod vector {
    use crate::vector::Vector;

    #[test]
    fn test_add() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        v1.add(&v2);
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }

    #[test]
    fn test_sub() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        v1.sub(&v2);
        assert_eq!(v1, res, "sub {:?} and {:?}", v1, res);
    }

    #[test]
    fn test_scale() {
        let mut v1 = Vector::from([2., 3.]);
        let res = Vector::from([4., 6.]);
        v1.scl(2.);
        assert_eq!(v1, res, "scale {:?} and {:?}", v1, res);
    }

    #[test]
    fn add_assign_test() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        v1 += v2;
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }
    #[test]
    fn add_test() {
        let v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        let result = v1 + v2;
        assert_eq!(result, res, "add {:?} and {:?}", v1, res);
    }

    #[test]
    fn sub_assign_test() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        v1 -= v2;
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }
    #[test]
    fn sub_test() {
        let v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        let result = v1 - v2;
        assert_eq!(result, res, "add {:?} and {:?}", v1, res);
    }

    #[test]
    fn mul_assign_test() {
        let mut v1 = Vector::from([2., 3.]);
        let scalar = 2.;
        let res = Vector::from([4., 6.]);
        v1 *= scalar;
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }
    #[test]
    fn mul_test() {
        let v1: Vector<f32, 2> = Vector::from([2., 3.]);
        let scalar: f32 = 2.;
        let res = Vector::from([4., 6.]);
        let result = v1 * scalar;
        assert_eq!(result, res, "add {:?} and {:?}", v1, res);
    }

    #[test]
    fn test_norm_1() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(
            u.norm_1(),
            0.0,
            "assert {:?}, receive {:?}",
            0.0,
            u.norm_1()
        );
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(
            u.norm_1(),
            6.0,
            "assert {:?}, receive {:?}",
            6.0,
            u.norm_1()
        );
        let u = Vector::from([-1., -2.]);
        assert_eq!(
            u.norm_1(),
            3.0,
            "assert {:?}, receive {:?}",
            3.0,
            u.norm_1()
        );
    }

    #[test]
    fn test_norm() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm(), 0.0, "assert {:?}, receive {:?}", 0.0, u.norm());
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(
            u.norm(),
            3.74165738,
            "assert {:?}, receive {:?}",
            3.74165738,
            u.norm()
        );
        let u = Vector::from([-1., -2.]);
        assert_eq!(
            u.norm(),
            2.236067977,
            "assert {:?}, receive {:?}",
            2.236067977,
            u.norm()
        );
    }

    #[test]
    fn test_norm_inf() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(
            u.norm_inf(),
            0.0,
            "assert {:?}, receive {:?}",
            0.0,
            u.norm_inf()
        );
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(
            u.norm_inf(),
            3.0,
            "assert {:?}, receive {:?}",
            3.0,
            u.norm_inf()
        );
        let u = Vector::from([-1., -2.]);
        assert_eq!(
            u.norm_inf(),
            2.0,
            "assert {:?}, receive {:?}",
            2.0,
            u.norm_inf()
        );
    }

    #[test]
    fn test_dot() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        let res = u.dot(v);
        assert_eq!(res, 0.0, "assert {:?}, receive {:?}", 0.0, res);
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        let res = u.dot(v);
        assert_eq!(res, 2.0, "assert {:?}, receive {:?}", 2.0, res);
        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        let res = u.dot(v);
        assert_eq!(res, 9.0, "assert {:?}, receive {:?}", 9.0, res);
    }
}

#[cfg(test)]
mod lerp {
    use crate::linear_interpolation::lerp;
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn test_linear_interpolation() {}

    #[test]
    fn test_basic() {
        assert_eq!(
            lerp(0., 1., 0.),
            0.,
            "testing {:?} and {:?}",
            [0., 1., 0.],
            0.
        );
        assert_eq!(
            lerp(0., 1., 1.),
            1.,
            "testing {:?} and {:?}",
            [0., 1., 1.],
            1.
        );
        assert_eq!(
            lerp(0., 1., 0.5),
            0.5,
            "testing {:?} and {:?}",
            [0., 1., 0.5],
            0.5
        );
        assert_eq!(
            lerp(21., 42., 0.3),
            27.3,
            "testing {:?} and {:?}",
            [21., 42., 0.3],
            27.3
        );
    }

    #[test]
    fn test_vector() {
        let v1 = Vector::from([2., 1.]);
        let v2 = Vector::from([4., 2.]);
        let scalar = 0.3;
        let res = lerp(v1, v2, scalar);
        let wanted = [2.6, 1.3].into();
        assert_eq!(
            res, wanted,
            "testing {:?}, {:?}, {:?} and {:?}",
            v1, v2, scalar, wanted
        );
    }

    #[test]
    fn test_matrix() {
        let v1 = Matrix::from([[2., 1.], [3., 4.]]);
        let v2 = Matrix::from([[20., 10.], [30., 40.]]);
        let scalar = 0.5;
        let res = lerp(v1, v2, scalar);
        let wanted = [[11., 5.5], [16.5, 22.]].into();
        assert_eq!(
            res, wanted,
            "testing {:?}, {:?}, {:?} and {:?}",
            v1, v2, scalar, wanted
        );
    }
}

#[cfg(test)]
mod linear_combination {
    use crate::linear_combination::linear_combination;
    use crate::vector::Vector;

    #[test]
    fn test_basic() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        let assert_res1 = Vector::from([10., -2., 0.5]);
        let assert_res2 = Vector::from([10., 0., 230.]);

        let res = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);
        assert_eq!(res, assert_res1, "{:?} not {:?}", res, assert_res1);
        let res = linear_combination(&[v1, v2], &[10., -2.]);
        assert_eq!(res, assert_res2, "{:?} not {:?}", res, assert_res2);
    }
}

#[cfg(test)]
mod cross_product {
    use crate::cross_product::cross_product;
    use crate::vector::Vector;

    #[test]
    fn test_basic() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        assert_eq!(cross_product(u, v), [0., 1., 0.].into());
        // [0.]
        // [1.]
        // [0.]
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(cross_product(u, v), [-3., 6., -3.].into());
        // [-3.]
        // [6.]
        // [-3.]
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        assert_eq!(cross_product(u, v), [17., -58., -16.].into());
        // [17.]
        // [-58.]
        // [-16.]
    }
}

#[cfg(test)]
mod cosinus {
    use crate::cos::angle_cos;
    use crate::vector::Vector;
    use assert_float_eq::assert_float_relative_eq;

    #[test]
    fn test_cos() {
        let u = Vector::from([0., 1.]);
        let v = Vector::from([0., 1.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 1.0);

        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 0.);

        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, -1.);

        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 1.);

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 0.974631846);
    }
}
