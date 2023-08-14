import { useQuery } from "@tanstack/react-query";
import { TreeResponse } from "./api";
import NodeTree from "./NodeTree";
import { useState } from "react";

function App() {
    const { data, isSuccess } = useQuery({
        queryKey: ["nodes"],
        queryFn: async () => {
            const res = await fetch("/nodes");
            return (await res.json()) as TreeResponse;
        },
    });
    const [selectedId, setSelectedId] = useState<number | null>(null);

    return (
        <div className="flex h-screen">
            <div className="left-0 w-1/2 overflow-y-scroll">
                {isSuccess && (
                    <NodeTree
                        nodes={data.value}
                        selectedId={selectedId}
                        selectId={setSelectedId}
                    />
                )}
            </div>
            <div className="right-0 w-1/2 overflow-y-scroll"></div>
        </div>
    );
}

export default App;
