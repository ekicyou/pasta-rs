--[[
SHIORI requestを処理し、responseを返します。
]]

local request = coroutine.wrap(
    function(req)
        -- load
        local load_dir = req.load_dir

        -- request
        while req do
            local res = "res"
            req = coroutine.yield(res)
        end

        -- unload
    end
)

return request