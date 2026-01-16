pub mod admin;
mod company;
pub mod dto;
mod generic_user;
mod student;
mod university;

pub use company::Company;
pub use generic_user::GenericUser;
pub use student::Student;
pub use university::University;
