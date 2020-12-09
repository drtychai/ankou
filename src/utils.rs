//pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Returns an uninitialized SimpleLogger 
///
/// By simpling calling `<self::get_logger>.init().unwrap()` we
/// can instantiate a logger in the calling scope. 
pub fn get_logger() -> crate::simple_logger::SimpleLogger {
    crate::simple_logger::SimpleLogger::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        assert_eq!(self::get_logger().init().unwrap(), ());
    }

}
