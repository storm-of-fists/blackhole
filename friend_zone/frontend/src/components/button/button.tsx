import { Button as MaterialButton } from "@suid/material";
import { grey } from "@suid/material/colors";
import { createSignal } from "solid-js";

export function Button(props) {
    const [count, setCount] = createSignal(0);

    function inner() {
        setCount(count() + 1);
        props.onClick();
    }

    return (
        <MaterialButton { ...props } disableRipple="true" variant="outlined">{ props.children }</MaterialButton>
    );
}