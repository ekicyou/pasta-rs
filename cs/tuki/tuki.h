#pragma once
#include "ishiori.h"

namespace Setugekka {
    namespace Tuki {
        ref class Tuki : Setugekka::IShioriUnsafe
        {
        private:
            System::AppDomain^ proxy_domain;
            Setugekka::IShioriUnsafe^ proxy;
        public:
            Tuki();
            ~Tuki();
            void CreateProxy(System::String^ load_dir);
            virtual BOOL load(void* hinst, void* hGlobal_loaddir, long loaddir_len);
            virtual BOOL unload(void);
            virtual void* request(void* hGlobal_request, long & len);
        };
    }
}
