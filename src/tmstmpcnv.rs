use chrono::{DateTime, TimeZone, Utc};

pub fn convert_timestamp(unix_time: i64) -> String {
    //target format: Nov-14-2022
    //current format: 14.11.2022 20:35


    let readable_stamp = Utc.timestamp_opt(unix_time, 0).unwrap().to_string();
    let v: Vec<&str> = readable_stamp.split(|c| c == ' ' || c == '-' || c == ':').collect();
    //println!("{:?}", &v);


    // let hour: i32 = v[3].parse::<i32>().unwrap();
    // let mut day: i32 = v[0].parse::<i32>().unwrap();
    // if (hour + 8 > 24) {
    //     let new_day = &day + 1; //to do: account for end of month 
    //     let new_string: &str = &new_day.to_string();
    //     v[0] = new_string;
    //     //println!("new day: {}", v[0]);

    match v[1] {
        "01" => return build_string("Jan", v[2], v[0]),
        "02" => return build_string("Feb", v[2], v[0]),
        "03" => return build_string("Mar", v[2], v[0]),
        "04" => return build_string("Apr", v[2], v[0]),
        "05" => return build_string("May", v[2], v[0]),
        "06" => return build_string("Jun", v[2], v[0]),
        "07" => return build_string("Jul", v[2], v[0]),
        "08" => return build_string("Aug", v[2], v[0]),
        "09" => return build_string("Sep", v[2], v[0]),
        "10" => return build_string("Oct", v[2], v[0]),
        "11" => return build_string("Nov", v[2], v[0]),
        "12" => return build_string("Dec", v[2], v[0]),
        _ => println!("invalid month"),
    }

    return String::from("issue with building string"); //handle with error
}

pub fn build_string(month: &str, day: &str, year: &str) -> String {
    let dash: &str = "-";
    let together = format!("{}{}{}{}{}", month, dash, day, dash, year);
    return together;
}

fn print_type_of<T>(_: &T) { // utility (unused)
    println!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {
    #[test]
    fn timestamp_conversion() {
        use super::*;
        assert_eq!(convert_timestamp(1657552276), "Jul-11-2022");
        //assert_eq!(Utc.timestamp_opt(1431648000, 0).unwrap().to_string(), "2015-05-15 00:00:00 UTC");
    }
}
