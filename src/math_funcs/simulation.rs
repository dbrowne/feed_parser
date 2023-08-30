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
use std::str::FromStr;
use rand::Rng;
use rand::rngs::ThreadRng;
use rust_decimal::prelude::*;

use crate::event_structs::{EventList, MuEvent};
use crate::time_funcs::time_dec_string;

const  MAX_TIC_VAL:i32 = 999999999;



fn  get_mics(inp: &str) -> i32{
    let  mut usec =inp.split(".");
    _=usec.next();
    usec.next().unwrap().parse::<i32>().unwrap()
}

fn  get_whole(inp: &String) -> String{
    let  mut usec =inp.split(".");
    usec.next().unwrap().to_string()
}
pub  struct StockPriceGenerator{
    seed: ThreadRng,
    tics: EventList,
}
impl StockPriceGenerator {
    pub  fn new(tic_list:EventList) -> Self {
        let mut seed = rand::thread_rng();
        StockPriceGenerator {
            seed,
            tics: tic_list
        }
    }

    pub fn gen_new_tics(&mut self) {
        for (sec,events) in self.tics.events.clone().iter_mut()  {
            println!("at {} {:?} , full {}, usec {}",sec, events.tics.len(), events.tics[0].string_time,
            get_mics(&events.tics[0].string_time));

            let  x = self.seed.gen_bool(0.55);
            let new_tics = self.create_interval(&events.tics, x, 0.1);
            if  let  Some(tics) = new_tics {
                println!("new tics  {:?}", tics);
            }
            // if  Some(new_tics.clone()) != None {
            //     let  tics = new_tics.unwrap();
            //     println!("new tics  {:?}", tics);
            // }
        }
    }

    fn create_interval(&mut self, tics: &Vec<MuEvent>, up: bool, variance:f32)->Option<Vec<(String,String,i32)>>{
       let  tic_count = tics.len();
       let  start:i32 = get_mics(&tics[0].string_time);
       let  end:i32 = get_mics(&tics[tic_count-1].string_time);
       let  delta = end - start;
        let  mut new_size:i32 =0;
       if up {
           new_size = tic_count as i32 + (tic_count as  f32 * variance) as i32 +1;
       } else {
           new_size = tic_count as i32 - (tic_count as  f32 * variance) as i32 -1;
           if new_size==0 {
                return None;
           }
       }
        let mut new_tics:Vec<(String,String,i32)> = Vec::with_capacity(new_size as usize);
        let  mut time_incr = end / new_size;
        let  mut delta_start= time_incr / 2+self.seed.gen_range(101..time_incr);

        let mut tot_time= delta_start;
        let mut idx = 0;
        let mut ctr:i32 =0;
        let big_time = get_whole(&tics[0].string_time);
        let  mut idx =0;
        while  ctr <new_size as  i32 {
            let randTime = self.seed.gen_range((time_incr/10000)..time_incr);
            tot_time += randTime;
            let  mut volume = 0;
            let mut price:f32 = 0.0;

            if  tot_time < MAX_TIC_VAL{
                let  new_time = format!("{}.{:09}", big_time, tot_time);
                let  pp :i32 = (tics[idx].price.to_f32().unwrap()  *100.0) as  i32;
                if self.seed.gen_bool(0.6) {
                   price = (pp  + (self.seed.gen_range(0..(pp as  f32*0.01)as  i32))) as  f32;
                   volume = tics[idx].volume + self.seed.gen_range(0..tics[idx].volume);
               }else{
                     price =( pp - (self.seed.gen_range(0..(pp as  f32 *0.01)as  i32))) as f32;
                     volume = tics[idx].volume - self.seed.gen_range(0..tics[idx].volume);
               }

                if  idx < tic_count-1{
                    idx +=1;
                }
                let  f_secs = time_dec_string(&new_time).unwrap();

                let  price_string= format!("{:.2}", price/100.0);

                new_tics.push((new_time,price_string,volume));
                ctr +=1;
            } else {
                tot_time -= randTime;
                delta_start = delta_start/2;
                time_incr = time_incr/2;
            }

        }

        Some(new_tics)


    }

}


#[cfg(test)]
mod  test{
    use crate::event_structs::Event;
    use crate::math_funcs::simulation::StockPriceGenerator;
    use crate::event_structs::EventList;
    #[test]
    fn t_001(){
    let mut el:EventList = EventList::new();
        let _ = el.update("09:20:00.491720704", "6.0", 1);
        let _ = el.update("09:20:00.491720705", "12.0", 2);
        let _ = el.update("09:20:00.491730704", "11.37", 3);
        let _ = el.update("09:21:00.491730707", "11.38", 4);
        let _ = el.update("09:21:02.491720704", "6.0", 5);
        let _ = el.update("09:21:02.491720705", "12.0", 6);
        let _ = el.update("09:21:02.491720706", "12.0", 7);
        let _ = el.update("09:21:02.491720709", "12.0", 8);
        let _ = el.update("09:21:02.491720715", "12.0", 9);
        let _ = el.update("09:21:04.491730704", "11.37", 10);
        let _ = el.update("09:21:05.491730707", "11.38", 11);
        let _ = el.update("09:22:05.492730708", "11.38", 12);
        let _ = el.update("09:23:05.496730709", "11.38", 13);
        let _ = el.update("09:22:05.501730708", "11.38", 14);
        let _ = el.update("09:23:05.591730709", "11.38", 15);
        let _ = el.update("09:22:05.891730708", "11.38", 16);
        let _ = el.update("09:23:05.891780709", "11.38", 17);
        let _ = el.update("09:22:05.991730708", "11.38", 18);
        let _ = el.update("09:23:05.999730709", "11.38", 19);
        let _ = el.update("15:23:05.999999999", "11.38", 19);

        let  mut spg = StockPriceGenerator::new(el);
        spg.gen_new_tics();

    }
}