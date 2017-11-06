#include "stdafx.h"
#include "tuki.h"
#include "assembly_utils.h"

using namespace System;
using namespace System::Text;
using namespace System::Runtime::InteropServices;

Setugekka::Tuki::Tuki() {
}

Setugekka::Tuki::~Tuki() {
}

BOOL Setugekka::Tuki::load(HINSTANCE hinst, HGLOBAL hGlobal_loaddir, long loaddir_len) {
    // loaddirを解決
    auto loaddir = Marshal::PtrToStringAnsi(IntPtr((void*)hGlobal_loaddir), loaddir_len);
    auto bin_dir = System::IO::Path::Combine(loaddir, "bin");
    auto tmp_dir = System::IO::Path::Combine(loaddir, "temp");

    // 自分のディレクトリをパス解決に含める
    auto di(AssemblyUtil::AddCurrentAssemblyResolvePath(loaddir));

    // アプリケーションドメインの作成
    auto info = gcnew AppDomainSetup();
    info->ApplicationBase = loaddir;
    info->PrivateBinPath = bin_dir;
    info->DynamicBase = tmp_dir;
    info->ShadowCopyFiles = "true";
    auto name = gcnew StringBuilder();
    name->Append("TukiProxy_");
    name->Append(Guid::NewGuid().ToString());
    proxy_domain = AppDomain::CreateDomain(name->ToString(), nullptr, info);

    // プロキシを読み込む

    return FALSE;
}

BOOL Setugekka::Tuki::unload(void)
{
    if (proxy == nullptr) return FALSE;
    delete proxy;
    delete proxy_domain;
    proxy = nullptr;
    proxy_domain = nullptr;
    return TRUE;
}

HGLOBAL Setugekka::Tuki::request(HGLOBAL hGlobal_request, long & len)
{
    return HGLOBAL();
}