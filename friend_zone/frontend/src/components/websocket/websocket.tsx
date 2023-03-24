import { createSignal, type Component, Index, onCleanup } from "solid-js";
import { Button } from "../button/button";
import { TextField } from "@suid/material"
import ReconnectingWebSocket from "reconnecting-websocket";

const WebSocketDemo: Component = (props) => {
    const [ messageToSend, setMessageToSend ] = createSignal("")
    const [ messages, setMessages ] = createSignal([]);
    const [ warning, setWarning ] = createSignal();
    const [ socketClosed, setSocketClosed ] = createSignal(true);

    let socket = new ReconnectingWebSocket("ws://localhost:8888/ws");

    socket.addEventListener("message", (event) => {
        const new_messages = [...messages()];
        new_messages.push(event.data);
        if (new_messages.length > 10) {
            new_messages.shift()
        }
        setMessages(new_messages);
    });

    socket.addEventListener("close", () => {
        setWarning("connection closed, attempting to reconnect!")
        setSocketClosed(true);
    });

    socket.addEventListener("open", () => {
        setWarning(<></>)
        setSocketClosed(false);
    });

    onCleanup(() => {socket.close()})

    return <>
        <TextField variant="outlined" onChange={(_, input) => {
            setMessageToSend(input)
        }}/>
        <Button disabled={socketClosed()} onClick={() => {
            socket.send(JSON.stringify({ "subscribe": messageToSend() }));
        }}>subscribe</Button>
        <Button disabled={socketClosed()} onClick={() => {
            socket.send(JSON.stringify({ "unsubscribe": messageToSend() }));
        }}>unsubscribe</Button>
        <div>{ warning() }</div>
        <ul>
            <Index each={messages()}>
                {(message) => <li>{message()}</li>}
            </Index>
        </ul>
    </>;

    
}

export default WebSocketDemo;