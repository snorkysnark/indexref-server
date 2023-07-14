import { createEffect, createSignal } from "solid-js";
import NodeIcon from "./NodeIcon";
import { GetSet } from "./signals/getset";
import { NodeWithChildren } from "./signals/server";

export default function NodeLabel(props: {
    node: NodeWithChildren;
    selectedId: GetSet<number>;
}) {
    const selected = () => props.selectedId.get() === props.node.id;
    const [element, setElement] = createSignal<HTMLElement>();

    createEffect(() => {
        if (selected() && element()) {
            element().scrollIntoView({ block: "nearest" });
        }
    });

    return (
        <p
            class="select-none overflow-x-hidden whitespace-nowrap \
            text-ellipsis cursor-pointer"
            classList={{
                "bg-white hover:bg-blue-200": !selected(),
                "bg-blue-300": selected(),
            }}
            onClick={[props.selectedId.set, props.node.id]}
            ref={setElement}
        >
            <NodeIcon node={props.node} />
            <span class="h-[1lh] pl-2">{props.node.title}</span>
        </p>
    );
}
