import NodeIcon from "./NodeIcon";
import { NodeRel } from "./server";

export default function NodeLabel(props: { node: NodeRel }) {
    return (
        <p class="h-[1lh] select-none overflow-hidden whitespace-nowrap text-ellipsis">
            <NodeIcon type={props.node.type} />
            <span class="pl-2">{props.node.title}</span>
        </p>
    )
}
