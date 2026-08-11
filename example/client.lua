local pluginURL = "ws://127.0.0.1:8080"

print("Connecting to", pluginURL)
while true do
    local (_, character) = os.pullEvent("char")
    if character = 'q' then
        return
    end

    local rc = http.checkURL(pluginURL)
    if rc then
        print("Found Server!")
        break
    end
    sleep(5)
end

local ws = http.websocket(pluginURL, {
    ["Computer-Name"] = "CraftOS-Example"
    ["Plugin-Name"] = "example"
})
print("Connected to Server!")

