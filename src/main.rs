use std::fs::File;
use std::io::Error;
use gpx_project::read_gpx;
use gpx_project::read_tcx;
use gpx_project::get_elev_gain;
use gpx_project::get_distance;

fn main() -> Result<(),Error> {
    // let file = File::open("./125k M7 loop.gpx")?;
    // Ok(read_gpx(file))

    // let file2 = File::open("./125k M7 loop.tcx")?;
    // Ok(read_tcx(file2))

    let file3 = File::open("./125k M7 loop.gpx")?;
    // Ok(get_elev_gain(file3))
    Ok(get_distance(file3))
}
