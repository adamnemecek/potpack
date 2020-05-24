use potpack::prelude::*;

fn main() {
    let mut input = vec![];
    for i in (0..10000).into_iter() {
        let h = (i%10)*10;
        let w = i as _;
        input.push(SizedItem {
            id: i.into(),
            w,
            h: h as _
        })
    }

    let now = std::time::Instant::now();
    let packing = PotPack::new(&input);

    let res = now.elapsed();
    println!("{}", res.as_millis());
    println!("width: {}, height: {}, fill: {}", packing.packing.w, packing.packing.h, packing.packing.fill);
}

