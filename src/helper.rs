use mac_mouse_sys::*;

const MAX_SEN: i32 = 1990;
const MIN_SEN: i32 = 10;
const MAX_ACC: i32 = 10000000;
const MIN_ACC: i32 = 0;

pub fn res_to_sen(res: i32) -> i32 {
    2000 - res / 65536
}

pub fn sen_to_res(sen: i32) -> i32 {
    (2000 - sen) * 65536
}

pub fn read_mouse_cfg() -> Result<(i32, i32), String> {
    let res = get_pointer_resolution()?;
    let sen = if res == -1 { -1 } else { res_to_sen(res) };
    let acc = get_mouse_acceleration()?;

    Ok((sen, acc))
}

pub fn write_mouse_cfg(sen: i32, acc: i32) -> Result<(), String> {
    if !(MIN_SEN..=MAX_SEN).contains(&sen) {
        return Err(format!(
            "Invalid sensitivity value: {}, must be in range {}..={}",
            sen, MIN_SEN, MAX_SEN
        ));
    }

    if !(MIN_ACC..=MAX_ACC).contains(&acc) {
        return Err(format!(
            "Invalid acceleration value: {}, must be in range {}..={}",
            acc, MIN_ACC, MAX_ACC
        ));
    }

    set_pointer_resolution(sen_to_res(sen))?;
    set_mouse_acceleration(acc)?;

    Ok(())
}
