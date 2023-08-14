import NodeIcon from "./NodeIcon";
import { Node } from "./api";
import treeCss from "./tree.module.css";
import classNames from "classnames";

function NodeLabel(props: {
    node: Node;
    selected?: boolean;
    onClick?: () => void;
}) {
    let classes = classNames(
        "overflow-x-hidden whitespace-nowrap text-ellipsis cursor-pointer select-none",
        {
            "bg-blue-300": props.selected,
            "bg-white hover:bg-blue-200": !props.selected,
        }
    );

    return (
        <p className={classes} onClick={props.onClick}>
            <NodeIcon node={props.node} />
            <span className="h-[1lh] pl-2">{props.node.title}</span>
        </p>
    );
}

function NodeTree(props: {
    nodes: Node[];
    selectedId: number | null;
    selectId: (id: number | null) => void;
}) {
    return (
        <ul className={treeCss.dottedTree}>
            {props.nodes.map((node) => (
                <li key={node.id} className={treeCss.dottedTree}>
                    <NodeLabel
                        node={node}
                        selected={props.selectedId == node.id}
                        onClick={() => props.selectId(node.id)}
                    />
                    {node.children.length > 0 && (
                        <NodeTree {...props} nodes={node.children} />
                    )}
                </li>
            ))}
        </ul>
    );
}

export default NodeTree;
