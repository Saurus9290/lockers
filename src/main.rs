struct lockers {
    name_student: String,
    locker_assigned: Option<i32>
}

enum Nested {
    Op(Option<String>)
}

impl Nested {
    fn new(s:&str) -> Self  {
        if s.len() >=3 {
            Self::Op(None)
        } else {
            Self::Op(Some(s.to_owned()))
        }
    }
}

fn main() {
    let nest = Nested::new("go");

    match nest {
        Nested::Op(data) => {
            match data {
                Some(data) => println!("the data is {}", data ),
                None => println!("length is too short")
            }
        }

    }
    }