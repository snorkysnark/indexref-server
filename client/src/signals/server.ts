import { createResource } from "solid-js";

const base: string = import.meta.env.VITE_SERVER || "";

export type NodeType =
    | "Folder"
    | "Telegram"
    | "SingleFileZ"
    | "Scrapbook"
    | "OneTab"
    | "Zotero";

interface _Node<DATE> {
    id: number;
    type: NodeType;
    subtype: string;
    title: string;
    url: string;
    icon: string;
    created: DATE;
    modified: DATE;
    file: string;
    original_id: string;
    parent_id: number;
}

interface _NodeWithChildren<DATE> extends _Node<DATE> {
    children: number[];
}

interface _NodeTree<DATE> {
    root: number[];
    nodes: _NodeWithChildren<DATE>[]
}

export type Node = _Node<Date>;
export type NodeWithChildren = _NodeWithChildren<Date>;
export type NodeTree = _NodeTree<Date>;

export interface NodeResourceReturn {
    root: number[];
    nodes: NodeWithChildren[];
    nodeById: Map<number, NodeWithChildren>;
}

export function createNodes() {
    return createResource<NodeResourceReturn>(async () => {
        const response = await fetch(base + "/nodes");

        const nodeTree: _NodeTree<string> = await response.json();
        const nodeListParsed = nodeTree.nodes.map(
            (node) =>
            ({
                ...node,
                created: node.created ? Date.parse(node.created) : null,
                modified: node.modified ? Date.parse(node.modified) : null,
                icon: node.icon?.startsWith("/")
                    ? base + node.icon
                    : node.icon,
            } as unknown as NodeWithChildren)
        );

        const nodeById = new Map<number, NodeWithChildren>();
        for (const node of nodeListParsed) {
            nodeById.set(node.id, node);
        }

        return {
            root: nodeTree.root,
            nodes: nodeListParsed,
            nodeById,
        };
    });
}
