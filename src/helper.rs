use mac_mouse_sys::*;

const MAX_SEN: i32 = 1990;

pub fn res_to_sen(res: i32) -> i32 {
    res / 65536
}

pub fn sen_to_res(sen: i32) -> i32 {
    sen * 65536
}

pub fn read_mouse_cfg() -> Result<(i32, i32), String> {
    let sen = res_to_sen(get_pointer_resolution()?);
    let acc = get_mouse_acceleration()?;

    Ok((sen, acc))
}

pub fn write_mouse_cfg(sen: i32, acc: i32) -> Result<(), String> {
    if !(0..=MAX_SEN).contains(&sen) {
        return Err(format!("Invalid sensitivity value: {}", sen));
    }

    set_pointer_resolution(sen_to_res(sen))?;
    set_mouse_acceleration(acc)?;

    Ok(())
}
