// Here, i am testing the rate limit for websocket

require('dotenv').config({ path: '../.env' });
const WebSocket = require('ws');

const address = process.env.IP + ":" + process.env.PORT;
const websocket_url = 'ws://' + address + "/ws_licence_control";

console.log("WebSocket URL:", websocket_url);

function openSocket() {
    const socket = new WebSocket(websocket_url);
    socket.on('open', () => {
        console.log("Connection open:", socket.url);
    });
    socket.on('error', (err) => {
        console.error("Connection error:", err);
    });
}

// Initially, opens 1 connection and increases every time
let connectionsPerSecond = 1;
setInterval(() => {
    console.log(`Opening ${connectionsPerSecond} connections`);
    for (let i = 0; i < connectionsPerSecond; i++) {
        openSocket();
    }
    connectionsPerSecond++; // Increase every second
}, 1000);
