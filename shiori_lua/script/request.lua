--[[
SHIORI requestを処理し、responseを返します。
]]

local binser = require "binser"
local res = {}
local save = {}

local function load(args)
    res.load_dir = args.load_dir


end

local function unload()
end

local request = coroutine.wrap(
    function(req)
        load(req)
        while req do
            local res = "res"
            req = coroutine.yield(res)
        end
        unload()
    end
)

return request