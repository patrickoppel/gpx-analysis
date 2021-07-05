// Functions for interacting with GPX & TCX files

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use chrono::{DateTime,Utc,FixedOffset,TimeZone};
use geoutils::Location;
use std::error;
// use chrono::Utc;
// use chrono::FixedOffset;
// use chrono::TimeZone;

use crate::TcxLines::*;

// pub struct Position {
//     latitude: f64,
//     longitude: f64,
//     altitude: f64,
// }

// pub struct GPX {
//     position: Position,
// }

// pub struct TCX {
//     time: DateTime<Utc>,
//     position: Position,
//     distance: f64,    
// }

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

pub fn read_tcx(file: File) -> Vec<GPX> {
    // let mut tcx_points: Vec<TCX> = Vec::new();
    let mut gpx_points: Vec<GPX> = Vec::new(); 
    let mut lati: f64 = 0.0;
    let mut long: f64 = 0.0;
    let mut alti: f64 = 0.0;
    let mut time: DateTime::<Utc>= Utc::now();
    let mut pt: TcxLines = Tim;
    if let Ok(lines) = read_lines(file) {
        for line_iter in lines {   
            let line = line_iter.unwrap();      
            match pt {
                Tim => {
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
                        // let gps = GPX{
                        //     latitude: lati,
                        //     longitude: long,
                        //     altitude: alti,
                        // };
                        // tcx_points.append(&mut vec!(TCX{time, gps, distance}));
                        gpx_points.append(&mut vec!(GPX{latitude: lati, longitude: long, altitude: alti}));
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
    println!("{}",gpx_points.len());
    gpx_points
}

pub fn read_gpx(file: File) -> Vec<GPX> {
    let mut gpx_points: Vec<GPX> = Vec::new();    
    let mut lat: f64 = 0.0;
    let mut lon: f64 = 0.0;
    let mut pt = false;
    if let Ok(lines) = read_lines(file) {
        for line_iter in lines {   
            let line = line_iter.unwrap();      
            if pt {
                let alt = line[line.find(">").unwrap()+1..line.find("/").unwrap()-2].parse().unwrap();

                // let pos = Position{
                //     latitude: lat,
                //     longitude: lon,
                //     altitude: alt,
                // };
                gpx_points.append(&mut vec!(GPX{latitude: lat, longitude: lon, altitude: alt}));
                lat = 0.0;
                lon = 0.0;
                pt = false;
            }  else {                
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

    println!("{}",gpx_points.len());
    gpx_points
}

pub fn get_elev_gain(file:File) {
    let gpx = read_gpx(file);
    // let gpx = read_tcx(file);
    let mut elev = gpx[0].altitude;
    let mut gain: f64 = 0.0;
    for g in gpx {
        if g.altitude >= elev + 0.4 {
            gain += g.altitude - elev;
        }
        elev = g.altitude;
    }

    println!("{:?}",gain);
}

pub fn get_distance(file:File) {
    let gpx = read_gpx(file);
    let mut start = Location::new(gpx[0].latitude,gpx[0].longitude);
    let mut dist: f64 = 0.0;
    for g in gpx {
        let stop = Location::new(g.latitude,g.longitude);
        dist += stop.distance_to(&start).unwrap().meters();
        start = stop;
    }

    println!("{:?}",dist);    
}

// pub fn dist_and_elev(file:File) {
//     let gpx = match file.find(".gpx") {
//         Some(x) => read_gpx(file),
//         None => match file.find(".tcx") {
//             Some(x) => read_tcx(file),
//             None => panic!("Unknown file format"),
//         }
//     };
// }

fn read_lines(file: File) -> io::Result<io::Lines<io::BufReader<File>>> {
// where P: AsRef<Path>, {
    // let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}