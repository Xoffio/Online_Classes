use math_funcs::*;
use rand::prelude::*;

#[test]
fn it_add_rand(){
    let mut rng = rand::thread_rng();
    for i in 0..1000 {
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);

        assert_eq!(add_ops::add_two(x, y), x+y);
        assert_eq!(add_ops::add_three(x, y, i), x+y+i);
    }
}