use cucumber::{given, then, when, World as _};
use cucumber_test::animals::cat::Cat;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(cucumber::World, Debug, Default)]
// Accepts both sync/async and fallible/infallible functions.
#[world(init = Self::new)]
pub struct AnimalWorld {
    cat: Cat,
}

impl AnimalWorld {
    fn new() -> Self {
        Self {
            cat: Cat {
                alive: true,
                hungry: true,
                vomiting: false,
                starving: false,
            },
        }
    }
}

#[given(regex = r"^a (alive|hungry|satiated|starving|vomiting) cat$")]
fn given_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "alive" => {
            world.cat.alive = true;
            world.cat.hungry = false;
            world.cat.vomiting = false;
            world.cat.starving = false;
        }
        "dead" => {
            world.cat.alive = false;
            world.cat.hungry = false;
            world.cat.vomiting = false;
            world.cat.starving = false;
        }
        "hungry" => {
            world.cat.alive = true;
            world.cat.hungry = true;
            world.cat.vomiting = false;
            world.cat.starving = false;
        }
        "satiated" => {
            world.cat.alive = true;
            world.cat.hungry = false;
            world.cat.vomiting = false;
            world.cat.starving = false;
        }
        "vomiting" => {
            world.cat.alive = true;
            world.cat.hungry = false;
            world.cat.vomiting = true;
            world.cat.starving = false;
        }
        "starving" => {
            world.cat.alive = true;
            world.cat.hungry = true;
            world.cat.vomiting = false;
            world.cat.starving = true;
        }
        _ => unreachable!(),
    }

    let temp_cat = &world.cat;
    println!("Setting up cat in state {state} : ");
    println!("{temp_cat}");
}

// Don't forget to additionally `use cucumber::when;`.
#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    /*     let temp_cat = &world.cat;
    println!("Feeding cat : ");
    println!("\t{temp_cat}"); */

    world.cat.feed();

    /*     let temp_cat = &world.cat;
    println!("\tBecomes : ");
    println!("\t{temp_cat}"); */
}

// Don't forget to additionally `use cucumber::when;`.
#[when("I starve the cat")]
fn starve_cat(world: &mut AnimalWorld) {
    /*     let temp_cat = &world.cat;
    println!("Starving cat : ");
    println!("\t{temp_cat}"); */

    world.cat.starve();

    /*     let temp_cat = &world.cat;
    println!("\tBecomes : ");
    println!("\t{temp_cat}"); */
}

fn negate(predicate: bool, negation: bool) -> bool {
    if negation {
        !predicate
    } else {
        predicate
    }
}

#[then(regex = r"^the cat is( not)? (alive|dead|hungry|satiated|vomiting|starving)$")]
fn then_cat(world: &mut AnimalWorld, optional: String, state: String) {
    let negation = match optional.as_str() {
        " not" => true,
        _ => false,
    };

    /*
    let temp_cat = &world.cat;
    println!("Testing predicate : \"The cat is {optional} {state}.\" on cat : ");
    println!("{temp_cat}");
    */

    match state.as_str() {
        "alive" => assert!(world.cat.alive == negate(true, negation)),
        "dead" => assert!(world.cat.alive == negate(false, negation)),
        "hungry" => assert!(world.cat.hungry == negate(true, negation)),
        "satiated" => assert!(world.cat.hungry == negate(false, negation)),
        "starving" => assert!(world.cat.starving == negate(true, negation)),
        "vomiting" => assert!(world.cat.vomiting == negate(true, negation)),
        _ => unreachable!(),
    }
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(AnimalWorld::run("tests/features/cat"));
}
