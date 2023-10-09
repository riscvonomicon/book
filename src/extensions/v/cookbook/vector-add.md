# Array Element-Wise Addition

```rust
fn array_addition(mut x: &[u8], mut y: &[u8], result: &mut [u8]) {
    assert!(x.len() == y.len());
    assert!(x.len() == result.len());

    let mut ax = &[];
    let mut ay = &[];
    let mut aresult = &mut [];

    let mut vl = x.len();
    
    loop {
        let avl = setvli(vl, Element::E8, LMul::M1);

        (cx, x) = x.split_at(avl);
        (cy, y) = y.split_at(avl);
        (cresult, result) = result.split_at_mut(avl);

        let vx = vle8_vv(ax, avl);
        let vy = vle8_vv(ay, avl);

        let vresult = vadd_vv(vx, vy, avl);

        vse8_vv(vresult, aresult);

        vl -= avl;
    }
}
```