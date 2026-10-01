use cucumber::{given, then, when, World as _};
use cucumber_test::animals::cat::Cat;

// `World` is your shared, mutable state.
// Cucumber constructs a fresh one via `Default::default()` for each scenario,
// so no state leaks from one scenario to the next.
#[derive(cucumber::World, Debug, Default)]
pub struct AnimalWorld {
    cat: Cat,
}

#[given(regex = r"^an? (alive|dead|hungry|satiated|starving|vomiting) cat$")]
fn given_cat(world: &mut AnimalWorld, state: String) {
    world.cat = match state.as_str() {
        "alive" | "satiated" => Cat::default(),
        "dead" => Cat {
            alive: false,
            hungry: false,
            vomiting: false,
            starving: false,
        },
        "hungry" => Cat {
            alive: true,
            hungry: true,
            vomiting: false,
            starving: false,
        },
        "starving" => Cat {
            alive: true,
            hungry: true,
            vomiting: false,
            starving: true,
        },
        "vomiting" => Cat {
            alive: true,
            hungry: false,
            vomiting: true,
            starving: false,
        },
        other => panic!("Unknown cat state: {other}"),
    };

    println!("Setting up a {state} cat: {}", world.cat);
}

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
    println!("Fed the cat: {}", world.cat);
}

#[when("I starve the cat")]
fn starve_cat(world: &mut AnimalWorld) {
    world.cat.starve();
    println!("Starved the cat: {}", world.cat);
}

#[then(regex = r"^the cat is( not)? (alive|dead|hungry|satiated|vomiting|starving)$")]
fn then_cat(world: &mut AnimalWorld, negation: String, state: String) {
    let expected = negation.is_empty();
    let cat = &world.cat;

    let actual = match state.as_str() {
        "alive" => cat.alive,
        "dead" => !cat.alive,
        "hungry" => cat.hungry,
        "satiated" => !cat.hungry,
        "vomiting" => cat.vomiting,
        "starving" => cat.starving,
        other => panic!("Unknown cat state: {other}"),
    };

    assert_eq!(
        actual, expected,
        "Expected the cat to be{negation} {state}, but got: {cat}"
    );
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    //
    // Scenarios run one at a time (like Cucumber-JVM's default), so the cat
    // states printed by the steps are not interleaved between scenarios.
    futures::executor::block_on(
        AnimalWorld::cucumber()
            .max_concurrent_scenarios(1)
            .run_and_exit("tests/features/cat"),
    );
}
