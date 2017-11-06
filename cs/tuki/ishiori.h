#pragma once

namespace Setugekka {
    public interface class IShioriUnsafe
    {
    public:
        BOOL load(HINSTANCE hinst, HGLOBAL hGlobal_loaddir, long loaddir_len);
        BOOL unload(void);
        HGLOBAL request(HGLOBAL hGlobal_request, long& len);
    };
}
