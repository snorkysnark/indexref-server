import { For, mergeProps } from "solid-js";
import NodeLabel from "./NodeLabel";
import { NodeResourceReturn } from "./server";
import classes from "./tree.module.css";

export default function NodeTree(propsRaw: {
    nodes: NodeResourceReturn;
    rootId?: number;
}) {
    const props = mergeProps({ rootId: 1 }, propsRaw);
    const nodeById = (id: number) => props.nodes.nodeById.get(id);

    return (
        <ul class={classes.dottedTree}>
            <For each={nodeById(props.rootId).children}>
                {(childId) => (
                    <li class={classes.dottedTree}>
                        <NodeLabel node={nodeById(childId)} />
                        <NodeTree nodes={props.nodes} rootId={childId} />
                    </li>
                )}
            </For>
        </ul>
    );
}
