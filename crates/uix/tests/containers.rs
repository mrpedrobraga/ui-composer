use uix::uix;

#[test]
fn test_containers() {
    let v = uix!(
        <>
            {1}
            {2}
            {3}
        </>
    );

    dbg!(v);
}