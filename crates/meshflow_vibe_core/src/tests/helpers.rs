/// Helper utilities for headless Bevy testing.
///
/// These helpers enable writing tests without requiring a windowed session,
/// which is essential for CI environments.
use bevy::app::{App, Plugin};
use std::time::Duration;

/// Creates a headless Bevy App suitable for testing.
///
/// This function sets up a minimal Bevy App with headless configuration,
/// disabling features that require a window or graphics context.
///
/// # Example
///
/// ```rust
/// let mut app = headless_app!();
/// app.add_systems(bevy::app::Update, my_system);
/// app.update();
/// ```
#[macro_export]
macro_rules! headless_app {
    () => {{
        let app = bevy::app::App::new();
        app
    }};
}

/// Runs a Bevy app for a specified number of iterations for testing.
///
/// # Arguments
///
/// * `app` - Mutable reference to the Bevy App
/// * `iterations` - Number of update iterations to run
///
/// # Example
///
/// ```rust
/// let mut app = headless_app!();
/// run_iterations(&mut app, 10);
/// ```
pub fn run_iterations(app: &mut App, iterations: usize) {
    for _ in 0..iterations {
        app.update();
    }
}

/// Runs a Bevy app for a specified duration for testing.
///
/// # Arguments
///
/// * `app` - Mutable reference to the Bevy App
/// * `duration` - Duration to run the app
///
/// # Example
///
/// ```rust
/// let mut app = headless_app!();
/// run_for_duration(&mut app, std::time::Duration::from_millis(100));
/// ```
pub fn run_for_duration(app: &mut App, duration: Duration) {
    let start = std::time::Instant::now();
    while start.elapsed() < duration {
        if app.should_exit().is_some() {
            break;
        }
        app.update();
    }
}

/// Sets up a minimal Bevy plugin for testing.
///
/// This helper allows you to add plugins to a headless app in tests.
///
/// # Example
///
/// ```rust
/// use bevy::app::Plugin;
///
/// struct TestPlugin;
/// impl Plugin for TestPlugin {
///     fn build(&self, app: &mut App) {
///         app.add_systems(bevy::app::Update, test_system);
///     }
/// }
///
/// let mut app = headless_app!();
/// add_test_plugin(&mut app, TestPlugin);
/// ```
pub fn add_test_plugin<P: Plugin>(app: &mut App, plugin: P) {
    app.add_plugins(plugin);
}

/// Creates a test system that can be added to a Bevy App.
///
/// # Example
///
/// ```rust
/// let mut app = headless_app!();
/// app.add_systems(bevy::app::Update, test_system!(|world| {
///     // test code here
/// }));
/// ```
#[macro_export]
macro_rules! test_system {
    ($body:expr) => {
        |mut world: bevy::ecs::world::World| $body(&mut world)
    };
}

/// Waits for a condition to become true with a timeout.
///
/// # Arguments
///
/// * `app` - Mutable reference to the Bevy App
/// * `condition` - Closure that returns true when the condition is met
/// * `max_iterations` - Maximum number of iterations to wait
///
/// # Returns
///
/// * `true` if the condition was met
/// * `false` if the timeout was reached
///
/// # Example
///
/// ```rust
/// let mut app = headless_app!();
/// let result = wait_for_condition(&mut app, 100, |app| {
///     app.world().get_resource::<MyResource>().is_some()
/// });
/// assert!(result);
/// ```
pub fn wait_for_condition<F>(app: &mut App, max_iterations: usize, mut condition: F) -> bool
where
    F: FnMut(&App) -> bool,
{
    for _ in 0..max_iterations {
        if condition(app) {
            return true;
        }
        app.update();
    }
    false
}

/// Helper to check if an App should exit.
pub fn app_should_exit(app: &App) -> bool {
    app.should_exit().is_some()
}

/// Creates a minimal event builder for testing.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// let mut app = headless_app!();
/// let event = create_event::<MyEvent>(&mut app);
/// ```
pub fn create_event<E: bevy::ecs::event::Event + Default>(_app: &mut App) -> E {
    E::default()
}
