import { createEffect, createSignal, onCleanup, Show } from "solid-js";
import NodeTree from "./NodeTree";
import { ImSearch } from 'solid-icons/im'
import { getSet } from "./signals/getset";
import { createNodes } from "./signals/server";
import { selectNextNode, selectPrevNode } from "./treeNavigation";
import JsonView from "./JsonView";

export default function App() {
    const [nodes] = createNodes();
    const selectedId = getSet(createSignal<number>());

    createEffect(() => console.log(nodes()));

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
            <div class="w-1/2 overflow-hidden">
                <div class="h-10 w-full p-1 flex items-center shadow-md">
                    <ImSearch class="mx-1" />
                    <input class="bg-transparent border-2 border-gray-200 shadow-inner w-full h-full" />
                </div>
                <div class="overflow-y-scroll pb-4 h-full">
                    {
                        /*<Show when={nodes()}>
                            <NodeTree nodes={nodes()} selectedId={selectedId} />
                        </Show>*/
                    }
                </div>
            </div>
            <div class="w-1/2 overflow-y-scroll">
                <Show when={selectedId.get()}>
                    <JsonView value={nodes().nodeById.get(selectedId.get())} />
                </Show>
            </div>
        </div>
    );
}
