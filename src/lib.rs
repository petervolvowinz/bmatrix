
//! boolean matrix
//!
//! An data structure that handles boolean matrix operations.
//! The boolean values are stored bitwise.
//!
//! # Layout
//!
//! Creating a matrix is done within the submodule booleanmatrix
//! We have addition,multiplication and setting and getting specific indexes.

pub mod booleanmatrix {

    pub struct Matrix{
        pub n: usize,
        pub m: usize,
        bits:Vec<u64>,
    }


    // n rows, m columns
    // matrix indexed in flat array style index = i*m + j
    pub fn NewMatrix(n:usize,m:usize) -> Matrix {
        let zeros: u64 = 0;
        let size = getvectorsize(n * m);
        let mut avector = Vec::new();
        avector.resize(size,0u64);
        let mut A = Matrix {
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


        pub fn Multiply(B:Matrix) -> Matrix {
            let mut C = Matrix{
                n : B.n,
                m : B.m,
                bits : Vec::new()
            };
            return C;
        }

        pub fn Add(B:Matrix) -> Matrix{
            let mut C = Matrix{
                n : B.n,
                m : B.m,
                bits : Vec::new()
            };
            return C;
        }

        fn checkrange(this: &Self, i:usize,j:usize){
            if ( (i < 0) || i > (this.n - 1)) || (j < 0 || j > (this.m - 1)) {
                panic!(" index out of range ");
            }
        }

        pub fn Set(self: &mut Self,mut i:usize,mut j:usize,val:bool) {
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

        pub fn Get(self: &Self, mut i:usize,mut j:usize) -> bool{
            i -= 1;
            j -= 1;

            Self::checkrange(self,i,j);

            let bitsperslot :usize = std::mem::size_of::<u64>() * 8 ;

            let index: usize = (i * self. m) + j;
            let bitindex:usize = index / bitsperslot;
            let bitnumber:u64 = (index % bitsperslot).try_into().unwrap();
            let bitar:u64 = self.bits[bitindex];

            return bitar & (1 << bitnumber) > 0;

        }

        pub fn Print(self: &Self){
            for i in 1..self.n+1{
                for j in 1..self.m+1{
                    if Self::Get(self,i,j){
                        print!("1");
                    }else{
                        print!("0");
                    }
                }
                println!();
            }
        }

    }

}



