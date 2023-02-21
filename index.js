import init, { run } from "./pkg/webdev.js";
init().then(() => {
	run("WebAssembly");
});
