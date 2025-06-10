// Crates are not created for subfolders of tests folder.

// But the name of this module is coming from the parent folder 'common'.

// So function setup() will be referenced as common::setup() in the integration tests.

pub fn setup() {
    // Setup code goes here, e.g., initializing a database connection or setting up environment variables.
    println!("Setting up the test environment...");
}