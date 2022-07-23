import("./app").then(() => {
    console.log("loaded...");
});

const rust = import('../pkg/index');
rust.then(func => {
    func.run()
});


