import init, { run } from "./pkg/webdev.js";

const parser = new DOMParser();

init().then(() => {
    const doc = parser.parseFromString("<root><one></one><two></two></root>", "text/xml");

    for (const child of doc.children) {
        console.log(child.nodeName);
    }

    run(doc);
});
