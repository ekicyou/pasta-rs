#pragma once

namespace Setugekka {
    ref class Tuki
    {
    public:
        Tuki();
        ~Tuki();
        BOOL load(HINSTANCE hinst, HGLOBAL hGlobal_loaddir, long loaddir_len);
        BOOL  unload(void);
        HGLOBAL request(HGLOBAL hGlobal_request, long& len);
    };
}
