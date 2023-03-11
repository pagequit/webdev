import init, { render } from "./pkg/webdev";

init().then(() => {
    const parser = new DOMParser();
    const doc = parser.parseFromString(`
        <root>
            <one>
                <oo></oo>
                <ot>
                    <oto></oto>
                </ot>
            </one>
            <two>
                <to></to>
            </two>
            <tree>
                <tro>
                    <troo></troo>
                </tro>
                <trt></trt>
            </tree>
            <four></four>
        </root>
        `, "text/xml"
    );
    const canvas = document.querySelector("canvas");

    render(doc, canvas as HTMLCanvasElement);
});
