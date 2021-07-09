use std::fs;
use std::io::Error;
use gpx_project::Route;

fn main() -> Result<(),Error> {
    let paths = fs::read_dir("./files/routes/").unwrap();

    let mut st: String = "Name,Distance,Elevation Gain,Average Gradient,Total Time\n".to_string();

    for path in paths {
        // println!("{:?}",path.unwrap().path());
        let mut out = Route::new();
        out.get_info(path.unwrap().path().to_str().unwrap());        
        
        st.push_str(&out.name);
        st.push(',');
        st.push_str(&out.distance.to_string());
        st.push(',');
        st.push_str(&out.elevation.to_string());
        st.push(',');
        st.push_str(&out.gradient.to_string());
        st.push(',');
        if out.time != 0.0 {
            if out.time >= 3600.0 {
                let h = (out.time/3600.0).floor();
                out.time -= h*3600.0;
                st.push_str(&h.to_string());                
                st.push(':');
            }
            if out.time >= 60.0 {
                let min = (out.time/60.0).floor();
                out.time -= min*60.0;
                st.push_str(&min.to_string());
                st.push(':');
            }
            st.push_str(&out.time.round().to_string());
        }
        st.push(',');
        st.push_str(&out.direction);
        st.push('\n')
    }
    Ok(fs::write("./GPX-files.csv",st)?)
}