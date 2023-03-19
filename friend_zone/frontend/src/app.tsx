import { createSignal } from "solid-js";
import { Button } from './components/button/button';
import Header from './components/header/header';
import { Routes, Route } from "@solidjs/router";

export default function App(props) {
    let userState = JSON.parse(window.localStorage.getItem("userState")) || 0;
    const [userStateSignal, setUserState] = createSignal(userState);

    return (
        <>
            <Header />
            <Routes>
                <Route path="/" component={() => <div>home</div>}/>
                <Route path="/users" component={() => <div>users</div>}/>
                <Route path="/about" component={() => <div>about</div>}/>
            </Routes>
            {/* <Button onClick={() => setUserState(userStateSignal() + 1)}>Heasdasdahee</ Button>
            <div>User State: { userStateSignal() }</div>
            <Button onClick={() => window.localStorage.setItem("userState", JSON.stringify(userStateSignal()))}/> */}
        </>
    );
}