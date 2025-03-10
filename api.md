# STILL PREPARING.. last 10.03.25
# Websocket
    - "/ws_licence_control":
        About:
            Control the licences
        Usage:
            1. wait server message "prepare hwid"

            send:
            {
                hwid,
            }

            2. wait server message "prepare licence_key" with **b64** 
            and salt hwid (licence_key+hwid)

            send:
            {
                final_key,
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