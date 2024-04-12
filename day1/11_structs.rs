struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    fn years(&self) -> u32 {
        return self.age * 2;
    }
}

fn main() {
    let mut emp1 = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 25,
    };

    println!(
        "Name: {}, Company: {}, Age: {}",
        emp1.name, emp1.company, emp1.age
    );

    emp1.name = String::from("Jane");
    emp1.company = String::from("Apple");
    emp1.age = 30;

    println!(
        "Name: {}, Company: {}, Age: {}",
        emp1.name, emp1.company, emp1.age
    );

    // display(emp1);

    println!("{}", emp1.years());
}

fn display(emp: Employee) {
    println!(
        "Name is {}, Company is {}, and Age is {}",
        emp.name, emp.company, emp.age
    );
}
