use back::*;
use std::io::stdin;

#[path = "../controllers/userController.rs"] pub mod user_controller;
use user_controller::user_model::User;

fn main() {
    // User::read();

    let mut username = String::new();
    let mut pwd = String::new();

    println!("Who's the user you are looking for?");
    stdin().read_line(&mut username).unwrap();

    println!("Ok! What's your password {}?", username);
    stdin().read_line(&mut pwd).unwrap();

    let post = User::add(&username, &pwd);
    println!("\nSaved draft {} with id {}", username, post.userid);

}