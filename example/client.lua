local addr = "127.0.0.1:8080"
local addr_ws = "ws://" .. addr
local addr_http = "http://" .. addr

print("Waiting on network...")
while true do
    local stat = http.checkURL(addr_http)
    if stat then
        break
    end
    sleep(1)
end

local ws = http.websocket(addr_ws, {
    ["Computer-Name"] = "Flash-Setup",
    ["Plugin-Name"] = "flash"
})
print("Connected to " .. addr_ws)

-- Write your client code there...

-- Example with example_plugin
local msg = "hello-John"
ws.send(msg)
local res = ws.receive(3)
print(res)
sleep(5)