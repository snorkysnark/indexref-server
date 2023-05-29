import { createSignal, onCleanup, Show } from "solid-js";
import NodeData from "./NodeData";
import NodeTree from "./NodeTree";
import { getSet } from "./signals/getset";
import { createNodes } from "./signals/server";
import { selectNextNode, selectPrevNode } from "./treeNavigation";

export default function App() {
    const [nodes] = createNodes();
    const selectedId = getSet(createSignal<number>());

    function onKeyDown(event: KeyboardEvent) {
        if (!nodes()) return;

        switch (event.key) {
            case "ArrowUp":
                event.preventDefault();
                selectPrevNode(nodes(), selectedId);
                break;
            case "ArrowDown":
                event.preventDefault();
                selectNextNode(nodes(), selectedId);
                break;
        }
    }
    window.addEventListener("keydown", onKeyDown);
    onCleanup(() => window.removeEventListener("keydown", onKeyDown));

    return (
        <div class="flex h-screen">
            <div class="w-1/2 overflow-y-scroll pb-4">
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
