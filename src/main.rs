extern crate redis;
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    do_something();
}

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    let mut con = client.get_connection()?;

    /* do something here */
    let e1: Employee = build_employee(1);
    println!("{:?}", e1);

    Ok(())
}

#[derive(Debug)]
struct Employee {
    id: i16,
    first_name: String,
    last_name: String,
    occupation: String,
    salary: i16
}

fn build_employee(id: i16) -> Employee {

    let first = generate_name(5);
    let last = generate_name(8);

    Employee {
        id: id,
        first_name: first,
        last_name: last,
        occupation: generate_occupation(1),
        salary: generate_salary()
    }
}

fn get_email(fist_name: String, last_name: String) -> String {

    let mut email: String = String::from(first.as_str());
    email.push_str(".");
    email.push_str(last.as_str());
    email.push_str("@company.com");
    return email;
}

fn generate_name(random_num: i16) -> String {

    let rand_string: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(10)
        .collect();

    return rand_string;
}

fn generate_occupation(random_num: i16) -> String {
    return String::from("Job");
}

fn generate_salary() -> i16 {
    return 100_000;
}
