import { createResource } from "solid-js";

const base: string = import.meta.env.VITE_SERVER || "";

export type NodeType =
    | "Root"
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
    file: string;
    file_proxy: string;
    original_id: string;
}

interface _NodeRel<DATE> extends _Node<DATE> {
    parent_id: number;
    children: number[];
}

export type Node = _Node<Date>;
export type NodeRel = _NodeRel<Date>;

export interface NodeExpanded {
    node: Node;
    data: any;
}

export interface NodeResourceReturn {
    nodes: NodeRel[];
    nodeById: Map<number, NodeRel>;
}

export function createNodes() {
    return createResource(async () => {
        const response = await fetch(base + "/nodes");

        const nodeList: _NodeRel<string>[] = await response.json();
        const nodeListParsed = nodeList.map(
            (node) =>
                ({
                    ...node,
                    created: node.created ? Date.parse(node.created) : null,
                    icon: node.icon?.startsWith("/")
                        ? base + node.icon
                        : node.icon,
                } as unknown as NodeRel)
        );

        const nodeById = new Map<number, NodeRel>();
        for (const node of nodeListParsed) {
            nodeById.set(node.id, node);
        }

        return {
            nodes: nodeListParsed,
            nodeById,
        };
    });
}

export async function getNodeData(nodeId: number): Promise<NodeExpanded> {
    const response = await fetch(`${base}/node/${nodeId}`);
    return await response.json();
}
