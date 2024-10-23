// This function shows how you instantiate, configure and run
// the simplest possible Rocket web server.
//
// It uses macros defined in the third party "rocket" crate - as defined in cargo.toml and
// in lib.rs.
//
// The annotation and macros used below ultimately generate the main() function entry point for a binary crate.
//
// Here I've put it into a function only to avoid making a binary crate - it just shows how it's done,
// and gives me some practice. The goal atm is only to make it compile.
//

fn _foo() {
    // Anatomy of the next few lines... informed guesses
    //
    // - # is core rust syntax for an attribute
    // - get is a macro supplied by Rocket to define a route and corresponding handler.
    // - It consumes the function below - that don't think has to be called index.
    // - The handler returns a plaintext str, implying that Rocket will package and send
    //   the HTTP response.
    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    // - launch is another Rocket macro that generates the main() function.
    // - it first builds the rocket instance with the internal Rocket build() function.
    // - and then "mounts" the routes supplied at the given root path.
    // - It's not clear if the mounting starts the server listening, or
    //   if the generated main() first calls rocket(), and then later
    //   explicitly sets up a network listener.
    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![index])
    }
}
