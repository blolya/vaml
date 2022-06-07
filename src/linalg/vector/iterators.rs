use super::Vector;

impl<T, const L: usize> std::iter::IntoIterator for Vector<T, L> {
    type Item = T;
    type IntoIter = std::array::IntoIter<Self::Item, L>;

    fn into_iter(self) -> Self::IntoIter {
       self.0.into_iter() 
    }
}

use std::mem;
impl<T, const L: usize> std::iter::FromIterator<T> for Vector<T, L> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let cells = {
            let mut data: [mem::MaybeUninit<T>; L] = unsafe {
                mem::MaybeUninit::uninit().assume_init()
            };

            data.iter_mut()
                .zip( iter )
                .for_each( |(v1, v2)| { mem::replace(v1, mem::MaybeUninit::new(v2) ); } );

            unsafe { mem::transmute_copy::<_, [T; L] >(&data) }
        }; 

        Vector::from(cells)
    }
}
