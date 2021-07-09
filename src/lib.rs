// Functions for interacting with GPX & TCX files
mod read_files;

use std::fs::File;
use geoutils::Location;
use core::f64::consts::PI;

pub use read_files::*;

pub struct Route {
    pub name: String,
    pub distance: f64,
    pub elevation: f64,
    pub gradient: f64,
    pub time: f64,
    pub direction: String,
}

impl Route {
    pub fn new() -> Self {
        Route{
            name: "".to_string(),
            distance: 0.0,
            elevation: 0.0,
            gradient: 0.0,
            time: 0.0,
            direction: "".to_string(),
        }
    }

    pub fn get_info(&mut self,s: &str) -> Self {
        let mut gps: Vec<GPS> = Vec::new();
        match s.find("gpx") {
            Some(_) => {            
                let (name,gpx) = read_gpx(File::open(s).unwrap());
                for g in gpx {
                    gps.push(g.gps);
                } 
                self.gps_info(name,gps)
            },
            None => {
                match s.find("tcx") {
                    Some(_) => {                    
                        let (name,totaltime,tcx) = read_tcx(File::open(s).unwrap());
                        for t in tcx {
                            gps.push(t.gps);
                        }
                        self.gps_info(name,gps)                       
                    },
                    None => panic!("Unknown file extension"),
                }
            }        
        }               
    }

    fn gps_info(&mut self,name: String, gps: Vec<GPS>) -> Self {
        let mut dist: f64 = 0.0;
        let mut gain: f64 = 0.0;
        let mut grad: f64 = 0.0;
        let mut direction = "".to_string();

        let mut start = gps[0].location;
        let mut elev: f64 = gps[0].altitude;
        let middle = gps[(((gps.len()/2) as f64).floor())as usize].location;
        let start0 = start;
        let mut stop = start;
        let mut dx: f64 = 0.0;
        for g in gps {
            stop = g.location;
            dx = stop.distance_to(&start).unwrap().meters();
            dist += dx;
            // elevation gain beq .85% incline
            if g.altitude-elev >= 0.0085*dx {
                gain += g.altitude - elev;
                grad += dx;
            }
            elev = g.altitude;
            start = stop;                
        }
                
        // Time estimate 1h/26.5km + 10min/250m
        self.name = name;
        self.distance = (dist/10.0).round()/100.0;
        self.elevation = (gain*100.0).round()/100.0;
        self.time = self.distance/26.5*3600.0 + self.elevation/250.0*600.0;     
        self.gradient = (self.elevation/grad*10000.0).round()/100.0;

        let startstop = stop.distance_to(&start0).unwrap().meters();
        let startmiddle = middle.distance_to(&start0).unwrap().meters();
        if startstop > 2000.0 && startstop > startmiddle {
            self.direction = get_direction(stop,start0);
        } else {
            self.direction = get_direction(middle,start0);
        }
        // if  startstop > 2000.0 && startstop > startmiddle {
        //     direction.push_str(&get_direction(stop,start0));
        // } else {
        //     direction.push_str(&get_direction(middle,start0));
        // }  
        Route{
            name: self.name.clone(),
            direction: self.direction.clone(),
            distance: self.distance,
            elevation: self.elevation,
            time: self.time,
            gradient: self.gradient
        }
        // Route{
        //     name,
        //     distance,
        //     elevation,
        //     time,
        //     gradient,
        //     direction,
        // } 
    }
}

fn get_direction(stop:Location,start:Location) -> String {
    match (stop.latitude()-start.latitude()).atan2(stop.longitude()-start.longitude())/PI*8.0 {
        -8.0..=-7.0 => "W".to_string(),
        -7.0..=-5.0 => "SW".to_string(),
        -5.0..=-3.0 => "S".to_string(),
        -3.0..=-1.0 => "SE".to_string(),
        -1.0..=1.0 => "E".to_string(),
        1.0..=3.0 => "NE".to_string(),
        3.0..=5.0 => "N".to_string(),
        5.0..=7.0 => "NW".to_string(),
        7.0..=8.0 => "W".to_string(),
        _ => "N/A".to_string(),
    }   
}