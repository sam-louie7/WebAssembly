
const rust = import('./pkg/test_bindgen');
rust.then(func => {
    func.run()
});

