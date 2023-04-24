use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::gerald::{Fitness, Gerald};

pub struct WithFitness<T: Gerald> {
    pub fitness: Fitness,
    pub gerald: T,
}

// some caching code so that geralds don't have to recompute their fitness (or try to cache it themselves)
impl<T: Gerald> WithFitness<T> {
    fn new(gerald: T) -> Self {
        Self {
            fitness: Default::default(),
            gerald,
        }
    }
    fn update(&mut self, ctx: &T::Context) {
        self.fitness = self.gerald.fitness(ctx);
    }
}

pub struct Pool<T: Gerald + Send + Sync> {
    geralds: Vec<WithFitness<T>>,
    ctx: T::Context,
}

impl<T: Gerald + Send + Sync> Pool<T> {
    pub fn new<Gen: Fn(&T::Context, usize) -> T>(gen: Gen, amount: usize, ctx: T::Context) -> Self {
        Self {
            geralds: (0..amount)
                .map(|i| WithFitness::new(gen(&ctx, i)))
                .collect(),
            ctx,
        }
    }

    pub fn run(&mut self) {
        self.geralds
            .par_iter_mut()
            .for_each(|g| g.update(&self.ctx));
    }

    pub fn cross(&mut self) {
        let min_fitness = self
            .geralds
            .iter()
            .min_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap()
            .fitness;
        self.geralds = (0..self.geralds.len())
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                let a = self
                    .geralds
                    .choose_weighted(&mut rng, |f| f.fitness - min_fitness)
                    .unwrap();
                let b = self
                    .geralds
                    .choose_weighted(&mut rng, |f| f.fitness - min_fitness)
                    .unwrap();
                let c = T::cross(&a.gerald, &b.gerald);
                WithFitness::new(c)
            })
            .collect::<Vec<_>>();
    }

    pub fn best(&self) -> &WithFitness<T> {
        self.geralds.iter().max_by_key(|f| f.fitness).unwrap()
    }
}
