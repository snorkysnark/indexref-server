import { Node } from "./api";

import scrapbookIcon from "./icons/scrapbookx_32.png";
import telegramIcon from "./icons/t_logo_sprite.svg";
import singleFileZIcon from "./icons/singlefilez_128.png";
import onetabIcon from "./icons/onetab-icon128.png";
import zoteroIcon from "./icons/Zotero.png";

function getDefaultIcon(node: Node) {
    switch (node.node_type) {
        case "Scrapbook":
            return scrapbookIcon;
        case "Telegram":
            return telegramIcon;
        case "SingleFileZ":
            return singleFileZIcon;
        case "OneTab":
            return onetabIcon;
        case "Zotero":
            return zoteroIcon;
    }
}

export default function NodeIcon({ node }: { node: Node }) {
    return (
        <img
            className="h-[1lh] inline-block"
            src={getDefaultIcon(node)}
            alt={node.node_type}
        />
    );
}
