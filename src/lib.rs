
//! allowing non snake case for matrix variables A x B = C, which is the
//! correct mathematical notation !!
//! boolean matrix
//!
//! An data structure that handles boolean matrix operations.
//! The boolean values are stored bitwise.
//!
//! # Layout
//!
//! Creating a matrix is done within the submodule booleanmatrix
//! We have addition,multiplication and setting and getting specific indexes.
mod depgraph;

use crate::booleanmatrix::Matrix;

#[allow(non_snake_case)]
pub mod booleanmatrix {

    pub struct Matrix{
        pub n: usize,
        pub m: usize,
        bits:Vec<u64>,
    }


    // n rows, m columns
    // matrix indexed in flat array style index = i*m + j
    pub fn NewMatrix(n:usize,m:usize) -> Matrix {
        let size = getvectorsize(n * m);
        let mut avector = Vec::new();
        avector.resize(size,0u64);
        let A = Matrix {
            n,
            m,
            bits: avector,
        };
        // A.Print();
        return A;
    }

    //return the minimum number of slots needed to fit n*m bits onto the size of selected integer type.
    fn getvectorsize(size:usize) -> usize {
        let bitsize :usize = std::mem::size_of::<u64>() * 8;
        let slots :usize = (size / bitsize) + std::cmp::min(size % bitsize,1);
        return slots
    }

    impl Matrix{


        pub fn multiply(self: & Self,B:Matrix) -> Matrix {

            if self.m != B.n {
                panic!("cols and rows need to match" );
            }

            // The matrix product has the dim = (self rows x B cols)
            let mut C = NewMatrix(self.n,B.m);

            for i in 1..self.n+1{
                let mut sum = false;
                for j in 1..B.m+1{
                    for k in 1..B.n+1{
                        sum = sum || self.get(i,k) && B.get(k,j);
                    }
                    C.set(i,j,sum);
                }
            }

            return C;
        }

        pub fn addition(self: &Self, B:Matrix) -> Matrix{
            if (self.n != B.n) || (self.m != B.m) {
                panic!("matrix dimensions need to match" );
            }

            let mut C = NewMatrix(B.n,B.m);
            for i in 1..B.n+1{
                for j in 1..B.m+1{
                    C.set(i,j,self.get(i,j) || B.get(i,j));
                }
            }

            return C;
        }

       /*fn checkrange(this: &Self, i:usize,j:usize){
            if ( (i < 0) || i > (this.n - 1)) || (j < 0 || j > (this.m - 1)) {
                panic!(" index out of range ");
            }
        }*/

        pub fn set(self: &mut Self,mut i:usize,mut j:usize,val:bool) {
            i -= 1;
            j -= 1;

            // Self::checkrange(self,i,j);

            let bitsperslot :usize = std::mem::size_of::<u64>() * 8 ;
            let index = (i * self.m) + j; // matrix mapped onto array of n+m bits + .
            let slot = index / bitsperslot; // we have x number of bits per slot in array

            let thebit = index % bitsperslot;
            let bitnumber :u64 = thebit.try_into().unwrap();

            if val == true {
                self.bits[slot] = self.bits[slot] | (1 << bitnumber);
            } else {
                let mask:u64 =  !(1 << bitnumber);
                self.bits[slot] &= mask;
            }
        }

        pub fn get(self: &Self, mut i:usize,mut j:usize) -> bool{
            i -= 1;
            j -= 1;

            // Self::checkrange(self,i,j);

            let bitsperslot :usize = std::mem::size_of::<u64>() * 8 ;

            let index: usize = (i * self. m) + j;
            let bitindex:usize = index / bitsperslot;
            let bitnumber:u64 = (index % bitsperslot).try_into().unwrap();
            let bitar:u64 = self.bits[bitindex];

            return bitar & (1 << bitnumber) > 0;

        }

        pub fn print(self: &Self){
            for i in 1..self.n+1{
                for j in 1..self.m+1{
                    if Self::get(self,i,j){
                        print!("1");
                    }else{
                        print!("0");
                    }
                }
                println!();
            }
        }

        pub fn transpose(self: &Self) -> Matrix{
            let mut C = NewMatrix(self.m,self.n);
            for i in 1..self.n+1{
                for j in 1..self.m+1{
                    C.set(j,i,self.get(i,j));
                }
            }
            return C;
        }

    }

}

//implements multiply trait for overloading * operator
#[allow(non_snake_case)]
impl std::ops::Mul for booleanmatrix::Matrix {
    type Output = Matrix;
    fn mul(self, B: Matrix) -> Matrix {
        return self.multiply(B);
    }
}

