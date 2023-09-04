



trait UsernameWidget { fn get(&self) ->String;}
trait AgeWidget {fn get(&self) -> u8;}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "ldd".to_owned(),
        age: 18,
    };

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("ldd".to_owned(),username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(18,age);
}