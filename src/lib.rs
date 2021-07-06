// Functions for interacting with GPX & TCX files

use std::fs::File;
use std::io::{self, BufRead};
use chrono::{DateTime,Utc};
use geoutils::Location;
use core::f64::consts::PI;

use crate::TcxLines::*;

pub struct Route {
    pub name: String,
    pub distance: f64,
    pub elevation: f64,
    pub gradient: f64,
    pub time: f64,
    pub direction: String,
}

pub struct GPX {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

pub struct TCX {
    time: DateTime<Utc>,
    gps: GPX,
    distance: f64,
}

enum TcxLines {
    Tim,
    Lat,
    Lon,
    Alt,
    Dis,
}

pub fn read_tcx(file: File) -> (String,f64,Vec<TCX>) {
    let mut tcx_points: Vec<TCX> = Vec::new();
    // let mut gpx_points: Vec<GPX> = Vec::new(); 
    let mut name: String = "".to_string();
    let mut totaltime: f64 = 0.0;
    let mut lati: f64 = 0.0;
    let mut long: f64 = 0.0;
    let mut alti: f64 = 0.0;
    let mut time: DateTime::<Utc>= Utc::now();
    let mut pt: TcxLines = Tim;
    let mut namefound = false;
    let mut totaltimefound = false;
    if let Ok(lines) = read_lines(file) {
        for line_iter in lines {   
            let line = line_iter.unwrap();      
            match pt {
                Tim => {
                    if line.find("<Name>") != None && !namefound {
                        name.push_str(&line[line.find(">").unwrap()+1..line.find("/").unwrap()-1]);
                        namefound = true;
                    }
                    if line.find("<TotalTimeSeconds>") != None && !totaltimefound {
                        totaltime = line[line.find(">").unwrap()+1..line.find("/").unwrap()-1].parse().unwrap();
                        totaltimefound = true;
                    }
                    if line.find("<Time>") != None {
                        time = DateTime::parse_from_rfc3339(&line[line.find(">").unwrap()+1..line.find("/").unwrap()-1]).unwrap().with_timezone(&Utc);
                        pt = Lat;
                    }           
                }
                Lat => {
                    if line.find("Latitude") != None {
                        lati = line[line.find(">").unwrap()+1..line.find("/").unwrap()-2].parse().unwrap();
                        pt = Lon;
                    }
                }
                Lon => {
                    if line.find("Longitude") != None {
                        long = line[line.find(">").unwrap()+1..line.find("/").unwrap()-2].parse().unwrap();
                        pt = Alt;
                    }
                }
                Alt => {
                    if line.find("Altitude") != None {
                        alti = line[line.find(">").unwrap()+1..line.find("/").unwrap()-2].parse().unwrap();
                        pt = Dis;
                    }
                }
                Dis => {
                    if line.find("Distance") != None {
                        let distance: f64 = line[line.find(">").unwrap()+1..line.find("/").unwrap()-2].parse().unwrap();
                        pt = Tim;
                        let gps = GPX{
                            latitude: lati,
                            longitude: long,
                            altitude: alti,
                        };
                        tcx_points.append(&mut vec!(TCX{time, gps, distance}));
                        // gpx_points.append(&mut vec!(GPX{latitude: lati, longitude: long, altitude: alti}));
                        lati = 0.0;
                        long = 0.0;
                        alti = 0.0;
                        time = Utc::now();
                    }
                }
            }                                                                          
        }
    }
    // println!("{}",tcx_points.len());
    // tcx_points
    // println!("{}",gpx_points.len());
    (name,totaltime,tcx_points)
}

pub fn read_gpx(file: File) -> (String,Vec<GPX>) {
    let mut gpx_points: Vec<GPX> = Vec::new();    
    let mut name: String = "".to_string();
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut pt = false;
    let mut namefound = false;
    if let Ok(lines) = read_lines(file) {
        for line_iter in lines {   
            let line = line_iter.unwrap();      
            if pt {
                let alt = line[line.find(">").unwrap()+1..line.find("/").unwrap()-2].parse().unwrap();

                gpx_points.append(&mut vec!(GPX{latitude: lat, longitude: lon, altitude: alt}));
                lat = 0.0;
                lon = 0.0;
                pt = false;
            }  else {               
                if line.find("<name>") != None && !namefound {
                    name.push_str(&line[line.find(">").unwrap()+1..line.find("/").unwrap()-1]);
                    namefound = true;
                } 
                match line.find("lat") {
                    Some(x) => {
                        match line.find("lon") {
                            Some(y) => {
                                lat = line[x+5..y-2].parse().unwrap();
                                lon = line[y+5..line.len()-2].parse().unwrap();
                                pt = true;                            
                            }
                            None => continue,
                        }
                    }
                    None => continue,
                }
            }                                    
        }
    }

    // println!("{}",gpx_points.len());
    (name,gpx_points)
}

pub fn get_distance(s: &str) -> Route {
    let mut dist: f64 = 0.0;
    let mut gain: f64 = 0.0;
    let mut grad: f64 = 0.0;
    let mut direction = "".to_string();
    match s.find("gpx") {
        Some(_) => {            
            let (name,gpx) = read_gpx(File::open(s).unwrap());
            let mut start = Location::new(gpx[0].latitude,gpx[0].longitude);
            let mut elev: f64 = gpx[0].altitude;
            let middle = Location::new(gpx[(((gpx.len()/2) as f64).floor())as usize].latitude,gpx[(((gpx.len()/2) as f64).floor())as usize].longitude);
            let start0 = start;
            let mut stop = start;
            for g in gpx {
                stop = Location::new(g.latitude,g.longitude);
                let dx = stop.distance_to(&start).unwrap().meters();
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
            let distance = (dist/10.0).round()/100.0;
            let elevation = (gain*100.0).round()/100.0;
            let time = distance/26.5*3600.0 + elevation/250.0*600.0;     
            let gradient = (elevation/grad*10000.0).round()/100.0;
            let startstop = stop.distance_to(&start0).unwrap().meters();
            let startmiddle = middle.distance_to(&start0).unwrap().meters();
            if  startstop > 2000.0 && startstop > startmiddle {
                direction.push_str(&get_direction(stop,start0));
            } else {
                direction.push_str(&get_direction(middle,start0));
            }
            Route{
                name,
                distance,
                elevation,
                gradient,
                time,
                direction,
            }
        },
        None => {
            match s.find("tcx") {
                Some(_) => {                    
                    let (name,totaltime,tcx) = read_tcx(File::open(s).unwrap());
                    let mut start = Location::new(tcx[0].gps.latitude,tcx[0].gps.longitude);
                    let middle = Location::new(tcx[(((tcx.len()/2) as f64).floor())as usize].gps.latitude,tcx[(((tcx.len()/2) as f64).floor())as usize].gps.longitude);
                    let start0 = start;
                    let mut stop = start;
                    let mut elev: f64 = tcx[0].gps.altitude;                    

                    for t in tcx {
                        stop = Location::new(t.gps.latitude,t.gps.longitude);
                        let dx = stop.distance_to(&start).unwrap().meters();
                        dist += dx;
                        // elevation gain beq .85% incline
                        if t.gps.altitude-elev >= 0.0085*dx {
                            gain += t.gps.altitude - elev;
                            grad += dx;
                        }
                        elev = t.gps.altitude;
                        start = stop;                        
                    }            
                    let elevation = (gain*100.0).round()/100.0;
                    let gradient = (elevation/grad*10000.0).round()/100.0;
                    let startstop = stop.distance_to(&start0).unwrap().meters();
                    let startmiddle = middle.distance_to(&start0).unwrap().meters();
                    if  startstop > 2000.0 && startstop > startmiddle {
                        direction.push_str(&get_direction(stop,start0));
                    } else {
                        direction.push_str(&get_direction(middle,start0));
                    }
                    Route{
                        name,
                        distance: (dist/10.0).round()/100.0,
                        elevation,
                        gradient,
                        time: totaltime,
                        direction,
                    }
                },
                None => panic!("Unknown file extension"),
            }
        }        
    }      
}

fn read_lines(file: File) -> io::Result<io::Lines<io::BufReader<File>>> {
// where P: AsRef<Path>, {
    // let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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