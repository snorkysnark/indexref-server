import { createResource } from "solid-js";

const base: string = import.meta.env.VITE_SERVER || "";

export type NodeType =
    | "Root"
    | "Folder"
    | "Telegram"
    | "SingleFileZ"
    | "Scrapbook";

interface _Node<DATE> {
    id: number;
    type: NodeType;
    title: string;
    url: string;
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
