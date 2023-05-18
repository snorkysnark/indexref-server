import jsonview from "@pgrabovets/json-view";
import { createEffect, createMemo, createSignal, onCleanup } from "solid-js";

export default function JsonView(props: { value: object | string }) {
    const tree = createMemo(() => {
        const treeObj = jsonview.create(props.value);
        onCleanup(() => {
            jsonview.destroy(treeObj);
        });

        return treeObj;
    });

    const [container, setContainer] = createSignal<HTMLElement>();
    createEffect(() => {
        if (tree() && container()) {
            jsonview.render(tree(), container());
            jsonview.expand(tree());
        }
    });

    return (
        <div class="w-full h-full" ref={setContainer}></div>
    )
}
