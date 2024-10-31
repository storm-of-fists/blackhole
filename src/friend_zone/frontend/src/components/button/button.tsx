import { Button as MaterialButton } from "@suid/material";
import { grey } from "@suid/material/colors";
import { createSignal } from "solid-js";

export function Button(props) {
    return (
        <MaterialButton { ...props } disableRipple="true" variant="outlined">{ props.children }</MaterialButton>
    );
}