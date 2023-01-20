pub struct MxC<const ROW: usize, const COL: usize> {
    pub size: usize,
    pub data: [[f64; COL]; ROW],
}

impl<const ROW: usize, const COL: usize> MxC<ROW, COL> {
    pub fn idty<D: SquareMx>() -> MxC<ROW, COL> {
        MxC {
            size: ROW * COL,
            data: [[0.0; COL]; ROW],
        }
    }
}

pub struct Mx {
    pub size: usize,
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Mx {
    pub fn idt<D: SquareMx>() -> Mx {
        let (rows, cols) = D::dim();
        Mx {
            size: rows * cols,
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }
}

pub trait SquareMx {
    fn dim() -> (usize, usize);
}

macro_rules! impl_dim {
    ($($nm:ident, $dim:literal),*) => {
        $(
            #[allow(non_camel_case_types)]
            pub struct $nm {}

            impl SquareMx for $nm {
                fn dim() -> (usize, usize) {
                    ($dim, $dim)
                }
            }
        )*
    };
    // ($row:literal, $col:literal) => {
    //     impl Dim for ($row, $col) {
    //         fn dim() -> (usize, usize) {
    //             ($row, $col)
    //         }
    //     }
    // };
}
impl_dim!(x2, 2, x3, 3, x4, 4);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identity() {
        let mx = Mx::idt::<x3>();
        assert_eq!(mx.size, 9);
        assert_eq!(mx.rows, 3);
        assert_eq!(mx.cols, 3);
        assert_eq!(mx.data, [[0.0; 3]; 3]);
    }
}
