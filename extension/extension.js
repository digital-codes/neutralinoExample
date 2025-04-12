const fs = require('fs');
const process = require('process');
const WS = require('websocket').w3cwebsocket;
const { v4: uuidv4 } = require('uuid');
const chalk = require('chalk');

// Obtain required params to start a WS connection from stdIn.
const processInput = JSON.parse(fs.readFileSync(process.stdin.fd, 'utf-8'));
const NL_PORT = processInput.nlPort;
const NL_TOKEN = processInput.nlToken;
const NL_CTOKEN = processInput.nlConnectToken;
const NL_EXTID = processInput.nlExtensionId;
console.log(`NL_PORT: ${NL_PORT}`);
console.log(`NL_TOKEN: ${NL_TOKEN}`);
console.log(`NL_CTOKEN: ${NL_CTOKEN}`);
console.log(`NL_EXTID: ${NL_EXTID}`);

const url = `ws://localhost:${NL_PORT}?extensionId=${NL_EXTID}&connectToken=${NL_CTOKEN}`;
console.log(`Connecting to ${url}`);

const client = new WS(url)

client.onerror = () => log("Connection error!", "ERROR");
client.onopen = () => log("Connected");
client.onclose = () => process.exit();

client.onmessage = (e) => {
  const { event, data } = JSON.parse(e.data);

  if (event === "eventToExtension") {
    log(data);

    client.send(
      JSON.stringify({
        id: uuidv4(),
        method: "app.broadcast",
        accessToken: NL_TOKEN,
        data: { event: "eventFromExtension", data: "Hello app!" },
      })
    );
  }
};

function log(message, type = "INFO") {
  const logLine = `[${NL_EXTID}]: ${chalk[
    type === "INFO" ? "green" : "red"
  ](type)} ${message}`;
  console[type === "INFO" ? "log" : "error"](logLine);
}
