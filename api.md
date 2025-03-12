# STILL PREPARING.. last 10.03.25
# Websocket
    - "/ws_licence_control":
        About:
            Control the licences
        Usage:
            1. wait server message "prepare hwid" [LEVEL 0]

            send:
            {
                hwid: 0,
            }

            2. wait server message "prepare hwid" [LEVEL 1]
            // we are preparing the secret key in there
            // server gives a key like:
            {
                key: 0,
            }
            // and client decrypt with aes this key

            send:
            {
                resultKey: 0,
            }

            3. wait server message "prepare licence_key" with this path

            client encyrpt sorting: (json { licence_key:0 hwid:0 } ) + aes encyrpt (with public key) + **b58** + **b64** + **b58**
            server decrypt sorting: **b58** + **b64** + **b58** + aes decrypt + json deserialize

            send:
            { 
                licence_key:0,
                hwid:0,
            }
        Limitations:
            1. Rate limit:
                3 connection in minute
                punishment:
                blacklist 10min
            2. Suspicious request 
                all time maximum 3 
                punishment:
                blacklist 180min