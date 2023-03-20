import Header from './components/header/header';
import MinecraftStatus from './components/minecraft_server/minecraft_server';
import { Routes, Route } from "@solidjs/router";
import { ThemeProvider, createTheme } from "@suid/material/styles";
import { CssBaseline } from '@suid/material';
import WebSocketDemo from "./components/websocket/websocket";

const darkTheme = createTheme({
    palette: {
        mode: "dark",
    }
});

export default function App(props) {
    return (
        <>
            <ThemeProvider theme={darkTheme}>
                <CssBaseline />
                <Header />
                <Routes>
                    <Route path="/" component={() => <div>home</div>}/>
                    <Route path="/minecraft_server" component={MinecraftStatus}/>
                    <Route path="/about" component={() => <div>about</div>}/>
                    <Route path="/websocket" component={WebSocketDemo}/>
                </Routes>
            </ThemeProvider>
        </>
    );
}