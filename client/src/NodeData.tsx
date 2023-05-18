import { createResource, Show } from "solid-js";
import JsonView from "./JsonView";
import { getNodeData } from "./signals/server";

export default function NodeData(props: { nodeId: number }) {
    const [data] = createResource(() => props.nodeId, getNodeData);

    return (
        <Show when={data()}>
            <JsonView value={data()} />
        </Show>
    )
}
