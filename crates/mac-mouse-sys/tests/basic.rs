use mac_mouse_sys::*;

#[test]
fn resolution() {
    let res = 1900 * 65536;

    set_pointer_resolution(res).unwrap();

    let ret = get_pointer_resolution().unwrap();

    assert_eq!(ret, res);
}

#[test]
fn acceleration() {
    let prev_acc = get_mouse_acceleration().unwrap();
    let acc = 0;

    set_mouse_acceleration(acc).unwrap();

    let ret = get_mouse_acceleration().unwrap();

    assert_eq!(ret, acc);

    set_mouse_acceleration(prev_acc).unwrap();
}
