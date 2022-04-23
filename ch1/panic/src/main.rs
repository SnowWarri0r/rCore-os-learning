fn main() {
    slip(50);
}

fn slip(n: i16) {
    if n == 0 {
        panic!("ahh");
    }
    if n >> 1 & 1 == 1 {
        slip(n >> 1);
    } else {
        slap(n >> 1);
    }
}
fn slap(n: i16) {
    if n == 0 {
        panic!("ahh");
    }
    if n >> 1 & 1 == 1 {
        slip(n >> 1);
    } else {
        slap(n >> 1);
    }
}
