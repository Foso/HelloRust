type Int = i32;

type Boolean = bool;


struct Person {
    name: String,
    age: Int,
}

impl Person {
    fn is_adult(&self) -> Boolean {
        return self.age > 18;
    }
}


fn main() {
    let mut f_as_variable: fn() -> String = fn_as_variable;
    let _integer: Int = 3;

    let mut x = [1, 2, 3];
    x[0] = 2;

    println!("{}", x[0]);


    println!("Hello, world!");
    println!("{}", f_as_variable());

    f_as_variable = fn_as_variable2;
    println!("{}", f_as_variable());

    my_function();

    let person = Person { age: 32, name: "Peter".to_string() };
    my_print("Hello, world!");


    check_adult(person)

}

fn check_adult(person: Person) -> () {
    if person.is_adult() {
        let tt = "Person: is an Adult";
        my_print(tt);
    } else {
        let tt = "Person: is not an Adult";
        my_print(tt);
    }
}


fn my_print(text: &str) {
    println!("{}", text);
}

fn my_function() {
    println!("Hello, world! from my_function");
}

fn fn_as_variable() -> String {
    return "fn_as_variable".to_string();
}

fn fn_as_variable2() -> String {
    return "fn_as_variable2".to_string();
}