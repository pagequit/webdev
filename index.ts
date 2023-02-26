import init, { render } from "./pkg/webdev";

init().then(() => {
    const parser = new DOMParser();
    const doc = parser.parseFromString(
        "<root><one></one><two></two></root>",
        "text/xml"
    );
    const canvas = document.querySelector("canvas");

    render(doc, canvas as HTMLCanvasElement);
});
