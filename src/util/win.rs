use winapi::um::winuser;

pub fn is_async_key_pressed(key: i32) -> Result<bool, String> {
    if key < 0 || key > 255 {
        return Err(String::from("Invalid key code")); // Invalid key code
    }

    let result = unsafe { winuser::GetAsyncKeyState(key) };

    if result == 0 {
        return Ok(false);
    }

    Ok(result < 0) // The most significant bit is set if the key is down
}
