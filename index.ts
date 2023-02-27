import init, { render } from "./pkg/webdev";

init().then(() => {
    const parser = new DOMParser();
    const doc = parser.parseFromString(
        "<root><one><oo></oo><ot></ot></one><two></two><tree><to></to></tree></root>",
        "text/xml"
    );
    const canvas = document.querySelector("canvas");

    render(doc, canvas as HTMLCanvasElement);
});
