import { Match, Switch } from "solid-js";
import { NodeType } from "./signals/server";
import { FaSolidFolder } from "solid-icons/fa";
import scrapbookIcon from "./icons/scrapbookx_32.png";
import telegramIcon from "./icons/t_logo_sprite.svg";
import singleFileZIcon from "./icons/singlefilez_128.png";
import onetabIcon from "./icons/onetab-icon128.png";

export default function NodeIcon(props: { type: NodeType }) {
    return (
        <Switch>
            <Match when={props.type === "Folder"}>
                <FaSolidFolder class="inline-block fill-yellow-300 scale-125" />
            </Match>
            <Match when={props.type === "Scrapbook"}>
                <img
                    class="h-[1lh] inline-block"
                    src={scrapbookIcon}
                    alt="Scrapbook"
                />
            </Match>
            <Match when={props.type === "Scrapbook"}>
                <img
                    class="h-[1lh] inline-block"
                    src={scrapbookIcon}
                    alt="Scrapbook"
                />
            </Match>
            <Match when={props.type === "Telegram"}>
                <img
                    class="h-[0.9lh] inline-block"
                    src={telegramIcon}
                    alt="Telegram"
                />
            </Match>
            <Match when={props.type === "SingleFileZ"}>
                <img
                    class="h-[1lh] inline-block"
                    src={singleFileZIcon}
                    alt="SingleFileZ"
                />
            </Match>
            <Match when={props.type === "OneTab"}>
                <img
                    class="h-[1lh] inline-block"
                    src={onetabIcon}
                    alt="OneTab"
                />
            </Match>
        </Switch>
    );
}
