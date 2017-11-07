#pragma once
#include "ishiori.h"

namespace Setugekka {
    ref class Tuki :IShioriUnsafe
    {
    private:
        System::AppDomain^ proxy_domain;
        IShioriUnsafe^ proxy;
    public:
        Tuki();
        ~Tuki();
        void CreateProxy(System::String^ load_dir);
        virtual BOOL load(HINSTANCE hinst, HGLOBAL hGlobal_loaddir, long loaddir_len);
        virtual BOOL unload(void);
        virtual HGLOBAL request(HGLOBAL hGlobal_request, long & len);
    };
}
