use wasm_bindgen_test::*;
use wasm_promises_example::*;

#[wasm_bindgen_test]
async fn it_works() {
    let app = Context::new();
    let fut1 = app.load();
    let fut2 = app.load();
    let fut3 = app.load_with_delta(2);

    assert_eq!(1, fut1.await);
    assert_eq!(2, fut2.await);
    assert_eq!(4, fut3.await);
}
