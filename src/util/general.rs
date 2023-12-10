/**
 * Take an array/vector of Option or Result and return an array/vector of the unwrapped values.
 * If one of the values is None or Err, return None or Err.
 */
pub fn unwrap_all_option<T>(options: Vec<Option<T>>) -> Option<Vec<T>> {
    let mut result = Vec::new();

    for option in options {
        match option {
            Some(value) => result.push(value),
            None => return None,
        }
    }

    Some(result)
}
