import axios from "axios";
import React, {useState} from "react";

export function App(): JSX.Element {
    const [name, setName] = useState("");
    const handleChange = (e: any) => {
        setName(e.target.value);
    };

    const handleClick = () => {
        axios.get("/echo/" + name)
        .then(response => {
            alert(response.data);
        });
    };

    return (
        <>
            <div>
                <h1>Hi there</h1>
                <button onClick={() => alert("big computation")}>Run Computation</button>
            </div>
            <div>
                <input type="text" onChange={handleChange} />
                <button onClick={handleClick}>Say hello!</button>
            </div>
        </>
    );
};