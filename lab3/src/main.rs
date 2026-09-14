#![allow(dead_code)]

#[derive(Debug, Clone)]
enum StudentStatus {
    Active,
    OnLeave,
    Suspended,
    Graduated,
}

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    status: StudentStatus,
}

fn main() {
    let student = Student {
        id: 1,
        name: String::from("Israel"),
        status: StudentStatus::Active,
    };

    println!("Student Details: {:?}", student);
}
