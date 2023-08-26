/*
 *
 *  *
 *  *
 *  *
 *  *
 *  * MIT License
 *  * Copyright (c) 2023. Dwight J. Browne
 *  * dwight[-dot-]browne[-at-]dwightjbrowne[-dot-]com
 *  *
 *  *
 *  * Permission is hereby granted, free of charge, to any person obtaining a copy
 *  * of this software and associated documentation files (the "Software"), to deal
 *  * in the Software without restriction, including without limitation the rights
 *  * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *  * copies of the Software, and to permit persons to whom the Software is
 *  * furnished to do so, subject to the following conditions:
 *  *
 *  * The above copyright notice and this permission notice shall be included in all
 *  * copies or substantial portions of the Software.
 *  *
 *  * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *  * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  * SOFTWARE.
 *
 */

extern crate rand;

use rand::Rng;
use rand::rngs::ThreadRng;

struct StockPriceGenerator{
    seed: ThreadRng,
    init_price: f64,
    end_price: f64,
    tot_time: f64,
    mu: f64,
    sigma: f64,
    steps: usize,
}

impl StockPriceGenerator{
    fn new(init_price: f64, end_price: f64, tot_time: f64, mu: f64, sigma: f64, steps: usize) -> Self{
        let seed = rand::thread_rng();
        StockPriceGenerator{
            seed,
            init_price,
            end_price,
            tot_time,
            mu,
            sigma,
            steps,
        }
    }

    pub fn generate(&mut self) -> Vec<f64> {
        let mut price_path = Vec::with_capacity(self.steps);
        let dt = self.tot_time / self.steps as f64;
        let mut s = self.init_price;

        for _ in 0..self.steps {
            let z: f64 = self.seed.sample(rand_distr::StandardNormal);
            s *= 1.0 + self.mu * dt + self.sigma * z * (dt.sqrt());
            price_path.push(s);
        }

        price_path
    }

    pub  fn generate_intra(&mut self) ->Vec<f64> {
        let  mut price_path = Vec::with_capacity(self.steps);
        let  dt = self.tot_time / self.steps as f64;
        let  mut s = self.init_price;
        for _ in 0..self.steps{
            let z :f64 = self.seed.sample(rand_distr::StandardNormal);
            s *= 1.0 + self.mu * dt + self.sigma * z * (dt.sqrt());
            price_path.push(s);
        }
        price_path
    }
}

#[cfg(test)]
mod  test{
    use crate::math_funcs::simulation::StockPriceGenerator;

    #[test]
    fn t_001(){
        let  mut gen = StockPriceGenerator::new(100.0, 105.25, 1.0, 0.2, 0.1, 100);
        println!("{:?}", gen.generate());

        println!("{:?}", gen.generate_intra());

    }
}