import { NodeRel } from "./signals/server";
import scrapbookIcon from "./icons/scrapbookx_32.png";
import telegramIcon from "./icons/t_logo_sprite.svg";
import singleFileZIcon from "./icons/singlefilez_128.png";
import onetabIcon from "./icons/onetab-icon128.png";
import zoteroIcon from "./icons/Zotero.png";

function getDefaultIcon(node: NodeRel) {
    switch (node.type) {
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

export default function NodeIcon(props: { node: NodeRel }) {
    return (
        <img
            class="h-[1lh] inline-block"
            src={props.node.icon || getDefaultIcon(props.node)}
            alt={props.node.type}
        />
    );
}
