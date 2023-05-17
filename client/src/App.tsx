import { Show } from "solid-js";
import NodeTree from "./NodeTree";
import { createNodes } from "./server";

export default function App() {
    const [nodes] = createNodes();

    return (
        <div>
            <Show when={nodes()}>
                <NodeTree nodes={nodes()} />
            </Show>
        </div>
    )
}