//implements add trait for overloading + operator
#[allow(non_snake_case)]
impl std::ops::Add for booleanmatrix::Matrix {
    type Output = Matrix;
    fn add(self, B: Matrix) -> Matrix {
        return self.addition(B);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests{
    use crate::booleanmatrix::{Matrix, NewMatrix};
    // use super::*;

    #[test]
    fn test_matrix_init(){
        let A: Matrix = NewMatrix(10,10);
        assert_eq!(A.n,10);
        assert_eq!(A.m,10);
    }

    #[test]
    fn test_set_get() -> Result<(),String>{

        let mut A: Matrix = NewMatrix(10,10);
        for i in 1..11{
            for j in (1 .. 10).step_by(2){
                A.set(i,j,true);
            }
        }

        for i in 1..11{
            for j in (1 .. 10).step_by(2){
                let val = A.get(i,j);
                assert!(val,"index {} and {} should return true ",i,j);
            }
        }
        Ok(())
    }


    #[test]
    fn test_multiply(){
        // the values of A
        let mut A: Matrix = NewMatrix(3,3);
        A.set(1,1,true);
        A.set(1,2,true);
        A.set(1,3,true);
        A.set(2,1,true);
        A.set(2,3,true);
        A.set(3,2,true);

        // The values of B
        let mut B: Matrix = NewMatrix(3,3);

        B.set(1,1,true);
        B.set(2,3,true);
        B.set(3,1,true);
        B.set(3,2,true);
        B.set(3,3,true);

        // Expected result from A x B
        let mut result: Matrix = NewMatrix(3,3);
        result.set(1,1,true);
        result.set(1,2,true);
        result.set(1,3,true);
        result.set(2,1,true);
        result.set(2,2,true);
        result.set(2,3,true);
        result.set(3,3,true);

        let D = A.multiply(B);

        // Test result of  A x B

        for i in 1 .. 4 {
            for j  in 1 .. 4 {
                assert_eq!(result.get(i,j),D.get(i,j)," testing the result of a matrix multiplication on boolean matrices") ;
            }
        }
    }

    #[test]
    fn test_new_dim(){
        let A = NewMatrix(1,3);
        let B = NewMatrix(3,1);

        let C = A.multiply(B);

        assert_eq!(C.n,1, "testing new dimensions from a matrix multiplication {} expected 1",C.n);
        assert_eq!(C.m,1, "testing new dimensions from a matrix multiplication {} expected 1",C.m);
    }

    #[test]
    #[should_panic]
    fn test_invalid_matrix_mul(){
        let A = NewMatrix(3,4);
        let B = NewMatrix(5,6);

        A.multiply(B);
    }

    //test panic unwind closure
    #[test]
    fn test_invalid_matrix_mul2(){
        let A = NewMatrix(3,4);
        let B = NewMatrix(5,6);

        let result = std::panic::catch_unwind(|| {
            A.multiply(B);
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_overloaded_mul(){
        // the values of A
        let mut A: Matrix = NewMatrix(3,3);
        A.set(1,1,true);
        A.set(1,2,true);
        A.set(1,3,true);
        A.set(2,1,true);
        A.set(2,3,true);
        A.set(3,2,true);

        // The values of B
        let mut B: Matrix = NewMatrix(3,3);

        B.set(1,1,true);
        B.set(2,3,true);
        B.set(3,1,true);
        B.set(3,2,true);
        B.set(3,3,true);

        // Expected result from A x B
        let mut result: Matrix = NewMatrix(3,3);
        result.set(1,1,true);
        result.set(1,2,true);
        result.set(1,3,true);
        result.set(2,1,true);
        result.set(2,2,true);
        result.set(2,3,true);
        result.set(3,3,true);

        let D = A * B;

        for i in 1 .. 4 {
            for j  in 1 .. 4 {
                assert_eq!(result.get(i,j),D.get(i,j)," testing the result of a matrix multiplication on boolean matrices") ;
            }
        }
    }

    #[test]
    fn test_add(){

        let mut A: Matrix = NewMatrix(4,4);
        A.set(1,1,false);
        A.set(1,2,false);
        A.set(1,3,true);
        A.set(1,4,false);

        A.set(2,1,false);
        A.set(2,2,false);
        A.set(2,3,false);
        A.set(2,4,true);

        A.set(3,1,false);
        A.set(3,2,false);
        A.set(3,3,true);
        A.set(3,4,true);

        A.set(4,1,false);
        A.set(4,2,false);
        A.set(4,3,false);
        A.set(4,4,false);

        let mut B: Matrix = NewMatrix(4,4);
        B.set(1,1,false);
        B.set(1,2,true);
        B.set(1,3,false);
        B.set(1,4,false);

        B.set(2,1,true);
        B.set(2,2,false);
        B.set(2,3,false);
        B.set(2,4,false);

        B.set(3,1,false);
        B.set(3,2,false);
        B.set(3,3,true);
        B.set(3,4,false);

        B.set(4,1,false);
        B.set(4,2,true);
        B.set(4,3,false);
        B.set(4,4,false);

        let mut R: Matrix = NewMatrix(4,4);
        R.set(1,1,false);
        R.set(1,2,true);
        R.set(1,3,true);
        R.set(1,4,false);

        R.set(2,1,true);
        R.set(2,2,false);
        R.set(2,3,false);
        R.set(2,4,true);

        R.set(3,1,false);
        R.set(3,2,false);
        R.set(3,3,true);
        R.set(3,4,true);

        R.set(4,1,false);
        R.set(4,2,true);
        R.set(4,3,false);
        R.set(4,4,false);

        let C = A.addition(B);

        for i in 1 .. 5 {
            for j  in 1 .. 5 {
                assert_eq!(R.get(i,j),C.get(i,j)," testing the result of a matrix addition on boolean matrices") ;
            }
        }
    }

    #[test]
    fn test_overloaded_add(){

            let mut A: Matrix = NewMatrix(4,4);

            A.set(1,1,false);
            A.set(1,2,false);
            A.set(1,3,true);
            A.set(1,4,false);

            A.set(2,1,false);
            A.set(2,2,false);
            A.set(2,3,false);
            A.set(2,4,true);

            A.set(3,1,false);
            A.set(3,2,false);
            A.set(3,3,true);
            A.set(3,4,true);

            A.set(4,1,false);
            A.set(4,2,false);
            A.set(4,3,false);
            A.set(4,4,false);

            let mut B: Matrix = NewMatrix(4,4);
            B.set(1,1,false);
            B.set(1,2,true);
            B.set(1,3,false);
            B.set(1,4,false);

            B.set(2,1,true);
            B.set(2,2,false);
            B.set(2,3,false);
            B.set(2,4,false);

            B.set(3,1,false);
            B.set(3,2,false);
            B.set(3,3,true);
            B.set(3,4,false);

            B.set(4,1,false);
            B.set(4,2,true);
            B.set(4,3,false);
            B.set(4,4,false);

            let mut R: Matrix = NewMatrix(4,4);

            R.set(1,1,false);
            R.set(1,2,true);
            R.set(1,3,true);
            R.set(1,4,false);

            R.set(2,1,true);
            R.set(2,2,false);
            R.set(2,3,false);
            R.set(2,4,true);

            R.set(3,1,false);
            R.set(3,2,false);
            R.set(3,3,true);
            R.set(3,4,true);

            R.set(4,1,false);
            R.set(4,2,true);
            R.set(4,3,false);
            R.set(4,4,false);

            let C = A + B;

            for i in 1 .. 5 {
                for j  in 1 .. 5 {
                    assert_eq!(R.get(i,j),C.get(i,j)," testing the result of a matrix addition on boolean matrices") ;
                }
            }
    }

    #[test]
    fn test_transpose(){
        let mut A: Matrix = NewMatrix(4,4);
        A.set(1,1,false);
        A.set(1,2,false);
        A.set(1,3,true);
        A.set(1,4,false);

        A.set(2,1,false);
        A.set(2,2,false);
        A.set(2,3,false);
        A.set(2,4,true);

        A.set(3,1,false);
        A.set(3,2,false);
        A.set(3,3,true);
        A.set(3,4,true);

        A.set(4,1,false);
        A.set(4,2,false);
        A.set(4,3,false);
        A.set(4,4,false);

        let mut R: Matrix = NewMatrix(4,4);
        R.set(1,1,false);
        R.set(1,2,false);
        R.set(1,3,false);
        R.set(1,4,false);

        R.set(2,1,false);
        R.set(2,2,false);
        R.set(2,3,false);
        R.set(2,4,false);

        R.set(3,1,true);
        R.set(3,2,false);
        R.set(3,3,true);
        R.set(3,4,false);

        R.set(4,1,false);
        R.set(4,2,true);
        R.set(4,3,true);
        R.set(4,4,false);

        let C = A.transpose();

        for i in 1 .. 5 {
            for j  in 1 .. 5 {
                assert_eq!(R.get(i,j),C.get(i,j)," testing the result of a matrix transpose on boolean matrices") ;
            }
        }
    }

}



