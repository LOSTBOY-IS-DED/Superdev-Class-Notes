#[derive(Debug)]. // Debug is a custom derive proc macro
// It automatically implements the Debug trait for the User struct,
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Subhajit"),
        age: 25,
    };
    println!("{:?}", user); // declarative macro
}

#[post("/users/")] // attribute like proc macro
fn create_user() {
    sqlx.query("INSERT INTO users (name, age) VALUES ($1, $2)") // function-like proc macro
        .bind("Subhajit")
        .bind(25)
        .execute(&pool)
        .await
        .unwrap();
}
