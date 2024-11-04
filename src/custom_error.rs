use thiserror::Error;

#[derive(error, Debug)]
pub enum Errors {
	#[error("This is a testing error for the badvoxels engine.")]
	TestError,
}