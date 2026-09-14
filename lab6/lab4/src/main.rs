trait Summary {
    fn summarize(&self) -> String;
}

struct CourseReport {
    title: String,
    score: u32,
}

impl Summary for CourseReport {
    fn summarize(&self) -> String {
        format!("Course: {}, Score: {}", self.title, self.score);
    }
}

fn get_lab_score(completed: bool) -> Result<u32, &'static str> {
    if completed {
        Ok(100)
    } else {
        Err("Lab incomplete")
    }
}

fn main() {
    let report = CourseReport {
        title: String::from("Rust Development"),
        score: 98,
    };
    println!("{}", report.summarize());

    match get_lab_score(true) {
        Ok(score) => println!("Score retrieved: {}", score),
        Err(e) => println!("Error: {}", e),
    }
}
