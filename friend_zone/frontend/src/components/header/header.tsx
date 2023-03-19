import { 
    AppBar,
    Box,
    IconButton,
    Toolbar,
    Typography
} from "@suid/material";
import { Routes, Route, A } from "@solidjs/router"
// import { createSignal } from "solid-js";

export default function Header() {
    return <>
        <Box>
            <AppBar position="static">
                <Toolbar>
                    <nav>
                        <Typography variant="h6" component="span"><A href = "/">Friendzone</A></Typography>
                        <Typography variant="h6" component="span"><A href = "/about"> About</A></Typography>
                        <Typography variant="h6" component="span"><A href = "/users"> Users</A></Typography>
                    </nav>
                </Toolbar>
            </AppBar>
        </Box> 
    </>
}