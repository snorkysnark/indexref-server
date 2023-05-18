import { For, mergeProps, Show } from "solid-js";
import NodeLabel from "./NodeLabel";
import { NodeResourceReturn } from "./signals/server";
import { GetSet } from "./signals/getset";
import classes from "./tree.module.css";

export default function NodeTree(propsRaw: {
    nodes: NodeResourceReturn;
    selectedId: GetSet<number>;
    rootId?: number;
}) {
    const props = mergeProps({ rootId: 1 }, propsRaw);
    const nodeById = (id: number) => props.nodes.nodeById.get(id);

    return (
        <ul class={classes.dottedTree}>
            <For each={nodeById(props.rootId).children}>
                {(childId) => (
                    <li class={classes.dottedTree}>
                        <NodeLabel
                            node={nodeById(childId)}
                            selectedId={props.selectedId}
                        />
                        <Show when={nodeById(childId).children.length > 0}>
                            <NodeTree
                                nodes={props.nodes}
                                rootId={childId}
                                selectedId={props.selectedId}
                            />
                        </Show>
                    </li>
                )}
            </For>
        </ul>
    );
}
