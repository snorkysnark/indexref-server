import { createEffect, createSignal } from "solid-js";
import NodeIcon from "./NodeIcon";
import { GetSet } from "./signals/getset";
import { NodeRel } from "./signals/server";

export default function NodeLabel(props: {
    node: NodeRel;
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
            class="h-[1lh] select-none overflow-hidden whitespace-nowrap \
            text-ellipsis cursor-pointer"
            classList={{
                "bg-white hover:bg-blue-200": !selected(),
                "bg-blue-300": selected(),
            }}
            onClick={[props.selectedId.set, props.node.id]}
            ref={setElement}
        >
            <NodeIcon node={props.node} />
            <span class="pl-2">{props.node.title}</span>
        </p>
    );
}
