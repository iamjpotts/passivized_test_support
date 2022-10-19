use http::StatusCode;

pub fn equals(status: StatusCode) -> Equals {
    Equals::new(status)
}

pub fn is_success() -> Success {
    Success{}
}

pub trait StatusTest {
    fn test(&self, status: StatusCode) -> bool;
}

#[derive(Copy, Clone)]
pub struct Equals {
    expected: StatusCode
}

impl Equals {
    fn new(expected: StatusCode) -> Self {
        Self {
            expected
        }
    }
}

impl StatusTest for Equals {
    fn test(&self, status: StatusCode) -> bool {
        status == self.expected
    }
}

#[derive(Copy, Clone)]
pub struct Success {}

impl StatusTest for Success {
    fn test(&self, status: StatusCode) -> bool {
        status.is_success()
    }
}