struct Person {
    name: String,
    citizenship: String, 
    age: i32, 
    gender: char, 
    salary: f32
}

struct Student {
    name_std: String,
    age: i32,
    gender: char,
    country: String
}

// implementation block is to simplify the code 
// can be used to define the functions of a structure
impl Person {

    //init function - can be named anything but recommendation is to call it new
    fn new () -> Self { // Self means returning an instance of the struct itself
        Person {
            name: String::from(""),
            citizenship: String::from("India"), 
            age: 18, 
            gender: 'm',
            salary: 0.0
        }
    }  

    fn compute_taxes(&self) -> f32 { // &self is the instance of the calling struct
        (self.salary / 3. ) * 0.5 // 3. is equivalent to 3.0
    }
}

// traits are like interfaces where you define the functions which needs to be implemented on the relevant structs following the trait

trait GeneralInfo {
    fn info(&self) -> (&str, i32, char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, i32, char) {
        (&self.name, self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, i32, char) {
        (&self.name_std, self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.country
    }
}

fn main() {
    let person1 = Person{ name: String::from("Vaibhav"), citizenship: String::from("India"), age: 33, gender: 'm', salary: 10_000.0 };
    println!("{}, {}, {}, {}, {}", person1.name, person1.citizenship, person1.age, person1.gender, person1.salary);

    let taxes = person1.compute_taxes(); // or could be also used as Person::compute_taxes(&person1)
    println!("Taxes: {}", taxes);

    let person2 = Person::new();
    println!("{}, {}, {}, {}, {}", person2.name, person2.citizenship, person2.age, person2.gender, person2.salary);

    // let person3 = Person{
    //     name: String::from("test"),
    //     age: 10,
    //     ..person1
    // };
    // println!("{}, {}, {}, {}, {}", person3.name, person3.citizenship, person3.age, person3.gender, person3.salary);


    // all these are immutable instances so values cannot be changed
    let mut person4 = Person::new();
    person4.name = String::from("test1");
    println!("{}, {}, {}, {}, {}", person4.name, person4.citizenship, person4.age, person4.gender, person4.salary);

    let student1 = Student {
        name_std: String::from("St 1"),
        age: 17,
        gender: 'm',
        country: String::from("India")
    };

    println!("{:?}", person1.info());
    println!("{:?}", student1.info());
}


