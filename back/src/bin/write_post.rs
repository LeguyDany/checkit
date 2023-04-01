use back::*;
use std::io::stdin;

#[path = "../controllers/userController.rs"] pub mod user_controller;
use user_controller::user_model::User;

fn main() {
    let connection = &mut establish_connection();

    User::read();

    let mut username = String::new();
    let mut pwd = String::new();

    println!("What would you like your username to be?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    println!("Ok! What's your password {}?", username);
    stdin().read_line(&mut pwd).unwrap();

    let post = User::add(connection, username, &pwd);
    println!("\nSaved draft {} with id {}", username, post.userid);
}