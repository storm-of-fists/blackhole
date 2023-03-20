import { createSignal, type Component, Index } from "solid-js";
import { Button } from "../button/button";
import { TextField } from "@suid/material"
import ReconnectingWebSocket from "reconnecting-websocket";

const WebSocketDemo: Component = (props) => {
    const [ messageToSend, setMessageToSend ] = createSignal("")
    const [ messages, setMessages ] = createSignal([]);
    const [ warning, setWarning ] = createSignal();
    const [ socketClosed, setSocketClosed ] = createSignal(false);

    let socket = new ReconnectingWebSocket("ws://localhost:8001");

    socket.addEventListener("message", (event) => {
        const new_messages = [...messages()];
        new_messages.push(event.data);
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

    return <>
        <TextField variant="outlined" onChange={(_, input) => {
            setMessageToSend(input)
        }}/>
        <Button disabled={socketClosed()} onClick={() => {
            console.log(messageToSend())
            socket.send(messageToSend());
        }}>Send Message</Button>
        <div>{ warning() }</div>
        <ul>
            <Index each={messages()}>
                {(message) => <li>{message()}</li>}
            </Index>
        </ul>
    </>;
}

export default WebSocketDemo;