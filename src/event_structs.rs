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



pub  const  DAY_OFFSET :f64 = 34_200.0; // 9:30:00 AM in seconds
pub  const TRADING_DAY_SECS :usize = 23_400; // 6.5 hours in seconds   6.5 * 60 * 60
pub  const  EXPECTED_TICS:usize = 16;
pub const PRICE_MULT:f32 = 10_000.0;
pub const  BILLION: i64 = 1_000_000_000;
use std::collections::BTreeMap;
/// MuEvent is a struct to hold the microsecond resolution trade event
#[derive(Debug,Clone,PartialEq,Hash)]
pub  struct MuEvent{
    pub  u_sec:i64,
    pub price:i32,
    pub volume:i32,
}

#[derive(Debug,Clone,PartialEq,Hash)]
pub  struct Event{

    pub tics:Vec<MuEvent>,
    pub total_price:i32,
    pub total_volume:i32,
    pub tick_count:i32,
}

#[derive(Debug,Clone,PartialEq,Hash)]
pub  struct EventList{
    pub events:BTreeMap<i32,Event>,
}

impl EventList {
    pub fn new()->EventList{
        EventList{
            events:BTreeMap::new(),
        }
    }
    pub  fn update(&mut self,seconds:f64,f_price:f32,volume:i32){
        let mut idx:i32 = (seconds - DAY_OFFSET).round() as i32;
        let u_sec =( ((seconds - DAY_OFFSET) -(idx as  f64))*BILLION as f64) as i64;

        let price = (f_price* PRICE_MULT) as i32;
        if let Some(event) = self.events.get_mut(&idx){
            event.update(price,volume,u_sec);
        }else{
            self.events.insert(idx,Event::new(price,volume,u_sec));
        }
    }

    pub  fn get_volume(&self)->i32{
        let mut total_volume = 0;
        for (_,event) in self.events.iter(){
            total_volume += event.get_volume();
        }
        total_volume
    }
    pub  fn get_average_price(&self)->f32{
        let mut total_price = 0;
        let mut total_ticks = 0;
        for (_,event) in self.events.iter(){
            total_price += event.total_price;
            total_ticks += event.tick_count;
        }
        (total_price as f32/PRICE_MULT)/total_ticks as f32
    }
    pub  fn get_event_count(&self)->usize{
        let  mut total_tics = 0;
        for (_,event) in self.events.iter(){
            total_tics += event.tick_count;
        }
        total_tics as  usize
    }
    pub fn get_time_series(&self)->Vec<(f64,f32,i32)>{
        let mut time_series = Vec::with_capacity(self.get_event_count());
        for (idx,event) in self.events.iter(){
            for tic in event.tics.iter(){
                time_series.push(((idx.clone() as f64 + DAY_OFFSET + tic.u_sec as f64/BILLION as f64),tic.price as f32/PRICE_MULT,tic.volume));
            }
        }
        time_series
    }
}
impl  Event{
    pub  fn new(price:i32,volume:i32,seconds:i64)->Event{
        let  mut initial_tic = Vec::with_capacity(EXPECTED_TICS);
        initial_tic.push(MuEvent{ u_sec:seconds,price,volume});
        Event{
            tics:initial_tic,
            total_price:price,
            total_volume:volume,
            tick_count:1,
        }
    }
    // would have error checking here if this were production code  but it's not
    pub  fn update(&mut self,price:i32,volume:i32,u_sec:i64){
        self.total_price += price;
        self.total_volume += volume;
        self.tick_count += 1;
        self.tics.push(MuEvent{u_sec,price,volume});
    }
    pub  fn get_avg_price(&self)->f32{
        (self.total_price as  f32/PRICE_MULT as  f32) / self.tick_count as f32
    }
    pub  fn get_volume(&self)->i32{
        self.total_volume
    }
}

#[cfg(test)]
mod  test{
    use crate::event_structs::EventList;

    #[test]
    fn test_event_list(){
        let  el = EventList::new();
        assert_eq!(el.events.len(),0);
    }
    #[test]
    fn test_event_list_insert(){
        let mut el = EventList::new();
        el.update(34_200.1001,1.0,1);
        assert_eq!(el.events.len(),1);
    }

    #[test]
    fn test_event_list_update(){
        let mut el = EventList::new();
        el.update(34_200.1001,1.15,1);
        el.update(34_200.1201,1.257,1);
        assert_eq!(el.events.len(),1);
    }

    #[test]
    fn test_event_list_volume(){
        let mut el = EventList::new();
        el.update(34_200.1001,1.15,1);
        el.update(34_200.1201,1.257,1);
        assert_eq!(el.get_volume(),2);
        el.update(34_220.1201,1.257,5);
        assert_eq!(el.get_volume(),7);
    }
    #[test]
    fn test_event_list_average_price(){
        let mut el = EventList::new();
        el.update(34_200.1001,6.0,1);
        el.update(34_200.1201,12.0,1);
        assert_eq!(el.get_average_price(),9.0);
    }

    #[test]
    fn test_get_time_series(){
        let  mut el = EventList::new();
        el.update(34_200.1001,6.0,1);
        el.update(34_200.1201,12.0,1);
        el.update(34_200.1250,11.0,1);
        let  ans:Vec<(f64,f32,i32)> = vec![(34_200.1001,6.0,1),(34_200.1201,12.0,1),(34_200.1250,11.0,1)];
        assert_eq!(el.get_time_series(),ans);
    }
    #[test]
    fn test_get_time_series2(){
        let  mut el = EventList::new();
        el.update(34_200.1001,6.0,1);
        el.update(34_200.1201,12.5,2);
        el.update(34_200.1250,11.0,3);
        el.update(34_202.1001,6.0,4);
        el.update(34_210.1201,12.0,5);
        el.update(34_250.1250,11.0,6);
        el.update(34_250.22K,11.0,7);
        el.update(34_250.2450,11.0,8);
        el.update(34_350.3450,11.0,9);
        el.update(34_350.6450,11.0,10);
        el.update(34_450.6450,11.0,12);
        let  ans:Vec<(f64,f32,i32)> = vec![
            (34_200.1001,6.0,1),
            (34_200.1201,12.5,2),
            (34_200.1250,11.0,3),
            (34_202.1001,6.0,4),
            (34_210.1201,12.0,5),
            (34_250.1250,11.0,6),
            (34_250.22,11.0,7),
            (34_250.2450,11.0,8),
            (34_350.3450,11.0,9),
            (34_350.6450,11.0,10),
            (34_450.6450,11.0,12),
        ];
        assert_eq!(el.get_time_series(), ans);



    }
}