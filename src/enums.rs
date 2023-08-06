enum Movement {
    UP,
    DOWN,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::UP => println!("Moving up!"),
        Movement::DOWN => println!("Moving down!"),
    }
}

pub fn run() {
    let avatar = Movement::UP;
    let avatar2 = Movement::DOWN;

    move_avatar(avatar);
    move_avatar(avatar2);
}
