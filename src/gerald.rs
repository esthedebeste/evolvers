pub type Fitness = i32;
pub trait Gerald {
    type Context: Send + Sync;
    fn cross(a: &Self, b: &Self) -> Self;
    // runs parrallel to all other fitness functions
    fn fitness(&self, ctx: &Self::Context) -> Fitness;
}

pub fn pick_parent<'a, T>(a: &'a T, b: &'a T) -> &'a T {
    match rand::random::<bool>() {
        false => a,
        true => b,
    }
}
