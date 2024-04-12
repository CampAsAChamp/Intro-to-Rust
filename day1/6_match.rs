fn main() {
    let sub_code = "PHY";

    let suject = match sub_code {
        "PHY" => "Physics",
        "CHM" => "Chemistry",
        "MAT" => "Mathematics",
        _ => "Other",
    };

    println!("The subject is {}", suject);
}
