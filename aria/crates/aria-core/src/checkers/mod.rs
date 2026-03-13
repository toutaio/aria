pub mod schema;
pub mod naming;
pub mod graph_checks;
pub mod composition;

pub use schema::check_schema;
pub use naming::check_naming;
pub use graph_checks::{check_cycles, check_cross_domain, check_type_compatibility};
pub use composition::check_stale_generated_files;
