pub fn names_match_loop(array:&Vec<(&str, &str)>,dane:&String)->String{
    for (x,y) in array{
        if *x == dane.as_str(){
            let z =y.to_string();
            return z
        }
    }
    "".to_string()
}