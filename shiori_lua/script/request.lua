--[[
SHIORI requestを処理し、responseを返します。
]]




local request = coroutine.wrap(
    function(req)
        local function load()
        end
        local function unload()
        end

        load()
        while req do
            local res = "res"
            req = coroutine.yield(res)
        end
        unload()
    end
)

return request