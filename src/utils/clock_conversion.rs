use regex::Regex;

use crate::error::Result;

pub fn from_human_readable(v: String) -> Result<usize> {
    let regex = Regex::new("(\\d+):(\\d+)(\\.(\\d+))?").unwrap();
    if let Some(caps) = regex.captures(&v) {
        let min = caps.get(1).unwrap().as_str().parse::<usize>()?;
        let sec = caps.get(2).unwrap().as_str().parse::<usize>()?;
        let mut par = 0;
        if let Some(m) = caps.get(4) {
            let par_pad = format!("{}000", m.as_str());
            par = par_pad[0..3].parse::<usize>()?;
        }

        return Ok(((sec + (min * 60)) * 1000) + par);
    }

    Ok(v.parse::<usize>()?)
}

pub fn to_human_readable(v: usize) -> String {
    let minutes = v / 1000 / 60;
    let seconds = (v / 1000) % 60;
    let partial = v % 1000;

    if partial == 0 {
        format!("{}:{:02}", minutes, seconds)
    } else {
        format!("{}:{:02}.{:02}", minutes, seconds, partial)
    }
}