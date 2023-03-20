import { createResource, createMemo, Index } from "solid-js";
import { type Component } from "solid-js";
import CheckCircleOutlinedIcon from "@suid/icons-material/CheckCircleOutlined";
import CancelOutlinedIcon from '@suid/icons-material/CancelOutlined';
import { Typography, Box } from "@suid/material";
// import "./minecraft_server.css"

const MINECRAFT_DATA_REFRESH = 30000; // millis
// https://mcstatus.io/docs
const MC_STATUS_URL = "https://api.mcstatus.io/v2/status/java";
const MINECRAFT_PORT = 25565;
const DOMAIN_NAME = "bean-bag-zone.com"

const MinecraftStatus: Component = (props) => {
    async function fetchMinecraftStatus() {
        const response = await (await fetch(`${MC_STATUS_URL}/${DOMAIN_NAME}:${MINECRAFT_PORT}`)).json();
        return response;
    }
    const [minecraftStatusResource, { refetch }] = createResource(fetchMinecraftStatus)

    setInterval(() => {
        refetch();
    }, MINECRAFT_DATA_REFRESH);

    const minecraftStatus = createMemo(() => {
        const data = minecraftStatusResource();

        if (data === undefined) {
            return <div>The MC Status API seems to be down right now. Cannot retrieve data :/</div>
        }

        const server_name = data.motd.clean;
        const host_name = data.host;
        const host_port = data.port;

        const server_version = data.version.name_clean;
        const server_online = data.online;

        const player_count = data.players.online;
        const players = data.players.list;

        const last_retrieval = new Date(data.retrieved_at);

        let players_list = <></>

        if (players.length > 0) {
            players_list = 
            <ul>
                <Index each={players}>{(player) => <li>{player().name_clean}</li>}</Index>
            </ul>
        }

        return <>
            <Box 
                sx = {{
                    border: 2,
                    borderColor: "blue"
                }}
            >
                <Typography>{ server_name } @ { host_name }:{ host_port } </Typography>
                <Typography>Server Version: { server_version }</Typography>
                <Typography>Server Online: { server_online ? 
                    <CheckCircleOutlinedIcon sx = {{ color: "green" }}/> : 
                    <CancelOutlinedIcon sx = {{ color: "red" }}/> }
                </Typography>
                <Typography>Current Players: { player_count }</Typography>
                { players_list }
                <span>
                    <Typography>Data last retrieved at: { last_retrieval }</Typography>
                </span>
            </Box>
        </>
    })

    return <>
        { minecraftStatus }
        
    </>
}

export default MinecraftStatus;