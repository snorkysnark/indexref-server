import { GetSet } from "./signals/getset";
import { NodeRel, NodeResourceReturn } from "./signals/server";

function nextNodeIdInTree(tree: NodeResourceReturn, current: NodeRel) {
    if (current.children.length > 0) {
        return current.children[0];
    }

    while (current.parent_id != null) {
        const parent = tree.nodeById.get(current.parent_id);
        const nextIndex =
            parent.children.findIndex((id) => id === current.id) + 1;

        if (nextIndex < parent.children.length) {
            return parent.children[nextIndex];
        } else {
            current = parent;
        }
    }

    return null;
}

function prevNodeIdInTree(tree: NodeResourceReturn, current: NodeRel) {
    if (current.parent_id == null) return null;

    const parent = tree.nodeById.get(current.parent_id);
    const prevIndex = parent.children.findIndex((id) => id === current.id) - 1;

    if (prevIndex >= 0) {
        let childAbove = tree.nodeById.get(parent.children[prevIndex]);

        while (childAbove.children.length > 0) {
            childAbove = tree.nodeById.get(
                childAbove.children[childAbove.children.length - 1]
            );
        }

        return childAbove.id;
    } else {
        return parent.id;
    }
}

function selectRelative(
    tree: NodeResourceReturn,
    selectedId: GetSet<number>,
    algorithm: (tree: NodeResourceReturn, current: NodeRel) => number
) {
    const selectedNode =
        selectedId.get() != null ? tree.nodeById.get(selectedId.get()) : null;
    if (selectedNode) {
        const targetNodeId = algorithm(tree, selectedNode);
        if (targetNodeId != null) {
            selectedId.set(targetNodeId);
        }
    }
}

export function selectNextNode(
    tree: NodeResourceReturn,
    selectedId: GetSet<number>,
) {
    selectRelative(tree, selectedId, nextNodeIdInTree);
}

export function selectPrevNode(
    tree: NodeResourceReturn,
    selectedId: GetSet<number>,
) {
    selectRelative(tree, selectedId, prevNodeIdInTree);
}
