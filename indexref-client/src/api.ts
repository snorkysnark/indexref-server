// TODO: maybe remove the status field, since the status code already signifies success/error
export interface TreeResponse {
    status: "ok";
    value: Node[];
}

export interface Node {
    id: number;
    file_id: number;
    file_path: string;
    node_type: string;
    title: string;
    subtype: string;
    url: string;
    icon: string;
    created: string;
    modified: string;
    original_id: string;
    parent_id: number;
    children: Node[];
}
