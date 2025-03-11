//here i am testing the websocket api with nodejs because its very fast to try

require('dotenv').config({ path: '../.env' });

const address = process.env.IP + ":" + process.env.PORT
const websocket_url = 'ws://' + address + "/ws_licence_control"

console.log(websocket_url)

const socket = new WebSocket(websocket_url);

socket.onopen = function(event) {
    console.log("Connection open")
};

socket.onmessage = function(event) {
    console.log("An message came: "+ event.data)
};

socket.onclose = function(event) {
    console.log("Disconnected")
};

function sendMessage(message) {
  socket.send(message);
}
