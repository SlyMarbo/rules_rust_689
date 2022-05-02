mod generated {
    include!(env!("GENERATED_RS"));
}
mod source;

pub use generated::GENERATED;
pub use source::SOURCE;
