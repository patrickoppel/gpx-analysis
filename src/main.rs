use std::fs;
use std::io::Error;
use gpx_project::get_distance;

fn main() -> Result<(),Error> {
    let paths = fs::read_dir("./files").unwrap();

    let mut st: String = "Name,Distance,Elevation Gain\n".to_string();

    for path in paths {
        // println!("{:?}",path.unwrap().path());
        let out = get_distance(path.unwrap().path().to_str().unwrap());        
        
        st.push_str(&out.name);
        st.push(',');
        st.push_str(&out.distance.to_string());
        st.push(',');
        st.push_str(&out.elevation.to_string());
        st.push('\n')
    }
    Ok(fs::write("./GPX-files.csv",st)?)
}