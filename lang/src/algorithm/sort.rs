pub fn quick_sort(v: &mut Vec<i32>) -> Vec<i32> {
    if v.len() <= 1 {
        return v.clone();
    }

    let pviot = v.pop();
    let pviot: i32 = match pviot {
        Some(i) => i,
        _ => panic!("crash!")
    };

    let mut greater = vec![];
    let mut lesser = vec![];

    for i in v {
        let x = *i;
        if x > pviot {
            greater.push(x);
        } else {
            lesser.push(x);
        }
    }

    let right = quick_sort(&mut greater);
    let mut left = quick_sort(&mut lesser);
    left.extend(vec![pviot]);
    left.extend(right);

    left
}

fn main() {
    let mut vec = vec![3, 2, 1];
    let r = sort::quick_sort(&mut vec);
    for i in r {
        print!("{}", i);
    }
}