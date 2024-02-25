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
                hungry: true,
                vomit: false,
                starved: false,
            },
        }
    }
}

#[given(regex = r"^a (hungry|satiated|starved) cat$")]
fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        "starved" => world.cat.starved = true,
        _ => unreachable!(),
    }
}

// Don't forget to additionally `use cucumber::when;`.
#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

// Don't forget to additionally `use cucumber::then;`.
#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}

#[then("the cat vomits")]
fn cat_is_over_full(world: &mut AnimalWorld) {
    assert!(world.cat.vomit);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(AnimalWorld::run("tests/features/cat"));
}
