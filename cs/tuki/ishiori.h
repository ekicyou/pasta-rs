#pragma once

namespace Setugekka {
    public interface class IShioriUnsafe
    {
    public:
        BOOL load(void* hinst, void* hGlobal_loaddir, long loaddir_len);
        BOOL unload(void);
        void* request(void* hGlobal_request, long& len);
    };
}
