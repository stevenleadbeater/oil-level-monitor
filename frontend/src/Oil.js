import React from "react";

export default () => {
    const connection = new WebSocket(`ws://192.168.1.245:8120/ws/`);
    connection.onmessage = evt => {
        console.log(`Got message ${evt.data}`);
    };
    setInterval(_ => {
        connection.send(Math.random())
    }, 4000);
    return (
        <div>Oil</div>
    );
}