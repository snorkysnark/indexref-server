import { createEffect, createMemo, createSignal, onCleanup } from "solid-js";
import React from "react";
import { createRoot } from "react-dom/client";
import { defineDataType, JsonViewer, stringType } from "@textea/json-viewer";

// Hack to use the JsonViewer react component in solid

const linkType = defineDataType({
    ...stringType,
    is(value) {
        if (typeof value !== "string") return false;
        try {
            new URL(value);
        } catch {
            return false;
        }
        return true;
    },
    Component: (props) => {
        const url = props.value.toString();
        return React.createElement(
            "a",
            { href: url, target: "_blank", class: "link" },
            url
        );
    },
});

export default function JsonView(props: { value: object }) {
    const [container, setContainer] = createSignal<HTMLElement>();
    const root = createMemo(() => {
        if (container()) {
            const myRoot = createRoot(container());
            onCleanup(() => myRoot.unmount());

            return myRoot;
        }
    });

    createEffect(() => {
        if (root()) {
            root().render(
                React.createElement(JsonViewer, {
                    value: props.value,
                    valueTypes: [linkType],
                    displayDataTypes: false,
                    rootName: false,
                    displaySize: false,
                })
            );
        }
    });

    return <div class="w-full h-full text-lg" ref={setContainer}></div>;
}
