#![allow(dead_code)]

#[derive(Debug)]
enum DoorState {
    Closed,
    Open
}

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    HasArrived(Floor),
    Door(DoorState),
    ButtonPressed(Floor, Direction),
    ButtonPressedHere(Floor),
}

type Floor = i32;

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::HasArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::Door(DoorState::Open)
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::Door(DoorState::Closed)
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(floor, dir)
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressedHere(floor)
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
