import { 
    AppBar,
    Button,
    ButtonGroup,
    Toolbar,
    Typography,
} from "@suid/material";
import { type Component } from "solid-js";
import { A } from "@solidjs/router"
// import { createSignal } from "solid-js";

const HeaderMember: Component = (props) => {
    const name = props.name;
    const route = props.route ? `${props.route}` : `/${props.name}`;
    return <>
        <Button><Typography variant="body1" component="span">
            <A 
            style={{
                "text-decoration": "none",
                "color": "inherit"
            }}
            href = {route}>{name}</A>
        </Typography></Button>
    </>
}

const Header: Component = () => {
    return <>
        <AppBar 
            position="static">
            <Toolbar style={{
                "min-height": 0,
                "padding": 0
            }}>
                <nav style={{
                    "min-height": 0,
                    "padding": 0
                }}>
                    <HeaderMember name="home" route="/"/>
                    <HeaderMember name="minecraft server" route="/minecraft_server"/>
                    <HeaderMember name="users"/>
                </nav>
            </Toolbar>
        </AppBar>
    </>
}

export default Header;