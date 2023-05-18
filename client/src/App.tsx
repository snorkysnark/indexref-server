import { createSignal, Show } from "solid-js";
import NodeData from "./NodeData";
import NodeTree from "./NodeTree";
import { getSet } from "./signals/getset";
import { createNodes } from "./signals/server";

export default function App() {
    const [nodes] = createNodes();
    const selectedId = getSet(createSignal<number>());

    return (
        <div class="flex h-screen">
            <div class="w-1/2 overflow-y-scroll">
                <Show when={nodes()}>
                    <NodeTree nodes={nodes()} selectedId={selectedId} />
                </Show>
            </div>
            <div class="w-1/2 overflow-y-scroll">
                <Show when={selectedId.get()}>
                    <NodeData nodeId={selectedId.get()} />
                </Show>
            </div>
        </div>
    );
}
