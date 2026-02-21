use uix::uix;

#[test]
fn test_call() {
    #![allow(non_snake_case)]
    fn Call<F>(what: F)
    where
        F: Fn(usize),
    {
        what(10)
    }

    fn Log<T: std::fmt::Debug>(what: T) {
        println!("{:?}", what)
    }

    uix! (
        <Call>
            @|_| <Log>{10}</Log>
        </Call>
    );
}
