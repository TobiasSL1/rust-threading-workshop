use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};



fn bit_reverse(mut u: u32) -> u32
{
    let mut result : u32 = 0;
    for _i in 0..32
    {
        let  b = u & 1;
        result <<= 1;
        result |= b;
        u = u >> 1;
    }
    result
}

#[test]
fn bit_reverse_should_work()
{
    assert_eq!(0b10000000_00000000_00000000_00000000u32, bit_reverse(1));
    assert_eq!(0b01000000_00000000_00000000_00000000u32, bit_reverse(2));
    assert_eq!(0b10000000_00000000_00000000u32, bit_reverse(256));
    assert_eq!(0b10000000_00000000_u32, bit_reverse(256*256));
}
fn main() {
    //println!("Hello, world!");
    const SIZE : usize = 100_0000;

    let mut arr : [u32; SIZE] = [0; SIZE];
    for i in 0..SIZE {
        arr[i] = i as u32;
    }


    //arr.iter_mut().for_each(|v: &mut u32| *v = bit_reverse(*v));
    arr.par_iter_mut().for_each(|v: &mut u32| *v = bit_reverse(*v));  
}
