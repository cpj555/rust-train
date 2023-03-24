pub fn sum(ns: &[u32]) -> Option<u32> {
    let mut s: u32 = 0;
    let mut overflow = false;
    for n in ns {
        match s.checked_add(*n) {
            None => {
                // println!("{}", n);
                overflow = true;
                break;
            }
            Some(v) => {
                println!("{}", v);
                s = v;
            }
        }
    }
    if overflow {
        return None;
    }

    return Some(s);
}