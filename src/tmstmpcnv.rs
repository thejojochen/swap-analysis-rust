pub fn convert_timestamp(fromSwapData: String) -> String {
    //target format: Nov-14-2022
    //current format: 14.11.2022 20:35

    
    let mut v: Vec<&str> = fromSwapData.split(|c| c == '.' || c == ' ' || c == ':').collect();
    //println!("{:?}", v);

    //if (v[3] )
    let hour: i32 = v[3].parse::<i32>().unwrap();
    let mut day: i32 = v[0].parse::<i32>().unwrap();
    if (hour + 8 > 24) {
        let new_day = &day + 1; //to do: account for end of month 
        let new_string: &str = &new_day.to_string();
        v[0] = new_string;
        //println!("new day: {}", v[0]);

        match v[1] {
            "01" => return buildString("Jan", v[0], v[2]),
            "02" => return buildString("Feb", v[0], v[2]),
            "03" => return buildString("Mar", v[0], v[2]),
            "04" => return buildString("Apr", v[0], v[2]),
            "05" => return buildString("May", v[0], v[2]),
            "06" => return buildString("Jun", v[0], v[2]),
            "07" => return buildString("Jul", v[0], v[2]),
            "08" => return buildString("Aug", v[0], v[2]),
            "09" => return buildString("Sep", v[0], v[2]),
            "10" => return buildString("Oct", v[0], v[2]),
            "11" => return buildString("Nov", v[0], v[2]),
            "12" => return buildString("Dec", v[0], v[2]),
            _ => println!("bad month"),
        }

    } else { match v[1] {
        "01" => return buildString("Jan", v[0], v[2]),
        "02" => return buildString("Feb", v[0], v[2]),
        "03" => return buildString("Mar", v[0], v[2]),
        "04" => return buildString("Apr", v[0], v[2]),
        "05" => return buildString("May", v[0], v[2]),
        "06" => return buildString("Jun", v[0], v[2]),
        "07" => return buildString("Jul", v[0], v[2]),
        "08" => return buildString("Aug", v[0], v[2]),
        "09" => return buildString("Sep", v[0], v[2]),
        "10" => return buildString("Oct", v[0], v[2]),
        "11" => return buildString("Nov", v[0], v[2]),
        "12" => return buildString("Dec", v[0], v[2]),
        _ => println!("bad month"),
    }}

    return String::from("invlaid");
}

pub fn buildString(month: &str, day: &str, year: &str) -> String {
    let dash: &str = "-";
    let together = format!("{}{}{}{}{}", month, dash, day, dash, year);
    return together;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
