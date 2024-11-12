pub mod app;
pub mod router;
pub mod testmod;
// pub mod tutorial_app;

pub use self::app::App;
pub use self::router::{AppRoute, switch};
pub use self::testmod::{Main};
// pub use self::router::CheckElement;
// pub use self::tutorial_app::app;

