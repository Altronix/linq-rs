use crate::window::*;
use futures::executor::block_on;

#[test]
fn test_binding() {
    let mut a = Box::new(false);
    let mut b = Box::new(false);
    let mut v = WindowBuilder::new()
        .title("test")
        .size(800, 400)
        .bind("funcA", move |_, _| *a = true)
        .bind("funcB", move |_, _| *b = true)
        .build()
        .unwrap()
        .start();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    v.join_handle.unwrap().join().unwrap().unwrap();
    /*
    let _ready = block_on(async {
        v.eval("window.funcA(\"hello\").then(r => funcB(r.result))")
            .await
            .unwrap()
    });
    */
    // assert_eq!(*a, false);
    // assert_eq!(*b, false);
}
