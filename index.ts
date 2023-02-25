import init, { render } from "./pkg/webdev.js";

const parser = new DOMParser();

function render_t(collection: HTMLCollection) {
    for (const element of collection) {
        console.log(element.nodeName);

        if (element.children.length > 0) {
            render_t(element.children);
        }
    }
}

init().then(() => {
    const canvas = document.querySelector("canvas");
    const doc = parser.parseFromString("<root><one></one><two></two></root>", "text/xml");

    render_t(doc.children);

    render(doc, canvas as HTMLCanvasElement);
});
