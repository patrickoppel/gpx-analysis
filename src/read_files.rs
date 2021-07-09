use std::fs::File;
use std::io::{self, BufRead};
use chrono::{DateTime,Utc};
use geoutils::Location;

use crate::read_files::TcxLines::*;

// pub struct GPS {
//     pub latitude: f64,
//     pub longitude: f64,
//     pub altitude: f64,
// }
pub struct GPS {
    pub location: Location,
    pub altitude: f64,
}

pub struct GPX {
    pub gps: GPS,
    pub time: DateTime<Utc>,
    pub hr: u8,
}

pub struct TCX {
    pub time: DateTime<Utc>,
    pub gps: GPS,
    pub distance: f64,
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
                        let gps = GPS{
                            location: Location::new(lati,long),                            
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
    let mut time: DateTime::<Utc>= Utc::now();
    let mut hr: u8 = 0;
    if let Ok(lines) = read_lines(file) {
        for line_iter in lines {   
            let line = line_iter.unwrap();      
            if pt {
                let alt = line[line.find(">").unwrap()+1..line.find("/").unwrap()-2].parse().unwrap();

                gpx_points.append(&mut vec!(GPX{gps:GPS{location: Location::new(lat,lon), altitude: alt},time,hr}));
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

fn read_lines(file: File) -> io::Result<io::Lines<io::BufReader<File>>> {
// where P: AsRef<Path>, {
    // let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}