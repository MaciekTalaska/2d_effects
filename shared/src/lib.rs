use std::str::FromStr;


pub fn get_image_name(args: &Vec<String>) -> String {
    let default_image = String::from(r"../resources/sky.ppm");
    // make it simple - assume first argument is bitmap name
    let image_name = match args.len() > 1 {
        true => args.get(1).unwrap().to_owned(),
        false => default_image,
    };
    if !image_name.ends_with(".ppm") {
        println!("only .ppm images are supported");
        std::process::exit(2);
    }
    image_name
}

pub fn get_option_or_default_number<T :FromStr>(args: &Vec<String>, index: usize, default: T) -> T
    where <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let result = match args.len() > index {
        true  => args.get(index).unwrap().to_owned().parse::<T>().expect("can't cast option..."),
        false => default,
    };

    result
}
